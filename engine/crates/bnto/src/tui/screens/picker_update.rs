// File picker state transitions — pure update function for the TEA pattern.
//
// Extracted from picker.rs to keep the model file focused on data types.

use std::collections::BTreeSet;
use std::path::PathBuf;

use super::picker::{PickerMessage, PickerModel};
use super::viewport;

/// Pure state transition for the picker screen.
pub fn update(mut model: PickerModel, msg: PickerMessage) -> PickerModel {
    match msg {
        PickerMessage::CursorDown => {
            if model.entries.is_empty() {
                return model;
            }
            model.cursor = if model.cursor + 1 >= model.entries.len() {
                0
            } else {
                model.cursor + 1
            };
            model.viewport_offset = viewport::ensure_cursor_visible(
                model.cursor,
                model.viewport_offset,
                model.viewport_height,
            );
            model
        }
        PickerMessage::CursorUp => {
            if model.entries.is_empty() {
                return model;
            }
            model.cursor = if model.cursor == 0 {
                model.entries.len() - 1
            } else {
                model.cursor - 1
            };
            model.viewport_offset = viewport::ensure_cursor_visible(
                model.cursor,
                model.viewport_offset,
                model.viewport_height,
            );
            model
        }
        PickerMessage::ToggleSelect => {
            if model.entries.is_empty() {
                return model;
            }
            if model.entries[model.cursor].is_dir && !model.allow_dirs {
                return model;
            }
            let path = model.entries[model.cursor].path.clone();
            if model.selected.contains(&path) {
                model.selected.remove(&path);
            } else {
                model.selected.insert(path);
            }
            model
        }
        PickerMessage::EnterDir => {
            if model.entries.is_empty() || !model.entries[model.cursor].is_dir {
                return model;
            }
            model.nav_history.push(model.cursor, model.viewport_offset);
            let new_dir = model.entries[model.cursor].path.clone();
            let entries =
                super::picker_loader::load_entries(&new_dir, &model.extensions, model.show_hidden);
            model.current_dir = new_dir;
            model.entries = entries;
            model.cursor = 0;
            model.viewport_offset = 0;
            model.selected = BTreeSet::new();
            model
        }
        PickerMessage::ParentDir => {
            let parent = match model.current_dir.parent() {
                Some(p) => p.to_path_buf(),
                None => return model,
            };
            let entries =
                super::picker_loader::load_entries(&parent, &model.extensions, model.show_hidden);
            model.current_dir = parent;
            model.entries = entries;
            if let Some(entry) = model.nav_history.pop() {
                model.cursor = entry.cursor.min(model.entries.len().saturating_sub(1));
                model.viewport_offset = entry.viewport_offset;
            } else {
                model.cursor = 0;
                model.viewport_offset = 0;
            }
            model.selected = BTreeSet::new();
            model
        }
        PickerMessage::Confirm => model,
        PickerMessage::DirLoaded { dir, entries } => {
            model.current_dir = dir;
            model.entries = entries;
            model.cursor = 0;
            model.viewport_offset = 0;
            model.selected = BTreeSet::new();
            model
        }
        PickerMessage::PageDown => {
            let total = model.entries.len();
            let (c, o) = viewport::page_down(
                model.cursor,
                model.viewport_offset,
                model.viewport_height,
                total,
            );
            model.cursor = c;
            model.viewport_offset = o;
            model
        }
        PickerMessage::PageUp => {
            let (c, o) =
                viewport::page_up(model.cursor, model.viewport_offset, model.viewport_height);
            model.cursor = c;
            model.viewport_offset = o;
            model
        }
        PickerMessage::GoToTop => {
            let (c, o) = viewport::go_to_top();
            model.cursor = c;
            model.viewport_offset = o;
            model
        }
        PickerMessage::GoToBottom => {
            let (c, o) = viewport::go_to_bottom(model.viewport_height, model.entries.len());
            model.cursor = c;
            model.viewport_offset = o;
            model
        }
        PickerMessage::ToggleHidden => {
            model.show_hidden = !model.show_hidden;
            let entries = super::picker_loader::load_entries(
                &model.current_dir,
                &model.extensions,
                model.show_hidden,
            );
            model.entries = entries;
            model.cursor = 0;
            model.viewport_offset = 0;
            model.selected = BTreeSet::new();
            model
        }
        PickerMessage::SelectAll => {
            let file_paths: BTreeSet<PathBuf> = model
                .entries
                .iter()
                .filter(|e| !e.is_dir || model.allow_dirs)
                .map(|e| e.path.clone())
                .collect();
            if model.selected == file_paths {
                model.selected.clear();
            } else {
                model.selected = file_paths;
            }
            model
        }
        PickerMessage::Resize { height } => {
            model.viewport_height = height;
            model.viewport_offset = viewport::ensure_cursor_visible(
                model.cursor,
                model.viewport_offset,
                model.viewport_height,
            );
            model
        }
        PickerMessage::EnterSearch => {
            model.searching = true;
            model
        }
        PickerMessage::ExitSearch => {
            model.searching = false;
            model
        }
        PickerMessage::SearchInput(ch) => {
            model.query.push(ch);
            model.cursor = 0;
            model.viewport_offset = 0;
            model
        }
        PickerMessage::SearchBackspace => {
            if model.query.is_empty() {
                model.searching = false;
            } else {
                model.query.pop();
                model.cursor = 0;
                model.viewport_offset = 0;
            }
            model
        }
        PickerMessage::SearchClear => {
            model.query.clear();
            model.cursor = 0;
            model.viewport_offset = 0;
            model
        }
        PickerMessage::ToggleMetadata => {
            model.show_metadata = !model.show_metadata;
            model
        }
    }
}
