// File picker screen — TEA state + transitions for filesystem browsing.
//
// Model (state) + Message (events) + update() (pure transitions).
// Filesystem I/O lives in picker_loader.rs.

use std::collections::BTreeSet;
use std::path::PathBuf;

/// A single filesystem entry shown in the picker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileEntry {
    pub name: String,
    pub is_dir: bool,
    pub path: PathBuf,
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
    /// Indices of selected files (multi-select via Space).
    pub selected: BTreeSet<usize>,
    /// Allowed file extensions (e.g., ["jpg", "png", "webp"]).
    pub extensions: Vec<String>,
}

/// Messages the picker screen can handle.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum PickerMessage {
    /// Move cursor down one entry.
    CursorDown,
    /// Move cursor up one entry.
    CursorUp,
    /// Toggle selection on the file at cursor (ignored for directories).
    ToggleSelect,
    /// Enter the directory at cursor.
    EnterDir,
    /// Navigate to parent directory.
    ParentDir,
    /// Confirm selection (only when files are selected).
    Confirm,
    /// Directory contents loaded after navigation.
    DirLoaded {
        dir: PathBuf,
        entries: Vec<FileEntry>,
    },
}

/// Result of confirming the picker — selected file paths.
#[allow(dead_code)]
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
        }
    }

    /// Build a picker from a recipe slug — resolves extensions from engine metadata.
    pub fn from_slug(
        slug: &str,
        dir: &std::path::Path,
        registry: &bnto_core::registry::NodeRegistry,
    ) -> Self {
        let extensions = super::picker_loader::extensions_for_recipe(slug, registry);
        let entries = super::picker_loader::load_entries(dir, &extensions);
        Self {
            slug: slug.to_string(),
            current_dir: dir.to_path_buf(),
            entries,
            cursor: 0,
            selected: BTreeSet::new(),
            extensions,
        }
    }

    /// Collect selected file paths for execution.
    #[allow(dead_code)]
    pub fn confirm(&self) -> Option<PickerResult> {
        if self.selected.is_empty() {
            return None;
        }
        let files = self
            .selected
            .iter()
            .filter_map(|&i| self.entries.get(i))
            .filter(|e| !e.is_dir)
            .map(|e| e.path.clone())
            .collect::<Vec<_>>();
        if files.is_empty() {
            return None;
        }
        Some(PickerResult { files })
    }
}

/// Pure state transition for the picker screen.
pub fn update(model: PickerModel, msg: PickerMessage) -> PickerModel {
    match msg {
        PickerMessage::CursorDown => {
            if model.entries.is_empty() {
                return model;
            }
            let next = if model.cursor + 1 >= model.entries.len() {
                0
            } else {
                model.cursor + 1
            };
            PickerModel {
                cursor: next,
                ..model
            }
        }
        PickerMessage::CursorUp => {
            if model.entries.is_empty() {
                return model;
            }
            let prev = if model.cursor == 0 {
                model.entries.len() - 1
            } else {
                model.cursor - 1
            };
            PickerModel {
                cursor: prev,
                ..model
            }
        }
        PickerMessage::ToggleSelect => {
            if model.entries.is_empty() || model.entries[model.cursor].is_dir {
                return model;
            }
            let mut selected = model.selected;
            if selected.contains(&model.cursor) {
                selected.remove(&model.cursor);
            } else {
                selected.insert(model.cursor);
            }
            PickerModel { selected, ..model }
        }
        PickerMessage::EnterDir => {
            if model.entries.is_empty() || !model.entries[model.cursor].is_dir {
                return model;
            }
            let new_dir = model.entries[model.cursor].path.clone();
            let entries = super::picker_loader::load_entries(&new_dir, &model.extensions);
            PickerModel {
                current_dir: new_dir,
                entries,
                cursor: 0,
                selected: BTreeSet::new(),
                ..model
            }
        }
        PickerMessage::ParentDir => {
            let parent = match model.current_dir.parent() {
                Some(p) => p.to_path_buf(),
                None => return model,
            };
            let entries = super::picker_loader::load_entries(&parent, &model.extensions);
            PickerModel {
                current_dir: parent,
                entries,
                cursor: 0,
                selected: BTreeSet::new(),
                ..model
            }
        }
        PickerMessage::Confirm => model,
        PickerMessage::DirLoaded { dir, entries } => PickerModel {
            current_dir: dir,
            entries,
            cursor: 0,
            selected: BTreeSet::new(),
            ..model
        },
    }
}

/// Resolve a MIME type to file extensions.
pub fn mime_to_extensions(mime: &str) -> &'static [&'static str] {
    match mime {
        "image/jpeg" => &["jpg", "jpeg"],
        "image/png" => &["png"],
        "image/webp" => &["webp"],
        "image/gif" => &["gif"],
        "image/bmp" => &["bmp"],
        "image/tiff" => &["tiff", "tif"],
        "image/svg+xml" => &["svg"],
        "text/csv" => &["csv"],
        "application/json" => &["json"],
        "text/plain" => &["txt"],
        "video/mp4" => &["mp4"],
        "video/webm" => &["webm"],
        "audio/mpeg" => &["mp3"],
        _ => &[],
    }
}

/// Collect unique extensions from a list of MIME types.
pub fn extensions_from_mimes(mimes: &[String]) -> Vec<String> {
    let mut exts: Vec<String> = mimes
        .iter()
        .flat_map(|m| mime_to_extensions(m))
        .map(|s| s.to_string())
        .collect();
    exts.sort();
    exts.dedup();
    exts
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Test helpers ---

    fn sample_entries() -> Vec<FileEntry> {
        vec![
            FileEntry {
                name: "docs".into(),
                is_dir: true,
                path: PathBuf::from("/home/user/docs"),
            },
            FileEntry {
                name: "photos".into(),
                is_dir: true,
                path: PathBuf::from("/home/user/photos"),
            },
            FileEntry {
                name: "cat.jpg".into(),
                is_dir: false,
                path: PathBuf::from("/home/user/cat.jpg"),
            },
            FileEntry {
                name: "dog.png".into(),
                is_dir: false,
                path: PathBuf::from("/home/user/dog.png"),
            },
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

    // --- Initial state ---

    #[test]
    fn initial_state_has_slug_and_dir() {
        let m = picker();
        assert_eq!(m.slug, "compress-images");
        assert_eq!(m.current_dir, PathBuf::from("/home/user"));
        assert_eq!(m.cursor, 0);
        assert!(m.selected.is_empty());
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

    // --- Selection ---

    #[test]
    fn toggle_select_adds_file_to_selected() {
        let mut m = picker();
        m.cursor = 2; // cat.jpg (a file)
        let m = update(m, PickerMessage::ToggleSelect);
        assert!(m.selected.contains(&2));
    }

    #[test]
    fn toggle_select_removes_if_already_selected() {
        let mut m = picker();
        m.cursor = 2;
        m.selected.insert(2);
        let m = update(m, PickerMessage::ToggleSelect);
        assert!(!m.selected.contains(&2));
    }

    #[test]
    fn toggle_select_ignores_directories() {
        let m = picker(); // cursor = 0, which is a dir
        let m = update(m, PickerMessage::ToggleSelect);
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
        m.selected.insert(2); // cat.jpg
        m.selected.insert(3); // dog.png
        let result = m.confirm().expect("should return result");
        assert_eq!(result.files.len(), 2);
        assert!(result.files.contains(&PathBuf::from("/home/user/cat.jpg")));
        assert!(result.files.contains(&PathBuf::from("/home/user/dog.png")));
    }

    // --- Extension helpers ---

    #[test]
    fn mime_to_extensions_known_types() {
        assert_eq!(mime_to_extensions("image/jpeg"), &["jpg", "jpeg"]);
        assert_eq!(mime_to_extensions("image/png"), &["png"]);
        assert_eq!(mime_to_extensions("text/csv"), &["csv"]);
    }

    #[test]
    fn mime_to_extensions_unknown_returns_empty() {
        assert!(mime_to_extensions("application/x-unknown").is_empty());
    }

    #[test]
    fn extensions_from_mimes_deduplicates_and_sorts() {
        let mimes = vec!["image/jpeg".into(), "image/png".into(), "image/jpeg".into()];
        let exts = extensions_from_mimes(&mimes);
        assert_eq!(exts, vec!["jpeg", "jpg", "png"]);
    }
}
