// Settings helpers — theme, telemetry, picker integration, config persistence.

use super::super::app::{AppModel, DetailOrigin, Screen};
use super::super::screens::picker::PickerModel;
use super::super::theme::{Theme, ThemeVariant};

/// Handle theme change — persist via TOML and update settings display.
pub(crate) fn handle_theme_changed(model: AppModel, variant: ThemeVariant) -> AppModel {
    let mut config = model.config.clone();
    config.theme = variant.as_slug().to_string();
    let mut toml_config = model.toml_config.clone();
    toml_config.tui.theme = variant.as_slug().to_string();
    let status_message = match toml_config.save(&model.paths) {
        Ok(()) => None,
        Err(e) => Some(format!("Failed to save: {e}")),
    };
    let settings = model.settings.map(|mut s| {
        if let Some(f) = s.fields.iter_mut().find(|f| f.key == "theme") {
            f.value = variant.display_name().to_string();
        }
        s
    });
    AppModel {
        theme: Theme::from_variant(variant),
        theme_variant: variant,
        config,
        toml_config,
        settings,
        status_message,
        ..model
    }
}

/// Handle telemetry toggle — persist setting and update display.
pub(crate) fn handle_telemetry_toggled(model: AppModel, enabled: bool) -> AppModel {
    crate::telemetry::set_enabled(enabled);
    let mut toml_config = model.toml_config.clone();
    toml_config.telemetry.enabled = enabled;
    let status_message = match toml_config.save(&model.paths) {
        Ok(()) => None,
        Err(e) => Some(format!("Failed to save: {e}")),
    };
    let settings = model.settings.map(|mut s| {
        if let Some(f) = s.fields.iter_mut().find(|f| f.key == "telemetry") {
            f.value = if enabled { "On" } else { "Off" }.to_string();
        }
        s
    });
    AppModel {
        toml_config,
        settings,
        status_message,
        ..model
    }
}

/// Open the file picker from settings to browse for a directory.
pub(crate) fn handle_open_settings_picker(model: AppModel, field_key: String) -> AppModel {
    let current_value = model
        .settings
        .as_ref()
        .and_then(|s| s.fields.iter().find(|f| f.key == field_key))
        .map(|f| f.value.clone())
        .unwrap_or_default();
    let start_dir = if !current_value.is_empty() {
        let p = std::path::PathBuf::from(&current_value);
        if p.is_dir() {
            p
        } else {
            std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))
        }
    } else {
        std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))
    };
    let picker = Some(PickerModel::from_dir(&field_key, &start_dir));
    AppModel {
        screen: Screen::Picker {
            slug: field_key.clone(),
            from: DetailOrigin::Home,
        },
        picker,
        settings_picker_field: Some(field_key),
        ..model
    }
}

/// Confirm a picker directory as a settings field value and persist.
pub(crate) fn handle_settings_path_confirmed(model: AppModel) -> AppModel {
    let dir_path = model
        .picker
        .as_ref()
        .map(|p| p.current_dir.to_string_lossy().to_string())
        .unwrap_or_default();
    let field_key = model.settings_picker_field.clone().unwrap_or_default();
    let settings = model.settings.map(|mut s| {
        if let Some(f) = s.fields.iter_mut().find(|f| f.key == field_key) {
            f.value = dir_path;
        }
        s
    });
    let config = settings
        .as_ref()
        .map(|s| s.to_config(model.theme_variant))
        .unwrap_or_else(|| model.config.clone());
    let mut toml_config = model.toml_config.clone();
    toml_config.tui.theme = config.theme.clone();
    toml_config.output.dir = config.output_dir.clone();
    toml_config.picker.default_path = config.default_path.clone();
    let status_message = match toml_config.save(&model.paths) {
        Ok(()) => None,
        Err(e) => Some(format!("Failed to save: {e}")),
    };
    AppModel {
        screen: Screen::Settings,
        picker: None,
        settings_picker_field: None,
        settings,
        config,
        toml_config,
        status_message,
        ..model
    }
}
