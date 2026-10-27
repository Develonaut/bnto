// A run as a batch node sees it, where a later step still reads one file:
// `held_elsewhere` names that step's file for `path`, and nothing else. Lets
// the batch nodes' tests check their refusals without a whole pipeline. It
// compares paths as given (the executor matches by file identity), so tests
// pass the canonical path the node asks about.

use std::path::{Path, PathBuf};

use bnto_core::{BntoError, NoopContext, ProcessContext};

/// A context where a later step reads the file at `path` as `name`.
pub(crate) struct HeldBy {
    path: PathBuf,
    name: String,
}

impl HeldBy {
    /// A later step reads the file at `path` as `name`.
    pub(crate) fn file(path: &Path, name: &str) -> Self {
        Self {
            path: path.to_path_buf(),
            name: name.to_string(),
        }
    }
}

impl ProcessContext for HeldBy {
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
    fn held_elsewhere(&self, path: &Path) -> Option<String> {
        (path == self.path).then(|| self.name.clone())
    }
}
