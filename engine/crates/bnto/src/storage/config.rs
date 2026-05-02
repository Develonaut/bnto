// TOML-based config for bnto.
//
// Schema-versioned. Fields use serde(default) so new fields can be
// added without breaking existing config files.

use serde::{Deserialize, Serialize};

use super::atomic::atomic_write;
use super::paths::BntoPaths;

/// Current config schema version. Bump when the TOML structure changes.
const CURRENT_VERSION: u32 = 2;

/// Persistent settings in TOML format.
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

    /// Path overrides.
    #[serde(default)]
    pub paths: PathsSection,

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
pub struct PathsSection {
    /// Custom recipes directory (None = ~/.bnto/recipes/).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipes: Option<String>,
    /// Custom output directory (None = system temp).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
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
            paths: PathsSection::default(),
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

    /// Effective output directory — config override or None (= system temp).
    pub fn output_dir(&self) -> Option<&str> {
        self.paths.output.as_deref().filter(|s| !s.is_empty())
    }

    /// Effective recipes directory override — None means use BntoPaths default.
    pub fn recipes_dir_override(&self) -> Option<&str> {
        self.paths.recipes.as_deref().filter(|s| !s.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_paths() -> (tempfile::TempDir, BntoPaths) {
        let tmp = tempfile::tempdir().unwrap();
        let paths = BntoPaths {
            home: tmp.path().to_path_buf(),
        };
        paths.ensure_dirs().unwrap();
        (tmp, paths)
    }

    #[test]
    fn default_has_version_2() {
        let config = TomlConfig::default();
        assert_eq!(config.version, 2);
        assert_eq!(config.tui.theme, "los-angeles");
        assert!(config.paths.recipes.is_none());
        assert!(config.paths.output.is_none());
        assert!(config.telemetry.enabled);
    }

    #[test]
    fn paths_section_serializes() {
        let config = TomlConfig {
            paths: PathsSection {
                recipes: Some("/my/recipes".into()),
                output: Some("/my/output".into()),
            },
            ..TomlConfig::default()
        };

        let serialized = toml::to_string_pretty(&config).unwrap();
        assert!(serialized.contains("[paths]"));
        assert!(serialized.contains("recipes = \"/my/recipes\""));
        assert!(serialized.contains("output = \"/my/output\""));
    }

    #[test]
    fn paths_section_omits_none() {
        let config = TomlConfig::default();
        let serialized = toml::to_string_pretty(&config).unwrap();
        assert!(!serialized.contains("recipes ="));
        assert!(!serialized.contains("output ="));
    }

    #[test]
    fn save_load_roundtrip() {
        let (_tmp, paths) = test_paths();

        let config = TomlConfig {
            tui: TuiSection {
                theme: "tokyo".into(),
            },
            paths: PathsSection {
                recipes: Some("/recipes".into()),
                output: Some("/output".into()),
            },
            telemetry: TelemetrySection { enabled: false },
            ..TomlConfig::default()
        };

        config.save(&paths).unwrap();
        let loaded = TomlConfig::load(&paths);
        assert_eq!(loaded, config);
    }

    #[test]
    fn missing_file_returns_default() {
        let (_tmp, paths) = test_paths();
        let loaded = TomlConfig::load(&paths);
        assert_eq!(loaded, TomlConfig::default());
    }

    #[test]
    fn version_present_in_serialized() {
        let config = TomlConfig::default();
        let serialized = toml::to_string_pretty(&config).unwrap();
        assert!(
            serialized.contains("version = 2"),
            "serialized TOML should contain version = 2, got:\n{serialized}"
        );
    }

    #[test]
    fn serde_default_fields() {
        let minimal = "version = 2\n";
        let config: TomlConfig = toml::from_str(minimal).unwrap();
        assert_eq!(config.tui.theme, "los-angeles");
        assert!(config.paths.recipes.is_none());
        assert!(config.paths.output.is_none());
        assert!(config.telemetry.enabled);
    }

    #[test]
    fn output_dir_accessor() {
        let mut config = TomlConfig::default();
        assert!(config.output_dir().is_none());

        config.paths.output = Some("/tmp/out".into());
        assert_eq!(config.output_dir(), Some("/tmp/out"));

        config.paths.output = Some(String::new());
        assert!(
            config.output_dir().is_none(),
            "empty string treated as None"
        );
    }

    #[test]
    fn recipes_dir_override_accessor() {
        let mut config = TomlConfig::default();
        assert!(config.recipes_dir_override().is_none());

        config.paths.recipes = Some("/custom".into());
        assert_eq!(config.recipes_dir_override(), Some("/custom"));
    }

    #[test]
    fn preserves_all_fields() {
        let (_tmp, paths) = test_paths();

        let config = TomlConfig {
            version: 2,
            tui: TuiSection {
                theme: "tokyo".into(),
            },
            paths: PathsSection {
                recipes: Some("/recipes".into()),
                output: Some("/out".into()),
            },
            telemetry: TelemetrySection { enabled: false },
        };

        config.save(&paths).unwrap();
        let loaded = TomlConfig::load(&paths);

        assert_eq!(loaded.version, 2);
        assert_eq!(loaded.tui.theme, "tokyo");
        assert_eq!(loaded.paths.recipes, Some("/recipes".into()));
        assert_eq!(loaded.paths.output, Some("/out".into()));
        assert!(!loaded.telemetry.enabled);
    }

    #[test]
    fn v1_config_loads_with_defaults_for_new_fields() {
        let v1_toml = r#"
version = 1

[tui]
theme = "tokyo"

[output]
dir = "/old-output"

[picker]
default_path = "/old-default"

[telemetry]
enabled = false
"#;
        let config: TomlConfig = toml::from_str(v1_toml).unwrap();
        assert_eq!(config.tui.theme, "tokyo");
        assert!(config.paths.recipes.is_none());
        assert!(config.paths.output.is_none());
        assert!(!config.telemetry.enabled);
    }
}
