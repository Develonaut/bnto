// Controlled system access for node processors.
//
// Processors that wrap external tools (yt-dlp, ffmpeg) need to run commands,
// create temp files, and read env vars. This trait is the boundary:
//   - Browser (WASM): `NoopContext` — all methods return errors
//   - CLI (native): `NativeContext` — full `std::process` access
//   - Desktop (future): `SandboxedContext` — scoped to approved directories

use std::path::{Path, PathBuf};

use crate::command_options::CommandOptions;
use crate::errors::BntoError;

/// System access boundary for processors that need external tools.
pub trait ProcessContext: Send + Sync {
    /// Run an external command, capturing stdout.
    fn run_command(&self, cmd: &str, args: &[&str]) -> Result<Vec<u8>, BntoError>;

    /// Run an external command, streaming output lines via a callback.
    ///
    /// Calls `on_output` for each line of stdout and stderr as it arrives,
    /// enabling live progress feedback from tools like yt-dlp.
    /// Returns stdout bytes on success. Default falls back to `run_command()`.
    fn run_command_streaming(
        &self,
        cmd: &str,
        args: &[&str],
        on_output: &dyn Fn(&str),
    ) -> Result<Vec<u8>, BntoError> {
        let _ = on_output;
        self.run_command(cmd, args)
    }

    /// Run an external command with `options` (extra env, a time limit),
    /// streaming output like `run_command_streaming`. A context that spawns
    /// real processes must apply both. The default, for contexts that never
    /// do (the browser, test doubles), ignores them.
    fn run_command_with(
        &self,
        cmd: &str,
        args: &[&str],
        options: &CommandOptions,
        on_output: &dyn Fn(&str),
    ) -> Result<Vec<u8>, BntoError> {
        let _ = options;
        self.run_command_streaming(cmd, args, on_output)
    }

    /// Return a unique temporary file path with atomic creation (mkstemp).
    /// The file IS pre-created on disk to prevent TOCTOU races.
    fn temp_file(&self, suffix: &str) -> Result<PathBuf, BntoError>;

    /// Read an environment variable.
    fn env_var(&self, key: &str) -> Option<String>;

    /// Get the working directory for this execution.
    fn work_dir(&self) -> Result<&Path, BntoError>;

    /// Locate the executable `run_command(name, ..)` would spawn, without
    /// spawning anything. Returns `None` when it can't be found — always in
    /// the browser (WASM), which has no processes.
    fn find_binary(&self, name: &str) -> Option<PathBuf> {
        let _ = name;
        None
    }

    /// Get the bnto home directory (~/.bnto/ by default).
    /// Returns `None` in browser (WASM) — filesystem paths don't apply.
    fn home_dir(&self) -> Option<&Path> {
        None
    }

    /// Get the default output directory for recipe results (~/.bnto/output/).
    /// Returns `None` in browser (WASM) — filesystem paths don't apply.
    fn output_dir(&self) -> Option<PathBuf> {
        None
    }

    /// True on a dry run: a node that writes decides what it would do and
    /// writes nothing. Defaults to false.
    fn dry_run(&self) -> bool {
        false
    }

    // --- The run, as a batch node sees it ---
    //
    // The executor answers these for a batch node (`executor/run_scope.rs`,
    // which forwards every method above to the consumer's context: a method
    // added here needs a line there). Elsewhere they hold nothing.

    /// The name of a file a later step of this run still reads from the file
    /// at `path`, outside the files this node was handed: another loop
    /// iteration's, a branch's that runs later. `None` when nothing does.
    fn held_elsewhere(&self, path: &Path) -> Option<String> {
        let _ = path;
        None
    }

    /// Note that this run wrote the file at `path` (a copy made onto it).
    fn record_write(&self, path: &Path) {
        let _ = path;
    }

    /// Every file this run has written so far, in order (`record_write`).
    fn recorded_writes(&self) -> Vec<PathBuf> {
        Vec::new()
    }

    /// `BntoError::TimedOut` once the run has passed its time limit
    /// (`settings.timeout`), so a batch node can stop between files. `Ok`
    /// for a run with no limit, and everywhere but a batch node's context.
    fn check_time_limit(&self) -> Result<(), BntoError> {
        Ok(())
    }

    /// The enclosing loop's item: the metadata its iteration's file carried
    /// when the iteration started. A node that fills `{{item.*}}` itself puts
    /// it on top of each file's metadata (`placeholder::file_item`) and fills
    /// its once-per-call templates from it. `None` outside a loop.
    fn loop_item(&self) -> Option<serde_json::Map<String, serde_json::Value>> {
        None
    }
}

/// No-op context for browser (WASM) execution.
/// All system-access methods return errors — browser has no shell.
pub struct NoopContext;

impl ProcessContext for NoopContext {
    fn run_command(&self, _cmd: &str, _args: &[&str]) -> Result<Vec<u8>, BntoError> {
        Err(BntoError::ProcessingFailed(
            "System commands not available in browser".to_string(),
        ))
    }

    fn temp_file(&self, _suffix: &str) -> Result<PathBuf, BntoError> {
        Err(BntoError::ProcessingFailed(
            "Temp files not available in browser".to_string(),
        ))
    }

    fn env_var(&self, _key: &str) -> Option<String> {
        None
    }

    fn work_dir(&self) -> Result<&Path, BntoError> {
        Err(BntoError::ProcessingFailed(
            "Working directory not available in browser".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noop_context_run_command_returns_err() {
        let ctx = NoopContext;
        let result = ctx.run_command("ls", &[]);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("not available in browser")
        );
    }

    #[test]
    fn test_noop_context_temp_file_returns_err() {
        let ctx = NoopContext;
        let result = ctx.temp_file(".txt");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("not available in browser")
        );
    }

    #[test]
    fn test_noop_context_env_var_returns_none() {
        let ctx = NoopContext;
        assert_eq!(ctx.env_var("PATH"), None);
    }

    #[test]
    fn test_noop_context_streaming_falls_back_to_run_command() {
        let ctx = NoopContext;
        let called = std::cell::Cell::new(false);
        let result = ctx.run_command_streaming("ls", &[], &|_| called.set(true));
        assert!(result.is_err());
        assert!(
            !called.get(),
            "on_output should not be called in noop fallback"
        );
    }

    #[test]
    fn test_noop_context_work_dir_returns_err() {
        let ctx = NoopContext;
        let result = ctx.work_dir();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("not available in browser")
        );
    }

    #[test]
    fn test_noop_context_find_binary_returns_none() {
        assert!(NoopContext.find_binary("git").is_none());
    }

    #[test]
    fn test_noop_context_home_dir_returns_none() {
        let ctx = NoopContext;
        assert!(ctx.home_dir().is_none());
    }

    #[test]
    fn test_noop_context_dry_run_is_false() {
        assert!(!NoopContext.dry_run());
    }

    #[test]
    fn test_noop_context_output_dir_returns_none() {
        let ctx = NoopContext;
        assert!(ctx.output_dir().is_none());
    }

    #[test]
    fn test_noop_context_has_no_time_limit() {
        // The browser's context: its runs are timed by the executor between
        // steps and by the worker race in `@bnto/core`, never from here.
        assert!(NoopContext.check_time_limit().is_ok());
    }
}
