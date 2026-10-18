// Per-run scratch directory for a run's temp files.
//
// Nodes such as shell-command write their output to temp files. The output
// step copies results out (it must never move a user's own files), so the
// run deletes its temp files itself. Creating them under one bnto-owned
// directory lets it delete exactly what it made.

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
            let created = create_scratch_dir()?;
            lock_live().push(created.path().to_path_buf());
            *dir = Some(created);
        }
        let root = dir.as_ref().map(TempDir::path).expect("created above");
        persisted_temp_file(root, suffix)
    }

    /// The scratch dir, if one has been created.
    #[cfg(test)]
    pub fn path(&self) -> Option<PathBuf> {
        let dir = self.dir.lock().unwrap_or_else(|e| e.into_inner());
        dir.as_ref().map(|d| d.path().to_path_buf())
    }
}

impl Drop for RunScratch {
    /// Unregister before the `TempDir` field deletes the directory.
    fn drop(&mut self) {
        let dir = self.dir.get_mut().unwrap_or_else(|e| e.into_inner());
        if let Some(path) = dir.as_ref().map(TempDir::path) {
            lock_live().retain(|live| live != path);
        }
    }
}

/// Scratch dirs that exist right now. `process::exit` (the Ctrl+C path)
/// skips destructors, so the signal handler removes these itself.
static LIVE_DIRS: Mutex<Vec<PathBuf>> = Mutex::new(Vec::new());

fn lock_live() -> std::sync::MutexGuard<'static, Vec<PathBuf>> {
    LIVE_DIRS.lock().unwrap_or_else(|e| e.into_inner())
}

/// Delete every live scratch dir. Called before `process::exit` — from the
/// signal handler after child processes are killed, and when a run fails.
pub fn remove_live_dirs() {
    let dirs = std::mem::take(&mut *lock_live());
    remove_dirs(&dirs);
}

/// Remove each dir and its contents; `remove_dir_all` deletes symlinks
/// without following them, so nothing outside a dir is touched.
fn remove_dirs(dirs: &[PathBuf]) {
    for dir in dirs {
        let _ = std::fs::remove_dir_all(dir);
    }
}

#[cfg(test)]
fn live_dirs() -> Vec<PathBuf> {
    lock_live().clone()
}

/// A fresh, uniquely named directory owned by this run.
fn create_scratch_dir() -> Result<TempDir, BntoError> {
    tempfile::Builder::new()
        .prefix("bnto-run-")
        .tempdir()
        .map_err(|e| BntoError::ProcessingFailed(format!("Failed to create scratch dir: {e}")))
}

/// Atomically create a `bnto-*<suffix>` file in `dir` that outlives its handle.
fn persisted_temp_file(dir: &Path, suffix: &str) -> Result<PathBuf, BntoError> {
    tempfile::Builder::new()
        .prefix("bnto-")
        .suffix(suffix)
        .tempfile_in(dir)
        .map_err(|e| BntoError::ProcessingFailed(format!("Failed to create temp file: {e}")))?
        .into_temp_path()
        .keep()
        .map_err(|e| BntoError::ProcessingFailed(format!("Failed to persist temp file: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Interrupt cleanup ---

    #[test]
    fn test_scratch_dir_is_live_until_dropped() {
        let scratch = RunScratch::default();
        scratch.temp_file("-output").unwrap();
        let dir = scratch.path().unwrap();
        assert!(live_dirs().contains(&dir));
        drop(scratch);
        assert!(!live_dirs().contains(&dir));
    }

    #[test]
    fn test_remove_dirs_deletes_scratch_and_contents() {
        // What the Ctrl+C handler does for each live dir before exiting,
        // since `process::exit` skips the scratch's own drop.
        let scratch = RunScratch::default();
        let file = scratch.temp_file("-output").unwrap();
        let dir = scratch.path().unwrap();
        std::fs::create_dir_all(file.with_extension("d")).unwrap();
        remove_dirs(std::slice::from_ref(&dir));
        assert!(!dir.exists());
    }

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
        let file = persisted_temp_file(dir.path(), ".txt").unwrap();
        assert!(file.exists());
        assert!(file.to_string_lossy().ends_with(".txt"));
    }
}
