// TOML-based config for the TUI — replaces the old JSON format.
//
// Schema-versioned from day one. Fields use serde(default) so new
// fields can be added without breaking existing config files.

use serde::{Deserialize, Serialize};

use super::atomic::atomic_write;
use super::paths::BntoPaths;

/// Current config schema version. Bump when the TOML structure changes.
const CURRENT_VERSION: u32 = 1;

/// Persistent TUI settings in TOML format.
///
/// All fields have defaults so partial or empty config files
/// deserialize without error. The `version` field enables future
/// migration logic.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TomlConfig {
    /// Schema version for migration support.
    #[serde(default = "default_version")]
    pub version: u32,

    /// TUI-specific settings.
    #[serde(default)]
    pub tui: TuiSection,

    /// Output directory settings.
    #[serde(default)]
    pub output: OutputSection,

    /// File picker settings.
    #[serde(default)]
    pub picker: PickerSection,

    /// Telemetry consent settings.
    #[serde(default)]
    pub telemetry: TelemetrySection,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TuiSection {
    /// Theme variant: "los-angeles", "tokyo", or "monaco".
    #[serde(default = "default_theme")]
    pub theme: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OutputSection {
    /// Output directory for recipe results (None = temp dir).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dir: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PickerSection {
    /// Default starting directory for the file picker.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TelemetrySection {
    /// Whether anonymous telemetry is enabled.
    #[serde(default = "default_telemetry_enabled")]
    pub enabled: bool,
}

fn default_version() -> u32 {
    CURRENT_VERSION
}

fn default_theme() -> String {
    "los-angeles".to_string()
}

fn default_telemetry_enabled() -> bool {
    true
}

impl Default for TuiSection {
    fn default() -> Self {
        Self {
            theme: default_theme(),
        }
    }
}

impl Default for TelemetrySection {
    fn default() -> Self {
        Self {
            enabled: default_telemetry_enabled(),
        }
    }
}

impl Default for TomlConfig {
    fn default() -> Self {
        Self {
            version: CURRENT_VERSION,
            tui: TuiSection::default(),
            output: OutputSection::default(),
            picker: PickerSection::default(),
            telemetry: TelemetrySection::default(),
        }
    }
}

impl TomlConfig {
    /// Load config from disk, falling back to defaults on any error.
    pub fn load(paths: &BntoPaths) -> Self {
        let path = paths.config_file();
        let Ok(contents) = std::fs::read_to_string(&path) else {
            return Self::default();
        };
        toml::from_str(&contents).unwrap_or_default()
    }

    /// Save config to disk atomically.
    pub fn save(&self, paths: &BntoPaths) -> Result<(), std::io::Error> {
        let path = paths.config_file();
        let toml_str = toml::to_string_pretty(self)
            .map_err(|e| std::io::Error::other(format!("TOML serialize: {e}")))?;
        atomic_write(&path, toml_str.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_paths() -> (tempfile::TempDir, BntoPaths) {
        let tmp = tempfile::tempdir().unwrap();
        unsafe { std::env::set_var("BNTO_HOME", tmp.path().as_os_str()) };
        let paths = BntoPaths::resolve().unwrap();
        unsafe { std::env::remove_var("BNTO_HOME") };
        paths.ensure_dirs().unwrap();
        (tmp, paths)
    }

    #[test]
    fn toml_config_default() {
        let config = TomlConfig::default();
        assert_eq!(config.version, 1);
        assert_eq!(config.tui.theme, "los-angeles");
        assert!(config.output.dir.is_none());
        assert!(config.picker.default_path.is_none());
        assert!(config.telemetry.enabled);
    }

    #[test]
    fn toml_config_roundtrip() {
        let config = TomlConfig {
            version: 1,
            tui: TuiSection {
                theme: "tokyo".into(),
            },
            output: OutputSection {
                dir: Some("/tmp/out".into()),
            },
            picker: PickerSection {
                default_path: Some("/photos".into()),
            },
            telemetry: TelemetrySection { enabled: false },
        };

        let serialized = toml::to_string_pretty(&config).unwrap();
        let deserialized: TomlConfig = toml::from_str(&serialized).unwrap();
        assert_eq!(config, deserialized);
    }

    #[test]
    fn toml_config_save_load() {
        let (_tmp, paths) = test_paths();

        let config = TomlConfig {
            tui: TuiSection {
                theme: "monaco".into(),
            },
            output: OutputSection {
                dir: Some("/output".into()),
            },
            ..TomlConfig::default()
        };

        config.save(&paths).unwrap();
        let loaded = TomlConfig::load(&paths);
        assert_eq!(loaded.tui.theme, "monaco");
        assert_eq!(loaded.output.dir, Some("/output".into()));
    }

    #[test]
    fn toml_config_missing_file_returns_default() {
        let (_tmp, paths) = test_paths();
        // Don't save anything — config file doesn't exist.
        let loaded = TomlConfig::load(&paths);
        assert_eq!(loaded, TomlConfig::default());
    }

    #[test]
    fn toml_config_version_present() {
        let config = TomlConfig::default();
        let serialized = toml::to_string_pretty(&config).unwrap();
        assert!(
            serialized.contains("version = 1"),
            "serialized TOML should contain version = 1, got:\n{serialized}"
        );
    }

    #[test]
    fn toml_config_serde_default_fields() {
        // A minimal TOML with only the version field — all others should default.
        let minimal = "version = 1\n";
        let config: TomlConfig = toml::from_str(minimal).unwrap();
        assert_eq!(config.tui.theme, "los-angeles");
        assert!(config.output.dir.is_none());
        assert!(config.picker.default_path.is_none());
        assert!(config.telemetry.enabled);
    }

    #[test]
    fn toml_config_preserves_all_fields() {
        let (_tmp, paths) = test_paths();

        let config = TomlConfig {
            version: 1,
            tui: TuiSection {
                theme: "tokyo".into(),
            },
            output: OutputSection {
                dir: Some("/out".into()),
            },
            picker: PickerSection {
                default_path: Some("/photos".into()),
            },
            telemetry: TelemetrySection { enabled: false },
        };

        config.save(&paths).unwrap();
        let loaded = TomlConfig::load(&paths);

        assert_eq!(loaded.version, 1);
        assert_eq!(loaded.tui.theme, "tokyo");
        assert_eq!(loaded.output.dir, Some("/out".into()));
        assert_eq!(loaded.picker.default_path, Some("/photos".into()));
        assert!(!loaded.telemetry.enabled);
    }
}
