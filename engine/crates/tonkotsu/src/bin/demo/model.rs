//! Demo model, messages, and key handling.
//!
//! `DemoModel` wraps `FormModel` with quit/reset semantics.
//! `DemoMessage` adds Quit and Reset on top of form messages.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use tonkotsu::demo::fields::build_fields;
use tonkotsu::{FormEffect, FormMessage, FormMode, FormModel, map_key_event};

/// Top-level demo state wrapping the form.
pub struct DemoModel {
    pub form: FormModel,
    pub should_quit: bool,
}

impl DemoModel {
    /// Create a fresh demo model with all kitchen-sink fields.
    pub fn new(viewport_height: usize) -> Self {
        Self {
            form: FormModel::new(build_fields())
                .with_mode(FormMode::FullScreenEdit)
                .with_viewport(viewport_height),
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
        .map(|f| !matches!(f.state, tonkotsu::FieldState::Idle))
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
///
/// After each form update, drains pending effects (e.g. directory reads)
/// and feeds results back into the form via `FilePathDirLoaded`.
pub fn update_demo(mut model: DemoModel, msg: DemoMessage) -> DemoModel {
    match msg {
        DemoMessage::Quit => {
            model.should_quit = true;
            model
        }
        DemoMessage::Reset => {
            let viewport = model.form.viewport_height;
            DemoModel {
                form: FormModel::new(build_fields())
                    .with_mode(FormMode::FullScreenEdit)
                    .with_viewport(viewport),
                should_quit: false,
            }
        }
        DemoMessage::Form(fm) => {
            model.form = tonkotsu::update(model.form, fm);
            drain_effects(&mut model);
            model
        }
    }
}

/// Drain pending form effects and fulfill them with filesystem I/O.
fn drain_effects(model: &mut DemoModel) {
    let effects: Vec<FormEffect> = model.form.pending_effects.drain(..).collect();
    for effect in effects {
        match effect {
            FormEffect::LoadDirectory {
                field_id,
                path,
                extensions,
                show_hidden,
            } => {
                let entries = read_directory(&path, &extensions, show_hidden);
                model.form = tonkotsu::update(
                    model.form.clone(),
                    tonkotsu::FormMessage::FilePathDirLoaded {
                        field_id,
                        dir: path,
                        entries,
                    },
                );
            }
        }
    }
}

/// Read a directory and return sorted FileEntry items.
///
/// Dirs come first (alphabetical), then files (alphabetical).
/// Filters files by extension when the list is non-empty.
/// Hidden entries (starting with '.') are excluded unless show_hidden is true.
fn read_directory(
    path: &std::path::Path,
    extensions: &[String],
    show_hidden: bool,
) -> Vec<tonkotsu::FileEntry> {
    let read_dir = match std::fs::read_dir(path) {
        Ok(rd) => rd,
        Err(_) => return Vec::new(),
    };

    let mut dirs = Vec::new();
    let mut files = Vec::new();

    for entry in read_dir.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();

        // Skip hidden unless requested
        if !show_hidden && name.starts_with('.') {
            continue;
        }

        let metadata = entry.metadata().ok();
        let is_dir = metadata.as_ref().is_some_and(|m| m.is_dir());
        let size = if is_dir {
            None
        } else {
            metadata.as_ref().map(|m| m.len())
        };

        // Filter files by extension (dirs always pass)
        if !is_dir && !extensions.is_empty() {
            let has_ext = entry
                .path()
                .extension()
                .and_then(|e| e.to_str())
                .is_some_and(|ext| {
                    extensions
                        .iter()
                        .any(|allowed| allowed.eq_ignore_ascii_case(ext))
                });
            if !has_ext {
                continue;
            }
        }

        let fe = tonkotsu::FileEntry {
            name,
            is_dir,
            path: entry.path(),
            size,
        };

        if is_dir {
            dirs.push(fe);
        } else {
            files.push(fe);
        }
    }

    dirs.sort_by_key(|a| a.name.to_lowercase());
    files.sort_by_key(|a| a.name.to_lowercase());
    dirs.append(&mut files);
    dirs
}
