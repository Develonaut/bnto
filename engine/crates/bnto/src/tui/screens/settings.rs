// Settings screen — TEA state + transitions for TUI configuration.
//
// Path fields open the file picker for directory browsing (Enter).
// Theme field cycles with left/right arrows.

use super::super::theme::ThemeVariant;
use crate::storage::config::TomlConfig;
use crate::telemetry;

/// A single configurable field in the settings screen.
#[derive(Debug, Clone)]
pub struct SettingsField {
    /// Machine key for identifying the field.
    pub key: &'static str,
    /// Human-readable label shown in the UI.
    pub label: &'static str,
    /// Current value as a display string.
    pub value: String,
    /// Help text shown below the field.
    pub description: &'static str,
    /// Whether this field opens the picker for browsing (false = theme picker).
    pub editable: bool,
}

/// Settings screen state.
#[derive(Debug, Clone)]
pub struct SettingsModel {
    /// Configurable fields.
    pub fields: Vec<SettingsField>,
    /// Which field currently has focus.
    pub focused: usize,
}

/// Messages the settings screen can handle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettingsMessage {
    /// Move focus to the next field.
    FocusNext,
    /// Move focus to the previous field.
    FocusPrev,
}

impl SettingsModel {
    /// Create a settings model from the TOML config and resolved paths.
    ///
    /// Path fields show the effective resolved path (config override or
    /// default from BntoPaths) so the user always sees where files live.
    pub fn from_toml_config(config: &TomlConfig, paths: &crate::storage::BntoPaths) -> Self {
        let recipes_value = config
            .paths
            .recipes
            .clone()
            .unwrap_or_else(|| paths.recipes_dir().to_string_lossy().into_owned());

        let fields = vec![
            SettingsField {
                key: "theme",
                label: "Theme",
                value: config.tui.theme.clone(),
                description: "Color theme (use arrow keys to cycle)",
                editable: false,
            },
            SettingsField {
                key: "recipes_dir",
                label: "Recipes Directory",
                value: recipes_value,
                description: "Where your recipes live (Enter to browse)",
                editable: true,
            },
            SettingsField {
                key: "output_dir",
                label: "Output Directory",
                value: config.paths.output.clone().unwrap_or_default(),
                description: "Recipe output directory (Enter to browse)",
                editable: true,
            },
            SettingsField {
                key: "telemetry",
                label: "Telemetry",
                value: if telemetry::config::TelemetryConfig::load().enabled {
                    "On".to_string()
                } else {
                    "Off".to_string()
                },
                description: "Anonymous usage data (use arrow keys to toggle)",
                editable: false,
            },
        ];
        Self { fields, focused: 0 }
    }

    /// Whether the currently focused field supports text editing.
    pub fn is_focused_editable(&self) -> bool {
        self.fields.get(self.focused).is_some_and(|f| f.editable)
    }

    /// Apply current settings back to a partial TomlConfig update.
    ///
    /// If the recipes dir matches the default BntoPaths path, store `None`
    /// so the config doesn't shadow the default (lets future default changes
    /// take effect automatically).
    pub fn apply_to_config(
        &self,
        config: &mut TomlConfig,
        variant: ThemeVariant,
        paths: &crate::storage::BntoPaths,
    ) {
        config.tui.theme = variant.as_slug().to_string();

        let default_recipes = paths.recipes_dir().to_string_lossy().into_owned();
        config.paths.recipes = self
            .fields
            .iter()
            .find(|f| f.key == "recipes_dir")
            .map(|f| f.value.clone())
            .filter(|v| !v.is_empty() && *v != default_recipes);

        config.paths.output = self
            .fields
            .iter()
            .find(|f| f.key == "output_dir")
            .map(|f| f.value.clone())
            .filter(|v| !v.is_empty());
    }
}

/// Pure state transition for the settings screen.
pub fn update(model: SettingsModel, msg: SettingsMessage) -> SettingsModel {
    match msg {
        SettingsMessage::FocusNext => {
            if model.fields.is_empty() {
                return model;
            }
            let next = (model.focused + 1) % model.fields.len();
            SettingsModel {
                focused: next,
                ..model
            }
        }
        SettingsMessage::FocusPrev => {
            if model.fields.is_empty() {
                return model;
            }
            let prev = if model.focused == 0 {
                model.fields.len() - 1
            } else {
                model.focused - 1
            };
            SettingsModel {
                focused: prev,
                ..model
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_paths() -> crate::storage::BntoPaths {
        crate::storage::BntoPaths {
            home: std::path::PathBuf::from("/test/.bnto"),
        }
    }

    fn default_settings() -> SettingsModel {
        SettingsModel::from_toml_config(&TomlConfig::default(), &test_paths())
    }

    #[test]
    fn four_settings_fields() {
        let m = default_settings();
        assert_eq!(m.fields.len(), 4);
        assert_eq!(m.fields[0].key, "theme");
        assert_eq!(m.fields[1].key, "recipes_dir");
        assert_eq!(m.fields[2].key, "output_dir");
        assert_eq!(m.fields[3].key, "telemetry");
    }

    #[test]
    fn recipes_dir_shows_default_path() {
        let m = default_settings();
        assert_eq!(m.fields[1].value, "/test/.bnto/recipes");
    }

    #[test]
    fn recipes_dir_shows_override_when_set() {
        let config = TomlConfig {
            paths: crate::storage::config::PathsSection {
                recipes: Some("/custom/recipes".into()),
                ..Default::default()
            },
            ..TomlConfig::default()
        };
        let m = SettingsModel::from_toml_config(&config, &test_paths());
        assert_eq!(m.fields[1].value, "/custom/recipes");
    }

    #[test]
    fn no_default_path_field() {
        let m = default_settings();
        assert!(
            m.fields.iter().all(|f| f.key != "default_path"),
            "default_path field should not exist"
        );
    }

    #[test]
    fn recipes_dir_field_is_editable() {
        let m = default_settings();
        assert!(m.fields[1].editable);
    }

    #[test]
    fn theme_field_is_not_editable() {
        let m = default_settings();
        assert!(!m.fields[0].editable);
    }

    #[test]
    fn telemetry_field_is_not_editable() {
        let m = default_settings();
        assert!(!m.fields[3].editable);
    }

    #[test]
    fn telemetry_field_shows_on_or_off() {
        let m = default_settings();
        let value = &m.fields[3].value;
        assert!(value == "On" || value == "Off");
    }

    #[test]
    fn path_fields_are_editable() {
        let m = default_settings();
        assert!(m.fields[1].editable);
        assert!(m.fields[2].editable);
    }

    #[test]
    fn focus_next_advances() {
        let m = default_settings();
        let m = update(m, SettingsMessage::FocusNext);
        assert_eq!(m.focused, 1);
    }

    #[test]
    fn focus_next_wraps() {
        let mut m = default_settings();
        m.focused = 3;
        let m = update(m, SettingsMessage::FocusNext);
        assert_eq!(m.focused, 0);
    }

    #[test]
    fn focus_prev_wraps() {
        let m = default_settings();
        let m = update(m, SettingsMessage::FocusPrev);
        assert_eq!(m.focused, 3);
    }

    #[test]
    fn apply_to_config_maps_fields() {
        let paths = test_paths();
        let mut m = default_settings();
        m.fields[1].value = "/recipes".to_string();
        m.fields[2].value = "/output".to_string();
        let mut config = TomlConfig::default();
        m.apply_to_config(&mut config, ThemeVariant::Tokyo, &paths);
        assert_eq!(config.tui.theme, "tokyo");
        assert_eq!(config.paths.recipes, Some("/recipes".to_string()));
        assert_eq!(config.paths.output, Some("/output".to_string()));
    }

    #[test]
    fn apply_default_recipes_dir_becomes_none() {
        let paths = test_paths();
        let m = default_settings();
        let mut config = TomlConfig::default();
        m.apply_to_config(&mut config, ThemeVariant::LosAngeles, &paths);
        assert!(
            config.paths.recipes.is_none(),
            "default path should not be persisted as override"
        );
        assert!(config.paths.output.is_none());
    }

    #[test]
    fn apply_preserves_output_when_only_recipes_changed() {
        let paths = test_paths();
        let mut m = default_settings();
        m.fields[2].value = "/existing-output".to_string();
        m.fields[1].value = "/new-recipes".to_string();
        let mut config = TomlConfig::default();
        m.apply_to_config(&mut config, ThemeVariant::LosAngeles, &paths);
        assert_eq!(config.paths.recipes, Some("/new-recipes".to_string()));
        assert_eq!(config.paths.output, Some("/existing-output".to_string()));
    }

    #[test]
    fn from_toml_config_roundtrips() {
        let paths = test_paths();
        let config = TomlConfig {
            tui: crate::storage::config::TuiSection {
                theme: "tokyo".into(),
            },
            paths: crate::storage::config::PathsSection {
                recipes: Some("/recipes".into()),
                output: Some("/output".into()),
            },
            ..TomlConfig::default()
        };
        let model = SettingsModel::from_toml_config(&config, &paths);
        let mut roundtripped = TomlConfig::default();
        model.apply_to_config(&mut roundtripped, ThemeVariant::Tokyo, &paths);
        assert_eq!(roundtripped.tui.theme, "tokyo");
        assert_eq!(roundtripped.paths.recipes, config.paths.recipes);
        assert_eq!(roundtripped.paths.output, config.paths.output);
    }
}
