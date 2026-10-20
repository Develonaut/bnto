// NativeContext — full system access for CLI execution.
//
// Provides real implementations for running external commands, creating
// temp files, reading env vars, and accessing the working directory.
// Env var resolution: system env > project .env > user ~/.bnto/.env.

use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::Arc;

use bnto_core::CommandOptions;
use bnto_core::context::ProcessContext;
use bnto_core::dotenv::parse_dotenv;
use bnto_core::errors::BntoError;

use crate::command_stream;
use crate::process_registry::ProcessRegistry;
use crate::scratch::RunScratch;

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
    /// Set by `bnto run --dry-run`: writing nodes decide and write nothing.
    dry_run: bool,
    /// Holds the run's temp files; dropping the context deletes them. The
    /// output step copies results out, so the context must outlive it.
    scratch: RunScratch,
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
            dry_run: false,
            scratch: RunScratch::default(),
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
            dry_run: false,
            scratch: RunScratch::default(),
        })
    }

    /// Mark this context as a dry run (see `ProcessContext::dry_run`).
    pub fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

    /// The registry this context's commands are tracked in, for whatever
    /// must kill them (a run's time limit).
    pub fn registry(&self) -> Arc<ProcessRegistry> {
        Arc::clone(&self.registry)
    }

    /// The same context, tracking its commands in `registry` instead, so a
    /// test's kills can't reach other tests' children.
    #[cfg(test)]
    pub fn with_registry(mut self, registry: Arc<ProcessRegistry>) -> Self {
        self.registry = registry;
        self
    }

    /// Prepare a command in its own process group so cancel/quit/signal
    /// paths can kill the whole spawned tree via `kill(-pgid, …)`. Windows
    /// gets the same reach from the Job Object `spawn_tracked` adds.
    fn group_command(&self, cmd: &str, args: &[&str]) -> Command {
        let path_var = std::env::var_os("PATH");
        let mut command = Command::new(spawn_program(cmd, path_var.as_deref()));
        command.args(args).current_dir(&self.work_dir);
        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            command.process_group(0);
        }
        command
    }

    /// Spawn `command` and register its process group for cancel/quit kills.
    #[cfg(unix)]
    fn spawn_tracked(&self, command: &mut Command) -> std::io::Result<Child> {
        let child = command.spawn()?;
        self.registry.register(child.id());
        Ok(child)
    }

    /// Spawn `command` in its own Job Object and register the job for
    /// cancel/quit kills.
    #[cfg(windows)]
    fn spawn_tracked(&self, command: &mut Command) -> std::io::Result<Child> {
        let (child, job) = crate::job_object::spawn_in_job(command)?;
        self.registry.register(child.id(), job);
        Ok(child)
    }
}

/// Program to hand `Command::new` on Windows: the PATH-resolved file, so a
/// `.cmd` / `.bat` shim (npm's global tools) runs through cmd.exe with std's
/// argument escaping. Unresolved names pass through for std's own search.
#[cfg(windows)]
fn spawn_program(cmd: &str, path_var: Option<&OsStr>) -> OsString {
    path_var
        .and_then(|path| bnto_core::find_on_path(cmd, path))
        .map_or_else(|| OsString::from(cmd), PathBuf::into_os_string)
}

/// Unix `execvp` already runs PATH scripts with a shebang, so the name
/// passes through unchanged.
#[cfg(not(windows))]
fn spawn_program(cmd: &str, _path_var: Option<&OsStr>) -> OsString {
    OsString::from(cmd)
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

/// A non-zero exit, naming the command, its code and stderr, and keeping
/// the stdout it printed so a failed step can still hand it over.
fn command_failed(
    cmd: &str,
    status: std::process::ExitStatus,
    stderr: &str,
    stdout: Vec<u8>,
) -> BntoError {
    let code = status.code().unwrap_or(-1);
    let message = format!("Command '{cmd}' failed (exit {code}): {}", stderr.trim());
    BntoError::CommandFailed { message, stdout }
}

/// A finished command's stdout, or the failure it ends with: stopped for
/// outliving its timeout, or a non-zero exit. Either failure hands over the
/// stdout printed so far.
/// A command bnto stopped (cancel, quit, a signal) fails, keeping the
/// stdout it printed, whatever exit code it ended with.
fn command_stopped(cmd: &str, stdout: Vec<u8>) -> BntoError {
    BntoError::CommandFailed {
        message: format!("Command '{cmd}' was stopped before it finished"),
        stdout,
    }
}

fn command_outcome(
    cmd: &str,
    options: &CommandOptions,
    finished: command_stream::Finished,
) -> Result<Vec<u8>, BntoError> {
    if finished.timed_out {
        let secs = options.timeout.unwrap_or_default().as_secs();
        let message = format!("Command '{cmd}' timed out after {secs}s and was stopped");
        return Err(BntoError::CommandFailed {
            message,
            stdout: finished.stdout,
        });
    }
    if finished.status.success() {
        return Ok(finished.stdout);
    }
    let stderr = finished.stderr.join(
        "
",
    );
    Err(command_failed(
        cmd,
        finished.status,
        &stderr,
        finished.stdout,
    ))
}

impl ProcessContext for NativeContext {
    fn dry_run(&self) -> bool {
        self.dry_run
    }

    fn run_command(&self, cmd: &str, args: &[&str]) -> Result<Vec<u8>, BntoError> {
        let mut command = self.group_command(cmd, args);
        command.stdout(Stdio::piped()).stderr(Stdio::piped());
        let child = self
            .spawn_tracked(&mut command)
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to run '{cmd}': {e}")))?;
        let pgid = child.id();
        let result = child.wait_with_output();
        let stopped = self.registry.take_stopped(pgid);
        self.registry.deregister(pgid);
        let output = result
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to wait for '{cmd}': {e}")))?;
        if stopped {
            return Err(command_stopped(cmd, output.stdout));
        }

        if output.status.success() {
            Ok(output.stdout)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(command_failed(cmd, output.status, &stderr, output.stdout))
        }
    }

    fn run_command_streaming(
        &self,
        cmd: &str,
        args: &[&str],
        on_output: &dyn Fn(&str),
    ) -> Result<Vec<u8>, BntoError> {
        self.run_command_with(cmd, args, &CommandOptions::default(), on_output)
    }

    /// Spawn in `options.dir` (else the work dir) with `options.env` set on
    /// the child only (over a cleared environment when `options.clear_env`),
    /// relay its output, and kill its whole tree if it outlives
    /// `options.timeout`.
    fn run_command_with(
        &self,
        cmd: &str,
        args: &[&str],
        options: &CommandOptions,
        on_output: &dyn Fn(&str),
    ) -> Result<Vec<u8>, BntoError> {
        let mut command = self.group_command(cmd, args);
        if options.clear_env {
            command.env_clear();
            command.envs(crate::child_env::kept_vars(std::env::vars_os()));
        }
        command.envs(options.env.iter().map(|(k, v)| (k, v)));
        if let Some(dir) = &options.dir {
            command.current_dir(dir);
        }
        command.stdout(Stdio::piped()).stderr(Stdio::piped());
        let mut child = self
            .spawn_tracked(&mut command)
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to run '{cmd}': {e}")))?;
        let pid = child.id();
        let kill = || self.registry.terminate(pid);
        let finished = command_stream::stream(&mut child, on_output, options.timeout, &kill);
        let stopped = self.registry.take_stopped(pid);
        self.registry.deregister(pid);
        let finished = finished
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to wait for '{cmd}': {e}")))?;
        if stopped && !finished.timed_out {
            return Err(command_stopped(cmd, finished.stdout));
        }
        command_outcome(cmd, options, finished)
    }

    /// Temp files go in the run's scratch dir, removed when this context
    /// drops — after a real run's output step has copied them out.
    fn temp_file(&self, suffix: &str) -> Result<PathBuf, BntoError> {
        self.scratch.temp_file(suffix)
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

    /// Resolves against the process PATH (not `.env` files) because that
    /// is the PATH spawned commands inherit.
    fn find_binary(&self, name: &str) -> Option<PathBuf> {
        bnto_core::find_on_path(name, &std::env::var_os("PATH")?)
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

    /// The platform shell and the args that run `script` through it.
    fn shell(script: &str) -> (&'static str, [&str; 2]) {
        if cfg!(windows) {
            ("cmd", ["/c", script])
        } else {
            ("sh", ["-c", script])
        }
    }

    #[test]
    fn test_native_context_run_command_echo() {
        let ctx = NativeContext::current_dir().unwrap();
        let (cmd, args) = shell("echo hello");
        let result = ctx.run_command(cmd, &args);
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
    fn test_find_binary_agrees_with_run_command() {
        let ctx = NativeContext::current_dir().unwrap();
        let (cmd, args) = shell("echo hello");
        let found = ctx.find_binary(cmd).expect("platform shell on PATH");
        assert!(found.is_absolute(), "{found:?}");
        assert!(ctx.run_command(cmd, &args).is_ok());
    }

    #[test]
    fn test_find_binary_missing_returns_none() {
        let ctx = NativeContext::current_dir().unwrap();
        assert!(ctx.find_binary("bnto_nonexistent_binary_xyz").is_none());
    }

    #[cfg(windows)]
    #[test]
    fn test_spawn_program_resolves_cmd_shim_on_path() {
        let dir = tempfile::tempdir().unwrap();
        let shim = dir.path().join("greet.cmd");
        std::fs::write(&shim, "@echo hi %~1\r\n").unwrap();
        let path = std::env::join_paths([dir.path()]).unwrap();
        assert_eq!(
            spawn_program("greet", Some(path.as_os_str())),
            shim.into_os_string()
        );
    }

    #[test]
    fn test_spawn_program_falls_back_to_bare_name() {
        // Unresolved names go to std unchanged, keeping its own search.
        let path = std::ffi::OsString::new();
        assert_eq!(spawn_program("nope", Some(path.as_os_str())), "nope");
        assert_eq!(spawn_program("nope", None), "nope");
    }

    #[cfg(windows)]
    #[test]
    fn test_run_command_runs_cmd_shim_with_quoted_args() {
        // std runs a `.cmd` path through cmd.exe and escapes its arguments,
        // which is what makes npm-installed tools spawnable on Windows.
        let dir = tempfile::tempdir().unwrap();
        let shim = dir.path().join("greet.cmd");
        std::fs::write(&shim, "@echo hi %~1\r\n").unwrap();
        let ctx = NativeContext::current_dir().unwrap();
        let out = ctx
            .run_command(shim.to_str().unwrap(), &["two words"])
            .unwrap();
        assert_eq!(String::from_utf8_lossy(&out).trim(), "hi two words");
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
    fn test_dry_run_temp_file_removed_when_context_drops() {
        let ctx = NativeContext::current_dir().unwrap().with_dry_run(true);
        let path = ctx.temp_file("-output").unwrap();
        let out_dir = path.with_extension("d");
        std::fs::create_dir_all(&out_dir).unwrap();
        std::fs::write(out_dir.join("note.txt"), b"x").unwrap();
        assert!(path.exists());
        drop(ctx);
        assert!(!path.exists(), "dry-run temp file must not outlive the run");
        assert!(
            !out_dir.exists(),
            "dry-run output dir must not outlive the run"
        );
    }

    #[test]
    fn test_real_run_temp_file_removed_when_context_drops() {
        // The output step copies results out, so a real run's temp files
        // are the run's to delete once the context (and the run) is done.
        let ctx = NativeContext::current_dir().unwrap();
        let path = ctx.temp_file(".txt").unwrap();
        assert!(path.exists());
        drop(ctx);
        assert!(
            !path.exists(),
            "real-run temp file must not outlive the run"
        );
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
        let (cmd, args) = shell("echo err >&2 && echo out");
        let result = ctx.run_command_streaming(cmd, &args, &|line| {
            lines.borrow_mut().push(line.trim().to_string())
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
        let (cmd, args) = shell("echo hello");
        let result = ctx.run_command_streaming(cmd, &args, &|line| {
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
        let (cmd, args) = shell("echo fail-info >&2 && exit 1");
        let result = ctx.run_command_streaming(cmd, &args, &|line| {
            lines.borrow_mut().push(line.trim().to_string())
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

    /// The stdout a failed command printed, from its `CommandFailed` error.
    fn failed_stdout(result: Result<Vec<u8>, BntoError>) -> (String, String) {
        match result {
            Err(e @ BntoError::CommandFailed { .. }) => {
                let message = e.to_string();
                let BntoError::CommandFailed { stdout, .. } = e else {
                    unreachable!()
                };
                (message, String::from_utf8_lossy(&stdout).trim().to_string())
            }
            other => panic!("expected CommandFailed, got {other:?}"),
        }
    }

    #[test]
    fn run_command_streaming_failure_keeps_stdout() {
        let ctx = NativeContext::current_dir().unwrap();
        let (cmd, args) = shell("echo partial && exit 3");
        let (message, stdout) = failed_stdout(ctx.run_command_streaming(cmd, &args, &|_| {}));
        assert_eq!(stdout, "partial");
        assert!(
            message.starts_with("Processing failed: Command '"),
            "{message}"
        );
        assert!(message.contains("failed (exit 3)"), "{message}");
    }

    #[test]
    fn run_command_failure_keeps_stdout() {
        let ctx = NativeContext::current_dir().unwrap();
        let (cmd, args) = shell("echo partial && exit 3");
        let (message, stdout) = failed_stdout(ctx.run_command(cmd, &args));
        assert_eq!(stdout, "partial");
        assert!(message.contains("failed (exit 3)"), "{message}");
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
            dry_run: false,
            scratch: RunScratch::default(),
        }
    }

    #[test]
    fn streaming_deregisters_group_after_completion() {
        let registry = Arc::new(ProcessRegistry::new());
        let ctx = ctx_with_registry(registry.clone());
        let (cmd, args) = shell("echo done");
        let result = ctx.run_command_streaming(cmd, &args, &|_| {});
        assert!(result.is_ok());
        assert!(
            registry.active().is_empty(),
            "group must be deregistered after the child is reaped"
        );
    }

    /// Block until `registry` holds a child, returning its id. Spawn is quick;
    /// the long bound only absorbs a loaded machine.
    fn wait_until_registered(registry: &ProcessRegistry) -> u32 {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
        while registry.active().is_empty() {
            assert!(
                std::time::Instant::now() < deadline,
                "child never registered"
            );
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        registry.active()[0]
    }

    /// How long a killed command may take to return. A kill normally lands in
    /// well under a second; the slack absorbs a loaded machine. It stays far
    /// below the commands' own ~300s runtime, so a pass still proves the kill
    /// is what unblocked them.
    const UNBLOCK_DEADLINE: std::time::Duration = std::time::Duration::from_secs(30);

    /// Stream `cmd` on a thread, call `terminate_all` once it registers, and
    /// return the registered id with the command's result. Panics if the
    /// command is still blocked after `UNBLOCK_DEADLINE`.
    fn kill_streaming_command(
        cmd: &'static str,
        args: &'static [&'static str],
    ) -> (u32, Result<Vec<u8>, BntoError>) {
        let registry = Arc::new(ProcessRegistry::new());
        let ctx = ctx_with_registry(registry.clone());
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let _ = tx.send(ctx.run_command_streaming(cmd, args, &|_| {}));
        });
        let id = wait_until_registered(&registry);

        registry.terminate_all();
        match rx.recv_timeout(UNBLOCK_DEADLINE) {
            Ok(result) => (id, result),
            Err(err) => panic!("run_command_streaming must unblock after the kill: {err}"),
        }
    }

    // Unix-only: drives `sh` and probes the process group with `kill(-pgid, 0)`.
    // The Windows twin below covers the Job Object path.
    #[cfg(unix)]
    #[test]
    fn terminate_all_unblocks_streaming_command() {
        // Backgrounded sleep = grandchild; only a group kill reaps it.
        let (pgid, result) = kill_streaming_command("sh", &["-c", "sleep 300 & wait"]);

        assert!(result.is_err(), "killed command must report failure");
        let alive = unsafe { libc::kill(-(pgid as i32), 0) };
        assert_eq!(alive, -1, "whole process group must be dead");
    }

    // A shell whose child died first can exit 0 from a bare `wait`, so a
    // cancelled step could count as a success. The trap makes that certain.
    #[cfg(unix)]
    #[test]
    fn a_stopped_command_fails_even_when_it_exits_zero() {
        const ARGS: &[&str] = &["-c", "trap 'exit 0' TERM; sleep 300 & wait"];
        let (_, result) = kill_streaming_command("sh", ARGS);
        let err = result.expect_err("a stopped command must not report success");
        assert!(err.to_string().contains("was stopped"), "{err}");
    }

    #[cfg(unix)]
    #[test]
    fn a_stopped_command_fails_without_streaming_too() {
        let registry = Arc::new(ProcessRegistry::new());
        let ctx = ctx_with_registry(registry.clone());
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let script = "trap 'exit 0' TERM; sleep 300 & wait";
            let _ = tx.send(ctx.run_command("sh", &["-c", script]));
        });
        wait_until_registered(&registry);
        registry.terminate_all();
        let result = rx
            .recv_timeout(UNBLOCK_DEADLINE)
            .expect("must unblock after the kill");
        assert!(result.unwrap_err().to_string().contains("was stopped"));
    }

    #[cfg(windows)]
    #[test]
    fn terminate_all_unblocks_streaming_command_on_windows() {
        // ~300s on its own; only a tree kill makes it return early.
        let (_, result) = kill_streaming_command("ping", &["-n", "300", "127.0.0.1"]);

        assert!(result.is_err(), "killed command must report failure");
    }

    /// Poll `done` for up to 10s; true once it holds.
    #[cfg(windows)]
    fn wait_until(done: impl Fn() -> bool) -> bool {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        while std::time::Instant::now() < deadline {
            if done() {
                return true;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        done()
    }

    // The child exits on its own, so walking parent links from it (taskkill
    // /T) can't reach the grandchild; only the child's job still holds it.
    #[cfg(windows)]
    #[test]
    fn terminate_all_kills_orphaned_grandchild_on_windows() {
        use crate::process_registry::tests::windows_pid_alive;
        let registry = Arc::new(ProcessRegistry::new());
        let ctx = ctx_with_registry(registry.clone());
        let (tx, rx) = std::sync::mpsc::channel::<String>();

        // `start /b` runs the grandchild on cmd's pipes: cmd exits at once,
        // while the grandchild prints its PID and keeps the stream open.
        let handle = std::thread::spawn(move || {
            let (cmd, args) = shell("start /b powershell -NoProfile -Command $PID; Start-Sleep 30");
            ctx.run_command_streaming(cmd, &args, &|line| {
                let _ = tx.send(line.to_string());
            })
        });
        let child = wait_until_registered(&registry);
        let line = rx.recv_timeout(std::time::Duration::from_secs(20));
        let grandchild: u32 = line.unwrap().trim().parse().unwrap();
        assert!(wait_until(|| !windows_pid_alive(child)), "child must exit");

        registry.terminate_all();
        let killed = wait_until(|| !windows_pid_alive(grandchild));
        if !killed {
            // Clean up before failing so the stray grandchild can't linger.
            let pid = grandchild.to_string();
            let mut kill = Command::new("taskkill");
            let _ = kill
                .args(["/PID", &pid, "/F"])
                .stdout(Stdio::null())
                .status();
        }
        assert!(killed, "orphaned grandchild {grandchild} must be killed");
        // `cmd` exited 0, but the step was still streaming when it was
        // cancelled, so it reports stopped rather than success.
        let result = handle.join().expect("streaming thread panicked");
        let err = result.expect_err("a cancelled step must not report success");
        assert!(err.to_string().contains("was stopped"), "{err}");
    }

    // Matches unix: a command that finishes on its own leaves its background
    // processes running. Only cancel (`terminate_all`) kills the tree.
    #[cfg(windows)]
    #[test]
    fn finished_command_leaves_background_grandchild_running_on_windows() {
        use crate::process_registry::tests::windows_pid_alive;
        /// Long enough to catch a kill that lands as the command is reaped.
        const SURVIVE_WAIT_MS: u64 = 1500;
        let registry = Arc::new(ProcessRegistry::new());
        let ctx = ctx_with_registry(registry.clone());

        // Start-Process detaches the grandchild from our pipes, so the
        // command prints its PID and exits while the grandchild sleeps on.
        let script = "$p = Start-Process powershell -ArgumentList \
                      '-NoProfile','-Command','Start-Sleep 30' \
                      -PassThru -WindowStyle Hidden; $p.Id";
        let out = ctx.run_command("powershell", &["-NoProfile", "-Command", script]);
        let grandchild: u32 = String::from_utf8(out.unwrap())
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        assert!(registry.active().is_empty(), "command must be deregistered");

        std::thread::sleep(std::time::Duration::from_millis(SURVIVE_WAIT_MS));
        let alive = windows_pid_alive(grandchild);
        let pid = grandchild.to_string();
        let mut kill = Command::new("taskkill");
        let _ = kill
            .args(["/PID", &pid, "/F"])
            .stdout(Stdio::null())
            .status();
        assert!(
            alive,
            "background grandchild {grandchild} must outlive its command"
        );
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
            dry_run: false,
            scratch: RunScratch::default(),
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
            dry_run: false,
            scratch: RunScratch::default(),
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
            dry_run: false,
            scratch: RunScratch::default(),
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
            dry_run: false,
            scratch: RunScratch::default(),
        };
        assert_eq!(ctx.output_dir(), Some(PathBuf::from("/custom/bnto/output")),);
    }

    #[test]
    fn test_with_dry_run_sets_dry_run() {
        let ctx = NativeContext::current_dir().unwrap();
        assert!(!ctx.dry_run());
        assert!(ctx.with_dry_run(true).dry_run());
    }

    // --- Command options: env and timeout ---

    /// A command that runs far past any test's timeout.
    fn long_command() -> (&'static str, Vec<&'static str>) {
        if cfg!(windows) {
            ("ping", vec!["-n", "300", "127.0.0.1"])
        } else {
            ("sleep", vec!["300"])
        }
    }

    #[test]
    fn test_run_command_with_sets_env_for_the_command() {
        let ctx = ctx_with_registry(Arc::new(ProcessRegistry::new()));
        let script = if cfg!(windows) {
            "echo %BNTO_TEST_OPTION_VAR%"
        } else {
            "echo $BNTO_TEST_OPTION_VAR"
        };
        let (cmd, args) = shell(script);
        let options = bnto_core::CommandOptions {
            env: vec![("BNTO_TEST_OPTION_VAR".into(), "from options".into())],
            ..Default::default()
        };
        let out = ctx.run_command_with(cmd, &args, &options, &|_| {}).unwrap();
        assert_eq!(String::from_utf8_lossy(&out).trim(), "from options");
        assert!(
            std::env::var("BNTO_TEST_OPTION_VAR").is_err(),
            "bnto's own environment is unchanged"
        );
    }

    #[cfg(unix)]
    #[test]
    fn test_run_command_with_clear_env_keeps_only_the_allowlist() {
        // Cargo sets CARGO_MANIFEST_DIR for this test process; a cleared
        // command must not see it, but still finds programs on PATH.
        let ctx = ctx_with_registry(Arc::new(ProcessRegistry::new()));
        let (cmd, args) = shell(
            "printf '%s|%s' \"${CARGO_MANIFEST_DIR-unset}\" \"$BNTO_OPT\"; command -v ls >/dev/null && printf '|path'",
        );
        let options = bnto_core::CommandOptions {
            env: vec![("BNTO_OPT".into(), "mine".into())],
            clear_env: true,
            ..Default::default()
        };
        let out = ctx.run_command_with(cmd, &args, &options, &|_| {}).unwrap();
        assert_eq!(String::from_utf8_lossy(&out).trim_end(), "unset|mine|path");
    }

    #[test]
    fn test_run_command_with_kills_a_command_that_outlives_its_timeout() {
        let registry = Arc::new(ProcessRegistry::new());
        let ctx = ctx_with_registry(registry.clone());
        let (cmd, args) = long_command();
        let options = bnto_core::CommandOptions {
            timeout: Some(std::time::Duration::from_secs(1)),
            ..Default::default()
        };
        let started = std::time::Instant::now();
        let err = ctx
            .run_command_with(cmd, &args, &options, &|_| {})
            .expect_err("the command outlived its timeout");
        assert!(
            started.elapsed() < UNBLOCK_DEADLINE,
            "the kill unblocked it"
        );
        assert!(
            err.to_string().contains("timed out after 1s"),
            "the error names the timeout: {err}"
        );
        assert!(registry.active().is_empty(), "the child was deregistered");
    }

    #[test]
    fn test_run_command_with_leaves_a_quick_command_alone() {
        let ctx = ctx_with_registry(Arc::new(ProcessRegistry::new()));
        let (cmd, args) = shell("echo quick");
        let options = bnto_core::CommandOptions {
            timeout: Some(std::time::Duration::from_secs(30)),
            ..Default::default()
        };
        let out = ctx.run_command_with(cmd, &args, &options, &|_| {}).unwrap();
        assert_eq!(String::from_utf8_lossy(&out).trim(), "quick");
    }

    #[test]
    fn test_run_command_with_runs_the_command_in_options_dir() {
        let ctx = ctx_with_registry(Arc::new(ProcessRegistry::new()));
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("marker.txt"), b"").unwrap();
        let script = if cfg!(windows) {
            "if exist marker.txt echo here"
        } else {
            "test -f marker.txt && echo here"
        };
        let (cmd, args) = shell(script);
        let options = bnto_core::CommandOptions {
            dir: Some(dir.path().to_path_buf()),
            ..Default::default()
        };
        let out = ctx.run_command_with(cmd, &args, &options, &|_| {}).unwrap();
        assert_eq!(String::from_utf8_lossy(&out).trim(), "here");
    }
}
