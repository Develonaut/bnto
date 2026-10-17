// Windows Job Objects — what `ProcessRegistry` kills on Windows. Every process
// a spawned command starts joins the command's job, so `TerminateJobObject`
// reaches a grandchild whose parent already exited, which `taskkill /T`
// (walking parent links at kill time) misses. Holding the job, not a PID,
// also means a recycled PID can never be killed by mistake.

use std::io;
use std::os::windows::io::{AsRawHandle, FromRawHandle, OwnedHandle};
use std::os::windows::process::CommandExt;
use std::process::{Child, Command};

use windows_sys::Win32::Foundation::{HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, TH32CS_SNAPTHREAD, THREADENTRY32, Thread32First, Thread32Next,
};
use windows_sys::Win32::System::JobObjects::{
    AssignProcessToJobObject, CreateJobObjectW, JOB_OBJECT_LIMIT,
    JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE, JOBOBJECT_EXTENDED_LIMIT_INFORMATION,
    JobObjectExtendedLimitInformation, SetInformationJobObject, TerminateJobObject,
};
use windows_sys::Win32::System::Threading::{
    CREATE_SUSPENDED, OpenThread, ResumeThread, THREAD_SUSPEND_RESUME,
};
use windows_sys::core::BOOL;

/// Exit code of every process `terminate` kills (the code `taskkill /F` gave).
const KILLED_EXIT_CODE: u32 = 1;

/// `ResumeThread`'s failure return, `(DWORD)-1`.
const RESUME_FAILED: u32 = u32::MAX;

/// An owned Job Object with `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`.
///
/// The three ways a job ends keep Windows in step with unix process groups:
/// - cancel calls `terminate`, which kills the whole tree;
/// - a command that finishes on its own is `release`d, which lifts the flag
///   before closing, so anything it left running in the background lives on;
/// - if bnto itself dies, the OS closes the handle with the flag still set
///   and kills the tree, so no command outlives a crashed bnto.
pub struct JobObject(OwnedHandle);

impl JobObject {
    /// Create an empty job whose processes die when it is dropped.
    pub fn new() -> io::Result<Self> {
        // SAFETY: null attributes and name are documented inputs (default
        // security descriptor, unnamed job); a failure returns null.
        let handle = unsafe { CreateJobObjectW(std::ptr::null(), std::ptr::null()) };
        let job = Self(owned(handle, std::ptr::null_mut())?);
        job.set_limits(JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE)?;
        Ok(job)
    }

    /// Close the job without killing what is still in it: lift
    /// `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`, then drop the handle. If the
    /// flag can't be lifted, closing kills the leftovers as a plain drop would.
    pub fn release(self) {
        let _ = self.set_limits(0);
    }

    /// Replace the job's limit flags with `flags`.
    fn set_limits(&self, flags: JOB_OBJECT_LIMIT) -> io::Result<()> {
        let mut info = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
        info.BasicLimitInformation.LimitFlags = flags;
        let len = size_of_val(&info) as u32;
        let class = JobObjectExtendedLimitInformation;
        // SAFETY: `info` is the struct this information class reads, `len`
        // is its size, and it outlives the call.
        check(unsafe { SetInformationJobObject(self.raw(), class, (&raw const info).cast(), len) })
    }

    /// Put `child`, and every process it starts from then on, in this job.
    fn assign(&self, child: &Child) -> io::Result<()> {
        // SAFETY: both handles are open: the job is owned here and `child`
        // owns its process handle (opened with full access by CreateProcess).
        check(unsafe { AssignProcessToJobObject(self.raw(), child.as_raw_handle()) })
    }

    /// Kill every process in the job. Once they have all exited, it is a no-op.
    pub fn terminate(&self) {
        // SAFETY: the job handle is owned and open for the life of `self`.
        unsafe { TerminateJobObject(self.raw(), KILLED_EXIT_CODE) };
    }

    fn raw(&self) -> HANDLE {
        self.0.as_raw_handle()
    }
}

/// Spawn `command` inside a fresh job and return the child with its job.
///
/// The child starts suspended and runs only once it is in the job. Assigning
/// right after a normal spawn would leave a window in which the running child
/// could start a grandchild outside the job, beyond `terminate`'s reach. std
/// keeps no handle to the primary thread, so `resume_threads` finds it in a
/// thread snapshot instead.
pub fn spawn_in_job(command: &mut Command) -> io::Result<(Child, JobObject)> {
    let mut child = command.creation_flags(CREATE_SUSPENDED).spawn()?;
    match adopt(&child) {
        Ok(job) => Ok((child, job)),
        Err(e) => {
            // A child left suspended would never exit; reap it before failing.
            let _ = child.kill();
            let _ = child.wait();
            Err(e)
        }
    }
}

/// Assign the suspended `child` to a new job, then let it run.
fn adopt(child: &Child) -> io::Result<JobObject> {
    let job = JobObject::new()?;
    job.assign(child)?;
    resume_threads(child.id())?;
    Ok(job)
}

/// Resume each thread of the suspended process `pid` (its primary thread).
fn resume_threads(pid: u32) -> io::Result<()> {
    let threads = threads_of(pid)?;
    if threads.is_empty() {
        return Err(io::Error::other(format!("no thread to resume in {pid}")));
    }
    threads.into_iter().try_for_each(resume_thread)
}

/// IDs of the threads process `pid` owns, from a system-wide thread snapshot.
fn threads_of(pid: u32) -> io::Result<Vec<u32>> {
    // SAFETY: takes no pointers; the process id is ignored for thread
    // snapshots, and a failure returns INVALID_HANDLE_VALUE.
    let raw = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0) };
    let snapshot = owned(raw, INVALID_HANDLE_VALUE)?;
    let mut entry = THREADENTRY32 {
        dwSize: size_of::<THREADENTRY32>() as u32,
        ..Default::default()
    };
    let mut threads = Vec::new();
    // SAFETY: the snapshot is open and `entry` has `dwSize` set, as both
    // calls require; each call only writes into `entry`.
    let mut more = unsafe { Thread32First(snapshot.as_raw_handle(), &mut entry) } != 0;
    while more {
        if entry.th32OwnerProcessID == pid {
            threads.push(entry.th32ThreadID);
        }
        // SAFETY: as for `Thread32First` above.
        more = unsafe { Thread32Next(snapshot.as_raw_handle(), &mut entry) } != 0;
    }
    Ok(threads)
}

/// Resume the suspended thread `tid`.
fn resume_thread(tid: u32) -> io::Result<()> {
    // SAFETY: takes no pointers; a failure returns null, checked by `owned`.
    let raw = unsafe { OpenThread(THREAD_SUSPEND_RESUME, 0, tid) };
    let thread = owned(raw, std::ptr::null_mut())?;
    // SAFETY: `thread` is open with THREAD_SUSPEND_RESUME access.
    if unsafe { ResumeThread(thread.as_raw_handle()) } == RESUME_FAILED {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

/// Own the handle a Win32 call returned, or report the OS error when it
/// returned that call's failure value.
fn owned(handle: HANDLE, failed: HANDLE) -> io::Result<OwnedHandle> {
    if handle == failed {
        return Err(io::Error::last_os_error());
    }
    // SAFETY: the call just opened this handle and nothing else owns it.
    Ok(unsafe { OwnedHandle::from_raw_handle(handle) })
}

/// The OS error when a `BOOL`-returning Win32 call failed.
fn check(ok: BOOL) -> io::Result<()> {
    if ok != 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawn_in_job_runs_the_child_to_completion() {
        let (mut child, _job) = spawn_in_job(Command::new("cmd").args(["/c", "exit 3"])).unwrap();
        assert_eq!(
            child.wait().unwrap().code(),
            Some(3),
            "child must be resumed"
        );
    }

    #[test]
    fn terminate_kills_the_jobs_processes() {
        let mut command = Command::new("ping");
        command
            .args(["-n", "30", "127.0.0.1"])
            .stdout(std::process::Stdio::null());
        let (mut child, job) = spawn_in_job(&mut command).unwrap();
        job.terminate();
        assert_eq!(child.wait().unwrap().code(), Some(KILLED_EXIT_CODE as i32));
    }

    #[test]
    fn dropping_the_job_kills_its_processes() {
        let mut command = Command::new("ping");
        command
            .args(["-n", "30", "127.0.0.1"])
            .stdout(std::process::Stdio::null());
        let (mut child, job) = spawn_in_job(&mut command).unwrap();
        let start = std::time::Instant::now();
        drop(job);
        child.wait().unwrap();
        // ping alone would run ~30s.
        assert!(start.elapsed().as_secs() < 10, "kill-on-close must kill it");
    }

    #[test]
    fn releasing_the_job_leaves_its_processes_running() {
        let mut command = Command::new("ping");
        command
            .args(["-n", "30", "127.0.0.1"])
            .stdout(std::process::Stdio::null());
        let (mut child, job) = spawn_in_job(&mut command).unwrap();
        job.release();
        std::thread::sleep(std::time::Duration::from_millis(500));
        let still_running = child.try_wait().unwrap().is_none();
        let _ = child.kill();
        let _ = child.wait();
        assert!(still_running, "a released job must not kill its processes");
    }
}
