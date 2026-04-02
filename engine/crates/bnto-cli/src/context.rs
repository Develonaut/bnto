// NativeContext — full system access for CLI execution.
//
// Provides real implementations for running external commands, creating
// temp files, reading env vars, and accessing the working directory.

use std::path::{Path, PathBuf};

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
        let output = std::process::Command::new(cmd)
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

    fn temp_file(&self, suffix: &str) -> Result<PathBuf, BntoError> {
        let dir = std::env::temp_dir();
        let id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let path = dir.join(format!("bnto-{id}{suffix}"));
        std::fs::File::create(&path)
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to create temp file: {e}")))?;
        Ok(path)
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
        assert!(path.exists());
        // Clean up
        let _ = std::fs::remove_file(&path);
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
    fn test_native_context_work_dir() {
        let ctx = NativeContext::current_dir().unwrap();
        let result = ctx.work_dir();
        assert!(result.is_ok());
        assert!(result.unwrap().is_dir());
    }
}
