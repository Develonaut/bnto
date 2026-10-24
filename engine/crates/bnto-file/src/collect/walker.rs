// One directory walk for file-collect: what to match, where to report,
// and what it found.
//
// Built for folders of hundreds of thousands of files: the glob compiles
// once per walk, and each entry's type comes from `read_dir` (no extra
// stat) unless the entry is a link.

use std::fs::DirEntry;
use std::path::{Path, PathBuf};

use bnto_core::BntoError;
use bnto_core::processor::NodeInput;
use bnto_core::progress::ProgressReporter;

use super::{param_bool, param_str};
use crate::glob::Glob;

/// One directory walk: its settings, where it reports, and what it found.
pub(super) struct Walker<'a> {
    /// The walked root (for computing relative paths).
    base_dir: &'a Path,
    glob: Glob,
    recursive: bool,
    flatten: bool,
    progress: &'a ProgressReporter,
    /// `(output name, full path)` of each matched file, in walk order.
    pub(super) found: Vec<(String, PathBuf)>,
}

/// What a directory entry is, links followed.
enum Kind {
    Dir,
    File,
    /// Neither, or a broken link.
    Other,
}

impl<'a> Walker<'a> {
    pub(super) fn new(
        base_dir: &'a Path,
        input: &NodeInput,
        progress: &'a ProgressReporter,
    ) -> Self {
        Self {
            base_dir,
            glob: Glob::new(param_str(input, "pattern").unwrap_or("*")),
            recursive: param_bool(input, "recursive").unwrap_or(true),
            flatten: param_bool(input, "flatten").unwrap_or(true),
            progress,
            found: Vec::new(),
        }
    }

    /// Collect the files in `dir` that match the pattern, and those in its
    /// subdirectories when recursive.
    ///
    /// Reports the running count after every entry, so a walk of a huge
    /// tree keeps moving; the reporter throttles what reaches the user.
    pub(super) fn walk(&mut self, dir: &Path) -> Result<(), BntoError> {
        let entries = std::fs::read_dir(dir).map_err(|e| {
            BntoError::ProcessingFailed(format!("Failed to read directory {}: {e}", dir.display()))
        })?;
        let label = self.folder_label(dir);
        for entry in entries {
            let entry = entry.map_err(|e| {
                BntoError::ProcessingFailed(format!("Failed to read directory entry: {e}"))
            })?;
            self.visit(&entry)?;
            self.report(&label);
        }
        Ok(())
    }

    fn visit(&mut self, entry: &DirEntry) -> Result<(), BntoError> {
        match kind(entry) {
            Kind::Dir if is_staged_copies(entry) => Ok(()),
            Kind::Dir if self.recursive => self.walk(&entry.path()),
            Kind::File => {
                self.collect(entry.path());
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn collect(&mut self, path: PathBuf) {
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if !self.glob.matches(file_name) {
            return;
        }
        let output_name = if self.flatten {
            file_name.to_string()
        } else {
            bnto_core::slash_path(path.strip_prefix(self.base_dir).unwrap_or(&path))
        };
        self.found.push((output_name, path));
    }

    /// `dir` relative to the walked root, `/`-separated; empty for the root.
    fn folder_label(&self, dir: &Path) -> String {
        bnto_core::slash_path(dir.strip_prefix(self.base_dir).unwrap_or(dir))
    }

    fn report(&self, label: &str) {
        let count = self.found.len();
        let message = if label.is_empty() {
            format!("Found {count} files")
        } else {
            format!("Found {count} files · {label}")
        };
        self.progress.report_count(count as u64, &message);
    }
}

/// A folder of copies the CLI's output step set aside for a swap, left by
/// an interrupted run: never a run's input (see `STAGED_COPIES_PREFIX`).
fn is_staged_copies(entry: &DirEntry) -> bool {
    let name = entry.file_name();
    name.to_string_lossy()
        .starts_with(crate::STAGED_COPIES_PREFIX)
}

/// The entry's type as `Path::is_dir` / `is_file` would see it. `read_dir`
/// hands back the type for free (on Windows, and on most Unix filesystems);
/// only a link (a symlink, or a Windows junction) costs a stat to follow.
fn kind(entry: &DirEntry) -> Kind {
    let Ok(file_type) = entry.file_type() else {
        return Kind::Other;
    };
    if !file_type.is_symlink() {
        return kind_of(file_type.is_dir(), file_type.is_file());
    }
    match std::fs::metadata(entry.path()) {
        Ok(target) => kind_of(target.is_dir(), target.is_file()),
        Err(_) => Kind::Other,
    }
}

fn kind_of(is_dir: bool, is_file: bool) -> Kind {
    match (is_dir, is_file) {
        (true, _) => Kind::Dir,
        (_, true) => Kind::File,
        _ => Kind::Other,
    }
}
