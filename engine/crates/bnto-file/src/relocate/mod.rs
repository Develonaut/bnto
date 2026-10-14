// Checks and runs a batch of file moves so no move ever replaces a file.
//
// Shared by `output` overwrite mode (renames within one folder) and
// `file-move` (moves under a destination folder). A plan that would put two
// files on one path, or onto a file that isn't moving away, is refused whole,
// so a mapping table with a mistake can't silently replace a file. Accepted
// plans run in an order that keeps every file (a→b, b→c moves b first; a swap
// goes through a temporary name), and each move re-checks its target by file
// identity, not by name, so case-sensitive filesystems are safe too.

mod conflicts;
mod execute;
mod platform;

use std::path::{Path, PathBuf};

pub use conflicts::{conflicts, listing};
pub use execute::run_in_order;
pub use platform::check_platform_name;

/// One move, `src` to `dest`. A rename is a move within one folder.
#[derive(Debug, Clone, PartialEq)]
pub struct Move {
    pub src: PathBuf,
    pub dest: PathBuf,
}

/// Case-folded path, so `A.wav` and `a.wav` count as one name, as they are on
/// Windows and macOS. Only ever used to find candidates; identity is decided
/// by `same_file`.
fn key(path: &Path) -> String {
    path.to_string_lossy().to_lowercase()
}

/// True when `a` and `b` are the same file on disk (false if either is missing).
pub(crate) fn same_file(a: &Path, b: &Path) -> bool {
    same_file::is_same_file(a, b).unwrap_or(false)
}

/// True when anything (file, folder, link) sits at `path`.
pub(crate) fn occupied(path: &Path) -> bool {
    std::fs::symlink_metadata(path).is_ok()
}

#[cfg(test)]
mod tests;
