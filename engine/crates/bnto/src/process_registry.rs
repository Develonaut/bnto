// Process-group registry — tracks external commands spawned by NativeContext
// so cancel/quit/signal paths can terminate the entire child process tree
// (e.g. patreon-dl → patreon-dl-vimeo → yt-dlp), not just the direct child.
//
// A process-wide default registry exists (rather than plumbing an instance
// through every layer) because OS signal handlers only have 'static access.

use std::collections::HashSet;
use std::sync::{Arc, LazyLock, Mutex};

/// Grace period between SIGTERM and SIGKILL when terminating groups.
const TERM_GRACE_MS: u64 = 300;

/// Tracks the process-group IDs of live spawned commands.
///
/// Children are spawned with `process_group(0)`, so each child's PID is also
/// its process-group ID and killing `-pgid` reaps the whole tree.
#[derive(Default)]
pub struct ProcessRegistry {
    groups: Mutex<HashSet<u32>>,
}

impl ProcessRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a live process group (the child's PID under `process_group(0)`).
    pub fn register(&self, pgid: u32) {
        self.groups.lock().unwrap().insert(pgid);
    }

    /// Remove a process group after its child has been reaped.
    pub fn deregister(&self, pgid: u32) {
        self.groups.lock().unwrap().remove(&pgid);
    }

    /// Snapshot of currently registered process groups.
    pub fn active(&self) -> Vec<u32> {
        self.groups.lock().unwrap().iter().copied().collect()
    }

    /// Terminate every registered process group: SIGTERM, a short grace
    /// period, then SIGKILL for stragglers. Groups that already exited are
    /// ignored (kill on a dead group is a harmless ESRCH).
    pub fn terminate_all(&self) {
        let groups = self.active();
        if groups.is_empty() {
            return;
        }
        #[cfg(unix)]
        {
            for pgid in &groups {
                unsafe { libc::kill(-(*pgid as i32), libc::SIGTERM) };
            }
            std::thread::sleep(std::time::Duration::from_millis(TERM_GRACE_MS));
            for pgid in &groups {
                unsafe { libc::kill(-(*pgid as i32), libc::SIGKILL) };
            }
        }
        for pgid in &groups {
            self.deregister(*pgid);
        }
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
mod tests {
    use super::*;

    #[test]
    fn register_and_deregister_track_groups() {
        let reg = ProcessRegistry::new();
        assert!(reg.active().is_empty());
        reg.register(1234);
        reg.register(5678);
        assert_eq!(reg.active().len(), 2);
        reg.deregister(1234);
        assert_eq!(reg.active(), vec![5678]);
    }

    #[test]
    fn terminate_all_on_empty_registry_is_noop() {
        let reg = ProcessRegistry::new();
        let start = std::time::Instant::now();
        reg.terminate_all();
        // Must not sleep the grace period when there is nothing to kill.
        assert!(start.elapsed().as_millis() < TERM_GRACE_MS as u128);
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

    #[test]
    fn global_returns_same_registry() {
        let a = global();
        let b = global();
        a.register(4242);
        assert!(b.active().contains(&4242));
        a.deregister(4242);
        assert!(!b.active().contains(&4242));
    }
}
