// Detail screen keys — routed by the focused section (input, params, run).

use crossterm::event::{KeyCode, KeyEvent};

use super::super::app::{AppMessage, AppModel};
use super::super::screens::detail::DetailFocus;
use super::super::screens::detail_bridge;
use super::super::screens::detail_scroll::DetailScrollMsg;
use super::super::screens::picker::PickerMessage;

/// Handle key events on the Detail screen.
///
/// Routes keys based on the active `DetailFocus` section:
/// - Input: picker navigation (j/k/Space/h/l/./a/g/G)
/// - Params: form field editing/navigation (j/k/h/l/d/Space + tonkotsu)
/// - Run: Enter/Tab to confirm and execute
///
/// Tab advances section forward, Shift+Tab goes backward.
/// Esc always navigates back (unless a field is being edited).
#[expect(clippy::too_many_lines)]
pub(super) fn handle_detail_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    let detail = model.detail.as_ref()?;
    let is_editing = detail_bridge::is_form_editing(&detail.form);

    // When a field is in editing mode, delegate everything to tonkotsu.
    if is_editing {
        return tonkotsu::map_key_event(key, &detail.form).map(AppMessage::DetailForm);
    }

    // Esc always goes back (when not editing a field).
    if key.code == KeyCode::Esc {
        return Some(AppMessage::Back);
    }

    // Whole-screen scroll keys — work in any detail section.
    match key.code {
        KeyCode::Char('J') | KeyCode::PageDown => {
            return Some(AppMessage::DetailScroll(DetailScrollMsg::PageDown));
        }
        KeyCode::Char('K') | KeyCode::PageUp => {
            return Some(AppMessage::DetailScroll(DetailScrollMsg::PageUp));
        }
        _ => {}
    }

    // g/G for top/bottom — only when NOT in Input section (conflicts with picker g/G).
    if !matches!(detail.focus, DetailFocus::Input) {
        match key.code {
            KeyCode::Char('g') => {
                return Some(AppMessage::DetailScroll(DetailScrollMsg::GoToTop));
            }
            KeyCode::Char('G') => {
                return Some(AppMessage::DetailScroll(DetailScrollMsg::GoToBottom));
            }
            _ => {}
        }
    }

    match detail.focus {
        DetailFocus::Input => handle_detail_input_key(model, key),
        DetailFocus::Params => handle_detail_params_key(model, key),
        DetailFocus::Run => handle_detail_run_key(model, key),
    }
}

/// Handle keys when the Input (file picker) section is focused.
#[expect(clippy::too_many_lines)]
fn handle_detail_input_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    let detail = model.detail.as_ref()?;
    let picker = detail.input_picker.as_ref()?;
    let on_dir = picker.cursor < picker.entries.len() && picker.entries[picker.cursor].is_dir;

    match key.code {
        // Tab → advance to Params section
        KeyCode::Tab => Some(AppMessage::DetailForm(tonkotsu::FormMessage::FocusNext)),
        // Picker navigation keys
        KeyCode::Char('j') | KeyCode::Down => {
            Some(AppMessage::DetailPicker(PickerMessage::CursorDown))
        }
        KeyCode::Char('k') | KeyCode::Up => Some(AppMessage::DetailPicker(PickerMessage::CursorUp)),
        KeyCode::Char(' ') => Some(AppMessage::DetailPicker(PickerMessage::ToggleSelect)),
        KeyCode::Char('h') | KeyCode::Left | KeyCode::Backspace => {
            Some(AppMessage::DetailPicker(PickerMessage::ParentDir))
        }
        KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
            if on_dir {
                Some(AppMessage::DetailPicker(PickerMessage::EnterDir))
            } else {
                Some(AppMessage::DetailPicker(PickerMessage::ToggleSelect))
            }
        }
        KeyCode::Char('g') => Some(AppMessage::DetailPicker(PickerMessage::GoToTop)),
        KeyCode::Char('G') => Some(AppMessage::DetailPicker(PickerMessage::GoToBottom)),
        KeyCode::Char('J') | KeyCode::PageDown => {
            Some(AppMessage::DetailPicker(PickerMessage::PageDown))
        }
        KeyCode::Char('K') | KeyCode::PageUp => {
            Some(AppMessage::DetailPicker(PickerMessage::PageUp))
        }
        KeyCode::Char('.') => Some(AppMessage::DetailPicker(PickerMessage::ToggleHidden)),
        KeyCode::Char('a') => Some(AppMessage::DetailPicker(PickerMessage::SelectAll)),
        _ => None,
    }
}

/// Handle keys when the Params (form fields) section is focused.
#[expect(clippy::too_many_lines)]
fn handle_detail_params_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    use tonkotsu::FormMessage;

    let detail = model.detail.as_ref()?;
    let slug = || detail.slug.clone();

    // No params → Enter or Tab confirms directly.
    if detail.params.is_empty() && matches!(key.code, KeyCode::Tab | KeyCode::Enter) {
        return Some(AppMessage::ConfigConfirmed { slug: slug() });
    }

    // Tab → advance focus through form fields.
    if key.code == KeyCode::Tab {
        return Some(AppMessage::DetailForm(FormMessage::FocusNext));
    }

    // Shift+Tab → go back to Input section (if file-mode), else no-op.
    if key.code == KeyCode::BackTab && detail.input_picker.is_some() {
        return Some(AppMessage::DetailFocusInput);
    }

    // Vim-style shortcuts mapped to FormMessage variants.
    let vim_msg = match key.code {
        KeyCode::Char('j') => Some(FormMessage::FocusNext),
        KeyCode::Char('k') => Some(FormMessage::FocusPrev),
        KeyCode::Char('h') => Some(FormMessage::CyclePrev),
        KeyCode::Char('l') => Some(FormMessage::CycleNext),
        KeyCode::Char('d') => Some(FormMessage::ResetDefault),
        KeyCode::Char(' ') => Some(FormMessage::ToggleConfirm),
        _ => None,
    };
    if let Some(msg) = vim_msg {
        return Some(AppMessage::DetailForm(msg));
    }

    // Fall through to tonkotsu's idle key mapping (Down/Up arrows, Enter→StartEdit, etc.)
    tonkotsu::map_key_event(key, &detail.form).map(AppMessage::DetailForm)
}

/// Handle keys when the Run button is focused.
fn handle_detail_run_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    let detail = model.detail.as_ref()?;
    let slug = || detail.slug.clone();

    match key.code {
        KeyCode::Enter | KeyCode::Tab => Some(AppMessage::ConfigConfirmed { slug: slug() }),
        // p → preview mode (run without writes, show file transformations).
        KeyCode::Char('p') => Some(AppMessage::PreviewRequested { slug: slug() }),
        // Up/k/Shift+Tab → go back to Params section.
        KeyCode::BackTab | KeyCode::Up | KeyCode::Char('k') => Some(AppMessage::DetailFocusParams),
        _ => None,
    }
}
