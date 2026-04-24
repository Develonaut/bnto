// Controlled system access for node processors.
//
// Processors that wrap external tools (yt-dlp, ffmpeg) need to run commands,
// create temp files, and read env vars. This trait is the boundary:
//   - Browser (WASM): `NoopContext` — all methods return errors
//   - CLI (native): `NativeContext` — full `std::process` access
//   - Desktop (future): `SandboxedContext` — scoped to approved directories

use std::path::{Path, PathBuf};

use crate::errors::BntoError;

/// System access boundary for processors that need external tools.
pub trait ProcessContext: Send + Sync {
    /// Run an external command, capturing stdout.
    fn run_command(&self, cmd: &str, args: &[&str]) -> Result<Vec<u8>, BntoError>;

    /// Run an external command, streaming stderr lines via a callback.
    ///
    /// Like `run_command()` but calls `on_stderr` for each line of stderr
    /// as it arrives, enabling live progress feedback from tools like yt-dlp.
    /// Returns stdout bytes on success. Default falls back to `run_command()`.
    fn run_command_streaming(
        &self,
        cmd: &str,
        args: &[&str],
        on_stderr: &dyn Fn(&str),
    ) -> Result<Vec<u8>, BntoError> {
        let _ = on_stderr;
        self.run_command(cmd, args)
    }

    /// Return a unique temporary file path. The file is NOT pre-created —
    /// the caller (or external tool) is responsible for writing to it.
    fn temp_file(&self, suffix: &str) -> Result<PathBuf, BntoError>;

    /// Read an environment variable.
    fn env_var(&self, key: &str) -> Option<String>;

    /// Get the working directory for this execution.
    fn work_dir(&self) -> Result<&Path, BntoError>;
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
            "on_stderr should not be called in noop fallback"
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
}
