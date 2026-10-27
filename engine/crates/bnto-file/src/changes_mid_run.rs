// A run as a batch node sees it, where something changes on disk just before
// the node's n-th file: `change` runs on the n-th `check_time_limit` call,
// which a batch node makes before each file. Lets the batch nodes' tests make
// a file fail partway without racing another thread.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use bnto_core::{BntoError, NoopContext, ProcessContext};

/// A context that runs `change` before the `at`-th file (1-based).
pub(crate) struct ChangesMidRun<F: Fn() + Send + Sync> {
    calls: AtomicUsize,
    at: usize,
    change: F,
}

impl<F: Fn() + Send + Sync> ChangesMidRun<F> {
    pub(crate) fn before_file(at: usize, change: F) -> Self {
        Self {
            calls: AtomicUsize::new(0),
            at,
            change,
        }
    }
}

impl<F: Fn() + Send + Sync> ProcessContext for ChangesMidRun<F> {
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
        if self.calls.fetch_add(1, Ordering::SeqCst) + 1 == self.at {
            (self.change)();
        }
        Ok(())
    }
}
