// Settings helpers — theme, telemetry, picker integration, config persistence.

use super::super::app::{AppModel, DetailOrigin, Screen};
use super::super::screens::picker::PickerModel;
use super::super::screens::settings::SettingsModel;
use super::super::theme::{Theme, ThemeVariant};
use crate::storage::config::TomlConfig;

/// Handle theme change — persist via TOML and update settings display.
pub(crate) fn handle_theme_changed(model: AppModel, variant: ThemeVariant) -> AppModel {
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
        toml_config,
        settings,
        status_message,
        ..model
    }
}

/// Handle telemetry toggle — persist setting and update display.
pub(crate) fn handle_telemetry_toggled(model: AppModel, enabled: bool) -> AppModel {
    let mut toml_config = model.toml_config.clone();
    toml_config.telemetry.enabled = enabled;
    let status_message = match crate::telemetry::set_enabled(&model.paths, enabled) {
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
#[expect(clippy::too_many_lines)]
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

/// Confirm a picker directory as the Settings home path and persist it.
pub(crate) fn handle_settings_path_confirmed(model: AppModel) -> AppModel {
    let dir_path = model
        .picker
        .as_ref()
        .map(|p| p.current_dir.to_string_lossy().to_string())
        .unwrap_or_default();
    let settings = model.settings.clone().map(|s| s.with_home_path(dir_path));
    let home = settings.as_ref().and_then(SettingsModel::home_path);
    let (toml_config, status_message) = save_home(&model, home);
    AppModel {
        screen: Screen::Settings,
        picker: None,
        settings_picker_field: None,
        settings,
        toml_config,
        status_message: Some(status_message),
        ..model
    }
}

/// The Settings screen for `model`: theme and telemetry from the session's
/// config, the home path from the config `BntoPaths::configured` reads.
pub(crate) fn settings_for(model: &AppModel) -> SettingsModel {
    let settings = SettingsModel::from_toml_config(&model.toml_config);
    settings.with_home_path(home_setting(model).unwrap_or_default())
}

/// Whether the session's own config is the one `BntoPaths::configured`
/// reads (no move in effect), so it must carry the home setting too.
/// Compared as files, not spellings, so a home reached through a symlink
/// isn't taken for a move. When either config can't be opened the two homes
/// are compared instead, and spellings decide when either home can't be.
fn config_is_the_sessions(model: &AppModel) -> bool {
    let (read, session) = (model.config_home.config_file(), model.paths.config_file());
    same_file::is_same_file(&read, &session)
        .or_else(|_| same_file::is_same_file(&model.config_home.home, &model.paths.home))
        .unwrap_or(read == session)
}

/// The `paths.home` the next launch follows.
fn home_setting(model: &AppModel) -> Option<String> {
    if config_is_the_sessions(model) {
        return model.toml_config.paths.home.clone();
    }
    TomlConfig::load(&model.config_home).paths.home
}

/// Save `home` into the config `BntoPaths::configured` reads. The session
/// keeps its home (its trust file and runs agree on it) until the next
/// launch. Returns the session's config, kept in step when it's that file.
fn save_home(model: &AppModel, home: Option<String>) -> (TomlConfig, String) {
    let same = config_is_the_sessions(model);
    let mut config = if same {
        model.toml_config.clone()
    } else {
        TomlConfig::load(&model.config_home)
    };
    config.paths.home = home;
    let status = match config.save(&model.config_home) {
        Ok(()) => "Home path saved; it takes effect on the next launch".to_string(),
        Err(e) => format!("Failed to save: {e}"),
    };
    let session_config = if same {
        config
    } else {
        model.toml_config.clone()
    };
    (session_config, status)
}

#[cfg(test)]
mod tests;
