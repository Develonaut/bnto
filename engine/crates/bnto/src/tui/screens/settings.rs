// Settings screen — TEA state + transitions for TUI configuration.
//
// Output dir opens the file picker for directory browsing (Enter).
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
    /// Create a settings model from the TOML config.
    pub fn from_toml_config(config: &TomlConfig) -> Self {
        let fields = vec![
            SettingsField {
                key: "theme",
                label: "Theme",
                value: config.tui.theme.clone(),
                description: "Color theme (use arrow keys to cycle)",
                editable: false,
            },
            SettingsField {
                key: "home_path",
                label: "Home Path",
                value: config.paths.home.clone().unwrap_or_default(),
                description: "Where bnto stores recipes, logs, and cache (Enter to browse)",
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
    pub fn apply_to_config(&self, config: &mut TomlConfig, variant: ThemeVariant) {
        config.tui.theme = variant.as_slug().to_string();

        config.paths.home = self
            .fields
            .iter()
            .find(|f| f.key == "home_path")
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

    fn default_settings() -> SettingsModel {
        SettingsModel::from_toml_config(&TomlConfig::default())
    }

    #[test]
    fn three_settings_fields() {
        let m = default_settings();
        assert_eq!(m.fields.len(), 3);
        assert_eq!(m.fields[0].key, "theme");
        assert_eq!(m.fields[1].key, "home_path");
        assert_eq!(m.fields[2].key, "telemetry");
    }

    #[test]
    fn theme_field_is_not_editable() {
        let m = default_settings();
        assert!(!m.fields[0].editable);
    }

    #[test]
    fn home_path_field_is_editable() {
        let m = default_settings();
        assert!(m.fields[1].editable);
    }

    #[test]
    fn telemetry_field_is_not_editable() {
        let m = default_settings();
        assert!(!m.fields[2].editable);
    }

    #[test]
    fn telemetry_field_shows_on_or_off() {
        let m = default_settings();
        let value = &m.fields[2].value;
        assert!(value == "On" || value == "Off");
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
        m.focused = 2;
        let m = update(m, SettingsMessage::FocusNext);
        assert_eq!(m.focused, 0);
    }

    #[test]
    fn focus_prev_wraps() {
        let m = default_settings();
        let m = update(m, SettingsMessage::FocusPrev);
        assert_eq!(m.focused, 2);
    }

    #[test]
    fn apply_to_config_maps_home_path() {
        let mut m = default_settings();
        m.fields[1].value = "/custom/bnto".to_string();
        let mut config = TomlConfig::default();
        m.apply_to_config(&mut config, ThemeVariant::Tokyo);
        assert_eq!(config.tui.theme, "tokyo");
        assert_eq!(config.paths.home, Some("/custom/bnto".to_string()));
    }

    #[test]
    fn apply_empty_home_becomes_none() {
        let m = default_settings();
        let mut config = TomlConfig::default();
        m.apply_to_config(&mut config, ThemeVariant::LosAngeles);
        assert!(config.paths.home.is_none());
    }

    #[test]
    fn from_toml_config_roundtrips_home() {
        let config = TomlConfig {
            tui: crate::storage::config::TuiSection {
                theme: "tokyo".into(),
            },
            paths: crate::storage::config::PathsSection {
                home: Some("/custom".into()),
            },
            ..TomlConfig::default()
        };
        let model = SettingsModel::from_toml_config(&config);
        let mut roundtripped = TomlConfig::default();
        model.apply_to_config(&mut roundtripped, ThemeVariant::Tokyo);
        assert_eq!(roundtripped.tui.theme, "tokyo");
        assert_eq!(roundtripped.paths.home, config.paths.home);
    }
}
