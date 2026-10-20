// The run's time limit — checked before every node, file and loop iteration
// starts, and once more when the run ends. Between-step checks are all the
// executor can do: a step already running is the consumer's to stop (the
// CLI kills its child processes; see `bnto/src/run_watchdog.rs`).

use crate::errors::BntoError;

use super::PipelineContext;

/// When a run with a time limit must be done by.
#[derive(Debug, Clone, Copy)]
pub(super) struct TimeLimit {
    /// The limit as the recipe gave it, for the error message.
    secs: u64,
    /// `now_ms` reading at which the limit has passed.
    deadline_ms: u64,
}

impl TimeLimit {
    /// The limit of a run that started at `started_ms`, if it has one.
    pub(super) fn starting_at(started_ms: u64, secs: Option<u64>) -> Option<Self> {
        secs.map(|secs| Self {
            secs,
            deadline_ms: started_ms.saturating_add(secs.saturating_mul(1000)),
        })
    }

    /// `TimedOut` once `now_ms` has reached the deadline.
    pub(super) fn check(&self, now_ms: u64) -> Result<(), BntoError> {
        if now_ms >= self.deadline_ms {
            return Err(BntoError::TimedOut { secs: self.secs });
        }
        Ok(())
    }
}

impl<F: Fn() -> u64 + Copy> PipelineContext<'_, F> {
    /// `TimedOut` once the run has passed its time limit; `Ok` without one.
    pub(super) fn check_time_limit(&self) -> Result<(), BntoError> {
        match self.time_limit {
            Some(limit) => limit.check((self.now_ms)()),
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_the_limit_passes_at_its_deadline() {
        let limit = TimeLimit::starting_at(1_000, Some(2)).unwrap();
        assert!(limit.check(2_999).is_ok());
        let err = limit.check(3_000).unwrap_err();
        assert_eq!(err.to_string(), "Run timed out after 2s");
    }

    #[test]
    fn test_no_limit_without_seconds() {
        assert!(TimeLimit::starting_at(1_000, None).is_none());
    }

    #[test]
    fn test_a_huge_limit_never_overflows() {
        let limit = TimeLimit::starting_at(u64::MAX - 5, Some(u64::MAX)).unwrap();
        assert!(limit.check(u64::MAX - 1).is_ok());
    }
}
