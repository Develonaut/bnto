// NativeContext — full system access for CLI execution.
//
// Provides real implementations for running external commands, creating
// temp files, reading env vars, and accessing the working directory.
// Env var resolution: system env > project .env > user ~/.bnto/.env.

use std::collections::HashMap;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;

use bnto_core::context::ProcessContext;
use bnto_core::dotenv::parse_dotenv;
use bnto_core::errors::BntoError;

use crate::process_registry::ProcessRegistry;

/// Native context for CLI execution with full system access.
///
/// Env var resolution order (first match wins):
/// 1. System environment (`std::env::var`)
/// 2. Project `.env` file in the working directory
/// 3. User dotenv at `~/.bnto/.env`
pub struct NativeContext {
    work_dir: PathBuf,
    /// Bnto home directory (~/.bnto/).
    home_dir: PathBuf,
    /// Default output directory (~/.bnto/output/).
    output_dir: PathBuf,
    /// Key-value pairs from the project-level `.env` file.
    project_env: HashMap<String, String>,
    /// Key-value pairs from the user-level `~/.bnto/.env` file.
    user_env: HashMap<String, String>,
    /// Live process groups of spawned commands, killable from cancel/quit
    /// paths. Defaults to the process-wide registry.
    registry: Arc<ProcessRegistry>,
}

impl NativeContext {
    /// Create a context rooted at the given working directory with resolved paths.
    #[allow(dead_code)]
    pub fn new(work_dir: PathBuf, paths: &crate::storage::BntoPaths) -> Self {
        let project_env = load_dotenv_file(&work_dir.join(".env"));
        let user_env = load_user_dotenv();
        Self {
            work_dir,
            home_dir: paths.home.clone(),
            output_dir: paths.output_dir(),
            project_env,
            user_env,
            registry: crate::process_registry::global(),
        }
    }

    /// Create a context using the current working directory.
    ///
    /// Resolves bnto paths from `BntoPaths::resolve()`. If paths
    /// can't be determined, home_dir/output_dir fall back to temp dir.
    pub fn current_dir() -> Result<Self, BntoError> {
        let work_dir = std::env::current_dir().map_err(|e| {
            BntoError::ProcessingFailed(format!("Failed to get current directory: {e}"))
        })?;
        let project_env = load_dotenv_file(&work_dir.join(".env"));
        let user_env = load_user_dotenv();
        let (home_dir, output_dir) = match crate::storage::BntoPaths::resolve() {
            Some(p) => {
                let output = p.output_dir();
                (p.home, output)
            }
            None => {
                let tmp = std::env::temp_dir().join("bnto");
                let output = tmp.join("output");
                (tmp, output)
            }
        };
        Ok(Self {
            work_dir,
            home_dir,
            output_dir,
            project_env,
            user_env,
            registry: crate::process_registry::global(),
        })
    }

    /// Prepare a command in its own process group so cancel/quit/signal
    /// paths can kill the whole spawned tree via `kill(-pgid, …)`.
    fn group_command(&self, cmd: &str, args: &[&str]) -> Command {
        let mut command = Command::new(cmd);
        command.args(args).current_dir(&self.work_dir);
        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            command.process_group(0);
        }
        command
    }
}

/// Load and parse a `.env` file, returning an empty map if it doesn't exist.
fn load_dotenv_file(path: &Path) -> HashMap<String, String> {
    match std::fs::read_to_string(path) {
        Ok(contents) => parse_dotenv(&contents),
        Err(_) => HashMap::new(),
    }
}

/// Load the user-level dotenv from `~/.bnto/.env`.
fn load_user_dotenv() -> HashMap<String, String> {
    let home = crate::storage::BntoPaths::resolve().map(|p| p.home);
    match home {
        Some(dir) => load_dotenv_file(&dir.join(".env")),
        None => HashMap::new(),
    }
}

impl ProcessContext for NativeContext {
    fn run_command(&self, cmd: &str, args: &[&str]) -> Result<Vec<u8>, BntoError> {
        let child = self
            .group_command(cmd, args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to run '{cmd}': {e}")))?;
        let pgid = child.id();
        self.registry.register(pgid);
        let result = child.wait_with_output();
        self.registry.deregister(pgid);
        let output = result
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to wait for '{cmd}': {e}")))?;

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
        on_output: &dyn Fn(&str),
    ) -> Result<Vec<u8>, BntoError> {
        let mut child = self
            .group_command(cmd, args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to run '{cmd}': {e}")))?;
        let pgid = child.id();
        self.registry.register(pgid);

        // Stream stdout line-by-line on a background thread, calling the
        // callback AND collecting raw bytes. This lets tools like yt-dlp
        // (which send progress to stdout) show lines in the TUI while we
        // still capture the full output for stdout mode.
        let stdout = child.stdout.take().expect("stdout piped");
        let (line_tx, line_rx) = std::sync::mpsc::channel::<String>();
        let stdout_handle = std::thread::spawn(move || {
            let reader = std::io::BufReader::new(stdout);
            let mut raw_bytes = Vec::new();
            for line in reader.lines().map_while(Result::ok) {
                let _ = line_tx.send(line.clone());
                raw_bytes.extend_from_slice(line.as_bytes());
                raw_bytes.push(b'\n');
            }
            raw_bytes
        });

        // Stream stderr line-by-line on another background thread.
        let stderr = child.stderr.take().expect("stderr piped");
        let (stderr_tx, stderr_rx) = std::sync::mpsc::channel::<String>();
        let stderr_handle = std::thread::spawn(move || {
            let reader = std::io::BufReader::new(stderr);
            let mut lines = Vec::new();
            for line in reader.lines().map_while(Result::ok) {
                let _ = stderr_tx.send(line.clone());
                lines.push(line);
            }
            lines
        });

        // Relay lines from both streams to the callback on the caller thread.
        // Poll both channels until both senders are dropped (threads finished).
        let mut stdout_done = false;
        let mut stderr_done = false;
        while !stdout_done || !stderr_done {
            if !stdout_done {
                match line_rx.try_recv() {
                    Ok(line) => on_output(&line),
                    Err(std::sync::mpsc::TryRecvError::Disconnected) => stdout_done = true,
                    Err(std::sync::mpsc::TryRecvError::Empty) => {}
                }
            }
            if !stderr_done {
                match stderr_rx.try_recv() {
                    Ok(line) => on_output(&line),
                    Err(std::sync::mpsc::TryRecvError::Disconnected) => stderr_done = true,
                    Err(std::sync::mpsc::TryRecvError::Empty) => {}
                }
            }
            // Small sleep to avoid busy-spinning when both channels are empty.
            if !stdout_done || !stderr_done {
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        }

        let wait_result = child.wait();
        self.registry.deregister(pgid);
        let status = wait_result
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to wait for '{cmd}': {e}")))?;

        let stdout_bytes = stdout_handle.join().map_err(|_| {
            BntoError::ProcessingFailed("stdout reader thread panicked".to_string())
        })?;

        let stderr_lines = stderr_handle.join().map_err(|_| {
            BntoError::ProcessingFailed("stderr reader thread panicked".to_string())
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
        tempfile::Builder::new()
            .prefix("bnto-")
            .suffix(suffix)
            .tempfile()
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to create temp file: {e}")))?
            .into_temp_path()
            .keep()
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to persist temp file: {e}")))
    }

    fn env_var(&self, key: &str) -> Option<String> {
        // 1. System environment (most explicit — user actively exported it).
        if let Ok(val) = std::env::var(key) {
            return Some(val);
        }
        // 2. Project .env (per-project credentials).
        if let Some(val) = self.project_env.get(key) {
            return Some(val.clone());
        }
        // 3. User dotenv (~/.bnto/.env) — global defaults.
        if let Some(val) = self.user_env.get(key) {
            return Some(val.clone());
        }
        None
    }

    fn work_dir(&self) -> Result<&Path, BntoError> {
        Ok(&self.work_dir)
    }

    fn home_dir(&self) -> Option<&Path> {
        Some(&self.home_dir)
    }

    fn output_dir(&self) -> Option<PathBuf> {
        Some(self.output_dir.clone())
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
    fn test_temp_file_is_created_on_disk() {
        let ctx = NativeContext::current_dir().unwrap();
        let path = ctx.temp_file(".txt").unwrap();
        assert!(path.exists(), "temp file must be atomically created");
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn test_temp_file_suffix_preserved() {
        let ctx = NativeContext::current_dir().unwrap();
        let path = ctx.temp_file(".txt").unwrap();
        assert!(
            path.to_string_lossy().ends_with(".txt"),
            "suffix must be preserved: {path:?}"
        );
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn test_temp_file_no_collisions() {
        let ctx = NativeContext::current_dir().unwrap();
        let paths: std::collections::HashSet<_> =
            (0..50).map(|_| ctx.temp_file(".dat").unwrap()).collect();
        assert_eq!(paths.len(), 50, "all 50 paths must be unique");
        for p in &paths {
            std::fs::remove_file(p).ok();
        }
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
    fn run_command_streaming_captures_both_streams() {
        let ctx = NativeContext::current_dir().unwrap();
        let lines = std::cell::RefCell::new(Vec::new());
        let result =
            ctx.run_command_streaming("sh", &["-c", "echo err >&2 && echo out"], &|line| {
                lines.borrow_mut().push(line.to_string())
            });
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), b"out\n");
        let captured = lines.borrow();
        assert!(
            captured.contains(&"err".to_string()),
            "should capture stderr"
        );
        assert!(
            captured.contains(&"out".to_string()),
            "should capture stdout"
        );
    }

    #[test]
    fn run_command_streaming_returns_stdout() {
        let ctx = NativeContext::current_dir().unwrap();
        let lines = std::cell::RefCell::new(Vec::new());
        let result = ctx.run_command_streaming("echo", &["hello"], &|line| {
            lines.borrow_mut().push(line.to_string())
        });
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), b"hello\n");
        assert_eq!(*lines.borrow(), vec!["hello"], "stdout lines relayed");
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
        let captured = lines.borrow();
        assert!(
            captured.contains(&"fail-info".to_string()),
            "callback should receive stderr"
        );
    }

    /// Helper: context with an isolated registry so kill-tests can't
    /// interfere with other tests' children via the global registry.
    fn ctx_with_registry(registry: Arc<ProcessRegistry>) -> NativeContext {
        NativeContext {
            work_dir: std::env::temp_dir(),
            home_dir: std::env::temp_dir(),
            output_dir: std::env::temp_dir(),
            project_env: HashMap::new(),
            user_env: HashMap::new(),
            registry,
        }
    }

    #[test]
    fn streaming_deregisters_group_after_completion() {
        let registry = Arc::new(ProcessRegistry::new());
        let ctx = ctx_with_registry(registry.clone());
        let result = ctx.run_command_streaming("echo", &["done"], &|_| {});
        assert!(result.is_ok());
        assert!(
            registry.active().is_empty(),
            "group must be deregistered after the child is reaped"
        );
    }

    #[cfg(unix)]
    #[test]
    fn terminate_all_unblocks_streaming_command() {
        let registry = Arc::new(ProcessRegistry::new());
        let ctx = ctx_with_registry(registry.clone());

        let handle = std::thread::spawn(move || {
            // Backgrounded sleep = grandchild; only a group kill reaps it.
            ctx.run_command_streaming("sh", &["-c", "sleep 30 & wait"], &|_| {})
        });

        // Wait for the child to be registered (spawn is quick).
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        while registry.active().is_empty() {
            assert!(
                std::time::Instant::now() < deadline,
                "child never registered"
            );
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        let pgid = registry.active()[0];

        let start = std::time::Instant::now();
        registry.terminate_all();
        let result = handle.join().expect("streaming thread panicked");

        assert!(result.is_err(), "killed command must report failure");
        assert!(
            start.elapsed() < std::time::Duration::from_secs(5),
            "run_command_streaming must unblock promptly after group kill"
        );
        let alive = unsafe { libc::kill(-(pgid as i32), 0) };
        assert_eq!(alive, -1, "whole process group must be dead");
    }

    #[test]
    fn test_native_context_work_dir() {
        let ctx = NativeContext::current_dir().unwrap();
        let result = ctx.work_dir();
        assert!(result.is_ok());
        assert!(result.unwrap().is_dir());
    }

    // --- Dotenv loading tests ---

    #[test]
    fn load_dotenv_file_reads_project_env() {
        let tmp = tempfile::tempdir().unwrap();
        let env_path = tmp.path().join(".env");
        std::fs::write(&env_path, "PROJECT_VAR=project_value\n").unwrap();

        let vars = super::load_dotenv_file(&env_path);
        assert_eq!(vars.get("PROJECT_VAR").unwrap(), "project_value");
    }

    #[test]
    fn load_dotenv_file_missing_returns_empty() {
        let vars = super::load_dotenv_file(Path::new("/nonexistent/.env"));
        assert!(vars.is_empty());
    }

    /// Helper: create a BntoPaths rooted at the given directory.
    fn test_paths(root: &std::path::Path) -> crate::storage::BntoPaths {
        crate::storage::BntoPaths {
            home: root.to_path_buf(),
        }
    }

    #[test]
    fn native_context_reads_project_dotenv() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(
            tmp.path().join(".env"),
            "BNTO_TEST_PROJECT_KEY=from_project\n",
        )
        .unwrap();

        let paths = test_paths(tmp.path());
        let ctx = NativeContext::new(tmp.path().to_path_buf(), &paths);
        // Should resolve from project .env (system env won't have this key).
        assert_eq!(
            ctx.env_var("BNTO_TEST_PROJECT_KEY"),
            Some("from_project".to_string())
        );
    }

    #[test]
    fn system_env_takes_priority_over_project_dotenv() {
        let tmp = tempfile::tempdir().unwrap();
        // PATH is always set in the system environment.
        std::fs::write(tmp.path().join(".env"), "PATH=overridden\n").unwrap();

        let paths = test_paths(tmp.path());
        let ctx = NativeContext::new(tmp.path().to_path_buf(), &paths);
        // System env should win — PATH should NOT be "overridden".
        let val = ctx.env_var("PATH").unwrap();
        assert_ne!(val, "overridden", "System env should take priority");
    }

    #[test]
    fn env_var_resolution_prefers_project_over_user() {
        // Test the layered resolution directly by constructing a context
        // with injected dotenv maps, without touching real env vars.
        let ctx = NativeContext {
            work_dir: PathBuf::from("/tmp"),
            home_dir: PathBuf::from("/tmp/bnto"),
            output_dir: PathBuf::from("/tmp/bnto/output"),
            project_env: [("KEY".to_string(), "project".to_string())]
                .into_iter()
                .collect(),
            user_env: [("KEY".to_string(), "user".to_string())]
                .into_iter()
                .collect(),
            registry: Arc::new(ProcessRegistry::new()),
        };
        assert_eq!(
            ctx.env_var("KEY"),
            Some("project".to_string()),
            "Project .env should take priority over user .env"
        );
    }

    #[test]
    fn env_var_falls_through_to_user_env() {
        let ctx = NativeContext {
            work_dir: PathBuf::from("/tmp"),
            home_dir: PathBuf::from("/tmp/bnto"),
            output_dir: PathBuf::from("/tmp/bnto/output"),
            project_env: HashMap::new(),
            user_env: [("USER_ONLY_KEY".to_string(), "user_val".to_string())]
                .into_iter()
                .collect(),
            registry: Arc::new(ProcessRegistry::new()),
        };
        assert_eq!(ctx.env_var("USER_ONLY_KEY"), Some("user_val".to_string()),);
    }

    #[test]
    fn home_dir_returns_configured_path() {
        let ctx = NativeContext {
            work_dir: PathBuf::from("/tmp"),
            home_dir: PathBuf::from("/custom/bnto"),
            output_dir: PathBuf::from("/custom/bnto/output"),
            project_env: HashMap::new(),
            user_env: HashMap::new(),
            registry: Arc::new(ProcessRegistry::new()),
        };
        assert_eq!(ctx.home_dir(), Some(Path::new("/custom/bnto")),);
    }

    #[test]
    fn output_dir_returns_configured_path() {
        let ctx = NativeContext {
            work_dir: PathBuf::from("/tmp"),
            home_dir: PathBuf::from("/custom/bnto"),
            output_dir: PathBuf::from("/custom/bnto/output"),
            project_env: HashMap::new(),
            user_env: HashMap::new(),
            registry: Arc::new(ProcessRegistry::new()),
        };
        assert_eq!(ctx.output_dir(), Some(PathBuf::from("/custom/bnto/output")),);
    }
}
