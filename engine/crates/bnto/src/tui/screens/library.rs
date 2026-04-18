// My Library screen — browse, search, and manage user recipes.
//
// TEA pattern: LibraryModel (state) + LibraryMessage (events) + update() (pure transitions).
// Loads `.bnto.json` files from the user's recipe directory, parses name/description.

use std::path::Path;

/// A recipe in the user's library with metadata parsed from JSON.
#[derive(Debug, Clone)]
pub struct LibraryEntry {
    /// Filename stem (e.g. "compress-images" from "compress-images.bnto.json").
    pub slug: String,
    /// Human-readable name parsed from JSON `name` field.
    pub name: String,
    /// Description parsed from JSON `metadata.description` field.
    pub description: String,
}

/// Library screen state.
#[derive(Debug)]
pub struct LibraryModel {
    /// All recipes loaded from disk.
    pub entries: Vec<LibraryEntry>,
    /// Indices into `entries` after search filtering.
    pub filtered: Vec<usize>,
    /// Cursor position within the `filtered` list.
    pub cursor: usize,
    /// Current search query text.
    pub search_query: String,
    /// Whether the search input is active.
    pub searching: bool,
    /// Index of entry pending delete confirmation (None = normal mode).
    pub confirming_delete: Option<usize>,
    /// Index of entry being renamed + current edit buffer (None = normal mode).
    pub renaming: Option<(usize, String)>,
}

/// Messages the library screen can handle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LibraryMessage {
    /// Move cursor down one item.
    CursorDown,
    /// Move cursor up one item.
    CursorUp,
    /// User typed a character into the search input.
    SearchInput(char),
    /// User deleted a character from the search input.
    SearchBackspace,
    /// Enter search mode.
    EnterSearch,
    /// Exit search mode.
    ExitSearch,
    /// Clear search query.
    SearchClear,
    /// Request deletion of the focused entry (shows confirmation).
    DeleteRequest,
    /// Confirm pending deletion.
    DeleteConfirm,
    /// Cancel pending deletion.
    DeleteCancel,
    /// Start renaming the focused entry.
    RenameStart,
    /// Type a character into the rename buffer.
    RenameInput(char),
    /// Delete a character from the rename buffer.
    RenameBackspace,
    /// Confirm the rename.
    RenameConfirm,
    /// Cancel renaming.
    RenameCancel,
}

/// Result of confirming a selection — the slug of the chosen recipe.
pub struct LibrarySelection {
    pub slug: String,
}

impl LibraryModel {
    /// Create from a list of entries (loaded externally).
    pub fn new(entries: Vec<LibraryEntry>) -> Self {
        let filtered = (0..entries.len()).collect();
        Self {
            entries,
            filtered,
            cursor: 0,
            search_query: String::new(),
            searching: false,
            confirming_delete: None,
            renaming: None,
        }
    }

    /// Confirm the current selection. Returns `None` if the list is empty.
    pub fn confirm(&self) -> Option<LibrarySelection> {
        let &idx = self.filtered.get(self.cursor)?;
        Some(LibrarySelection {
            slug: self.entries[idx].slug.clone(),
        })
    }
}

/// Pure state transition for the library screen.
pub fn update(mut model: LibraryModel, msg: LibraryMessage) -> LibraryModel {
    match msg {
        LibraryMessage::CursorDown => {
            if !model.filtered.is_empty() {
                model.cursor = (model.cursor + 1) % model.filtered.len();
            }
        }
        LibraryMessage::CursorUp => {
            if !model.filtered.is_empty() {
                model.cursor = if model.cursor == 0 {
                    model.filtered.len() - 1
                } else {
                    model.cursor - 1
                };
            }
        }
        LibraryMessage::SearchInput(ch) => {
            model.search_query.push(ch);
            refilter(&mut model);
        }
        LibraryMessage::SearchBackspace => {
            model.search_query.pop();
            refilter(&mut model);
        }
        LibraryMessage::EnterSearch => {
            model.searching = true;
        }
        LibraryMessage::ExitSearch => {
            model.searching = false;
        }
        LibraryMessage::SearchClear => {
            model.search_query.clear();
            refilter(&mut model);
        }
        LibraryMessage::DeleteRequest => {
            if let Some(&idx) = model.filtered.get(model.cursor) {
                model.confirming_delete = Some(idx);
            }
        }
        LibraryMessage::DeleteConfirm => {
            if let Some(idx) = model.confirming_delete.take()
                && idx < model.entries.len()
            {
                model.entries.remove(idx);
                refilter(&mut model);
            }
        }
        LibraryMessage::DeleteCancel => {
            model.confirming_delete = None;
        }
        LibraryMessage::RenameStart => {
            if let Some(&idx) = model.filtered.get(model.cursor) {
                let current_name = model.entries[idx].name.clone();
                model.renaming = Some((idx, current_name));
            }
        }
        LibraryMessage::RenameInput(ch) => {
            if let Some((_, ref mut buf)) = model.renaming {
                buf.push(ch);
            }
        }
        LibraryMessage::RenameBackspace => {
            if let Some((_, ref mut buf)) = model.renaming {
                buf.pop();
            }
        }
        LibraryMessage::RenameConfirm => {
            if let Some((idx, new_name)) = model.renaming.take()
                && idx < model.entries.len()
                && !new_name.is_empty()
            {
                model.entries[idx].name = new_name;
            }
        }
        LibraryMessage::RenameCancel => {
            model.renaming = None;
        }
    }
    model
}

/// Rebuild the filtered index list based on the current search query.
fn refilter(model: &mut LibraryModel) {
    let query = model.search_query.to_lowercase();
    model.filtered = model
        .entries
        .iter()
        .enumerate()
        .filter(|(_, e)| {
            if query.is_empty() {
                return true;
            }
            e.name.to_lowercase().contains(&query)
                || e.description.to_lowercase().contains(&query)
                || e.slug.to_lowercase().contains(&query)
        })
        .map(|(i, _)| i)
        .collect();

    if model.filtered.is_empty() {
        model.cursor = 0;
    } else if model.cursor >= model.filtered.len() {
        model.cursor = model.filtered.len() - 1;
    }
}

/// Load library entries from a directory of `.bnto.json` files.
///
/// Parses each file's JSON to extract `name` and `metadata.description`.
/// Silently skips files that can't be read or parsed.
pub fn load_library_entries(dir: &Path) -> Vec<LibraryEntry> {
    let mut entries: Vec<LibraryEntry> = std::fs::read_dir(dir)
        .map(|rd| {
            rd.filter_map(|e| e.ok())
                .filter(|e| {
                    e.path()
                        .file_name()
                        .and_then(|n| n.to_str())
                        .is_some_and(|n| n.ends_with(".bnto.json"))
                })
                .filter_map(|e| {
                    let path = e.path();
                    let slug = path
                        .file_name()?
                        .to_str()?
                        .trim_end_matches(".bnto.json")
                        .to_string();
                    let content = std::fs::read_to_string(&path).ok()?;
                    let val: serde_json::Value = serde_json::from_str(&content).ok()?;
                    let name = val["name"].as_str().unwrap_or(&slug).to_string();
                    let description = val["metadata"]["description"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string();
                    Some(LibraryEntry {
                        slug,
                        name,
                        description,
                    })
                })
                .collect()
        })
        .unwrap_or_default();
    entries.sort_by_key(|a| a.name.to_lowercase());
    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_entries() -> Vec<LibraryEntry> {
        vec![
            LibraryEntry {
                slug: "compress-images".into(),
                name: "Compress Images".into(),
                description: "Reduce image file size".into(),
            },
            LibraryEntry {
                slug: "resize-images".into(),
                name: "Resize Images".into(),
                description: "Change image dimensions".into(),
            },
            LibraryEntry {
                slug: "clean-csv".into(),
                name: "Clean CSV".into(),
                description: "Remove empty rows".into(),
            },
        ]
    }

    fn library() -> LibraryModel {
        LibraryModel::new(sample_entries())
    }

    // --- Initial state ---

    #[test]
    fn initial_state_shows_all_entries() {
        let m = library();
        assert_eq!(m.filtered.len(), 3);
        assert_eq!(m.cursor, 0);
        assert!(!m.searching);
        assert!(m.confirming_delete.is_none());
        assert!(m.renaming.is_none());
    }

    #[test]
    fn empty_library_has_zero_entries() {
        let m = LibraryModel::new(vec![]);
        assert!(m.entries.is_empty());
        assert!(m.filtered.is_empty());
        assert_eq!(m.cursor, 0);
    }

    // --- Cursor navigation ---

    #[test]
    fn cursor_down_advances() {
        let m = library();
        let m = update(m, LibraryMessage::CursorDown);
        assert_eq!(m.cursor, 1);
    }

    #[test]
    fn cursor_down_wraps_at_end() {
        let mut m = library();
        m.cursor = 2;
        let m = update(m, LibraryMessage::CursorDown);
        assert_eq!(m.cursor, 0);
    }

    #[test]
    fn cursor_up_moves_back() {
        let mut m = library();
        m.cursor = 2;
        let m = update(m, LibraryMessage::CursorUp);
        assert_eq!(m.cursor, 1);
    }

    #[test]
    fn cursor_up_wraps_at_start() {
        let m = library();
        let m = update(m, LibraryMessage::CursorUp);
        assert_eq!(m.cursor, 2);
    }

    #[test]
    fn cursor_movement_on_empty_list_is_safe() {
        let m = LibraryModel::new(vec![]);
        let m = update(m, LibraryMessage::CursorDown);
        assert_eq!(m.cursor, 0);
        let m = update(m, LibraryMessage::CursorUp);
        assert_eq!(m.cursor, 0);
    }

    // --- Search ---

    #[test]
    fn search_filters_by_name() {
        let mut m = library();
        for ch in "comp".chars() {
            m = update(m, LibraryMessage::SearchInput(ch));
        }
        assert_eq!(m.filtered.len(), 1);
        assert_eq!(m.entries[m.filtered[0]].slug, "compress-images");
    }

    #[test]
    fn search_filters_by_description() {
        let mut m = library();
        for ch in "dim".chars() {
            m = update(m, LibraryMessage::SearchInput(ch));
        }
        assert_eq!(m.filtered.len(), 1);
        assert_eq!(m.entries[m.filtered[0]].slug, "resize-images");
    }

    #[test]
    fn search_is_case_insensitive() {
        let mut m = library();
        for ch in "COMPRESS".chars() {
            m = update(m, LibraryMessage::SearchInput(ch));
        }
        assert_eq!(m.filtered.len(), 1);
    }

    #[test]
    fn search_backspace_widens_results() {
        let mut m = library();
        for ch in "compx".chars() {
            m = update(m, LibraryMessage::SearchInput(ch));
        }
        assert_eq!(m.filtered.len(), 0);
        let m = update(m, LibraryMessage::SearchBackspace);
        assert_eq!(m.filtered.len(), 1);
    }

    #[test]
    fn search_clear_resets() {
        let mut m = library();
        for ch in "csv".chars() {
            m = update(m, LibraryMessage::SearchInput(ch));
        }
        assert_eq!(m.filtered.len(), 1);
        let m = update(m, LibraryMessage::SearchClear);
        assert_eq!(m.filtered.len(), 3);
        assert_eq!(m.search_query, "");
    }

    #[test]
    fn search_clamps_cursor() {
        let mut m = library();
        m.cursor = 2;
        for ch in "csv".chars() {
            m = update(m, LibraryMessage::SearchInput(ch));
        }
        assert_eq!(m.filtered.len(), 1);
        assert_eq!(m.cursor, 0);
    }

    #[test]
    fn enter_search_activates() {
        let m = library();
        let m = update(m, LibraryMessage::EnterSearch);
        assert!(m.searching);
    }

    #[test]
    fn exit_search_deactivates() {
        let mut m = library();
        m.searching = true;
        let m = update(m, LibraryMessage::ExitSearch);
        assert!(!m.searching);
    }

    // --- Selection ---

    #[test]
    fn confirm_returns_selected_slug() {
        let mut m = library();
        m.cursor = 1;
        let result = m.confirm().expect("should have selection");
        assert_eq!(result.slug, "resize-images");
    }

    #[test]
    fn confirm_on_empty_returns_none() {
        let m = LibraryModel::new(vec![]);
        assert!(m.confirm().is_none());
    }

    // --- Delete ---

    #[test]
    fn delete_request_sets_confirmation() {
        let m = library();
        let m = update(m, LibraryMessage::DeleteRequest);
        assert_eq!(m.confirming_delete, Some(0));
    }

    #[test]
    fn delete_confirm_removes_entry() {
        let m = library();
        let m = update(m, LibraryMessage::DeleteRequest);
        let m = update(m, LibraryMessage::DeleteConfirm);
        assert_eq!(m.entries.len(), 2);
        assert!(m.confirming_delete.is_none());
    }

    #[test]
    fn delete_cancel_clears_confirmation() {
        let m = library();
        let m = update(m, LibraryMessage::DeleteRequest);
        assert!(m.confirming_delete.is_some());
        let m = update(m, LibraryMessage::DeleteCancel);
        assert!(m.confirming_delete.is_none());
        assert_eq!(m.entries.len(), 3);
    }

    #[test]
    fn delete_on_empty_is_safe() {
        let m = LibraryModel::new(vec![]);
        let m = update(m, LibraryMessage::DeleteRequest);
        assert!(m.confirming_delete.is_none());
    }

    // --- Rename ---

    #[test]
    fn rename_start_captures_current_name() {
        let m = library();
        let m = update(m, LibraryMessage::RenameStart);
        assert_eq!(m.renaming, Some((0, "Compress Images".into())));
    }

    #[test]
    fn rename_input_appends_chars() {
        let m = library();
        let m = update(m, LibraryMessage::RenameStart);
        let m = update(m, LibraryMessage::RenameInput('!'));
        assert_eq!(m.renaming, Some((0, "Compress Images!".into())));
    }

    #[test]
    fn rename_backspace_removes_char() {
        let m = library();
        let m = update(m, LibraryMessage::RenameStart);
        let m = update(m, LibraryMessage::RenameBackspace);
        assert_eq!(m.renaming, Some((0, "Compress Image".into())));
    }

    #[test]
    fn rename_confirm_updates_entry() {
        let m = library();
        let m = update(m, LibraryMessage::RenameStart);
        let m = update(m, LibraryMessage::RenameInput('!'));
        let m = update(m, LibraryMessage::RenameConfirm);
        assert_eq!(m.entries[0].name, "Compress Images!");
        assert!(m.renaming.is_none());
    }

    #[test]
    fn rename_confirm_empty_name_preserves_original() {
        let m = library();
        let m = update(m, LibraryMessage::RenameStart);
        // Clear the name completely
        let mut m = m;
        m.renaming = Some((0, String::new()));
        let m = update(m, LibraryMessage::RenameConfirm);
        assert_eq!(m.entries[0].name, "Compress Images");
    }

    #[test]
    fn rename_cancel_discards() {
        let m = library();
        let m = update(m, LibraryMessage::RenameStart);
        let m = update(m, LibraryMessage::RenameInput('!'));
        let m = update(m, LibraryMessage::RenameCancel);
        assert_eq!(m.entries[0].name, "Compress Images");
        assert!(m.renaming.is_none());
    }

    // --- File loading ---

    #[test]
    fn load_library_entries_reads_bnto_json_files() {
        let tmp = tempfile::tempdir().unwrap();
        let json = r#"{"name": "My Recipe", "metadata": {"description": "Does things"}}"#;
        std::fs::write(tmp.path().join("my-recipe.bnto.json"), json).unwrap();
        let entries = load_library_entries(tmp.path());
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].slug, "my-recipe");
        assert_eq!(entries[0].name, "My Recipe");
        assert_eq!(entries[0].description, "Does things");
    }

    #[test]
    fn load_library_entries_ignores_non_bnto_files() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("notes.txt"), "not a recipe").unwrap();
        std::fs::write(tmp.path().join("data.json"), "{}").unwrap();
        let json = r#"{"name": "Real"}"#;
        std::fs::write(tmp.path().join("real.bnto.json"), json).unwrap();
        let entries = load_library_entries(tmp.path());
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].slug, "real");
    }

    #[test]
    fn load_library_entries_empty_dir() {
        let tmp = tempfile::tempdir().unwrap();
        assert!(load_library_entries(tmp.path()).is_empty());
    }

    #[test]
    fn load_library_entries_missing_dir() {
        assert!(load_library_entries(Path::new("/nonexistent/path")).is_empty());
    }

    #[test]
    fn load_library_entries_skips_invalid_json() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("bad.bnto.json"), "not json").unwrap();
        let json = r#"{"name": "Good"}"#;
        std::fs::write(tmp.path().join("good.bnto.json"), json).unwrap();
        let entries = load_library_entries(tmp.path());
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, "Good");
    }

    #[test]
    fn load_library_entries_uses_slug_when_no_name() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("unnamed.bnto.json"), "{}").unwrap();
        let entries = load_library_entries(tmp.path());
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, "unnamed");
    }

    #[test]
    fn load_library_entries_sorted_by_name() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("z.bnto.json"), r#"{"name": "Zebra"}"#).unwrap();
        std::fs::write(tmp.path().join("a.bnto.json"), r#"{"name": "Apple"}"#).unwrap();
        let entries = load_library_entries(tmp.path());
        assert_eq!(entries[0].name, "Apple");
        assert_eq!(entries[1].name, "Zebra");
    }
}
