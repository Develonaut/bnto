// Process-group registry — tracks external commands spawned by NativeContext
// so cancel/quit/signal paths can terminate the entire child process tree
// (e.g. patreon-dl → patreon-dl-vimeo → yt-dlp), not just the direct child.
// Unix signals the process group; Windows terminates each child's Job Object.
//
// A process-wide default registry exists (rather than plumbing an instance
// through every layer) because OS signal handlers only have 'static access.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, LazyLock, Mutex};

#[cfg(windows)]
use crate::job_object::JobObject;

/// What the registry keeps per child besides its PID: nothing on unix, where
/// the PID names the process group; on Windows, the job holding its tree.
#[cfg(unix)]
type Tree = ();
#[cfg(windows)]
type Tree = JobObject;

/// Tracks the process-group IDs of live spawned commands.
///
/// Children are spawned with `process_group(0)`, so each child's PID is also
/// its process-group ID and killing `-pgid` reaps the whole tree. Windows has
/// no process groups, so each child is spawned into its own Job Object, owned
/// here under the child's PID until it is deregistered.
#[derive(Default)]
pub struct ProcessRegistry {
    groups: Mutex<HashMap<u32, Tree>>,
    /// Children bnto stopped, until their owner asks. A stopped command
    /// fails whatever its exit code: a shell whose child died first can exit
    /// 0 from a bare `wait`, and a cancelled step must not count as done.
    stopped: Mutex<HashSet<u32>>,
}

impl ProcessRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a live process group (the child's PID under `process_group(0)`).
    #[cfg(unix)]
    pub fn register(&self, pgid: u32) {
        self.groups.lock().unwrap().insert(pgid, ());
    }

    /// Record a live child and take ownership of the job holding its tree.
    #[cfg(windows)]
    pub fn register(&self, pid: u32, job: JobObject) {
        self.groups.lock().unwrap().insert(pid, job);
    }

    /// Remove a process group after its child has been reaped.
    #[cfg(unix)]
    pub fn deregister(&self, pgid: u32) {
        self.groups.lock().unwrap().remove(&pgid);
    }

    /// Remove a child after it has been reaped and release its job, so what
    /// it left running in the background survives, as a process group does
    /// on unix. Only `terminate_all` kills the tree.
    #[cfg(windows)]
    pub fn deregister(&self, pid: u32) {
        let job = self.groups.lock().unwrap().remove(&pid);
        if let Some(job) = job {
            job.release();
        }
    }

    /// Whether bnto stopped `pid`, forgetting it either way. The owner asks
    /// once, after reaping the child.
    pub fn take_stopped(&self, pid: u32) -> bool {
        self.stopped.lock().unwrap().remove(&pid)
    }

    /// Snapshot of currently registered process groups.
    pub fn active(&self) -> Vec<u32> {
        self.groups.lock().unwrap().keys().copied().collect()
    }

    /// Terminate every registered process group: SIGTERM, a short grace
    /// period, then SIGKILL for stragglers. On Windows, terminating each
    /// child's job kills its whole tree, even a grandchild whose parent
    /// already exited. Groups that already exited are ignored (a dead group
    /// is a harmless ESRCH, an emptied job a no-op).
    pub fn terminate_all(&self) {
        let groups = self.active();
        if groups.is_empty() {
            return;
        }
        self.stopped.lock().unwrap().extend(&groups);
        #[cfg(unix)]
        kill_trees(&groups);
        #[cfg(windows)]
        self.terminate_jobs(&groups);
        for pgid in &groups {
            self.deregister(*pgid);
        }
    }

    /// Terminate one registered child's whole tree, as `terminate_all` does
    /// for every child (a command that outlived its timeout). It stays
    /// registered until its owner reaps it and calls `deregister`.
    pub fn terminate(&self, pid: u32) {
        self.stopped.lock().unwrap().insert(pid);
        #[cfg(unix)]
        kill_trees(&[pid]);
        #[cfg(windows)]
        self.terminate_jobs(&[pid]);
    }

    /// `TerminateJobObject` the job of each child in `pids`.
    #[cfg(windows)]
    fn terminate_jobs(&self, pids: &[u32]) {
        let jobs = self.groups.lock().unwrap();
        for job in pids.iter().filter_map(|pid| jobs.get(pid)) {
            job.terminate();
        }
    }
}

/// SIGTERM each group, wait the grace period, then SIGKILL stragglers.
#[cfg(unix)]
fn kill_trees(groups: &[u32]) {
    /// Grace period between SIGTERM and SIGKILL.
    const TERM_GRACE_MS: u64 = 300;
    for pgid in groups {
        unsafe { libc::kill(-(*pgid as i32), libc::SIGTERM) };
    }
    std::thread::sleep(std::time::Duration::from_millis(TERM_GRACE_MS));
    for pgid in groups {
        unsafe { libc::kill(-(*pgid as i32), libc::SIGKILL) };
    }
}

/// The process-wide registry used by `NativeContext` by default and by the
/// signal handler / TUI quit path to kill children on the way out.
pub fn global() -> Arc<ProcessRegistry> {
    static GLOBAL: LazyLock<Arc<ProcessRegistry>> =
        LazyLock::new(|| Arc::new(ProcessRegistry::new()));
    GLOBAL.clone()
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// Register a made-up PID; on Windows it needs an (empty) job to own.
    fn register_fake(reg: &ProcessRegistry, pid: u32) {
        #[cfg(unix)]
        reg.register(pid);
        #[cfg(windows)]
        reg.register(pid, JobObject::new().unwrap());
    }

    #[test]
    fn register_and_deregister_track_groups() {
        let reg = ProcessRegistry::new();
        assert!(reg.active().is_empty());
        register_fake(&reg, 1234);
        register_fake(&reg, 5678);
        assert_eq!(reg.active().len(), 2);
        reg.deregister(1234);
        assert_eq!(reg.active(), vec![5678]);
    }

    #[test]
    fn terminate_all_on_empty_registry_is_noop() {
        // Well under the unix SIGTERM->SIGKILL grace period (300ms), so the
        // unix kill path running would trip it.
        const NOOP_BOUND_MS: u128 = 100;
        let reg = ProcessRegistry::new();
        let start = std::time::Instant::now();
        reg.terminate_all();
        assert!(start.elapsed().as_millis() < NOOP_BOUND_MS);
    }

    #[cfg(unix)]
    #[test]
    fn terminate_all_kills_registered_process_group() {
        use std::os::unix::process::CommandExt;
        use std::process::Command;

        // `sleep 30 & wait` — the backgrounded sleep is a grandchild that
        // only dies if the whole GROUP is killed, not just the shell.
        let mut child = Command::new("sh")
            .args(["-c", "sleep 30 & wait"])
            .process_group(0)
            .spawn()
            .unwrap();
        let pgid = child.id();

        let reg = ProcessRegistry::new();
        reg.register(pgid);
        reg.terminate_all();

        let status = child.wait().unwrap();
        assert!(!status.success(), "child must be killed, not exit cleanly");
        assert!(reg.active().is_empty(), "registry drained after terminate");
        // The whole group must be gone — signal 0 probes for existence.
        let alive = unsafe { libc::kill(-(pgid as i32), 0) };
        assert_eq!(alive, -1, "process group {pgid} should no longer exist");
    }

    #[cfg(windows)]
    pub(crate) fn windows_pid_alive(pid: u32) -> bool {
        let out = std::process::Command::new("tasklist")
            .args(["/FI", &format!("PID eq {pid}"), "/FO", "CSV", "/NH"])
            .output()
            .unwrap();
        String::from_utf8_lossy(&out.stdout).contains(&format!("\"{pid}\""))
    }

    #[cfg(windows)]
    #[test]
    fn terminate_all_kills_child_tree_on_windows() {
        use std::io::{BufRead, BufReader};
        use std::process::{Command, Stdio};

        // The hidden ping is a grandchild that only dies if the tree is killed.
        let script = "$p = Start-Process ping -ArgumentList '-n','30','127.0.0.1' \
                      -PassThru -WindowStyle Hidden; $p.Id; $p.WaitForExit()";
        let mut command = Command::new("powershell");
        command
            .args(["-NoProfile", "-Command", script])
            .stdout(Stdio::piped());
        let (mut child, job) = crate::job_object::spawn_in_job(&mut command).unwrap();
        let mut line = String::new();
        BufReader::new(child.stdout.take().unwrap())
            .read_line(&mut line)
            .unwrap();
        let grandchild: u32 = line.trim().parse().unwrap();

        let reg = ProcessRegistry::new();
        reg.register(child.id(), job);
        reg.terminate_all();

        let status = child.wait().unwrap();
        assert!(!status.success(), "child must be killed, not exit cleanly");
        assert!(reg.active().is_empty(), "registry drained after terminate");
        for _ in 0..50 {
            if !windows_pid_alive(grandchild) {
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        panic!("grandchild {grandchild} should no longer exist");
    }

    #[test]
    fn global_returns_same_registry() {
        let a = global();
        let b = global();
        register_fake(&a, 4242);
        assert!(b.active().contains(&4242));
        a.deregister(4242);
        assert!(!b.active().contains(&4242));
    }
}
