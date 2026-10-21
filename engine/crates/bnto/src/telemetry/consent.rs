// First-run consent notice — print to stderr, mark as shown.

use super::config::TelemetryConfig;

/// Whether to print the first-run notice. Skipped when env vars disable
/// telemetry: nothing is collected, so there is nothing to announce.
pub fn should_show_notice(consent_shown: bool, env_disabled: bool) -> bool {
    !consent_shown && !env_disabled
}

/// Show consent notice if this is the first run. Returns whether
/// telemetry should be active for THIS invocation (false on first run
/// so users see the notice before data is collected).
///
/// When `env_disabled`, the notice is skipped and `consent_shown` stays
/// false, so it still appears once if the env opt-out is later removed.
pub fn handle_consent(config: &mut TelemetryConfig, env_disabled: bool) -> bool {
    if config.consent_shown {
        return config.enabled;
    }
    if !should_show_notice(config.consent_shown, env_disabled) {
        return false;
    }

    eprintln!(
        "\n  bnto collects anonymous usage data to improve the tool.\n  \
         To disable: bnto telemetry disable\n  \
         More info: https://bnto.io/telemetry\n"
    );

    config.consent_shown = true;
    let _ = config.save();

    // Don't collect on the invocation that shows the notice.
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- should_show_notice: env opt-out suppresses the first-run notice ---

    #[test]
    fn notice_shows_on_first_run() {
        assert!(should_show_notice(false, false));
    }

    #[test]
    fn notice_skipped_once_shown() {
        assert!(!should_show_notice(true, false));
    }

    #[test]
    fn notice_skipped_when_env_disables_telemetry() {
        assert!(!should_show_notice(false, true));
    }

    #[test]
    fn notice_skipped_when_shown_and_env_disabled() {
        assert!(!should_show_notice(true, true));
    }

    #[test]
    fn env_disabled_first_run_leaves_notice_pending() {
        let mut config = TelemetryConfig {
            enabled: true,
            anonymous_id: "test".into(),
            consent_shown: false,
        };
        assert!(!handle_consent(&mut config, true));
        assert!(
            !config.consent_shown,
            "notice should still show once telemetry is enabled by env"
        );
    }

    #[test]
    fn first_run_shows_consent_and_returns_false() {
        let mut config = TelemetryConfig {
            enabled: true,
            anonymous_id: "test".into(),
            consent_shown: false,
        };
        let active = handle_consent(&mut config, false);
        assert!(!active, "should not be active on first run");
        assert!(config.consent_shown, "should mark as shown");
    }

    #[test]
    fn second_run_skips_consent_and_returns_enabled() {
        let mut config = TelemetryConfig {
            enabled: true,
            anonymous_id: "test".into(),
            consent_shown: true,
        };
        assert!(handle_consent(&mut config, false));
    }

    #[test]
    fn second_run_disabled_returns_false() {
        let mut config = TelemetryConfig {
            enabled: false,
            anonymous_id: "test".into(),
            consent_shown: true,
        };
        assert!(!handle_consent(&mut config, false));
    }
}
