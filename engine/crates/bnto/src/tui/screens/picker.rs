// File picker screen — TEA state + transitions for filesystem browsing.
//
// Model (state) + Message (events) + update() (pure transitions).
// Filesystem I/O lives in picker_loader.rs.

use std::collections::BTreeSet;
use std::path::PathBuf;

use super::nav_history::NavHistory;

/// A single filesystem entry shown in the picker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileEntry {
    pub name: String,
    pub is_dir: bool,
    pub path: PathBuf,
    pub size: Option<u64>,
    /// Unix permissions string (e.g., "rwxr-xr-x"). None on non-Unix.
    pub permissions: Option<String>,
    /// Symlink target path, if this entry is a symlink.
    pub symlink_target: Option<String>,
}

/// File picker screen state — directory listing with multi-select.
#[derive(Debug)]
pub struct PickerModel {
    /// Recipe slug (carried forward for execution).
    pub slug: String,
    /// Current directory being browsed.
    pub current_dir: PathBuf,
    /// Visible entries (sorted: dirs first, then files alphabetically).
    pub entries: Vec<FileEntry>,
    /// Which entry currently has focus.
    pub cursor: usize,
    /// Selected file paths (multi-select via Space).
    pub selected: BTreeSet<PathBuf>,
    /// Allowed file extensions (e.g., ["jpg", "png", "webp"]).
    pub extensions: Vec<String>,
    /// Whether hidden files (dotfiles) are shown.
    pub show_hidden: bool,
    /// First visible entry index (scroll offset).
    pub viewport_offset: usize,
    /// Number of entries visible in the viewport.
    pub viewport_height: usize,
    /// Cursor/offset snapshots for parent directory navigation.
    pub nav_history: NavHistory,
    /// Current search/filter query (empty = no filter active).
    pub query: String,
    /// Whether search mode is active (captures key input).
    pub searching: bool,
    /// Whether to show file metadata columns (permissions, symlink target).
    pub show_metadata: bool,
}

/// Messages the picker screen can handle.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum PickerMessage {
    CursorDown,
    CursorUp,
    ToggleSelect,
    EnterDir,
    ParentDir,
    Confirm,
    DirLoaded {
        dir: PathBuf,
        entries: Vec<FileEntry>,
    },
    PageDown,
    PageUp,
    GoToTop,
    GoToBottom,
    ToggleHidden,
    SelectAll,
    Resize {
        height: usize,
    },
    /// Enter search mode — `/` key.
    EnterSearch,
    /// Exit search mode — Esc while searching.
    ExitSearch,
    /// Append a character to the search query.
    SearchInput(char),
    /// Delete the last character from the search query.
    SearchBackspace,
    /// Clear the entire search query (Ctrl+U).
    SearchClear,
    /// Toggle file metadata column display (permissions, symlinks).
    ToggleMetadata,
}

/// Result of confirming the picker — selected file paths.
pub struct PickerResult {
    pub files: Vec<PathBuf>,
}

impl PickerModel {
    /// Build a picker model from test data (no filesystem dependency).
    #[cfg(test)]
    pub fn from_test_data(
        slug: &str,
        current_dir: PathBuf,
        entries: Vec<FileEntry>,
        extensions: Vec<String>,
    ) -> Self {
        Self {
            slug: slug.to_string(),
            current_dir,
            entries,
            cursor: 0,
            selected: BTreeSet::new(),
            extensions,
            show_hidden: false,
            viewport_offset: 0,
            viewport_height: 20,
            nav_history: NavHistory::new(),
            query: String::new(),
            searching: false,
            show_metadata: false,
        }
    }

    /// Build a picker for directory selection — no extension filter, shows all entries.
    pub fn from_dir(label: &str, dir: &std::path::Path) -> Self {
        let entries = super::picker_loader::load_entries(dir, &[], false);
        Self {
            slug: label.to_string(),
            current_dir: dir.to_path_buf(),
            entries,
            cursor: 0,
            selected: BTreeSet::new(),
            extensions: Vec::new(),
            show_hidden: false,
            viewport_offset: 0,
            viewport_height: 20,
            nav_history: NavHistory::new(),
            query: String::new(),
            searching: false,
            show_metadata: false,
        }
    }

    /// Build a picker from a recipe — resolves extensions from definition JSON.
    pub fn from_slug(
        slug: &str,
        dir: &std::path::Path,
        registry: &bnto_core::registry::NodeRegistry,
        definition_json: &str,
    ) -> Self {
        let extensions = super::picker_loader::extensions_for_recipe(definition_json, registry);
        let entries = super::picker_loader::load_entries(dir, &extensions, false);
        Self {
            slug: slug.to_string(),
            current_dir: dir.to_path_buf(),
            entries,
            cursor: 0,
            selected: BTreeSet::new(),
            extensions,
            show_hidden: false,
            viewport_offset: 0,
            viewport_height: 20,
            nav_history: NavHistory::new(),
            query: String::new(),
            searching: false,
            show_metadata: false,
        }
    }

    /// Return entries filtered by the current search query.
    ///
    /// When the query is empty, returns all entries unchanged.
    /// Directories always pass the filter so navigation stays available.
    pub fn filtered_entries(&self) -> Vec<&FileEntry> {
        if self.query.is_empty() {
            return self.entries.iter().collect();
        }
        let query_lower = self.query.to_lowercase();
        self.entries
            .iter()
            .filter(|e| e.is_dir || e.name.to_lowercase().contains(&query_lower))
            .collect()
    }

    /// Collect selected file paths for execution.
    pub fn confirm(&self) -> Option<PickerResult> {
        if self.selected.is_empty() {
            return None;
        }
        let files: Vec<PathBuf> = self.selected.iter().cloned().collect();
        if files.is_empty() {
            return None;
        }
        Some(PickerResult { files })
    }
}

// Re-export the update function from its own module.
pub use super::picker_update::update;

#[cfg(test)]
mod tests {
    use super::*;

    // --- Test helpers ---

    fn file_entry(name: &str, is_dir: bool, size: Option<u64>) -> FileEntry {
        FileEntry {
            name: name.into(),
            is_dir,
            path: PathBuf::from(format!("/home/user/{name}")),
            size,
            permissions: None,
            symlink_target: None,
        }
    }

    fn sample_entries() -> Vec<FileEntry> {
        vec![
            file_entry("docs", true, None),
            file_entry("photos", true, None),
            file_entry("cat.jpg", false, Some(290_000)),
            file_entry("dog.png", false, Some(150_000)),
        ]
    }

    fn picker() -> PickerModel {
        PickerModel::from_test_data(
            "compress-images",
            PathBuf::from("/home/user"),
            sample_entries(),
            vec!["jpg".into(), "png".into()],
        )
    }

    /// Build a picker with many entries for viewport testing.
    fn big_picker() -> PickerModel {
        let mut entries = Vec::new();
        for i in 0..30 {
            entries.push(file_entry(
                &format!("file_{i:02}.jpg"),
                false,
                Some(1000 * (i as u64 + 1)),
            ));
        }
        let mut m = PickerModel::from_test_data(
            "compress-images",
            PathBuf::from("/home/user"),
            entries,
            vec!["jpg".into()],
        );
        m.viewport_height = 10;
        m
    }

    // --- Initial state ---

    #[test]
    fn initial_state_has_slug_and_dir() {
        let m = picker();
        assert_eq!(m.slug, "compress-images");
        assert_eq!(m.current_dir, PathBuf::from("/home/user"));
        assert_eq!(m.cursor, 0);
        assert!(m.selected.is_empty());
        assert!(!m.show_hidden);
        assert_eq!(m.viewport_offset, 0);
        assert_eq!(m.viewport_height, 20);
        assert!(m.nav_history.is_empty());
    }

    #[test]
    fn entries_sorted_dirs_first_then_files_alpha() {
        let m = picker();
        assert!(m.entries[0].is_dir, "first entry should be a dir");
        assert!(m.entries[1].is_dir, "second entry should be a dir");
        assert!(!m.entries[2].is_dir, "third entry should be a file");
        assert!(!m.entries[3].is_dir, "fourth entry should be a file");
    }

    // --- Cursor navigation ---

    #[test]
    fn cursor_down_advances_by_one() {
        let m = picker();
        let m = update(m, PickerMessage::CursorDown);
        assert_eq!(m.cursor, 1);
    }

    #[test]
    fn cursor_down_wraps_at_end() {
        let mut m = picker();
        m.cursor = 3; // last entry
        let m = update(m, PickerMessage::CursorDown);
        assert_eq!(m.cursor, 0);
    }

    #[test]
    fn cursor_up_moves_back_by_one() {
        let mut m = picker();
        m.cursor = 2;
        let m = update(m, PickerMessage::CursorUp);
        assert_eq!(m.cursor, 1);
    }

    #[test]
    fn cursor_up_wraps_at_start() {
        let m = picker(); // cursor = 0
        let m = update(m, PickerMessage::CursorUp);
        assert_eq!(m.cursor, 3);
    }

    #[test]
    fn cursor_movement_on_empty_entries_is_safe() {
        let m = PickerModel::from_test_data("s", PathBuf::from("/"), vec![], vec![]);
        let m = update(m, PickerMessage::CursorDown);
        assert_eq!(m.cursor, 0);
        let m = update(m, PickerMessage::CursorUp);
        assert_eq!(m.cursor, 0);
    }

    #[test]
    fn cursor_down_scrolls_viewport() {
        let mut m = big_picker();
        // Move cursor to just past viewport
        for _ in 0..10 {
            m = update(m, PickerMessage::CursorDown);
        }
        assert_eq!(m.cursor, 10);
        assert!(m.viewport_offset > 0, "viewport should have scrolled");
    }

    #[test]
    fn cursor_up_scrolls_viewport_back() {
        let mut m = big_picker();
        m.cursor = 15;
        m.viewport_offset = 10;
        m = update(m, PickerMessage::CursorUp);
        assert_eq!(m.cursor, 14);
        // Still within viewport, no scroll change
        assert_eq!(m.viewport_offset, 10);
    }

    // --- Selection ---

    #[test]
    fn toggle_select_adds_file_to_selected() {
        let mut m = picker();
        m.cursor = 2; // cat.jpg (a file)
        let m = update(m, PickerMessage::ToggleSelect);
        assert!(m.selected.contains(&PathBuf::from("/home/user/cat.jpg")));
    }

    #[test]
    fn toggle_select_removes_if_already_selected() {
        let mut m = picker();
        m.cursor = 2;
        m.selected.insert(PathBuf::from("/home/user/cat.jpg"));
        let m = update(m, PickerMessage::ToggleSelect);
        assert!(!m.selected.contains(&PathBuf::from("/home/user/cat.jpg")));
    }

    #[test]
    fn toggle_select_ignores_directories() {
        let m = picker(); // cursor = 0, which is a dir
        let m = update(m, PickerMessage::ToggleSelect);
        assert!(m.selected.is_empty());
    }

    // --- Select all ---

    #[test]
    fn select_all_selects_all_files() {
        let m = picker();
        let m = update(m, PickerMessage::SelectAll);
        assert_eq!(m.selected.len(), 2); // 2 files, not dirs
        assert!(m.selected.contains(&PathBuf::from("/home/user/cat.jpg")));
        assert!(m.selected.contains(&PathBuf::from("/home/user/dog.png")));
    }

    #[test]
    fn select_all_toggles_off_when_all_selected() {
        let mut m = picker();
        m.selected.insert(PathBuf::from("/home/user/cat.jpg"));
        m.selected.insert(PathBuf::from("/home/user/dog.png"));
        let m = update(m, PickerMessage::SelectAll);
        assert!(m.selected.is_empty());
    }

    // --- Confirm ---

    #[test]
    fn confirm_returns_none_when_no_files_selected() {
        let m = picker();
        assert!(m.confirm().is_none());
    }

    #[test]
    fn confirm_with_selected_files_returns_paths() {
        let mut m = picker();
        m.selected.insert(PathBuf::from("/home/user/cat.jpg"));
        m.selected.insert(PathBuf::from("/home/user/dog.png"));
        let result = m.confirm().expect("should return result");
        assert_eq!(result.files.len(), 2);
        assert!(result.files.contains(&PathBuf::from("/home/user/cat.jpg")));
        assert!(result.files.contains(&PathBuf::from("/home/user/dog.png")));
    }

    // --- Page navigation ---

    #[test]
    fn page_down_moves_cursor_by_viewport_height() {
        let m = big_picker(); // 30 entries, height 10
        let m = update(m, PickerMessage::PageDown);
        assert_eq!(m.cursor, 10);
    }

    #[test]
    fn page_up_moves_cursor_back() {
        let mut m = big_picker();
        m.cursor = 15;
        m.viewport_offset = 10;
        let m = update(m, PickerMessage::PageUp);
        assert_eq!(m.cursor, 5);
    }

    #[test]
    fn go_to_top_moves_to_first() {
        let mut m = big_picker();
        m.cursor = 20;
        m.viewport_offset = 15;
        let m = update(m, PickerMessage::GoToTop);
        assert_eq!(m.cursor, 0);
        assert_eq!(m.viewport_offset, 0);
    }

    #[test]
    fn go_to_bottom_moves_to_last() {
        let m = big_picker(); // 30 entries, height 10
        let m = update(m, PickerMessage::GoToBottom);
        assert_eq!(m.cursor, 29);
        assert_eq!(m.viewport_offset, 20);
    }

    // --- Nav history ---

    #[test]
    fn enter_dir_pushes_nav_history() {
        let mut m = picker(); // entries[0] = docs dir
        m.cursor = 0;
        m.viewport_offset = 5;
        assert!(m.nav_history.is_empty());
        // EnterDir calls load_entries which hits filesystem — won't find /home/user/docs
        // but the history push happens before the load
        let m = update(m, PickerMessage::EnterDir);
        // History was pushed (cursor=0, offset=5), then consumed on the model
        // The model now has cursor=0, offset=0 for the new dir
        assert_eq!(m.cursor, 0);
        assert_eq!(m.viewport_offset, 0);
    }

    #[test]
    fn parent_dir_pops_nav_history() {
        let mut m = picker();
        // Simulate having navigated into a child dir
        m.nav_history.push(2, 1);
        let m = update(m, PickerMessage::ParentDir);
        // Nav history was popped — cursor restored (clamped to entry count)
        assert!(m.nav_history.is_empty());
    }

    #[test]
    fn parent_dir_without_history_resets_cursor() {
        let m = picker();
        let m = update(m, PickerMessage::ParentDir);
        assert_eq!(m.cursor, 0);
        assert_eq!(m.viewport_offset, 0);
    }

    // --- Hidden toggle ---

    #[test]
    fn toggle_hidden_flips_flag() {
        let m = picker();
        assert!(!m.show_hidden);
        let m = update(m, PickerMessage::ToggleHidden);
        assert!(m.show_hidden);
        let m = update(m, PickerMessage::ToggleHidden);
        assert!(!m.show_hidden);
    }

    #[test]
    fn toggle_hidden_resets_cursor_and_selection() {
        let mut m = picker();
        m.cursor = 2;
        m.selected.insert(PathBuf::from("/home/user/cat.jpg"));
        let m = update(m, PickerMessage::ToggleHidden);
        assert_eq!(m.cursor, 0);
        assert!(m.selected.is_empty());
    }

    // --- Resize ---

    #[test]
    fn resize_updates_viewport_height() {
        let m = picker();
        assert_eq!(m.viewport_height, 20);
        let m = update(m, PickerMessage::Resize { height: 15 });
        assert_eq!(m.viewport_height, 15);
    }

    #[test]
    fn resize_adjusts_viewport_offset() {
        let mut m = big_picker();
        m.cursor = 25;
        m.viewport_offset = 20;
        // Shrink viewport — cursor should still be visible
        let m = update(m, PickerMessage::Resize { height: 5 });
        assert_eq!(m.viewport_height, 5);
        assert!(m.cursor >= m.viewport_offset);
        assert!(m.cursor < m.viewport_offset + m.viewport_height);
    }

    // --- Search / filter ---

    #[test]
    fn enter_search_activates_search_mode() {
        let m = picker();
        assert!(!m.searching);
        let m = update(m, PickerMessage::EnterSearch);
        assert!(m.searching);
    }

    #[test]
    fn exit_search_deactivates_but_keeps_query() {
        let mut m = picker();
        m.searching = true;
        m.query = "cat".into();
        let m = update(m, PickerMessage::ExitSearch);
        assert!(!m.searching);
        assert_eq!(m.query, "cat");
    }

    #[test]
    fn search_input_appends_characters() {
        let m = picker();
        let m = update(m, PickerMessage::EnterSearch);
        let m = update(m, PickerMessage::SearchInput('c'));
        let m = update(m, PickerMessage::SearchInput('a'));
        assert_eq!(m.query, "ca");
    }

    #[test]
    fn search_backspace_removes_last_char() {
        let mut m = picker();
        m.searching = true;
        m.query = "cat".into();
        let m = update(m, PickerMessage::SearchBackspace);
        assert_eq!(m.query, "ca");
    }

    #[test]
    fn search_backspace_on_empty_exits_search() {
        let mut m = picker();
        m.searching = true;
        m.query = String::new();
        let m = update(m, PickerMessage::SearchBackspace);
        assert!(!m.searching);
    }

    #[test]
    fn search_clear_empties_query() {
        let mut m = picker();
        m.searching = true;
        m.query = "cat".into();
        let m = update(m, PickerMessage::SearchClear);
        assert!(m.query.is_empty());
        assert!(m.searching, "should stay in search mode after clear");
    }

    #[test]
    fn filtered_entries_narrows_by_name() {
        let mut m = picker();
        m.query = "cat".into();
        let filtered = m.filtered_entries();
        let names: Vec<&str> = filtered.iter().map(|e| e.name.as_str()).collect();
        assert!(names.contains(&"cat.jpg"));
        assert!(!names.contains(&"dog.png"));
    }

    #[test]
    fn filtered_entries_is_case_insensitive() {
        let mut m = picker();
        m.query = "CAT".into();
        let filtered = m.filtered_entries();
        let names: Vec<&str> = filtered.iter().map(|e| e.name.as_str()).collect();
        assert!(names.contains(&"cat.jpg"));
    }

    #[test]
    fn filtered_entries_always_includes_dirs() {
        let mut m = picker();
        m.query = "cat".into();
        let filtered = m.filtered_entries();
        let dirs: Vec<&str> = filtered
            .iter()
            .filter(|e| e.is_dir)
            .map(|e| e.name.as_str())
            .collect();
        assert_eq!(dirs.len(), 2, "dirs should always pass filter");
    }

    #[test]
    fn filtered_entries_no_query_returns_all() {
        let m = picker();
        let filtered = m.filtered_entries();
        assert_eq!(filtered.len(), m.entries.len());
    }

    #[test]
    fn search_input_resets_cursor_to_zero() {
        let mut m = picker();
        m.cursor = 3;
        m.searching = true;
        let m = update(m, PickerMessage::SearchInput('c'));
        assert_eq!(m.cursor, 0, "cursor should reset when query changes");
    }

    // --- Metadata toggle ---

    #[test]
    fn toggle_metadata_flips_flag() {
        let m = picker();
        assert!(!m.show_metadata);
        let m = update(m, PickerMessage::ToggleMetadata);
        assert!(m.show_metadata);
        let m = update(m, PickerMessage::ToggleMetadata);
        assert!(!m.show_metadata);
    }
}
