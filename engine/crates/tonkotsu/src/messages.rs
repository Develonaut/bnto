// Form message types and side effects.

use std::path::PathBuf;

use crate::file_entry::FileEntry;

/// Side effects that the form needs the caller to fulfill.
///
/// After each `update()`, the caller should drain `pending_effects` and
/// fulfill them (e.g., read a directory), then feed results back via
/// `FormMessage::FilePathDirLoaded`.
#[derive(Debug, Clone)]
pub enum FormEffect {
    /// Request directory listing. Caller reads the filesystem and responds
    /// with `FormMessage::FilePathDirLoaded`.
    LoadDirectory {
        field_id: String,
        path: PathBuf,
        extensions: Vec<String>,
        show_hidden: bool,
    },
}

/// All possible user actions across field types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormMessage {
    // --- Navigation ---
    FocusNext,
    FocusPrev,

    // --- Edit lifecycle ---
    StartEdit,
    CommitEdit,
    CancelEdit,

    // --- Text input (TextEditing / NumberEditing) ---
    EditChar(char),
    EditBackspace,
    DeleteForward,
    CursorLeft,
    CursorRight,
    CursorHome,
    CursorEnd,
    CursorWordBack,
    CursorWordForward,
    DeleteWordBack,

    // --- Inline actions (no edit mode needed) ---
    ToggleConfirm,
    CycleNext,
    CyclePrev,
    ResetDefault,

    // --- Select list (SelectExpanded) ---
    SelectHighlightNext,
    SelectHighlightPrev,
    SelectConfirm,
    SelectFilterChar(char),
    SelectFilterBackspace,

    // --- FilePath browser ---
    FilePathCursorDown,
    FilePathCursorUp,
    FilePathEnterDir,
    FilePathParentDir,
    FilePathConfirm,
    FilePathToggleHidden,
    FilePathPageDown,
    FilePathPageUp,
    FilePathGoToTop,
    FilePathGoToBottom,
    FilePathCancel,
    /// Caller response to `FormEffect::LoadDirectory`.
    FilePathDirLoaded {
        field_id: String,
        dir: PathBuf,
        entries: Vec<FileEntry>,
    },

    // --- MultiSelect ---
    MultiSelectToggle,
    MultiSelectHighlightNext,
    MultiSelectHighlightPrev,
    MultiSelectConfirm,

    // --- TextArea ---
    TextAreaNewline,
    TextAreaCursorUp,
    TextAreaCursorDown,

    // --- Viewport ---
    Resize {
        height: usize,
    },
}
