//! Demo model, messages, and key handling.
//!
//! `DemoModel` wraps `FormModel` with quit/reset semantics.
//! `DemoMessage` adds Quit and Reset on top of form messages.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use bnto_form::demo::fields::build_fields;
use bnto_form::{FormMessage, FormModel, map_key_event};

/// Top-level demo state wrapping the form.
pub struct DemoModel {
    pub form: FormModel,
    pub should_quit: bool,
}

impl DemoModel {
    /// Create a fresh demo model with all kitchen-sink fields.
    pub fn new(viewport_height: usize) -> Self {
        Self {
            form: FormModel::new(build_fields()).with_viewport(viewport_height),
            should_quit: false,
        }
    }
}

/// Messages the demo binary handles.
pub enum DemoMessage {
    Quit,
    Reset,
    Form(FormMessage),
}

/// Map a key event to a demo message, considering form state.
///
/// Esc quits only when the focused field is Idle (not editing).
/// Ctrl+R resets the entire form. Other keys delegate to the form.
pub fn handle_demo_key(key: KeyEvent, model: &DemoModel) -> Option<DemoMessage> {
    let field = model.form.focused_field();
    let is_editing = field
        .map(|f| !matches!(f.state, bnto_form::FieldState::Idle))
        .unwrap_or(false);

    match (key.code, key.modifiers, is_editing) {
        // Quit: Esc when idle, or Ctrl+C / q anytime
        (KeyCode::Esc, _, false) => Some(DemoMessage::Quit),
        (KeyCode::Char('q'), KeyModifiers::NONE, false) => Some(DemoMessage::Quit),
        (KeyCode::Char('c'), KeyModifiers::CONTROL, _) => Some(DemoMessage::Quit),

        // Reset entire form
        (KeyCode::Char('r'), KeyModifiers::CONTROL, false) => Some(DemoMessage::Reset),

        // Delegate to form key mapping
        _ => map_key_event(key, &model.form).map(DemoMessage::Form),
    }
}

/// Apply a demo message to the model.
pub fn update_demo(mut model: DemoModel, msg: DemoMessage) -> DemoModel {
    match msg {
        DemoMessage::Quit => {
            model.should_quit = true;
            model
        }
        DemoMessage::Reset => {
            let viewport = model.form.viewport_height;
            DemoModel {
                form: FormModel::new(build_fields()).with_viewport(viewport),
                should_quit: false,
            }
        }
        DemoMessage::Form(fm) => {
            model.form = bnto_form::update(model.form, fm);
            model
        }
    }
}
