// Security validation for shell-command parameters.
//
// This module is the security boundary between recipe JSON and system
// command execution. Every command passes through these checks before
// reaching ProcessContext::run_command().

/// Shell interpreters that must never be invoked as commands.
/// Allowing these would let a recipe bypass the argument separation
/// safety that std::process::Command provides (e.g., `bash -c "rm -rf /"`).
const SHELL_DENYLIST: &[&str] = &[
    "bash",
    "sh",
    "zsh",
    "fish",
    "dash",
    "csh",
    "tcsh",
    "ksh",
    "cmd",
    "cmd.exe",
    "powershell",
    "pwsh",
];

/// Environment variables that must never be set by recipe JSON.
/// These can hijack the subprocess in dangerous ways:
///   - LD_PRELOAD / DYLD_*: load arbitrary shared libraries
///   - PATH: redirect binary resolution to attacker-controlled dirs
///   - HOME / SHELL / TMPDIR: alter tool behavior unpredictably
const DENIED_ENV_VARS: &[&str] = &[
    "LD_PRELOAD",
    "LD_LIBRARY_PATH",
    "DYLD_INSERT_LIBRARIES",
    "DYLD_LIBRARY_PATH",
    "DYLD_FRAMEWORK_PATH",
    "PATH",
    "HOME",
    "SHELL",
    "TMPDIR",
];

/// Default maximum bytes per output file.
/// Configurable per-node via the `maxOutputSize` parameter (in MB).
pub const DEFAULT_MAX_OUTPUT_MB: u64 = 500;

/// Convert a megabyte value to bytes for comparison.
pub fn max_output_bytes(mb: u64) -> usize {
    (mb as usize) * 1024 * 1024
}

/// Default timeout in seconds for command execution.
pub const DEFAULT_TIMEOUT_SECS: u64 = 300;

/// Validate that a command name is safe to execute.
///
/// Rejects:
///   - Empty commands
///   - Shell interpreters (bash, sh, zsh, powershell, etc.)
///   - Paths (absolute, relative, or parent traversal)
///
/// Commands must be bare binary names resolved via PATH by the OS.
pub fn validate_command(cmd: &str) -> Result<(), String> {
    if cmd.is_empty() {
        return Err("command must not be empty".to_string());
    }

    // Reject shell interpreters — they bypass argument separation safety.
    // Compare the name Windows would run, so `bash.exe` and `cmd.` count.
    let stem = program_stem(cmd);
    for shell in SHELL_DENYLIST {
        if stem == program_stem(shell) {
            return Err(format!(
                "'{cmd}' is a shell interpreter and cannot be used as a command. \
                 Use the specific tool directly (e.g., 'ffmpeg', 'yt-dlp')"
            ));
        }
    }

    // Reject paths — commands must be bare binary names
    if cmd.contains('/') || cmd.contains('\\') {
        return Err(format!(
            "'{cmd}' contains a path separator. Commands must be bare binary \
             names (e.g., 'ffmpeg', not './ffmpeg' or '/usr/bin/ffmpeg')"
        ));
    }

    // Reject hidden-dot prefix (e.g., ".malicious")
    if cmd.starts_with('.') {
        return Err(format!(
            "'{cmd}' starts with a dot. Commands must be bare binary names"
        ));
    }

    Ok(())
}

/// The program name Windows resolves `cmd` to, case-folded: trailing dots
/// and spaces dropped (Windows ignores them) and a `.exe` / `.com` suffix
/// removed. On other OSes this only matters for the comparison.
pub(crate) fn program_stem(cmd: &str) -> String {
    let lower = cmd.to_lowercase();
    let name = lower.trim_end_matches(['.', ' ']);
    let name = name
        .strip_suffix(".exe")
        .or_else(|| name.strip_suffix(".com"))
        .unwrap_or(name);
    name.trim_end_matches(['.', ' ']).to_string()
}

/// Remove dangerous environment variables from a recipe-provided env map.
///
/// Returns the sanitized map (dangerous vars silently stripped).
/// This is intentionally silent — we don't want to leak information about
/// which vars are blocked, and benign recipes won't set these.
pub fn sanitize_env(
    env: &serde_json::Map<String, serde_json::Value>,
) -> serde_json::Map<String, serde_json::Value> {
    env.iter()
        .filter(|(key, _)| {
            let key_upper = key.to_uppercase();
            !DENIED_ENV_VARS.contains(&key_upper.as_str()) && !key_upper.starts_with("DYLD_")
        })
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Shell denylist ---

    #[test]
    fn rejects_bash_command() {
        let result = validate_command("bash");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("shell interpreter"));
    }

    #[test]
    fn rejects_sh_command() {
        assert!(validate_command("sh").is_err());
    }

    #[test]
    fn rejects_zsh_command() {
        assert!(validate_command("zsh").is_err());
    }

    #[test]
    fn rejects_powershell_command() {
        assert!(validate_command("powershell").is_err());
    }

    #[test]
    fn rejects_cmd_exe_command() {
        assert!(validate_command("cmd.exe").is_err());
    }

    #[test]
    fn rejects_pwsh_command() {
        assert!(validate_command("pwsh").is_err());
    }

    #[test]
    fn rejects_fish_command() {
        assert!(validate_command("fish").is_err());
    }

    #[test]
    fn rejects_shell_case_insensitive() {
        assert!(validate_command("BASH").is_err());
        assert!(validate_command("Sh").is_err());
        assert!(validate_command("PowerShell").is_err());
    }

    // --- Path validation ---

    // Windows resolves `bash.exe`, `cmd.` and `pwsh .` to the same shells
    // the denylist names (SEC-05).
    #[test]
    fn rejects_windows_spellings_of_shells() {
        for cmd in [
            "bash.exe",
            "sh.exe",
            "zsh.EXE",
            "PowerShell.exe",
            "pwsh.exe",
            "cmd.com",
            "cmd.",
            "bash.",
            "pwsh .",
            "bash.exe.",
            "powershell.exe  ",
        ] {
            let err = validate_command(cmd).expect_err(cmd);
            assert!(err.contains("shell interpreter"), "{cmd}: {err}");
        }
    }

    #[test]
    fn allows_tools_whose_names_only_start_like_shells() {
        for cmd in [
            "shellcheck",
            "bashate",
            "cmdtool",
            "python.exe",
            "ffmpeg.exe",
            "sh-tool",
        ] {
            assert!(validate_command(cmd).is_ok(), "{cmd} refused");
        }
    }

    #[test]
    fn rejects_relative_path_command() {
        let result = validate_command("./malicious");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("path separator"));
    }

    #[test]
    fn rejects_absolute_path_command() {
        let result = validate_command("/tmp/evil");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("path separator"));
    }

    #[test]
    fn rejects_parent_traversal_command() {
        let result = validate_command("../../../bin/evil");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_windows_path_command() {
        let result = validate_command("C:\\Windows\\System32\\cmd.exe");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_dot_prefixed_command() {
        let result = validate_command(".hidden-binary");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("starts with a dot"));
    }

    // --- Empty / missing ---

    #[test]
    fn rejects_empty_command() {
        let result = validate_command("");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must not be empty"));
    }

    // --- Valid commands ---

    #[test]
    fn accepts_legitimate_binaries() {
        assert!(validate_command("ffmpeg").is_ok());
        assert!(validate_command("yt-dlp").is_ok());
        assert!(validate_command("convert").is_ok());
        assert!(validate_command("curl").is_ok());
        assert!(validate_command("python3").is_ok());
    }

    // --- Env var sanitization ---

    #[test]
    fn strips_ld_preload_from_env() {
        let mut env = serde_json::Map::new();
        env.insert(
            "LD_PRELOAD".to_string(),
            serde_json::Value::String("/tmp/evil.so".to_string()),
        );
        env.insert(
            "RUST_LOG".to_string(),
            serde_json::Value::String("debug".to_string()),
        );
        let sanitized = sanitize_env(&env);
        assert!(!sanitized.contains_key("LD_PRELOAD"));
        assert!(sanitized.contains_key("RUST_LOG"));
    }

    #[test]
    fn strips_path_from_env() {
        let mut env = serde_json::Map::new();
        env.insert(
            "PATH".to_string(),
            serde_json::Value::String("/tmp/malicious".to_string()),
        );
        let sanitized = sanitize_env(&env);
        assert!(!sanitized.contains_key("PATH"));
    }

    #[test]
    fn strips_dyld_vars_from_env() {
        let mut env = serde_json::Map::new();
        env.insert(
            "DYLD_INSERT_LIBRARIES".to_string(),
            serde_json::Value::String("/tmp/evil.dylib".to_string()),
        );
        env.insert(
            "DYLD_FALLBACK_LIBRARY_PATH".to_string(),
            serde_json::Value::String("/tmp".to_string()),
        );
        let sanitized = sanitize_env(&env);
        assert!(!sanitized.contains_key("DYLD_INSERT_LIBRARIES"));
        assert!(!sanitized.contains_key("DYLD_FALLBACK_LIBRARY_PATH"));
    }

    #[test]
    fn strips_home_shell_tmpdir() {
        let mut env = serde_json::Map::new();
        env.insert("HOME".into(), serde_json::Value::String("/tmp".into()));
        env.insert("SHELL".into(), serde_json::Value::String("/bin/sh".into()));
        env.insert("TMPDIR".into(), serde_json::Value::String("/tmp".into()));
        let sanitized = sanitize_env(&env);
        assert!(sanitized.is_empty());
    }

    #[test]
    fn allows_safe_env_vars() {
        let mut env = serde_json::Map::new();
        env.insert(
            "RUST_LOG".to_string(),
            serde_json::Value::String("debug".to_string()),
        );
        env.insert(
            "FFMPEG_THREADS".to_string(),
            serde_json::Value::String("4".to_string()),
        );
        let sanitized = sanitize_env(&env);
        assert_eq!(sanitized.len(), 2);
        assert!(sanitized.contains_key("RUST_LOG"));
        assert!(sanitized.contains_key("FFMPEG_THREADS"));
    }

    #[test]
    fn env_var_denylist_is_case_insensitive() {
        let mut env = serde_json::Map::new();
        env.insert(
            "ld_preload".to_string(),
            serde_json::Value::String("/tmp/evil.so".to_string()),
        );
        env.insert(
            "path".to_string(),
            serde_json::Value::String("/tmp".to_string()),
        );
        let sanitized = sanitize_env(&env);
        assert!(sanitized.is_empty());
    }
}
