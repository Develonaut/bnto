// The environment a command started with `CommandOptions::clear_env` keeps
// from bnto's own: just what tools need to find programs, files and the
// user's locale. Everything else — secrets, BASH_ENV, SHELLOPTS, BASH_FUNC_*,
// ZDOTDIR — stays behind (SEC-22, SEC-26).

/// Names passed through exactly.
const KEPT: &[&str] = &[
    "PATH",
    "HOME",
    "USER",
    "LOGNAME",
    "LANG",
    "TERM",
    "TMPDIR",
    "TZ",
    // Windows needs these to start programs and `.cmd` shims at all.
    "SYSTEMROOT",
    "SYSTEMDRIVE",
    "WINDIR",
    "PATHEXT",
    "COMSPEC",
    "TEMP",
    "TMP",
    "USERPROFILE",
    "APPDATA",
    "LOCALAPPDATA",
];

/// Whether a command with a cleared environment keeps `name` from bnto's.
pub fn keeps(name: &str) -> bool {
    let upper = name.to_uppercase();
    KEPT.contains(&upper.as_str()) || upper.starts_with("LC_")
}

/// The variables a cleared command starts with, from `vars` (bnto's own).
pub fn kept_vars(
    vars: impl Iterator<Item = (std::ffi::OsString, std::ffi::OsString)>,
) -> Vec<(std::ffi::OsString, std::ffi::OsString)> {
    vars.filter(|(name, _)| name.to_str().is_some_and(keeps))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_what_tools_need_to_run() {
        for name in [
            "PATH",
            "HOME",
            "LANG",
            "LC_ALL",
            "LC_CTYPE",
            "TERM",
            "Path",
            "SystemRoot",
        ] {
            assert!(keeps(name), "{name}");
        }
    }

    #[test]
    fn drops_secrets_and_shell_startup_hooks() {
        for name in [
            "AWS_SECRET_ACCESS_KEY",
            "GITHUB_TOKEN",
            "BASH_ENV",
            "ENV",
            "SHELLOPTS",
            "PS4",
            "BASH_FUNC_ls%%",
            "ZDOTDIR",
            "FPATH",
            "LD_PRELOAD",
            "DYLD_INSERT_LIBRARIES",
            "NODE_OPTIONS",
            "SSH_AUTH_SOCK",
        ] {
            assert!(!keeps(name), "{name}");
        }
    }

    #[test]
    fn kept_vars_filters_an_environment() {
        let env =
            [("PATH", "/bin"), ("SECRET", "x"), ("LC_ALL", "C")].map(|(k, v)| (k.into(), v.into()));
        let kept: Vec<_> = kept_vars(env.into_iter())
            .into_iter()
            .map(|(k, _)| k.into_string().unwrap())
            .collect();
        assert_eq!(kept, ["PATH", "LC_ALL"]);
    }
}
