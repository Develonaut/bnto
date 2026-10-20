// Run watchdog — enforces a `bnto run` time limit even while a step is stuck
// inside a processor. A thread waits out the limit; if the run is still going
// it kills every registered child process tree, which unblocks a step stuck on
// a command. The run then reports the timeout itself. If it doesn't within a
// short grace period (a step stuck on something that isn't a child process),
// the watchdog kills again and ends the run with the same verdict.
//
// The engine also checks the limit before each step, file and loop iteration
// starts (`bnto-core/src/executor/time_limit.rs`), so nothing new starts once
// the limit has passed.

use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;

use bnto_core::BntoError;

use crate::process_registry;
use crate::run_verdict::RunVerdict;

/// How long a run whose children were killed has to report its own timeout
/// before the watchdog ends it.
const REPORT_GRACE: Duration = Duration::from_secs(2);

/// Where a watched run is, shared by the run and the watchdog thread.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    /// The limit hasn't passed and the run hasn't finished.
    Running,
    /// The run finished in time; the watchdog does nothing.
    Disarmed,
    /// The limit passed and the run's children were killed.
    Fired,
    /// The run came back after `Fired` and reports the timeout itself.
    Reported,
    /// The run never came back; the watchdog is ending it.
    Ending,
}

type Shared = Arc<(Mutex<State>, Condvar)>;

/// Watches one run's time limit until `disarm`.
pub struct RunWatchdog {
    shared: Shared,
}

impl RunWatchdog {
    /// Watch a `bnto run` limited to `secs`: kill its children once the
    /// limit passes, and end it with `Pipeline failed: Run timed out after
    /// <secs>s` if it doesn't report that itself.
    pub fn for_run(secs: u64, verdict: RunVerdict) -> Self {
        let kill = || process_registry::global().terminate_all();
        let end = move || {
            kill();
            verdict.fail("Pipeline failed:", BntoError::TimedOut { secs })
        };
        Self::arm(Duration::from_secs(secs), REPORT_GRACE, kill, end)
    }

    /// Start watching: `on_fire` runs once `limit` passes before `disarm`,
    /// then `on_stuck` if `disarm` hasn't been called within `grace`.
    fn arm(
        limit: Duration,
        grace: Duration,
        on_fire: impl FnOnce() + Send + 'static,
        on_stuck: impl FnOnce() + Send + 'static,
    ) -> Self {
        let shared: Shared = Arc::new((Mutex::new(State::Running), Condvar::new()));
        let watched = Arc::clone(&shared);
        std::thread::spawn(move || {
            if !advance(&watched, limit, State::Running, State::Fired) {
                return;
            }
            on_fire();
            if advance(&watched, grace, State::Fired, State::Ending) {
                on_stuck();
            }
        });
        Self { shared }
    }

    /// Stop watching. True when the limit had already passed, so the run
    /// must be reported as timed out whatever its result. Never returns once
    /// the watchdog is ending the run itself.
    pub fn disarm(&self) -> bool {
        let (lock, changed) = &*self.shared;
        let mut state = lock.lock().unwrap();
        let fired = match *state {
            State::Running => {
                *state = State::Disarmed;
                false
            }
            State::Fired => {
                *state = State::Reported;
                true
            }
            State::Ending => drop_and_wait_for_exit(state),
            State::Disarmed | State::Reported => *state == State::Reported,
        };
        changed.notify_all();
        fired
    }
}

/// Wait up to `wait` for the state to leave `from`; if it hasn't, move it to
/// `to`. Returns whether it moved.
fn advance(shared: &Shared, wait: Duration, from: State, to: State) -> bool {
    let (lock, changed) = &**shared;
    let state = lock.lock().unwrap();
    let (mut state, _) = changed
        .wait_timeout_while(state, wait, |s| *s == from)
        .unwrap();
    if *state != from {
        return false;
    }
    *state = to;
    true
}

/// Release the lock and block: the watchdog thread is exiting the process.
fn drop_and_wait_for_exit(state: std::sync::MutexGuard<'_, State>) -> ! {
    drop(state);
    loop {
        std::thread::park();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::Instant;

    /// Long enough that a test finishing before it proves nothing fired.
    const NEVER: Duration = Duration::from_secs(60);
    /// How long a test waits for a callback the watchdog should run.
    const WAIT_FOR: Duration = Duration::from_secs(10);

    fn flag() -> Arc<AtomicBool> {
        Arc::new(AtomicBool::new(false))
    }

    /// A callback that sets `flag`.
    fn setter(flag: &Arc<AtomicBool>) -> impl FnOnce() + Send + 'static {
        let flag = Arc::clone(flag);
        move || flag.store(true, Ordering::SeqCst)
    }

    fn wait_until(flag: &AtomicBool) -> bool {
        let start = Instant::now();
        while start.elapsed() < WAIT_FOR {
            if flag.load(Ordering::SeqCst) {
                return true;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        false
    }

    #[test]
    fn test_a_run_done_in_time_never_fires() {
        let (fired, stuck) = (flag(), flag());
        let dog = RunWatchdog::arm(NEVER, NEVER, setter(&fired), setter(&stuck));
        assert!(!dog.disarm(), "the run finished in time");
        assert!(!fired.load(Ordering::SeqCst));
    }

    #[test]
    fn test_the_limit_fires_and_the_run_reports_the_timeout() {
        let (fired, stuck) = (flag(), flag());
        let limit = Duration::from_millis(50);
        let dog = RunWatchdog::arm(limit, NEVER, setter(&fired), setter(&stuck));
        assert!(wait_until(&fired), "the limit fired");
        assert!(dog.disarm(), "the run must report a timeout");
        assert!(dog.disarm(), "asking again gives the same answer");
        assert!(!stuck.load(Ordering::SeqCst), "the run came back in time");
    }

    #[test]
    fn test_a_run_that_never_comes_back_is_ended() {
        let (fired, stuck) = (flag(), flag());
        let short = Duration::from_millis(50);
        let _dog = RunWatchdog::arm(short, short, setter(&fired), setter(&stuck));
        assert!(wait_until(&stuck), "the watchdog ended the stuck run");
        assert!(fired.load(Ordering::SeqCst), "it killed the children first");
    }
}
