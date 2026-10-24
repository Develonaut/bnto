// Copies set aside so a cycle of writes (a swap) can run.
//
// The copies live in a hidden `.bnto-staged-*` folder inside the output
// folder, made only when a cycle needs one. A finished write removes it and
// reports a removal that fails; a failed write keeps it, since a copy may
// then be the only one left. Ctrl+C exits without destructors, so the signal
// handler reports the folders still on disk (`report_kept_staging`), never
// deleting them. `file-collect` skips folders with this prefix.

use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard};

use bnto_file::STAGED_COPIES_PREFIX;

/// The staged copies of one write.
pub struct Staging {
    root: PathBuf,
    dir: Option<tempfile::TempDir>,
    count: usize,
}

impl Staging {
    /// Staging that would make its folder inside `root` when first needed.
    pub fn new(root: &Path) -> Self {
        Self {
            root: root.to_path_buf(),
            dir: None,
            count: 0,
        }
    }

    /// Copy `file` aside and return where the copy is.
    pub fn stage(&mut self, file: &Path) -> Result<PathBuf, String> {
        let leaf = file.file_name().map(|n| n.to_string_lossy().into_owned());
        let name = format!("{}-{}", self.count, leaf.unwrap_or_default());
        let copy = self.dir()?.join(name);
        std::fs::copy(file, &copy)
            .map_err(|e| format!("Cannot set {} aside for the swap: {e}", file.display()))?;
        self.count += 1;
        Ok(copy)
    }

    /// Keep the copies on disk (a failed write); where they are, if any.
    /// The caller reports the folder, so the signal handler no longer does.
    pub fn keep(&mut self) -> Option<PathBuf> {
        let dir = self.dir.take().filter(|_| self.count > 0)?;
        unregister(dir.path());
        Some(dir.keep())
    }

    /// Remove the staging folder after a finished write, naming it if that
    /// fails: a hidden folder of copies left in the user's folder.
    pub fn finish(&mut self) -> Result<(), String> {
        let Some(dir) = self.dir.take() else {
            return Ok(());
        };
        let path = dir.path().to_path_buf();
        unregister(&path);
        dir.close().map_err(|e| {
            format!(
                "Every file was written, but the folder of copies set aside for a swap, \
                 {}, could not be removed ({e}); delete it by hand",
                path.display()
            )
        })
    }

    fn dir(&mut self) -> Result<&Path, String> {
        if self.dir.is_none() {
            let made = tempfile::Builder::new()
                .prefix(STAGED_COPIES_PREFIX)
                .tempdir_in(&self.root)
                .map_err(|e| {
                    format!(
                        "Cannot create a staging folder in {}: {e}",
                        self.root.display()
                    )
                })?;
            lock_live().push(made.path().to_path_buf());
            self.dir = Some(made);
        }
        Ok(self.dir.as_ref().expect("made above").path())
    }
}

impl Drop for Staging {
    /// A staging dropped mid-write (a panic) is removed by its `TempDir`.
    fn drop(&mut self) {
        if let Some(dir) = &self.dir {
            unregister(dir.path());
        }
    }
}

/// Staging folders on disk right now, for the Ctrl+C handler to report.
static LIVE: Mutex<Vec<PathBuf>> = Mutex::new(Vec::new());

fn lock_live() -> MutexGuard<'static, Vec<PathBuf>> {
    LIVE.lock().unwrap_or_else(|e| e.into_inner())
}

fn unregister(path: &Path) {
    lock_live().retain(|live| live != path);
}

/// Print a warning for each staging folder still on disk. Called by the
/// Ctrl+C handler before `process::exit`, which skips destructors: an
/// interrupted swap's copy may be the only one, so it's kept, not deleted.
pub fn report_kept_staging() {
    use colored::Colorize;

    let dirs = std::mem::take(&mut *lock_live());
    for line in kept_lines(&dirs) {
        eprintln!("  {} {line}", "Warning:".yellow().bold());
    }
}

fn kept_lines(dirs: &[PathBuf]) -> Vec<String> {
    let line = |d: &PathBuf| {
        format!(
            "interrupted mid-write; copies set aside for a swap are kept in {}",
            d.display()
        )
    };
    dirs.iter().map(line).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An output folder holding `b.txt` = `BBBB`, staged once.
    fn staged() -> (tempfile::TempDir, Staging, PathBuf) {
        let out = tempfile::tempdir().unwrap();
        let file = out.path().join("b.txt");
        std::fs::write(&file, "BBBB").unwrap();
        let mut staging = Staging::new(out.path());
        let copy = staging.stage(&file).unwrap();
        (out, staging, copy)
    }

    fn is_live(path: &Path) -> bool {
        lock_live().iter().any(|live| path.starts_with(live))
    }

    #[test]
    fn test_nothing_is_created_until_a_file_is_staged() {
        let out = tempfile::tempdir().unwrap();
        let mut staging = Staging::new(out.path());
        assert_eq!(staging.keep(), None);
        staging.finish().unwrap();
        assert_eq!(std::fs::read_dir(out.path()).unwrap().count(), 0);
    }

    #[test]
    fn test_a_finished_write_removes_the_staged_copies() {
        let (out, mut staging, copy) = staged();
        assert_eq!(std::fs::read_to_string(&copy).unwrap(), "BBBB");
        staging.finish().unwrap();
        assert!(!copy.exists());
        assert_eq!(std::fs::read_dir(out.path()).unwrap().count(), 1);
    }

    #[test]
    fn test_a_failed_write_keeps_the_staged_copies() {
        let (_out, mut staging, copy) = staged();
        let kept = staging.keep().unwrap();
        drop(staging);
        assert!(copy.starts_with(&kept));
        assert_eq!(std::fs::read_to_string(&copy).unwrap(), "BBBB");
    }

    #[test]
    fn test_the_folder_is_named_as_file_collect_skips() {
        let (_out, staging, copy) = staged();
        let folder = copy.parent().unwrap().file_name().unwrap();
        assert!(folder.to_string_lossy().starts_with(STAGED_COPIES_PREFIX));
        drop(staging);
    }

    #[test]
    fn test_a_staging_folder_is_reported_on_interrupt_until_the_write_ends() {
        // Ctrl+C skips destructors; while the folder exists the handler must
        // be able to name it, and once removed or reported it drops out.
        let (_out, mut staging, copy) = staged();
        assert!(is_live(&copy));
        staging.finish().unwrap();
        assert!(!is_live(&copy));

        let (_out, mut staging, copy) = staged();
        staging.keep().unwrap();
        assert!(!is_live(&copy), "a failed write's error already names it");
    }

    #[test]
    fn test_the_interrupt_warning_names_each_kept_folder() {
        let lines = kept_lines(&[PathBuf::from("out/.bnto-staged-x")]);
        assert_eq!(lines.len(), 1);
        assert!(lines[0].contains("out/.bnto-staged-x"), "{lines:?}");
        assert!(lines[0].contains("kept"), "{lines:?}");
    }

    #[cfg(unix)]
    #[test]
    fn test_a_folder_that_cannot_be_removed_is_reported() {
        // A read-only folder's files can't be unlinked (unless run as root).
        use std::os::unix::fs::PermissionsExt;
        let (_out, mut staging, copy) = staged();
        let folder = copy.parent().unwrap().to_path_buf();
        let mode = |m| std::fs::Permissions::from_mode(m);
        std::fs::set_permissions(&folder, mode(0o555)).unwrap();
        let result = staging.finish();
        std::fs::set_permissions(&folder, mode(0o755)).unwrap();
        let error = result.unwrap_err();
        assert!(error.contains(&folder.display().to_string()), "{error}");
        assert!(error.contains("could not be removed"), "{error}");
    }

    #[test]
    fn test_two_files_with_one_name_are_staged_apart() {
        let out = tempfile::tempdir().unwrap();
        for dir in ["x", "y"] {
            std::fs::create_dir(out.path().join(dir)).unwrap();
            std::fs::write(out.path().join(dir).join("a.txt"), dir).unwrap();
        }
        let mut staging = Staging::new(out.path());
        let x = staging.stage(&out.path().join("x").join("a.txt")).unwrap();
        let y = staging.stage(&out.path().join("y").join("a.txt")).unwrap();
        assert_ne!(x, y);
        assert_eq!(std::fs::read_to_string(&x).unwrap(), "x");
        assert_eq!(std::fs::read_to_string(&y).unwrap(), "y");
    }
}
