// A test context inside a loop, for nodes that fill `{{item.*}}` themselves:
// `loop_item` answers its item, everything else is the no-op context.
// Test-only, so each crate keeps its own copy (twin: `bnto-file/src/in_loop.rs`).

use std::path::{Path, PathBuf};

use serde_json::{Map, Value};

use crate::context::{NoopContext, ProcessContext};
use crate::errors::BntoError;

pub(crate) struct InLoop(Map<String, Value>);

/// A context inside a loop whose item is `item` (a JSON object).
pub(crate) fn in_loop(item: Value) -> InLoop {
    InLoop(item.as_object().cloned().expect("a loop item is an object"))
}

impl ProcessContext for InLoop {
    fn run_command(&self, cmd: &str, args: &[&str]) -> Result<Vec<u8>, BntoError> {
        NoopContext.run_command(cmd, args)
    }
    fn temp_file(&self, suffix: &str) -> Result<PathBuf, BntoError> {
        NoopContext.temp_file(suffix)
    }
    fn env_var(&self, _key: &str) -> Option<String> {
        None
    }
    fn work_dir(&self) -> Result<&Path, BntoError> {
        NoopContext.work_dir()
    }
    fn loop_item(&self) -> Option<Map<String, Value>> {
        Some(self.0.clone())
    }
}
