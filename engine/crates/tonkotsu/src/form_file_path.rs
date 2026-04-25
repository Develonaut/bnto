// Form-level orchestration for FilePath messages.
//
// Routes FilePathXxx messages to the controls::file_path module and
// manages FormEffect emission for I/O delegation.

use crate::controls::file_path;
use crate::field::{Field, FieldKind, FieldState};
use crate::file_path_nav;
use crate::form::{FormEffect, FormMessage, FormModel};
use crate::viewport;

/// Handle a FilePath message at the form level.
///
/// This runs on the focused field's FilePathBrowsing state. If the focused
/// field isn't in FilePathBrowsing (and it's not a DirLoaded response),
/// the model is returned unchanged.
pub fn handle_file_path_message(mut model: FormModel, msg: FormMessage) -> FormModel {
    // DirLoaded targets a specific field by id, not necessarily the focused one.
    if let FormMessage::FilePathDirLoaded {
        field_id,
        dir,
        entries,
    } = msg
    {
        return file_path_nav::handle_dir_loaded(model, &field_id, dir, entries);
    }

    let focused = model.focused;
    let field = match model.fields.get(focused) {
        Some(f) => f.clone(),
        None => return model,
    };

    // StartEdit for FilePath: transition from Idle to FilePathBrowsing
    if matches!(msg, FormMessage::StartEdit)
        && matches!(field.kind, FieldKind::FilePath { .. })
        && matches!(field.state, FieldState::Idle)
    {
        return file_path_nav::start_browsing(model, focused, field);
    }

    // All other FilePath messages require FilePathBrowsing state
    let updated = match &field.state {
        FieldState::FilePathBrowsing { .. } => {
            apply_browsing_message(field, msg, &mut model.pending_effects)
        }
        _ => return model,
    };

    model.fields[focused] = updated;
    model
}

/// Apply a browsing message to a field in FilePathBrowsing state.
fn apply_browsing_message(field: Field, msg: FormMessage, effects: &mut Vec<FormEffect>) -> Field {
    let (current_dir, entries, cursor, show_hidden, offset, height, mut nav_history) =
        match &field.state {
            FieldState::FilePathBrowsing {
                current_dir,
                entries,
                cursor,
                show_hidden,
                viewport_offset,
                viewport_height,
                nav_history,
            } => (
                current_dir.clone(),
                entries.clone(),
                *cursor,
                *show_hidden,
                *viewport_offset,
                *viewport_height,
                nav_history.clone(),
            ),
            _ => return field,
        };

    let extensions = match &field.kind {
        FieldKind::FilePath { extensions, .. } => extensions.clone(),
        _ => Vec::new(),
    };

    match msg {
        FormMessage::FilePathCursorDown => {
            let (c, o) = file_path::cursor_down(&entries, cursor, offset, height);
            with_cursor(field, c, o)
        }
        FormMessage::FilePathCursorUp => {
            let (c, o) = file_path::cursor_up(&entries, cursor, offset, height);
            with_cursor(field, c, o)
        }
        FormMessage::FilePathPageDown => {
            let (c, o) = viewport::page_down(cursor, offset, height, entries.len());
            with_cursor(field, c, o)
        }
        FormMessage::FilePathPageUp => {
            let (c, o) = viewport::page_up(cursor, offset, height);
            with_cursor(field, c, o)
        }
        FormMessage::FilePathGoToTop => {
            let (c, o) = viewport::go_to_top();
            with_cursor(field, c, o)
        }
        FormMessage::FilePathGoToBottom => {
            let (c, o) = viewport::go_to_bottom(height, entries.len());
            with_cursor(field, c, o)
        }
        FormMessage::FilePathEnterDir => {
            if let Some(dir) = file_path::enter_dir(&entries, cursor) {
                nav_history.push(cursor, offset);
                effects.push(FormEffect::LoadDirectory {
                    field_id: field.id.clone(),
                    path: dir,
                    extensions,
                    show_hidden,
                });
                Field {
                    state: FieldState::FilePathBrowsing {
                        current_dir,
                        entries,
                        cursor,
                        show_hidden,
                        viewport_offset: offset,
                        viewport_height: height,
                        nav_history,
                    },
                    ..field
                }
            } else {
                try_confirm(field, &entries, cursor)
            }
        }
        FormMessage::FilePathParentDir => {
            if let Some(parent) = file_path::parent_dir(&current_dir) {
                effects.push(FormEffect::LoadDirectory {
                    field_id: field.id.clone(),
                    path: parent,
                    extensions,
                    show_hidden,
                });
                field
            } else {
                field
            }
        }
        FormMessage::FilePathConfirm => try_confirm(field, &entries, cursor),
        FormMessage::FilePathCancel => Field {
            state: FieldState::Idle,
            ..field
        },
        FormMessage::FilePathToggleHidden => {
            let new_hidden = !show_hidden;
            effects.push(FormEffect::LoadDirectory {
                field_id: field.id.clone(),
                path: current_dir.clone(),
                extensions,
                show_hidden: new_hidden,
            });
            Field {
                state: FieldState::FilePathBrowsing {
                    current_dir,
                    entries,
                    cursor: 0,
                    show_hidden: new_hidden,
                    viewport_offset: 0,
                    viewport_height: height,
                    nav_history,
                },
                ..field
            }
        }
        _ => field,
    }
}

/// Try to confirm the current selection. If it's a file, set value and return to Idle.
fn try_confirm(field: Field, entries: &[crate::file_entry::FileEntry], cursor: usize) -> Field {
    if let Some(path) = file_path::confirm_file(entries, cursor) {
        Field {
            value: path,
            state: FieldState::Idle,
            ..field
        }
    } else {
        field
    }
}

/// Update just the cursor and viewport offset within FilePathBrowsing.
fn with_cursor(field: Field, cursor: usize, offset: usize) -> Field {
    match field.state {
        FieldState::FilePathBrowsing {
            current_dir,
            entries,
            show_hidden,
            viewport_height,
            nav_history,
            ..
        } => Field {
            state: FieldState::FilePathBrowsing {
                current_dir,
                entries,
                cursor,
                show_hidden,
                viewport_offset: offset,
                viewport_height,
                nav_history,
            },
            ..field
        },
        _ => field,
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::field::FieldState;
    use crate::field::file_path as build_file_path;
    use crate::form::FormModel;
    use crate::form::{FormEffect, FormMessage, update};

    fn make_file_path_form() -> FormModel {
        FormModel::new(vec![
            build_file_path("input_file")
                .label("Input File")
                .extensions(&["jpg", "png"])
                .build(),
        ])
    }

    #[test]
    fn start_edit_emits_load_directory_effect() {
        let form = make_file_path_form();
        let form = update(form, FormMessage::StartEdit);
        assert!(
            matches!(form.fields[0].state, FieldState::FilePathBrowsing { .. }),
            "should enter FilePathBrowsing"
        );
        assert_eq!(form.pending_effects.len(), 1);
        match &form.pending_effects[0] {
            FormEffect::LoadDirectory {
                field_id,
                extensions,
                ..
            } => {
                assert_eq!(field_id, "input_file");
                assert_eq!(extensions, &["jpg", "png"]);
            }
        }
    }

    #[test]
    fn cancel_returns_to_idle() {
        let form = make_file_path_form();
        let form = update(form, FormMessage::StartEdit);
        let mut form = update(form, FormMessage::FilePathCancel);
        form.pending_effects.clear();
        assert!(matches!(form.fields[0].state, FieldState::Idle));
    }

    #[test]
    fn dir_loaded_populates_entries() {
        let form = make_file_path_form();
        let form = update(form, FormMessage::StartEdit);
        let entries = vec![crate::file_entry::FileEntry {
            name: "test.jpg".into(),
            is_dir: false,
            path: PathBuf::from("/home/user/test.jpg"),
            size: Some(1000),
        }];
        let form = update(
            form,
            FormMessage::FilePathDirLoaded {
                field_id: "input_file".into(),
                dir: PathBuf::from("/home/user"),
                entries: entries.clone(),
            },
        );
        match &form.fields[0].state {
            FieldState::FilePathBrowsing {
                entries: loaded, ..
            } => {
                assert_eq!(loaded.len(), 1);
                assert_eq!(loaded[0].name, "test.jpg");
            }
            other => panic!("expected FilePathBrowsing, got {other:?}"),
        }
    }
}
