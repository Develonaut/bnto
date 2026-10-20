// Test context whose command prints some stdout, writes a log into every
// folder it's given, then exits non-zero, the way a build tool fails.

use std::path::{Path, PathBuf};

use bnto_core::processor::{FileData, NodeInput, NodeOutput};
use bnto_core::{BntoError, NodeProcessor, NoopContext, ProcessContext, ProgressReporter};

use crate::ShellRun;

/// What the failing tool prints to stdout before it exits.
pub(crate) const STDOUT: &[u8] = b"compiling a\ncompiling b\nerror in b\n";

/// The failure message it exits with.
pub(crate) const MESSAGE: &str = "Command 'tool' failed (exit 2): boom";

/// Runs every command as a failing tool, with temp files in its own dir.
pub(crate) struct FailingTool(tempfile::TempDir);

impl FailingTool {
    pub(crate) fn new() -> Self {
        Self(tempfile::tempdir().unwrap())
    }

    /// Run shell-run with `params` (merged over `command: "tool"`).
    pub(crate) fn run(&self, params: serde_json::Value) -> Result<NodeOutput, BntoError> {
        let mut map = serde_json::Map::new();
        map.insert("command".into(), "tool".into());
        map.extend(params.as_object().cloned().unwrap_or_default());
        let input = NodeInput {
            data: FileData::Bytes(vec![]),
            filename: String::new(),
            mime_type: None,
            params: map,
        };
        ShellRun::new().process(input, &ProgressReporter::new_noop(), self)
    }
}

impl ProcessContext for FailingTool {
    fn run_command(&self, _cmd: &str, args: &[&str]) -> Result<Vec<u8>, BntoError> {
        for dir in args.iter().map(Path::new).filter(|p| p.is_dir()) {
            std::fs::write(dir.join("build.log"), b"log so far").unwrap();
        }
        Err(BntoError::CommandFailed {
            message: MESSAGE.to_string(),
            stdout: STDOUT.to_vec(),
        })
    }
    fn temp_file(&self, suffix: &str) -> Result<PathBuf, BntoError> {
        let path = self.0.path().join(format!("tmp{suffix}"));
        std::fs::write(&path, b"").unwrap();
        Ok(path)
    }
    fn env_var(&self, _key: &str) -> Option<String> {
        None
    }
    fn work_dir(&self) -> Result<&Path, BntoError> {
        NoopContext.work_dir()
    }
}
