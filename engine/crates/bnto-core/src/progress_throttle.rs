// Time-based throttle for live progress. Nodes report as often as they like;
// the throttle lets one report through per interval so consumers (a terminal,
// a Web Worker's postMessage) never drown in updates.

/// Lets one call through per `interval_ms`, timed by an injected clock.
///
/// The clock is injected (not `std::time::Instant`) so this works in WASM
/// and tests can step time by hand.
pub(crate) struct Throttle {
    #[allow(clippy::type_complexity)]
    clock: Box<dyn Fn() -> u64>,
    interval_ms: u64,
    last: std::cell::Cell<Option<u64>>,
}

impl Throttle {
    pub(crate) fn new(clock: impl Fn() -> u64 + 'static, interval_ms: u64) -> Self {
        Self {
            clock: Box::new(clock),
            interval_ms,
            last: std::cell::Cell::new(None),
        }
    }

    /// True for the first call, then once `interval_ms` has passed since
    /// the last call it let through.
    pub(crate) fn allow(&self) -> bool {
        let now = (self.clock)();
        if let Some(last) = self.last.get()
            && now.saturating_sub(last) < self.interval_ms
        {
            return false;
        }
        self.last.set(Some(now));
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    /// A clock the test moves by hand.
    fn manual_clock() -> (Rc<std::cell::Cell<u64>>, impl Fn() -> u64 + 'static) {
        let now = Rc::new(std::cell::Cell::new(0));
        let reader = Rc::clone(&now);
        (now, move || reader.get())
    }

    #[test]
    fn first_call_is_always_allowed() {
        let (_, clock) = manual_clock();
        assert!(Throttle::new(clock, 100).allow());
    }

    #[test]
    fn calls_inside_the_interval_are_dropped() {
        let (now, clock) = manual_clock();
        let throttle = Throttle::new(clock, 100);
        assert!(throttle.allow());
        now.set(50);
        assert!(!throttle.allow());
        now.set(99);
        assert!(!throttle.allow());
    }

    #[test]
    fn interval_counts_from_the_last_allowed_call() {
        let (now, clock) = manual_clock();
        let throttle = Throttle::new(clock, 100);
        assert!(throttle.allow());
        now.set(100);
        assert!(throttle.allow());
        now.set(150);
        assert!(!throttle.allow());
        now.set(200);
        assert!(throttle.allow());
    }

    #[test]
    fn a_clock_that_goes_backwards_does_not_open_the_gate() {
        let (now, clock) = manual_clock();
        now.set(1000);
        let throttle = Throttle::new(clock, 100);
        assert!(throttle.allow());
        now.set(10);
        assert!(!throttle.allow());
    }
}
