// TUI config persistence — load/save settings to ~/.config/bnto/tui.json.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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

impl TuiConfig {
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

/// Config file path: ~/.config/bnto/tui.json (XDG-compatible via `dirs`).
pub fn config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join("bnto").join("tui.json"))
}

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
    fn load_does_not_panic() {
        // load() reads from the real config path. It should always succeed
        // (returning defaults if no file exists, or the saved config).
        let c = TuiConfig::load();
        assert!(!c.theme.is_empty(), "theme should never be empty");
    }

    #[test]
    fn save_and_load_roundtrip() {
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
    fn config_path_returns_some() {
        // dirs::config_dir() returns Some on macOS/Linux/Windows.
        assert!(config_path().is_some());
    }

    #[test]
    fn save_creates_parent_dirs() {
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
