// Settings screen keys — theme, telemetry and path fields.

use crossterm::event::{KeyCode, KeyEvent};

use super::super::app::{AppMessage, AppModel};
use super::super::screens::settings::SettingsMessage;
use super::super::theme::ALL_VARIANTS;

/// Handle key events on the Settings screen.
///
/// Theme field: left/right arrows cycle through themes.
/// Telemetry field: left/right arrows toggle on/off.
/// Path fields: Enter opens file picker for directory browsing.
pub(super) fn handle_settings_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    let on_theme = model.settings.as_ref().is_some_and(|s| s.focused == 0);

    // Theme field: left/right cycle through variants.
    if on_theme {
        let current_idx = ALL_VARIANTS
            .iter()
            .position(|v| *v == model.theme_variant)
            .unwrap_or(0);
        match key.code {
            KeyCode::Left => {
                let prev = if current_idx == 0 {
                    ALL_VARIANTS.len() - 1
                } else {
                    current_idx - 1
                };
                return Some(AppMessage::ThemeChanged(ALL_VARIANTS[prev]));
            }
            KeyCode::Right => {
                let next = (current_idx + 1) % ALL_VARIANTS.len();
                return Some(AppMessage::ThemeChanged(ALL_VARIANTS[next]));
            }
            _ => {}
        }
    }

    // Telemetry field: left/right toggle on/off.
    let on_telemetry = model.settings.as_ref().is_some_and(|s| {
        s.fields
            .get(s.focused)
            .is_some_and(|f| f.key == "telemetry")
    });
    if on_telemetry && matches!(key.code, KeyCode::Left | KeyCode::Right) {
        let currently_on = model
            .settings
            .as_ref()
            .and_then(|s| s.fields.get(s.focused))
            .is_some_and(|f| f.value == "On");
        return Some(AppMessage::TelemetryToggled(!currently_on));
    }

    match key.code {
        KeyCode::Char('j') | KeyCode::Down => {
            Some(AppMessage::Settings(SettingsMessage::FocusNext))
        }
        KeyCode::Char('k') | KeyCode::Up => Some(AppMessage::Settings(SettingsMessage::FocusPrev)),
        KeyCode::Enter => {
            if model
                .settings
                .as_ref()
                .is_some_and(|s| s.is_focused_editable())
            {
                let field_key = model
                    .settings
                    .as_ref()
                    .and_then(|s| s.fields.get(s.focused))
                    .map(|f| f.key.to_string())
                    .unwrap_or_default();
                Some(AppMessage::OpenSettingsPicker { field_key })
            } else {
                None
            }
        }
        KeyCode::Esc => Some(AppMessage::Back),
        _ => None,
    }
}
