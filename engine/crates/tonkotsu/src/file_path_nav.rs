// FilePath navigation helpers — start browsing and dir-loaded handling.

use std::path::PathBuf;

use crate::controls::file_path;
use crate::field::{Field, FieldKind, FieldState};
use crate::file_entry::{FileEntry, NavHistory};
use crate::form::{FormEffect, FormModel};

/// Start browsing: emit LoadDirectory effect, enter FilePathBrowsing state.
pub fn start_browsing(mut model: FormModel, idx: usize, field: Field) -> FormModel {
    let (extensions, start_home) = match &field.kind {
        FieldKind::FilePath {
            extensions,
            start_home,
        } => (extensions.clone(), *start_home),
        _ => return model,
    };

    let start_dir = resolve_start_dir(&field.value, start_home);

    model.fields[idx] = Field {
        state: FieldState::FilePathBrowsing {
            current_dir: start_dir.clone(),
            entries: Vec::new(),
            cursor: 0,
            show_hidden: false,
            viewport_offset: 0,
            viewport_height: 20,
            nav_history: NavHistory::new(),
        },
        error: None,
        ..field
    };

    model.pending_effects.push(FormEffect::LoadDirectory {
        field_id: model.fields[idx].id.clone(),
        path: start_dir,
        extensions,
        show_hidden: false,
    });

    model
}

/// Determine which directory to start browsing from.
fn resolve_start_dir(value: &str, start_home: bool) -> PathBuf {
    if !value.is_empty() {
        PathBuf::from(value)
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| {
                if start_home {
                    home_dir()
                } else {
                    PathBuf::from(".")
                }
            })
    } else if start_home {
        home_dir()
    } else {
        PathBuf::from(".")
    }
}

/// Handle DirLoaded response — find the field by id and apply entries.
pub fn handle_dir_loaded(
    mut model: FormModel,
    field_id: &str,
    dir: PathBuf,
    entries: Vec<FileEntry>,
) -> FormModel {
    let idx = match model.fields.iter().position(|f| f.id == field_id) {
        Some(i) => i,
        None => return model,
    };

    let field = model.fields[idx].clone();
    let restored = try_pop_nav_history(&field, &dir);

    if let Some((nav, _)) = &restored {
        // Update nav_history before applying dir_loaded
        model.fields[idx] = update_nav_history(field.clone(), nav.clone());
    }

    let field = model.fields[idx].clone();
    model.fields[idx] = file_path::apply_dir_loaded(field, dir, entries, restored.map(|(_, p)| p));
    model
}

/// If navigating to a parent directory, pop the nav history and return
/// the restored cursor position. Returns (updated NavHistory, (cursor, offset)).
fn try_pop_nav_history(field: &Field, new_dir: &PathBuf) -> Option<(NavHistory, (usize, usize))> {
    match &field.state {
        FieldState::FilePathBrowsing {
            current_dir,
            nav_history,
            ..
        } => {
            if *new_dir != *current_dir && current_dir.starts_with(new_dir) {
                let mut nav = nav_history.clone();
                let popped = nav.pop()?;
                Some((nav, popped))
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Update the nav_history field within FilePathBrowsing state.
fn update_nav_history(field: Field, nav: NavHistory) -> Field {
    match field.state {
        FieldState::FilePathBrowsing {
            current_dir,
            entries,
            cursor,
            show_hidden,
            viewport_offset,
            viewport_height,
            ..
        } => Field {
            state: FieldState::FilePathBrowsing {
                current_dir,
                entries,
                cursor,
                show_hidden,
                viewport_offset,
                viewport_height,
                nav_history: nav,
            },
            ..field
        },
        _ => field,
    }
}

pub fn home_dir() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
}
