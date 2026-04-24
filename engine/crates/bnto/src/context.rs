// NativeContext — full system access for CLI execution.
//
// Provides real implementations for running external commands, creating
// temp files, reading env vars, and accessing the working directory.

use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use bnto_core::context::ProcessContext;
use bnto_core::errors::BntoError;

/// Native context for CLI execution with full system access.
pub struct NativeContext {
    work_dir: PathBuf,
}

impl NativeContext {
    /// Create a context rooted at the given working directory.
    #[allow(dead_code)]
    pub fn new(work_dir: PathBuf) -> Self {
        Self { work_dir }
    }

    /// Create a context using the current working directory.
    pub fn current_dir() -> Result<Self, BntoError> {
        let work_dir = std::env::current_dir().map_err(|e| {
            BntoError::ProcessingFailed(format!("Failed to get current directory: {e}"))
        })?;
        Ok(Self { work_dir })
    }
}

impl ProcessContext for NativeContext {
    fn run_command(&self, cmd: &str, args: &[&str]) -> Result<Vec<u8>, BntoError> {
        let output = Command::new(cmd)
            .args(args)
            .current_dir(&self.work_dir)
            .output()
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to run '{cmd}': {e}")))?;

        if output.status.success() {
            Ok(output.stdout)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(BntoError::ProcessingFailed(format!(
                "Command '{cmd}' failed (exit {}): {}",
                output.status.code().unwrap_or(-1),
                stderr.trim()
            )))
        }
    }

    fn run_command_streaming(
        &self,
        cmd: &str,
        args: &[&str],
        on_stderr: &dyn Fn(&str),
    ) -> Result<Vec<u8>, BntoError> {
        let mut child = Command::new(cmd)
            .args(args)
            .current_dir(&self.work_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to run '{cmd}': {e}")))?;

        // Collect stdout on a background thread.
        let stdout = child.stdout.take().expect("stdout piped");
        let stdout_handle =
            std::thread::spawn(move || std::io::Read::bytes(stdout).flatten().collect::<Vec<u8>>());

        // Stream stderr line-by-line on the caller thread.
        let stderr = child.stderr.take().expect("stderr piped");
        let mut stderr_lines = Vec::new();
        for line in std::io::BufReader::new(stderr)
            .lines()
            .map_while(Result::ok)
        {
            on_stderr(&line);
            stderr_lines.push(line);
        }

        let status = child
            .wait()
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to wait for '{cmd}': {e}")))?;

        let stdout_bytes = stdout_handle.join().map_err(|_| {
            BntoError::ProcessingFailed("stdout reader thread panicked".to_string())
        })?;

        if status.success() {
            Ok(stdout_bytes)
        } else {
            let stderr_text = stderr_lines.join("\n");
            Err(BntoError::ProcessingFailed(format!(
                "Command '{cmd}' failed (exit {}): {}",
                status.code().unwrap_or(-1),
                stderr_text.trim()
            )))
        }
    }

    fn temp_file(&self, suffix: &str) -> Result<PathBuf, BntoError> {
        let dir = std::env::temp_dir();
        let id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        Ok(dir.join(format!("bnto-{id}{suffix}")))
    }

    fn env_var(&self, key: &str) -> Option<String> {
        std::env::var(key).ok()
    }

    fn work_dir(&self) -> Result<&Path, BntoError> {
        Ok(&self.work_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_native_context_run_command_echo() {
        let ctx = NativeContext::current_dir().unwrap();
        let result = ctx.run_command("echo", &["hello"]);
        assert!(result.is_ok());
        let stdout = result.unwrap();
        let output = String::from_utf8_lossy(&stdout);
        assert!(output.contains("hello"));
    }

    #[test]
    fn test_native_context_run_command_not_found() {
        let ctx = NativeContext::current_dir().unwrap();
        let result = ctx.run_command("bnto_nonexistent_binary_xyz", &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_native_context_temp_file() {
        let ctx = NativeContext::current_dir().unwrap();
        let result = ctx.temp_file(".txt");
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.to_string_lossy().ends_with(".txt"));
        assert!(!path.exists()); // path is reserved, not pre-created
    }

    #[test]
    fn test_native_context_env_var_path() {
        let ctx = NativeContext::current_dir().unwrap();
        let result = ctx.env_var("PATH");
        assert!(result.is_some());
    }

    #[test]
    fn test_native_context_env_var_missing() {
        let ctx = NativeContext::current_dir().unwrap();
        let result = ctx.env_var("BNTO_NONEXISTENT_VAR_XYZ");
        assert!(result.is_none());
    }

    #[test]
    fn run_command_streaming_captures_stderr() {
        let ctx = NativeContext::current_dir().unwrap();
        let lines = std::cell::RefCell::new(Vec::new());
        let result =
            ctx.run_command_streaming("sh", &["-c", "echo err >&2 && echo out"], &|line| {
                lines.borrow_mut().push(line.to_string())
            });
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), b"out\n");
        assert_eq!(*lines.borrow(), vec!["err"]);
    }

    #[test]
    fn run_command_streaming_returns_stdout() {
        let ctx = NativeContext::current_dir().unwrap();
        let called = std::cell::Cell::new(false);
        let result = ctx.run_command_streaming("echo", &["hello"], &|_| called.set(true));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), b"hello\n");
        assert!(!called.get(), "no stderr = no callback");
    }

    #[test]
    fn run_command_streaming_error_includes_stderr() {
        let ctx = NativeContext::current_dir().unwrap();
        let lines = std::cell::RefCell::new(Vec::new());
        let result =
            ctx.run_command_streaming("sh", &["-c", "echo fail-info >&2 && exit 1"], &|line| {
                lines.borrow_mut().push(line.to_string())
            });
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("fail-info"),
            "error should contain stderr: {err}"
        );
        assert_eq!(
            *lines.borrow(),
            vec!["fail-info"],
            "callback should receive stderr"
        );
    }

    #[test]
    fn test_native_context_work_dir() {
        let ctx = NativeContext::current_dir().unwrap();
        let result = ctx.work_dir();
        assert!(result.is_ok());
        assert!(result.unwrap().is_dir());
    }
}
