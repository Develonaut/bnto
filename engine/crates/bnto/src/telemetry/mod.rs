// CLI telemetry — anonymous, opt-out usage data via PostHog.
//
// Privacy: anonymous UUID v4, bucketed values (never exact),
// no file names/paths/contents, no PII. Disable via:
//   bnto telemetry disable
//   BNTO_TELEMETRY_DISABLED=1
//   DO_NOT_TRACK=1

pub mod bucket;
mod ci;
mod client;
pub mod config;
mod consent;
mod debug;
pub mod events;

use std::sync::OnceLock;

use config::TelemetryConfig;
use events::Event;

/// Global telemetry state, initialized once via `init()`.
struct TelemetryState {
    active: bool,
    api_key: String,
    config: TelemetryConfig,
}

static STATE: OnceLock<TelemetryState> = OnceLock::new();

/// Initialize telemetry. Call once early in `main()`.
///
/// Loads config, shows consent on first run, checks env vars.
/// Safe to call multiple times — only the first call takes effect.
pub fn init() {
    STATE.get_or_init(|| {
        let mut config = TelemetryConfig::load();

        // First-run consent — returns false on the notice invocation.
        let consent_active = consent::handle_consent(&mut config);

        // Check env var overrides.
        let env_disabled = is_env_disabled();
        let active = consent_active && config.enabled && !env_disabled;

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

/// Enable or disable telemetry, persisting the choice to config.
pub fn set_enabled(enabled: bool) {
    let mut config = TelemetryConfig::load();
    config.enabled = enabled;
    config.consent_shown = true;
    let _ = config.save();
}

/// Print current telemetry status to stderr.
pub fn print_status() {
    let config = TelemetryConfig::load();
    let env_disabled = is_env_disabled();

    eprintln!("Telemetry status:");
    eprintln!("  Config enabled: {}", config.enabled);
    eprintln!("  Consent shown:  {}", config.consent_shown);
    eprintln!("  Env disabled:   {env_disabled}");
    eprintln!(
        "  Effective:      {}",
        config.enabled && !env_disabled && config.consent_shown
    );
    eprintln!("  Anonymous ID:   {}", config.anonymous_id);

    if let Some(path) = config::config_path() {
        eprintln!("  Config file:    {}", path.display());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

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
