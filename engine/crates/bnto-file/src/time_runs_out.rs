// A run as a batch node sees it, whose time limit passes after a set number
// of `check_time_limit` calls: the files the node may still start. Lets the
// batch nodes' tests stop a run between files without a clock or a sleep.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use bnto_core::{BntoError, NoopContext, ProcessContext};

/// The limit the run reports, as `settings.timeout` would give it.
const LIMIT_SECS: u64 = 2;

/// A context whose run has time for `left` more files.
pub(crate) struct TimeRunsOut {
    left: AtomicUsize,
}

impl TimeRunsOut {
    /// Time to start `files` files, then the limit has passed.
    pub(crate) fn after(files: usize) -> Self {
        Self {
            left: AtomicUsize::new(files),
        }
    }
}

impl ProcessContext for TimeRunsOut {
    fn run_command(&self, cmd: &str, args: &[&str]) -> Result<Vec<u8>, BntoError> {
        NoopContext.run_command(cmd, args)
    }
    fn temp_file(&self, suffix: &str) -> Result<PathBuf, BntoError> {
        NoopContext.temp_file(suffix)
    }
    fn env_var(&self, key: &str) -> Option<String> {
        NoopContext.env_var(key)
    }
    fn work_dir(&self) -> Result<&Path, BntoError> {
        NoopContext.work_dir()
    }
    fn check_time_limit(&self) -> Result<(), BntoError> {
        let take = |left: usize| left.checked_sub(1);
        match self
            .left
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, take)
        {
            Ok(_) => Ok(()),
            Err(_) => Err(BntoError::TimedOut { secs: LIMIT_SECS }),
        }
    }
}
