// TUI config — in-memory settings struct used during migration from old JSON format.
// Loading/saving now goes through TomlConfig + BntoPaths.

use serde::{Deserialize, Serialize};

/// Persistent TUI settings saved between sessions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiConfig {
    /// Theme variant name: "los-angeles", "tokyo", or "monaco".
    #[serde(default = "default_theme")]
    pub theme: String,
    /// Default starting directory for the file picker (None = cwd).
    #[serde(default)]
    pub default_path: Option<String>,
    /// Output directory for recipe results (None = temp dir).
    #[serde(default)]
    pub output_dir: Option<String>,
}

fn default_theme() -> String {
    "los-angeles".to_string()
}

impl Default for TuiConfig {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            default_path: None,
            output_dir: None,
        }
    }
}

// NOTE: TuiConfig::load() and config_path() removed — loading now goes through
// TomlConfig::load(&BntoPaths) with automatic migration from old JSON format.
// TuiConfig is still used as an in-memory compatibility struct for settings.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_has_los_angeles_theme() {
        let c = TuiConfig::default();
        assert_eq!(c.theme, "los-angeles");
        assert!(c.default_path.is_none());
        assert!(c.output_dir.is_none());
    }

    #[test]
    fn serde_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("bnto").join("tui.json");

        let config = TuiConfig {
            theme: "tokyo".to_string(),
            default_path: Some("/home/user/photos".to_string()),
            output_dir: Some("/tmp/bnto-out".to_string()),
        };

        // Manually save to the temp path.
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        let json = serde_json::to_string_pretty(&config).unwrap();
        std::fs::write(&path, &json).unwrap();

        // Read it back.
        let loaded: TuiConfig =
            serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(loaded.theme, "tokyo");
        assert_eq!(loaded.default_path, Some("/home/user/photos".to_string()));
        assert_eq!(loaded.output_dir, Some("/tmp/bnto-out".to_string()));
    }

    #[test]
    fn load_handles_partial_json() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("tui.json");
        std::fs::write(&path, r#"{"theme":"monaco"}"#).unwrap();

        let loaded: TuiConfig =
            serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(loaded.theme, "monaco");
        assert!(loaded.default_path.is_none());
        assert!(loaded.output_dir.is_none());
    }

    #[test]
    fn load_handles_invalid_json() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("tui.json");
        std::fs::write(&path, "not valid json!!!").unwrap();

        let result: Result<TuiConfig, _> =
            serde_json::from_str(&std::fs::read_to_string(&path).unwrap());
        // Should fail to parse — load() would return defaults.
        assert!(result.is_err());
        // Verify the fallback path works:
        let fallback = result.unwrap_or_default();
        assert_eq!(fallback.theme, "los-angeles");
    }

    #[test]
    fn serde_nested_dir_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let nested = dir.path().join("deep").join("nested").join("bnto");

        let config = TuiConfig {
            theme: "monaco".to_string(),
            default_path: None,
            output_dir: None,
        };

        // Manually test the create_dir_all + write pattern.
        std::fs::create_dir_all(&nested).unwrap();
        let path = nested.join("tui.json");
        let json = serde_json::to_string_pretty(&config).unwrap();
        std::fs::write(&path, &json).unwrap();

        assert!(path.exists());
        let loaded: TuiConfig =
            serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(loaded.theme, "monaco");
    }
}
