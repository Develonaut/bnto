// First-run consent notice — print to stderr, mark as shown.

use super::config::TelemetryConfig;

/// Show consent notice if this is the first run. Returns whether
/// telemetry should be active for THIS invocation (false on first run
/// so users see the notice before data is collected).
pub fn handle_consent(config: &mut TelemetryConfig) -> bool {
    if config.consent_shown {
        return config.enabled;
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

    #[test]
    fn first_run_shows_consent_and_returns_false() {
        let mut config = TelemetryConfig {
            enabled: true,
            anonymous_id: "test".into(),
            consent_shown: false,
        };
        let active = handle_consent(&mut config);
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
        assert!(handle_consent(&mut config));
    }

    #[test]
    fn second_run_disabled_returns_false() {
        let mut config = TelemetryConfig {
            enabled: false,
            anonymous_id: "test".into(),
            consent_shown: true,
        };
        assert!(!handle_consent(&mut config));
    }
}
