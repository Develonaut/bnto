// When the update check may run and its notice may show. Never in CI, never
// for piped or scripted runs (stderr isn't a terminal), and never once the
// user opts out with BNTO_NO_UPDATE_CHECK=1 or `[updates] check = false`.

/// Everything that decides whether the update check runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Context {
    /// `[updates] check` in config.toml.
    pub config_allows: bool,
    /// `BNTO_NO_UPDATE_CHECK` is set to something other than "" or "0".
    pub env_opted_out: bool,
    pub in_ci: bool,
    pub stderr_is_terminal: bool,
}

/// Whether the update check may run (and its notice show) in `context`.
pub fn checks_enabled(context: Context) -> bool {
    context.config_allows && !context.env_opted_out && !context.in_ci && context.stderr_is_terminal
}

/// Whether `BNTO_NO_UPDATE_CHECK`'s value opts out: set, and not "" or "0".
pub fn env_opts_out(value: Option<&str>) -> bool {
    value.is_some_and(|v| !v.is_empty() && v != "0")
}

#[cfg(test)]
mod tests {
    use super::*;

    const ON: Context = Context {
        config_allows: true,
        env_opted_out: false,
        in_ci: false,
        stderr_is_terminal: true,
    };

    #[test]
    fn an_interactive_run_checks() {
        assert!(checks_enabled(ON));
    }

    #[test]
    fn each_opt_out_turns_it_off() {
        assert!(!checks_enabled(Context {
            config_allows: false,
            ..ON
        }));
        assert!(!checks_enabled(Context {
            env_opted_out: true,
            ..ON
        }));
        assert!(!checks_enabled(Context { in_ci: true, ..ON }));
        assert!(!checks_enabled(Context {
            stderr_is_terminal: false,
            ..ON
        }));
    }

    #[test]
    fn the_env_var_reads_like_a_flag() {
        assert!(env_opts_out(Some("1")));
        assert!(env_opts_out(Some("true")));
        assert!(!env_opts_out(Some("0")));
        assert!(!env_opts_out(Some("")));
        assert!(!env_opts_out(None));
    }
}
