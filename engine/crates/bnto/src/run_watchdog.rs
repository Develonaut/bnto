// Run watchdog — enforces a run's time limit even while a step is stuck
// inside a processor. A thread waits out the limit; if the run is still going
// it kills every child process tree in the run's registry, which unblocks a
// step stuck on a command. The run then reports the timeout itself. If it
// doesn't within a short grace period (a step stuck on something that isn't a
// child process), the watchdog kills again and hands the timeout to the
// caller's `on_stuck`, which reports it in the run's place.
//
// `bnto run`, the TUI and `bnto dry-run`'s preview all run pipelines through
// `watch`. The engine also checks the limit before each step, file and loop
// iteration starts (`bnto-core/src/executor/time_limit.rs`), so nothing new
// starts once the limit has passed.

use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;

use bnto_core::BntoError;

use crate::process_registry::ProcessRegistry;

/// How long a run whose children were killed has to report its own timeout
/// before the watchdog reports it instead.
const REPORT_GRACE: Duration = Duration::from_secs(2);

/// How a watched run ended.
#[derive(Debug)]
pub enum Watched<T> {
    /// The run came back: its result, or `TimedOut` when it came back after
    /// the limit had passed, whatever it returned.
    Finished(Result<T, BntoError>),
    /// The run came back only after the watchdog had reported the timeout
    /// through `on_stuck`; report nothing more.
    Ended,
}

/// Run `run` under a time limit of `secs` seconds (no watchdog without one),
/// killing the trees in `registry` once it passes. `on_stuck` gets the
/// timeout if the run hasn't come back within the grace period after that.
pub fn watch<T>(
    registry: Arc<ProcessRegistry>,
    secs: Option<u64>,
    on_stuck: impl FnOnce(BntoError) + Send + 'static,
    run: impl FnOnce() -> Result<T, BntoError>,
) -> Watched<T> {
    let Some(secs) = secs else {
        return Watched::Finished(run());
    };
    let limit = Duration::from_secs(secs);
    watch_for(limit, REPORT_GRACE, registry, move || {
        on_stuck(BntoError::TimedOut { secs })
    })
    .finish(secs, run())
}

/// Block for good: the watchdog is ending the process in the run's place.
pub fn wait_for_exit() -> ! {
    loop {
        std::thread::park();
    }
}

/// Arm a watchdog that kills `registry`'s trees after `limit`, then again
/// before `on_stuck` if the run isn't back within `grace`.
fn watch_for(
    limit: Duration,
    grace: Duration,
    registry: Arc<ProcessRegistry>,
    on_stuck: impl FnOnce() + Send + 'static,
) -> RunWatchdog {
    let kill = Arc::clone(&registry);
    RunWatchdog::arm(
        limit,
        grace,
        move || kill.terminate_all(),
        move || {
            registry.terminate_all();
            on_stuck();
        },
    )
}

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
    /// The run never came back; the watchdog reported for it.
    Ending,
}

type Shared = Arc<(Mutex<State>, Condvar)>;

/// Watches one run's time limit until `finish`.
struct RunWatchdog {
    shared: Shared,
}

impl RunWatchdog {
    /// Start watching: `on_fire` runs once `limit` passes before `finish`,
    /// then `on_stuck` if `finish` hasn't been called within `grace`.
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

    /// Stop watching a run limited to `secs` that came back with `result`.
    fn finish<T>(&self, secs: u64, result: Result<T, BntoError>) -> Watched<T> {
        let (lock, changed) = &*self.shared;
        let mut state = lock.lock().unwrap();
        let watched = match *state {
            State::Running | State::Disarmed => {
                *state = State::Disarmed;
                Watched::Finished(result)
            }
            State::Fired | State::Reported => {
                *state = State::Reported;
                Watched::Finished(Err(BntoError::TimedOut { secs }))
            }
            State::Ending => Watched::Ended,
        };
        changed.notify_all();
        watched
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::Instant;

    /// Long enough that a test finishing before it proves nothing fired.
    const NEVER: Duration = Duration::from_secs(60);
    /// A limit (and grace) short enough to pass during a test.
    const SHORT: Duration = Duration::from_millis(50);
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

    /// A registry no other test's children are in.
    fn own_registry() -> Arc<ProcessRegistry> {
        Arc::new(ProcessRegistry::new())
    }

    #[test]
    fn test_a_run_done_in_time_keeps_its_result() {
        let (fired, stuck) = (flag(), flag());
        let dog = RunWatchdog::arm(NEVER, NEVER, setter(&fired), setter(&stuck));
        let watched = dog.finish(60, Ok(7));
        assert!(matches!(watched, Watched::Finished(Ok(7))), "{watched:?}");
        assert!(!fired.load(Ordering::SeqCst));
    }

    #[test]
    fn test_a_run_back_after_the_limit_timed_out_whatever_it_returned() {
        let (fired, stuck) = (flag(), flag());
        let dog = RunWatchdog::arm(SHORT, NEVER, setter(&fired), setter(&stuck));
        assert!(wait_until(&fired), "the limit fired");
        let watched = dog.finish(1, Ok(7));
        let Watched::Finished(Err(err)) = watched else {
            panic!("expected a timeout, got {watched:?}");
        };
        assert_eq!(err.to_string(), "Run timed out after 1s");
        assert!(!stuck.load(Ordering::SeqCst), "the run came back in time");
    }

    #[test]
    fn test_a_run_that_never_comes_back_is_reported_for() {
        let (fired, stuck) = (flag(), flag());
        let dog = RunWatchdog::arm(SHORT, SHORT, setter(&fired), setter(&stuck));
        assert!(
            wait_until(&stuck),
            "the watchdog reported for the stuck run"
        );
        assert!(fired.load(Ordering::SeqCst), "it killed the children first");
        assert!(matches!(dog.finish(1, Ok(7)), Watched::Ended));
    }

    #[test]
    fn test_watch_without_a_limit_just_runs() {
        let watched = watch(own_registry(), None, |_| panic!("not stuck"), || Ok(3));
        assert!(matches!(watched, Watched::Finished(Ok(3))));
    }

    #[test]
    fn test_watch_keeps_the_result_of_a_run_within_its_limit() {
        let watched = watch(own_registry(), Some(60), |_| panic!("not stuck"), || Ok(3));
        assert!(matches!(watched, Watched::Finished(Ok(3))));
    }

    #[test]
    fn test_a_watched_run_past_its_limit_times_out() {
        let slow = || {
            std::thread::sleep(Duration::from_millis(1500));
            Ok(3)
        };
        let watched = watch(own_registry(), Some(1), |_| panic!("not stuck"), slow);
        let Watched::Finished(Err(err)) = watched else {
            panic!("expected a timeout, got {watched:?}");
        };
        assert_eq!(err.to_string(), "Run timed out after 1s");
    }
}
