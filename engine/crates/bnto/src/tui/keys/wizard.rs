// Wizard screen keys — step navigation and form editing.

use crossterm::event::{KeyCode, KeyEvent};

use super::super::app::{AppMessage, AppModel};
use super::super::screens::detail_bridge;
use super::super::screens::wizard::{WizardMessage, WizardStep};

/// Handle key events on the Wizard screen.
///
/// Routes keys based on the current wizard step:
/// - Category/Operation: j/k navigate, Enter confirms, Esc goes back
/// - Configure: form keys (j/k/h/l/Enter for editing, Tab confirms, Esc back)
/// - Complete: Enter opens editor, Esc goes back
#[expect(clippy::too_many_lines)]
pub(super) fn handle_wizard_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    let wizard = model.wizard.as_ref()?;

    // Configure step with a form — route keys through the form system.
    if wizard.step == WizardStep::Configure
        && let Some(form) = &wizard.form
    {
        let is_editing = detail_bridge::is_form_editing(form);

        // When a field is in editing mode, delegate everything to tonkotsu.
        if is_editing {
            return tonkotsu::map_key_event(key, form).map(AppMessage::WizardForm);
        }

        // Tab advances to Complete step.
        if key.code == KeyCode::Tab {
            return Some(AppMessage::Wizard(WizardMessage::Confirm));
        }

        // Esc goes back.
        if key.code == KeyCode::Esc {
            return Some(AppMessage::Wizard(WizardMessage::Back));
        }

        // Vim-style form shortcuts.
        let vim_msg = match key.code {
            KeyCode::Char('j') => Some(tonkotsu::FormMessage::FocusNext),
            KeyCode::Char('k') => Some(tonkotsu::FormMessage::FocusPrev),
            KeyCode::Char('h') => Some(tonkotsu::FormMessage::CyclePrev),
            KeyCode::Char('l') => Some(tonkotsu::FormMessage::CycleNext),
            KeyCode::Char(' ') => Some(tonkotsu::FormMessage::ToggleConfirm),
            KeyCode::Char('d') => Some(tonkotsu::FormMessage::ResetDefault),
            _ => None,
        };
        if let Some(msg) = vim_msg {
            return Some(AppMessage::WizardForm(msg));
        }

        // Fall through to tonkotsu's idle key mapping.
        return tonkotsu::map_key_event(key, form).map(AppMessage::WizardForm);
    }

    // Category, Operation, Complete steps — simple list navigation.
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => Some(AppMessage::Wizard(WizardMessage::CursorDown)),
        KeyCode::Char('k') | KeyCode::Up => Some(AppMessage::Wizard(WizardMessage::CursorUp)),
        KeyCode::Enter | KeyCode::Tab => Some(AppMessage::Wizard(WizardMessage::Confirm)),
        KeyCode::Esc => Some(AppMessage::Wizard(WizardMessage::Back)),
        _ => None,
    }
}
