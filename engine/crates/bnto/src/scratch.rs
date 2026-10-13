// Per-run scratch directory for a dry run's temp files.
//
// A dry run still executes shell-command nodes, and their output lands in
// temp files that only a real run's output step moves away. Creating them
// under one bnto-owned directory lets the run delete exactly what it made.

use std::path::{Path, PathBuf};
use std::sync::Mutex;

use bnto_core::errors::BntoError;
use tempfile::TempDir;

/// A lazily created `bnto-run-*` directory in the system temp dir, removed
/// (with everything in it) when the scratch is dropped.
#[derive(Default)]
pub struct RunScratch {
    dir: Mutex<Option<TempDir>>,
}

impl RunScratch {
    /// Create a persisted temp file inside the scratch dir, creating the
    /// dir on first use so runs that never ask for one touch nothing.
    pub fn temp_file(&self, suffix: &str) -> Result<PathBuf, BntoError> {
        let mut dir = self.dir.lock().unwrap_or_else(|e| e.into_inner());
        if dir.is_none() {
            *dir = Some(create_scratch_dir()?);
        }
        let root = dir.as_ref().map(TempDir::path).expect("created above");
        persisted_temp_file(Some(root), suffix)
    }

    /// The scratch dir, if one has been created.
    #[cfg(test)]
    pub fn path(&self) -> Option<PathBuf> {
        let dir = self.dir.lock().unwrap_or_else(|e| e.into_inner());
        dir.as_ref().map(|d| d.path().to_path_buf())
    }
}

/// A fresh, uniquely named directory owned by this run.
fn create_scratch_dir() -> Result<TempDir, BntoError> {
    tempfile::Builder::new()
        .prefix("bnto-run-")
        .tempdir()
        .map_err(|e| BntoError::ProcessingFailed(format!("Failed to create scratch dir: {e}")))
}

/// Atomically create a `bnto-*<suffix>` file that outlives its handle, in
/// `dir` or (with `None`) the system temp dir.
pub fn persisted_temp_file(dir: Option<&Path>, suffix: &str) -> Result<PathBuf, BntoError> {
    let mut builder = tempfile::Builder::new();
    builder.prefix("bnto-").suffix(suffix);
    let file = match dir {
        Some(dir) => builder.tempfile_in(dir),
        None => builder.tempfile(),
    };
    file.map_err(|e| BntoError::ProcessingFailed(format!("Failed to create temp file: {e}")))?
        .into_temp_path()
        .keep()
        .map_err(|e| BntoError::ProcessingFailed(format!("Failed to persist temp file: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_dir_until_first_temp_file() {
        let scratch = RunScratch::default();
        assert!(scratch.path().is_none());
    }

    #[test]
    fn test_temp_file_lives_inside_scratch_dir() {
        let scratch = RunScratch::default();
        let file = scratch.temp_file("-output").unwrap();
        let dir = scratch.path().unwrap();
        assert!(file.exists());
        assert_eq!(file.parent(), Some(dir.as_path()));
        let name = dir.file_name().unwrap().to_string_lossy().into_owned();
        assert!(name.starts_with("bnto-run-"), "{name}");
    }

    #[test]
    fn test_temp_files_share_one_scratch_dir() {
        let scratch = RunScratch::default();
        let a = scratch.temp_file(".a").unwrap();
        let b = scratch.temp_file(".b").unwrap();
        assert_ne!(a, b);
        assert_eq!(a.parent(), b.parent());
    }

    #[test]
    fn test_drop_removes_files_and_sibling_dirs() {
        // Mirrors shell-command file mode: a temp file plus a `.d` output
        // dir next to it, with the command's output inside.
        let scratch = RunScratch::default();
        let file = scratch.temp_file("-output").unwrap();
        let out_dir = file.with_extension("d");
        std::fs::create_dir_all(&out_dir).unwrap();
        std::fs::write(out_dir.join("video.mp4"), b"big").unwrap();
        let dir = scratch.path().unwrap();
        drop(scratch);
        assert!(!dir.exists(), "scratch dir must be removed");
        assert!(!file.exists());
        assert!(!out_dir.exists());
    }

    #[test]
    fn test_drop_leaves_files_outside_scratch_alone() {
        let outside = tempfile::tempdir().unwrap();
        let kept = outside.path().join("kept.txt");
        std::fs::write(&kept, b"keep").unwrap();
        let scratch = RunScratch::default();
        scratch.temp_file(".tmp").unwrap();
        drop(scratch);
        assert_eq!(std::fs::read(&kept).unwrap(), b"keep");
    }

    #[test]
    fn test_persisted_temp_file_outlives_its_handle() {
        let dir = tempfile::tempdir().unwrap();
        let file = persisted_temp_file(Some(dir.path()), ".txt").unwrap();
        assert!(file.exists());
        assert!(file.to_string_lossy().ends_with(".txt"));
    }
}
