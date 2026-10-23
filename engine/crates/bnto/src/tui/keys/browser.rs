// Browser screen keys — recipe list and search mode.

use crossterm::event::{KeyCode, KeyEvent};

use super::super::app::{AppMessage, AppModel};
use super::super::screens::browser::BrowserMessage;

/// Handle key events on the Browser screen.
#[expect(clippy::too_many_lines)]
pub(super) fn handle_browser_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    if model.browser.searching {
        return match key.code {
            KeyCode::Esc => Some(AppMessage::Browser(BrowserMessage::ExitSearch)),
            KeyCode::Backspace => Some(AppMessage::Browser(BrowserMessage::SearchBackspace)),
            KeyCode::Enter => model
                .browser
                .confirm()
                .map(|r| AppMessage::RecipeSelected { slug: r.slug }),
            KeyCode::Char('u')
                if key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL) =>
            {
                Some(AppMessage::Browser(BrowserMessage::SearchClear))
            }
            KeyCode::Char(ch) => Some(AppMessage::Browser(BrowserMessage::SearchInput(ch))),
            _ => None,
        };
    }

    match key.code {
        KeyCode::Char('j') | KeyCode::Down => Some(AppMessage::Browser(BrowserMessage::CursorDown)),
        KeyCode::Char('k') | KeyCode::Up => Some(AppMessage::Browser(BrowserMessage::CursorUp)),
        KeyCode::Char('/') => Some(AppMessage::Browser(BrowserMessage::EnterSearch)),
        KeyCode::Char('s') => Some(AppMessage::OpenSettings),
        KeyCode::Char('a') => Some(AppMessage::AddToLibrary),
        KeyCode::Char('A') => model
            .browser
            .confirm()
            .map(|r| AppMessage::AddToLibraryConfirm { slug: r.slug }),
        KeyCode::Char('e') => Some(AppMessage::OpenEditorFromBrowser),
        KeyCode::Char('n') => Some(AppMessage::OpenWizard),
        KeyCode::Enter => model
            .browser
            .confirm()
            .map(|r| AppMessage::RecipeSelected { slug: r.slug }),
        _ => None,
    }
}
