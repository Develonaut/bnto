// Lets a quitting process stop writes between steps, never in the middle.
//
// Ctrl+C (`process::exit`) and quitting the TUI (main returns while the run's
// bridge thread may be writing) both end the process without waiting for
// other threads. Mid-swap, a staged copy can be the only copy of a file, so
// the process first stops the gate: no new write starts, a write in progress
// stops after its current step, and the quitter waits for that before it
// reports kept staging folders and exits. Only the final write goes through
// the gate: progressive writes (`progressive_output`) don't. They copy only
// onto names nothing holds yet and stage nothing, so one cut short leaves at
// worst a partial new file, which the final write would have replaced.

use std::sync::{Condvar, Mutex, MutexGuard};

/// Whether writes may run, and how many are mid-step right now.
pub struct WriteGate {
    state: Mutex<State>,
    idle: Condvar,
}

struct State {
    stopping: bool,
    active: usize,
}

/// The process's gate: every `write_all` goes through it.
pub static GATE: WriteGate = WriteGate::new();

impl WriteGate {
    pub const fn new() -> Self {
        Self {
            state: Mutex::new(State {
                stopping: false,
                active: 0,
            }),
            idle: Condvar::new(),
        }
    }

    /// Start a write; `None` once the process is stopping.
    pub fn enter(&self) -> Option<Writing<'_>> {
        let mut state = self.lock();
        if state.stopping {
            return None;
        }
        state.active += 1;
        Some(Writing(self))
    }

    /// Whether a write should stop before its next step.
    pub fn stopping(&self) -> bool {
        self.lock().stopping
    }

    /// Stop every write at its next step, and wait until none is running.
    /// `on_wait` runs first if a write is mid-step (say why bnto pauses).
    pub fn stop_and_wait(&self, on_wait: impl FnOnce()) {
        let mut state = self.lock();
        state.stopping = true;
        if state.active > 0 {
            on_wait();
        }
        while state.active > 0 {
            state = self.idle.wait(state).unwrap_or_else(|e| e.into_inner());
        }
    }

    fn lock(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(|e| e.into_inner())
    }
}

/// A write in progress; dropping it lets a waiting quitter go on.
pub struct Writing<'a>(&'a WriteGate);

impl Drop for Writing<'_> {
    fn drop(&mut self) {
        self.0.lock().active -= 1;
        self.0.idle.notify_all();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::time::Duration;

    #[test]
    fn test_no_write_starts_once_stopping() {
        let gate = WriteGate::new();
        gate.stop_and_wait(|| {});
        assert!(gate.stopping());
        assert!(gate.enter().is_none());
    }

    #[test]
    fn test_stopping_waits_for_a_write_in_progress() {
        let gate = WriteGate::new();
        let (done_tx, done_rx) = mpsc::channel();
        std::thread::scope(|s| {
            let writing = gate.enter().unwrap();
            s.spawn(|| {
                gate.stop_and_wait(|| {});
                done_tx.send(()).unwrap();
            });
            let early = done_rx.recv_timeout(Duration::from_millis(100));
            assert!(early.is_err(), "returned while a write was mid-step");
            drop(writing);
            done_rx.recv_timeout(Duration::from_secs(5)).unwrap();
        });
    }

    #[test]
    fn test_the_wait_is_announced_only_when_a_write_is_mid_step() {
        // Ctrl+C prints "Finishing the current file..." through `on_wait`.
        let idle = WriteGate::new();
        let mut said = false;
        idle.stop_and_wait(|| said = true);
        assert!(!said, "nothing to finish, nothing said");

        let busy = WriteGate::new();
        let (said_tx, said_rx) = mpsc::channel();
        std::thread::scope(|s| {
            let writing = busy.enter().unwrap();
            s.spawn(|| busy.stop_and_wait(|| said_tx.send(()).unwrap()));
            said_rx.recv_timeout(Duration::from_secs(5)).unwrap();
            drop(writing);
        });
    }
}
