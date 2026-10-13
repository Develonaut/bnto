// Home screen keys — 2D pane navigation.

use crossterm::event::{KeyCode, KeyEvent};

use super::super::app::{AppMessage, AppModel};
use super::super::screens::home::HomeMessage;

/// Handle key events on the Home screen — 2D pane navigation.
pub(super) fn handle_home_key(_model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    match key.code {
        KeyCode::Tab | KeyCode::Char('l') | KeyCode::Right => {
            Some(AppMessage::Home(HomeMessage::NextPane))
        }
        KeyCode::BackTab | KeyCode::Char('h') | KeyCode::Left => {
            Some(AppMessage::Home(HomeMessage::PrevPane))
        }
        KeyCode::Char('j') | KeyCode::Down => Some(AppMessage::Home(HomeMessage::CursorDown)),
        KeyCode::Char('k') | KeyCode::Up => Some(AppMessage::Home(HomeMessage::CursorUp)),
        KeyCode::Char('n') => Some(AppMessage::OpenWizard),
        KeyCode::Enter => Some(AppMessage::HomeConfirm),
        _ => None,
    }
}
