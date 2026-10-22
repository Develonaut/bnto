// CLI telemetry — anonymous, opt-out usage data via PostHog.
//
// Privacy: anonymous UUID v4, bucketed values (never exact),
// no file names/paths/contents, no PII. Disable via:
//   bnto telemetry disable
//   BNTO_TELEMETRY_DISABLED=1
//   DO_NOT_TRACK=1

pub mod bucket;
pub mod choice;
pub(crate) mod ci;
mod client;
pub mod config;
mod consent;
mod debug;
pub mod events;

use std::sync::OnceLock;

use config::TelemetryConfig;
use events::Event;

use crate::storage::paths::BntoPaths;

/// Global telemetry state, initialized once via `init()`.
struct TelemetryState {
    active: bool,
    api_key: String,
    config: TelemetryConfig,
}

static STATE: OnceLock<TelemetryState> = OnceLock::new();

/// Initialize telemetry. Call once early in `main()`.
///
/// Loads config, shows consent on first run, checks env vars. The on/off
/// choice comes from `paths`' config.toml; without a bnto home it falls
/// back to what telemetry.json still says.
/// Safe to call multiple times — only the first call takes effect.
pub fn init(paths: Option<&BntoPaths>) {
    STATE.get_or_init(|| {
        let mut config = TelemetryConfig::load();
        let chosen = match paths {
            Some(paths) => choice::settle(paths, &mut config),
            None => config.legacy_enabled.unwrap_or(true),
        };

        // Env opt-out also suppresses the first-run notice.
        let env_disabled = is_env_disabled();

        // First-run consent — returns false on the notice invocation.
        let consent_active = consent::handle_consent(&mut config, env_disabled);
        let active = compute_active(consent_active, chosen, env_disabled);

        // Read API key from env (write-only key, safe to embed).
        let api_key = std::env::var("BNTO_POSTHOG_KEY").unwrap_or_default();

        TelemetryState {
            active: active && !api_key.is_empty(),
            api_key,
            config,
        }
    });
}

/// Capture a telemetry event. Fire-and-forget — silent on failure.
pub fn capture(event: Event) {
    let Some(state) = STATE.get() else { return };

    if debug::is_debug() {
        debug::debug_event(&event);
    }

    if !state.active {
        return;
    }

    // Enrich with CI context.
    let mut event = event;
    if ci::is_ci() {
        event.properties.insert("is_ci".into(), true.into());
        if let Some(provider) = ci::ci_provider() {
            event
                .properties
                .insert("ci_provider".into(), provider.into());
        }
    }

    // Fire-and-forget — ignore errors.
    let _ = client::send_event(&state.api_key, &state.config.anonymous_id, &event);
}

/// Whether telemetry is currently active.
pub fn is_enabled() -> bool {
    STATE.get().is_some_and(|s| s.active)
}

/// Check whether env vars disable telemetry.
fn is_env_disabled() -> bool {
    std::env::var("BNTO_TELEMETRY_DISABLED").is_ok_and(|v| v == "1")
        || std::env::var("DO_NOT_TRACK").is_ok_and(|v| v == "1")
}

/// Compute whether telemetry should be active given inputs.
///
/// Extracted for testability — `init()` feeds this the same values.
fn compute_active(consent_active: bool, config_enabled: bool, env_disabled: bool) -> bool {
    consent_active && config_enabled && !env_disabled
}

/// Enable or disable telemetry: save the choice to config.toml, and count
/// it as having seen the notice.
pub fn set_enabled(paths: &BntoPaths, enabled: bool) -> std::io::Result<()> {
    choice::set(paths, enabled)?;
    let mut config = TelemetryConfig::load();
    config.consent_shown = true;
    let _ = config.save();
    Ok(())
}

/// Print current telemetry status to stderr.
pub fn print_status(paths: &BntoPaths) {
    let mut config = TelemetryConfig::load();
    let chosen = choice::settle(paths, &mut config);
    let env_disabled = is_env_disabled();

    eprintln!("Telemetry status:");
    eprintln!("  Config enabled: {chosen}");
    eprintln!("  Consent shown:  {}", config.consent_shown);
    eprintln!("  Env disabled:   {env_disabled}");
    eprintln!(
        "  Effective:      {}",
        chosen && !env_disabled && config.consent_shown
    );
    eprintln!("  Anonymous ID:   {}", config.anonymous_id);
    eprintln!("  Config file:    {}", paths.config_file().display());
    if let Some(path) = config::config_path() {
        eprintln!("  State file:     {}", path.display());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    // --- compute_active: the decision function for whether telemetry sends data ---

    #[test]
    fn active_when_all_conditions_met() {
        assert!(compute_active(true, true, false));
    }

    #[test]
    fn inactive_when_config_disabled() {
        assert!(!compute_active(true, false, false));
    }

    #[test]
    fn inactive_when_consent_not_given() {
        assert!(!compute_active(false, true, false));
    }

    #[test]
    fn inactive_when_env_disabled() {
        assert!(!compute_active(true, true, true));
    }

    #[test]
    fn inactive_when_all_disabled() {
        assert!(!compute_active(false, false, true));
    }

    // --- set_enabled: the choice lands in config.toml ---

    #[test]
    fn set_enabled_writes_config_toml() {
        let dir = tempfile::tempdir().unwrap();
        let paths = BntoPaths {
            home: dir.path().to_path_buf(),
        };
        paths.ensure_dirs().unwrap();
        set_enabled(&paths, false).unwrap();
        let config = crate::storage::config::TomlConfig::load(&paths);
        assert!(!config.telemetry.enabled);
    }

    #[test]
    fn an_old_telemetry_json_choice_is_read_but_never_written_back() {
        let old = r#"{"enabled":false,"anonymous_id":"id","consent_shown":true}"#;
        let config: TelemetryConfig = serde_json::from_str(old).unwrap();
        assert_eq!(config.legacy_enabled, Some(false));
        let saved = serde_json::to_string(&config).unwrap();
        assert!(!saved.contains("enabled"), "{saved}");
    }

    #[test]
    fn disabled_config_means_inactive() {
        // Simulates the full decision: config disabled → not active.
        let config_enabled = false;
        assert!(!compute_active(true, config_enabled, false));
    }

    // --- env var tests ---

    #[test]
    fn is_env_disabled_respects_bnto_var() {
        let _guard = ENV_LOCK.lock().unwrap();
        let saved = std::env::var("BNTO_TELEMETRY_DISABLED").ok();
        let saved_dnt = std::env::var("DO_NOT_TRACK").ok();
        // SAFETY: tests run serially under ENV_LOCK.
        unsafe {
            std::env::remove_var("DO_NOT_TRACK");

            std::env::set_var("BNTO_TELEMETRY_DISABLED", "1");
            assert!(is_env_disabled());

            std::env::set_var("BNTO_TELEMETRY_DISABLED", "0");
            assert!(!is_env_disabled());

            std::env::remove_var("BNTO_TELEMETRY_DISABLED");
            assert!(!is_env_disabled());

            match saved {
                Some(v) => std::env::set_var("BNTO_TELEMETRY_DISABLED", v),
                None => std::env::remove_var("BNTO_TELEMETRY_DISABLED"),
            }
            match saved_dnt {
                Some(v) => std::env::set_var("DO_NOT_TRACK", v),
                None => std::env::remove_var("DO_NOT_TRACK"),
            }
        }
    }

    #[test]
    fn is_env_disabled_respects_do_not_track() {
        let _guard = ENV_LOCK.lock().unwrap();
        let saved = std::env::var("DO_NOT_TRACK").ok();
        let saved_bnto = std::env::var("BNTO_TELEMETRY_DISABLED").ok();
        // SAFETY: tests run serially under ENV_LOCK.
        unsafe {
            std::env::remove_var("BNTO_TELEMETRY_DISABLED");

            std::env::set_var("DO_NOT_TRACK", "1");
            assert!(is_env_disabled());

            std::env::set_var("DO_NOT_TRACK", "0");
            assert!(!is_env_disabled());

            match saved {
                Some(v) => std::env::set_var("DO_NOT_TRACK", v),
                None => std::env::remove_var("DO_NOT_TRACK"),
            }
            match saved_bnto {
                Some(v) => std::env::set_var("BNTO_TELEMETRY_DISABLED", v),
                None => std::env::remove_var("BNTO_TELEMETRY_DISABLED"),
            }
        }
    }
}
