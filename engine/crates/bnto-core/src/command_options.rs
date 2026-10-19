// Command options — what a node asks of one external command beyond its
// program and arguments: extra environment variables, a time limit and the
// folder it runs in.

use std::path::PathBuf;
use std::time::Duration;

/// How to run one external command, beyond its program and arguments.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CommandOptions {
    /// Variables set for the command only, on top of the environment it
    /// inherits. The calling process's own environment is never changed.
    pub env: Vec<(String, String)>,
    /// Kill the command, and everything it started, once it has run this long.
    pub timeout: Option<Duration>,
    /// The folder the command runs in. `None` runs it in the work dir.
    pub dir: Option<PathBuf>,
}
