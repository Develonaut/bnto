// Key event routing — maps keys to AppMessages per screen.

use crossterm::event::{KeyCode, KeyEvent};

use super::app::{AppMessage, AppModel, Screen};
use super::event;
use super::screens::browser::BrowserMessage;
use super::screens::detail::DetailFocus;
use super::screens::detail_bridge;
use super::screens::editor::EditorMessage;
use super::screens::editor_bridge;
use super::screens::execution::{ExecutionMessage, ExecutionStatus};
use super::screens::home::HomeMessage;
use super::screens::library::LibraryMessage;
use super::screens::picker::PickerMessage;
use super::screens::results::ResultsMessage;
use super::screens::settings::SettingsMessage;
use super::screens::wizard::{WizardMessage, WizardStep};
use super::theme::ALL_VARIANTS;

/// Map a key event to an AppMessage based on the current screen.
///
/// When the browser is in search mode, screen-specific keys take priority
/// so that Esc exits search and character keys type into the query.
pub fn handle_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    let searching = matches!(&model.screen, Screen::Browser if model.browser.searching);
    if searching {
        return handle_browser_key(model, key);
    }

    // Picker search mode captures all keys (like browser search).
    let picker_searching = matches!(&model.screen, Screen::Picker { .. }
        if model.picker.as_ref().is_some_and(|p| p.searching));
    if picker_searching {
        return handle_picker_key(model, key);
    }

    // Library search/rename/delete modes capture all keys.
    let library_modal = matches!(&model.screen, Screen::Library
        if model.library.as_ref().is_some_and(|l|
            l.searching || l.renaming.is_some() || l.confirming_delete.is_some()));
    if library_modal {
        return handle_library_key(model, key);
    }

    // Editor picker/delete/dirty-confirm modes capture all keys.
    let editor_modal = matches!(&model.screen, Screen::Editor { .. }
        if model.editor.as_ref().is_some_and(|e|
            e.picker.is_some() || e.confirming_delete || e.confirming_dirty_exit));
    if editor_modal {
        return handle_editor_key(model, key);
    }

    // Editor inline form editing captures all keys.
    let editor_form_editing = matches!(&model.screen, Screen::Editor { .. }
        if model.editor.as_ref().is_some_and(|e|
            e.active_form.as_ref().is_some_and(editor_bridge::is_editing)));
    if editor_form_editing {
        return handle_editor_key(model, key);
    }

    // Wizard form editing captures all keys.
    let wizard_editing = matches!(&model.screen, Screen::Wizard { .. }
        if model.wizard.as_ref().is_some_and(|w|
            w.form.as_ref().is_some_and(detail_bridge::is_form_editing)));
    if wizard_editing {
        return handle_wizard_key(model, key);
    }

    // Detail editing mode captures all keys (like browser search mode).
    let detail_editing = matches!(&model.screen, Screen::Detail { .. }
        if model.detail.as_ref().is_some_and(|d| detail_bridge::is_form_editing(&d.form)));
    if detail_editing {
        return handle_detail_key(model, key);
    }

    // Execution screen captures Esc for cancel (not global Back).
    if matches!(&model.screen, Screen::Execution { .. }) && key.code == KeyCode::Esc {
        return Some(AppMessage::Execution(ExecutionMessage::Cancel));
    }

    if let Some(msg) = event::map_global_key(key) {
        return Some(msg);
    }

    match &model.screen {
        Screen::Home => handle_home_key(model, key),
        Screen::Library => handle_library_key(model, key),
        Screen::Browser => handle_browser_key(model, key),
        Screen::Settings => handle_settings_key(model, key),
        Screen::Detail { .. } => handle_detail_key(model, key),
        Screen::Picker { .. } => handle_picker_key(model, key),
        Screen::Execution { .. } => handle_execution_key(model, key),
        Screen::Results { .. } => handle_results_key(model, key),
        Screen::Editor { .. } => handle_editor_key(model, key),
        Screen::Wizard { .. } => handle_wizard_key(model, key),
    }
}

/// Handle key events on the Home screen — 2D pane navigation.
fn handle_home_key(_model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
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

/// Handle key events on the Library screen.
fn handle_library_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
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

/// Handle key events on the Browser screen.
fn handle_browser_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
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

/// Handle key events on the Detail screen.
///
/// Routes keys based on the active `DetailFocus` section:
/// - Input: picker navigation (j/k/Space/h/l/./a/g/G)
/// - Params: form field editing/navigation (j/k/h/l/d/Space + bnto-form)
/// - Run: Enter/Tab to confirm and execute
///
/// Tab advances section forward, Shift+Tab goes backward.
/// Esc always navigates back (unless a field is being edited).
fn handle_detail_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    let detail = model.detail.as_ref()?;
    let is_editing = detail_bridge::is_form_editing(&detail.form);

    // When a field is in editing mode, delegate everything to bnto-form.
    if is_editing {
        return bnto_form::map_key_event(key, &detail.form).map(AppMessage::DetailForm);
    }

    // Esc always goes back (when not editing a field).
    if key.code == KeyCode::Esc {
        return Some(AppMessage::Back);
    }

    match detail.focus {
        DetailFocus::Input => handle_detail_input_key(model, key),
        DetailFocus::Params => handle_detail_params_key(model, key),
        DetailFocus::Run => handle_detail_run_key(model, key),
    }
}

/// Handle keys when the Input (file picker) section is focused.
fn handle_detail_input_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    let detail = model.detail.as_ref()?;
    let picker = detail.input_picker.as_ref()?;
    let on_dir = picker.cursor < picker.entries.len() && picker.entries[picker.cursor].is_dir;

    match key.code {
        // Tab → advance to Params section
        KeyCode::Tab => Some(AppMessage::DetailForm(bnto_form::FormMessage::FocusNext)),
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
fn handle_detail_params_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    use bnto_form::FormMessage;

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

    // Fall through to bnto-form's idle key mapping (Down/Up arrows, Enter→StartEdit, etc.)
    bnto_form::map_key_event(key, &detail.form).map(AppMessage::DetailForm)
}

/// Handle keys when the Run button is focused.
fn handle_detail_run_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    let detail = model.detail.as_ref()?;
    let slug = || detail.slug.clone();

    match key.code {
        KeyCode::Enter | KeyCode::Tab => Some(AppMessage::ConfigConfirmed { slug: slug() }),
        // Shift+Tab → go back to Params section.
        KeyCode::BackTab => Some(AppMessage::DetailFocusParams),
        _ => None,
    }
}

/// Handle key events on the Picker screen.
///
/// When search mode is active, character keys type into the query and
/// navigation is limited to cursor + Esc. Otherwise, full picker keys apply.
fn handle_picker_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    let picker = model.picker.as_ref()?;

    // Search mode captures text input (same pattern as browser/library).
    if picker.searching {
        return match key.code {
            KeyCode::Esc => Some(AppMessage::Picker(PickerMessage::ExitSearch)),
            KeyCode::Backspace => Some(AppMessage::Picker(PickerMessage::SearchBackspace)),
            KeyCode::Char('u')
                if key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL) =>
            {
                Some(AppMessage::Picker(PickerMessage::SearchClear))
            }
            KeyCode::Char(ch) => Some(AppMessage::Picker(PickerMessage::SearchInput(ch))),
            _ => None,
        };
    }

    let on_dir = picker.cursor < picker.entries.len() && picker.entries[picker.cursor].is_dir;

    match key.code {
        KeyCode::Char('j') | KeyCode::Down => Some(AppMessage::Picker(PickerMessage::CursorDown)),
        KeyCode::Char('k') | KeyCode::Up => Some(AppMessage::Picker(PickerMessage::CursorUp)),
        KeyCode::Char(' ') => Some(AppMessage::Picker(PickerMessage::ToggleSelect)),
        KeyCode::Char('h') | KeyCode::Left | KeyCode::Backspace => {
            Some(AppMessage::Picker(PickerMessage::ParentDir))
        }
        KeyCode::Char('l') | KeyCode::Right => {
            if on_dir {
                Some(AppMessage::Picker(PickerMessage::EnterDir))
            } else {
                None
            }
        }
        KeyCode::Char('g') => Some(AppMessage::Picker(PickerMessage::GoToTop)),
        KeyCode::Char('G') => Some(AppMessage::Picker(PickerMessage::GoToBottom)),
        KeyCode::Char('J') | KeyCode::PageDown => Some(AppMessage::Picker(PickerMessage::PageDown)),
        KeyCode::Char('K') | KeyCode::PageUp => Some(AppMessage::Picker(PickerMessage::PageUp)),
        KeyCode::Char('.') => Some(AppMessage::Picker(PickerMessage::ToggleHidden)),
        KeyCode::Char('a') => Some(AppMessage::Picker(PickerMessage::SelectAll)),
        KeyCode::Char('/') => Some(AppMessage::Picker(PickerMessage::EnterSearch)),
        KeyCode::Char('p') => Some(AppMessage::Picker(PickerMessage::ToggleMetadata)),
        KeyCode::Tab => {
            if model.settings_picker_field.is_some() {
                Some(AppMessage::SettingsPathConfirmed)
            } else if !picker.selected.is_empty() {
                let slug = picker.slug.clone();
                Some(AppMessage::FilesSelected { slug })
            } else {
                None
            }
        }
        KeyCode::Enter => {
            if on_dir {
                Some(AppMessage::Picker(PickerMessage::EnterDir))
            } else if model.settings_picker_field.is_none() && !picker.selected.is_empty() {
                let slug = picker.slug.clone();
                Some(AppMessage::FilesSelected { slug })
            } else {
                None
            }
        }
        KeyCode::Esc => Some(AppMessage::Back),
        _ => None,
    }
}

/// Handle key events on the Execution screen.
///
/// Enter transitions to Results when the pipeline has finished (Completed or Failed).
/// Esc cancels a running pipeline (handled above in handle_key as a special case).
fn handle_execution_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    let finished = model.execution.as_ref().is_some_and(|e| {
        matches!(
            e.status,
            ExecutionStatus::Completed | ExecutionStatus::Failed
        )
    });

    match key.code {
        KeyCode::Enter if finished => {
            if let Screen::Execution { slug, .. } = &model.screen {
                Some(AppMessage::ExecutionComplete { slug: slug.clone() })
            } else {
                None
            }
        }
        KeyCode::Esc => Some(AppMessage::Execution(ExecutionMessage::Cancel)),
        _ => None,
    }
}

/// Handle key events on the Results screen.
fn handle_results_key(_model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => Some(AppMessage::Results(ResultsMessage::CursorDown)),
        KeyCode::Char('k') | KeyCode::Up => Some(AppMessage::Results(ResultsMessage::CursorUp)),
        KeyCode::Char('o') => Some(AppMessage::OpenFile),
        KeyCode::Char('O') => Some(AppMessage::OpenFolder),
        KeyCode::Char('r') => Some(AppMessage::RunAnother),
        _ => None,
    }
}

/// Handle key events on the Editor screen.
///
/// When a picker overlay or delete confirmation is active, keys are routed
/// to the overlay. Otherwise, normal editor navigation applies.
fn handle_editor_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    let editor = model.editor.as_ref()?;

    // Picker overlay captures text input.
    if editor.picker.is_some() {
        return match key.code {
            KeyCode::Esc => Some(AppMessage::Editor(EditorMessage::PickerCancel)),
            KeyCode::Enter => Some(AppMessage::Editor(EditorMessage::PickerSelect)),
            KeyCode::Backspace => Some(AppMessage::Editor(EditorMessage::PickerBackspace)),
            KeyCode::Char('u')
                if key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL) =>
            {
                Some(AppMessage::Editor(EditorMessage::PickerClear))
            }
            KeyCode::Down | KeyCode::Char('j') => {
                Some(AppMessage::Editor(EditorMessage::PickerCursorDown))
            }
            KeyCode::Up | KeyCode::Char('k') => {
                Some(AppMessage::Editor(EditorMessage::PickerCursorUp))
            }
            KeyCode::Char(ch) => Some(AppMessage::Editor(EditorMessage::PickerInput(ch))),
            _ => None,
        };
    }

    // Delete confirmation.
    if editor.confirming_delete {
        return match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                Some(AppMessage::Editor(EditorMessage::DeleteConfirm))
            }
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                Some(AppMessage::Editor(EditorMessage::DeleteCancel))
            }
            _ => None,
        };
    }

    // Dirty-exit confirmation.
    if editor.confirming_dirty_exit {
        return match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                Some(AppMessage::Editor(EditorMessage::DirtySave))
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                Some(AppMessage::Editor(EditorMessage::DirtyDiscard))
            }
            KeyCode::Esc => Some(AppMessage::Editor(EditorMessage::DirtyCancel)),
            _ => None,
        };
    }

    // If an inline form is active, route keys through the form.
    if let Some(active) = &editor.active_form {
        let is_editing = editor_bridge::is_editing(active);

        // When a field is in editing mode, delegate everything to bnto-form.
        if is_editing {
            return bnto_form::map_key_event(key, &active.form).map(AppMessage::EditorForm);
        }

        // Selected node is expanded with a form: form navigation + vim shortcuts.
        let vim_msg = match key.code {
            KeyCode::Char('j') => Some(bnto_form::FormMessage::FocusNext),
            KeyCode::Char('k') => Some(bnto_form::FormMessage::FocusPrev),
            KeyCode::Char('h') => Some(bnto_form::FormMessage::CyclePrev),
            KeyCode::Char('l') => Some(bnto_form::FormMessage::CycleNext),
            KeyCode::Char(' ') => Some(bnto_form::FormMessage::ToggleConfirm),
            _ => None,
        };
        if let Some(msg) = vim_msg {
            return Some(AppMessage::EditorForm(msg));
        }

        // Esc collapses the node (exits form).
        if key.code == KeyCode::Esc {
            return Some(AppMessage::Editor(EditorMessage::ExpandToggle));
        }

        // Fall through to bnto-form's idle key mapping.
        if let Some(msg) = bnto_form::map_key_event(key, &active.form) {
            return Some(AppMessage::EditorForm(msg));
        }
    }

    // Normal editor keys (no active form).
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => Some(AppMessage::Editor(EditorMessage::CursorDown)),
        KeyCode::Char('k') | KeyCode::Up => Some(AppMessage::Editor(EditorMessage::CursorUp)),
        KeyCode::Enter => Some(AppMessage::Editor(EditorMessage::ExpandToggle)),
        KeyCode::Char('a') => Some(AppMessage::Editor(EditorMessage::OpenPicker)),
        KeyCode::Char('d') => Some(AppMessage::Editor(EditorMessage::DeleteRequest)),
        KeyCode::Char('J') => Some(AppMessage::Editor(EditorMessage::MoveDown)),
        KeyCode::Char('K') => Some(AppMessage::Editor(EditorMessage::MoveUp)),
        KeyCode::Char('u') => Some(AppMessage::Editor(EditorMessage::Undo)),
        KeyCode::Char('s')
            if key
                .modifiers
                .contains(crossterm::event::KeyModifiers::CONTROL) =>
        {
            Some(AppMessage::Editor(EditorMessage::Save))
        }
        KeyCode::Char('r')
            if key
                .modifiers
                .contains(crossterm::event::KeyModifiers::CONTROL) =>
        {
            Some(AppMessage::Editor(EditorMessage::Redo))
        }
        KeyCode::Esc => Some(AppMessage::Editor(EditorMessage::Back)),
        _ => None,
    }
}

/// Handle key events on the Wizard screen.
///
/// Routes keys based on the current wizard step:
/// - Category/Operation: j/k navigate, Enter confirms, Esc goes back
/// - Configure: form keys (j/k/h/l/Enter for editing, Tab confirms, Esc back)
/// - Complete: Enter opens editor, Esc goes back
fn handle_wizard_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    let wizard = model.wizard.as_ref()?;

    // Configure step with a form — route keys through the form system.
    if wizard.step == WizardStep::Configure
        && let Some(form) = &wizard.form
    {
        let is_editing = detail_bridge::is_form_editing(form);

        // When a field is in editing mode, delegate everything to bnto-form.
        if is_editing {
            return bnto_form::map_key_event(key, form).map(AppMessage::WizardForm);
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
            KeyCode::Char('j') => Some(bnto_form::FormMessage::FocusNext),
            KeyCode::Char('k') => Some(bnto_form::FormMessage::FocusPrev),
            KeyCode::Char('h') => Some(bnto_form::FormMessage::CyclePrev),
            KeyCode::Char('l') => Some(bnto_form::FormMessage::CycleNext),
            KeyCode::Char(' ') => Some(bnto_form::FormMessage::ToggleConfirm),
            KeyCode::Char('d') => Some(bnto_form::FormMessage::ResetDefault),
            _ => None,
        };
        if let Some(msg) = vim_msg {
            return Some(AppMessage::WizardForm(msg));
        }

        // Fall through to bnto-form's idle key mapping.
        return bnto_form::map_key_event(key, form).map(AppMessage::WizardForm);
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

/// Handle key events on the Settings screen.
///
/// Theme field: left/right arrows cycle through themes.
/// Telemetry field: left/right arrows toggle on/off.
/// Path fields: Enter opens file picker for directory browsing.
fn handle_settings_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
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
