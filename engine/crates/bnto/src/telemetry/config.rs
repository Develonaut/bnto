// Telemetry config persistence — load/save to ~/.config/bnto/telemetry.json.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Persistent telemetry settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryConfig {
    /// The on/off choice from versions that kept it here. Read once to
    /// move an opt-out into config.toml (`choice::settle`), never written.
    #[serde(rename = "enabled", default, skip_serializing)]
    pub legacy_enabled: Option<bool>,

    /// Anonymous UUID v4 — no PII, generated on first run.
    #[serde(default = "generate_anonymous_id")]
    pub anonymous_id: String,

    /// Whether the first-run consent notice has been shown.
    #[serde(default)]
    pub consent_shown: bool,
}

fn generate_anonymous_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            legacy_enabled: None,
            anonymous_id: generate_anonymous_id(),
            consent_shown: false,
        }
    }
}

impl TelemetryConfig {
    /// Load config from disk, falling back to defaults on any error.
    pub fn load() -> Self {
        let Some(path) = config_path() else {
            return Self::default();
        };
        let Ok(contents) = std::fs::read_to_string(&path) else {
            return Self::default();
        };
        serde_json::from_str(&contents).unwrap_or_default()
    }

    /// Save config to disk, creating parent directories as needed.
    pub fn save(&self) -> Result<(), String> {
        let path = config_path().ok_or("Could not determine config directory")?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {e}"))?;
        }
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {e}"))?;
        std::fs::write(&path, json).map_err(|e| format!("Failed to write config: {e}"))
    }
}

/// Config file path: `<config dir>/bnto/telemetry.json` (`~/.config` on
/// Linux, `~/Library/Application Support` on macOS). Unit tests get none, so
/// loading falls back to defaults and saving does nothing: a test that
/// toggles telemetry must never rewrite the developer's own choice.
pub fn config_path() -> Option<PathBuf> {
    if cfg!(test) {
        return None;
    }
    dirs::config_dir().map(|d| d.join("bnto").join("telemetry.json"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_tests_never_read_or_write_the_real_config() {
        assert_eq!(config_path(), None);
        assert!(TelemetryConfig::default().save().is_err());
        assert_eq!(
            TelemetryConfig::load().legacy_enabled,
            None,
            "falls back to the default"
        );
    }

    #[test]
    fn default_config_values() {
        let c = TelemetryConfig::default();
        assert_eq!(c.legacy_enabled, None);
        assert!(!c.anonymous_id.is_empty());
        assert!(!c.consent_shown);
    }

    #[test]
    fn anonymous_id_is_uuid_v4_format() {
        let c = TelemetryConfig::default();
        let parsed = uuid::Uuid::parse_str(&c.anonymous_id);
        assert!(parsed.is_ok(), "anonymous_id should be valid UUID");
        assert_eq!(
            parsed.unwrap().get_version(),
            Some(uuid::Version::Random),
            "should be v4"
        );
    }

    #[test]
    fn each_default_gets_unique_id() {
        let a = TelemetryConfig::default();
        let b = TelemetryConfig::default();
        assert_ne!(a.anonymous_id, b.anonymous_id);
    }

    #[test]
    fn save_and_load_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("bnto").join("telemetry.json");

        let config = TelemetryConfig {
            legacy_enabled: None,
            anonymous_id: "test-uuid-1234".to_string(),
            consent_shown: true,
        };

        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        let json = serde_json::to_string_pretty(&config).unwrap();
        std::fs::write(&path, &json).unwrap();

        let loaded: TelemetryConfig =
            serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(loaded.anonymous_id, "test-uuid-1234");
        assert!(loaded.consent_shown);
    }

    #[test]
    fn load_handles_partial_json() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("telemetry.json");
        std::fs::write(&path, r#"{"enabled":false}"#).unwrap();

        let loaded: TelemetryConfig =
            serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(loaded.legacy_enabled, Some(false));
        assert!(!loaded.anonymous_id.is_empty(), "should generate default");
        assert!(!loaded.consent_shown);
    }

    #[test]
    fn load_handles_invalid_json() {
        let result: Result<TelemetryConfig, _> = serde_json::from_str("not valid json!!!");
        assert!(result.is_err());
        let fallback = result.unwrap_or_default();
        assert_eq!(fallback.legacy_enabled, None);
    }
}
