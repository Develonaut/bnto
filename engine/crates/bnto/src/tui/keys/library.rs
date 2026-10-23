// Library screen keys — list, search, rename and delete modes.

use crossterm::event::{KeyCode, KeyEvent};

use super::super::app::{AppMessage, AppModel};
use super::super::screens::library::LibraryMessage;

/// Handle key events on the Library screen.
#[expect(clippy::too_many_lines)]
pub(super) fn handle_library_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    let lib = model.library.as_ref()?;

    // Search mode captures text input.
    if lib.searching {
        return match key.code {
            KeyCode::Esc => Some(AppMessage::Library(LibraryMessage::ExitSearch)),
            KeyCode::Backspace => Some(AppMessage::Library(LibraryMessage::SearchBackspace)),
            KeyCode::Enter => Some(AppMessage::LibraryConfirm),
            KeyCode::Char('u')
                if key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL) =>
            {
                Some(AppMessage::Library(LibraryMessage::SearchClear))
            }
            KeyCode::Char(ch) => Some(AppMessage::Library(LibraryMessage::SearchInput(ch))),
            _ => None,
        };
    }

    // Rename mode captures text input.
    if lib.renaming.is_some() {
        return match key.code {
            KeyCode::Enter => Some(AppMessage::Library(LibraryMessage::RenameConfirm)),
            KeyCode::Esc => Some(AppMessage::Library(LibraryMessage::RenameCancel)),
            KeyCode::Backspace => Some(AppMessage::Library(LibraryMessage::RenameBackspace)),
            KeyCode::Char(ch) => Some(AppMessage::Library(LibraryMessage::RenameInput(ch))),
            _ => None,
        };
    }

    // Delete confirmation mode.
    if lib.confirming_delete.is_some() {
        return match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                Some(AppMessage::Library(LibraryMessage::DeleteConfirm))
            }
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                Some(AppMessage::Library(LibraryMessage::DeleteCancel))
            }
            _ => None,
        };
    }

    match key.code {
        KeyCode::Char('j') | KeyCode::Down => Some(AppMessage::Library(LibraryMessage::CursorDown)),
        KeyCode::Char('k') | KeyCode::Up => Some(AppMessage::Library(LibraryMessage::CursorUp)),
        KeyCode::Char('/') => Some(AppMessage::Library(LibraryMessage::EnterSearch)),
        KeyCode::Char('d') => Some(AppMessage::Library(LibraryMessage::DeleteRequest)),
        KeyCode::Char('r') => Some(AppMessage::Library(LibraryMessage::RenameStart)),
        KeyCode::Char('e') => Some(AppMessage::OpenEditorFromLibrary),
        KeyCode::Char('n') => Some(AppMessage::OpenWizard),
        KeyCode::Enter => Some(AppMessage::LibraryConfirm),
        KeyCode::Esc => Some(AppMessage::Back),
        _ => None,
    }
}
