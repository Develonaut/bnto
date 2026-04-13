// Settings screen — TEA state + transitions for TUI configuration.
//
// Path fields open the file picker for directory browsing (Enter).
// Theme field cycles with left/right arrows.

use super::super::config::TuiConfig;
use super::super::theme::ThemeVariant;

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
    /// Create a settings model from the current config.
    pub fn from_config(config: &TuiConfig) -> Self {
        let fields = vec![
            SettingsField {
                key: "theme",
                label: "Theme",
                value: config.theme.clone(),
                description: "Color theme (use arrow keys to cycle)",
                editable: false,
            },
            SettingsField {
                key: "default_path",
                label: "Default Path",
                value: config.default_path.clone().unwrap_or_default(),
                description: "Default file picker start directory (Enter to browse)",
                editable: true,
            },
            SettingsField {
                key: "output_dir",
                label: "Output Directory",
                value: config.output_dir.clone().unwrap_or_default(),
                description: "Recipe output directory (Enter to browse)",
                editable: true,
            },
        ];
        Self { fields, focused: 0 }
    }

    /// Whether the currently focused field supports text editing.
    pub fn is_focused_editable(&self) -> bool {
        self.fields.get(self.focused).is_some_and(|f| f.editable)
    }

    /// Apply current settings back to a TuiConfig.
    pub fn to_config(&self, variant: ThemeVariant) -> TuiConfig {
        let default_path = self
            .fields
            .iter()
            .find(|f| f.key == "default_path")
            .map(|f| f.value.clone())
            .filter(|v| !v.is_empty());

        let output_dir = self
            .fields
            .iter()
            .find(|f| f.key == "output_dir")
            .map(|f| f.value.clone())
            .filter(|v| !v.is_empty());

        TuiConfig {
            theme: variant.as_slug().to_string(),
            default_path,
            output_dir,
        }
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
        SettingsModel::from_config(&TuiConfig::default())
    }

    #[test]
    fn new_creates_three_fields() {
        let m = default_settings();
        assert_eq!(m.fields.len(), 3);
        assert_eq!(m.fields[0].key, "theme");
        assert_eq!(m.fields[1].key, "default_path");
        assert_eq!(m.fields[2].key, "output_dir");
    }

    #[test]
    fn theme_field_is_not_editable() {
        let m = default_settings();
        assert!(!m.fields[0].editable);
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
    fn to_config_maps_fields_to_config() {
        let mut m = default_settings();
        m.fields[1].value = "/photos".to_string();
        m.fields[2].value = "/output".to_string();
        let config = m.to_config(ThemeVariant::Tokyo);
        assert_eq!(config.theme, "tokyo");
        assert_eq!(config.default_path, Some("/photos".to_string()));
        assert_eq!(config.output_dir, Some("/output".to_string()));
    }

    #[test]
    fn to_config_empty_strings_become_none() {
        let m = default_settings();
        let config = m.to_config(ThemeVariant::LosAngeles);
        assert!(config.default_path.is_none());
        assert!(config.output_dir.is_none());
    }
}
