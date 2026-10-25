// Files a run of moves has parked under a temporary name right now.
//
// A swap parks one file (or folder, for `file-move`) under a
// `PARKED_NAME_PREFIX` name until its final rename. A host that ends the
// process mid-run (the CLI's Ctrl+C runs `process::exit`, skipping cleanup)
// takes this list and names what is still there; the parked file is the
// only copy of its content, so it is never deleted. Moves don't stop for
// that, so each park renames with the list locked, and once the list is
// taken no park runs: every parked file is either named or never made. A
// failed run keeps its entries: its error names them, but the process can
// end before that error is shown (a duplicate line at worst).

use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard};

/// The parked files, and whether the process's end already took them.
struct Registry {
    files: Vec<PathBuf>,
    closed: bool,
}

impl Registry {
    const fn new() -> Self {
        Self {
            files: Vec::new(),
            closed: false,
        }
    }

    fn park(
        &mut self,
        temp: &Path,
        rename: impl FnOnce() -> Result<(), String>,
    ) -> Result<(), String> {
        if self.closed {
            return Err(format!(
                "Stopped: the run is ending, so nothing was moved to {}",
                temp.display()
            ));
        }
        rename()?;
        self.files.push(temp.to_path_buf());
        Ok(())
    }

    fn take(&mut self) -> Vec<PathBuf> {
        self.closed = true;
        still_there(std::mem::take(&mut self.files))
    }
}

static PARKED: Mutex<Registry> = Mutex::new(Registry::new());

fn lock() -> MutexGuard<'static, Registry> {
    PARKED.lock().unwrap_or_else(|e| e.into_inner())
}

/// Park a file: run `rename` (its move to `temp`) with the list locked and
/// register `temp` once it succeeds. Refused once the list was taken.
pub(super) fn park(temp: &Path, rename: impl FnOnce() -> Result<(), String>) -> Result<(), String> {
    lock().park(temp, rename)
}

pub(super) fn unregister(path: &Path) {
    lock().files.retain(|p| p != path);
}

/// Every file a move parked and hasn't renamed since that is still on disk
/// (the user may have dealt with it already), emptying the list. For
/// whatever ends the process once a run is stopped: no park runs after.
pub fn take_parked() -> Vec<PathBuf> {
    lock().take()
}

fn still_there(files: Vec<PathBuf>) -> Vec<PathBuf> {
    files.into_iter().filter(|f| super::occupied(f)).collect()
}

/// Whether `path` is registered as parked.
#[cfg(test)]
pub(super) fn is_parked(path: &Path) -> bool {
    lock().files.iter().any(|p| p == path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_parked_file_removed_since_is_not_reported() {
        let tmp = tempfile::tempdir().unwrap();
        let here = tmp.path().join(".bnto-rename-0-a.wav");
        std::fs::write(&here, "A").unwrap();
        let gone = tmp.path().join(".bnto-rename-0-b.wav");
        assert_eq!(still_there(vec![gone, here.clone()]), [here]);
    }

    #[test]
    fn test_a_park_is_registered_only_once_its_rename_succeeds() {
        let tmp = tempfile::tempdir().unwrap();
        let temp = tmp.path().join(".bnto-rename-0-a.wav");
        let mut registry = Registry::new();
        let failed = registry.park(&temp, || Err("cannot rename".to_string()));
        assert_eq!(failed, Err("cannot rename".to_string()));
        assert!(registry.files.is_empty());
        registry
            .park(&temp, || {
                std::fs::write(&temp, "A").map_err(|e| e.to_string())
            })
            .unwrap();
        assert_eq!(registry.take(), [temp]);
    }

    #[test]
    fn test_nothing_is_parked_once_the_list_is_taken() {
        // RED (ISS-53 review): moves don't stop for the Ctrl+C handler, so a
        // file parked after the report took the list would go unnamed.
        let tmp = tempfile::tempdir().unwrap();
        let temp = tmp.path().join(".bnto-rename-0-a.wav");
        let mut registry = Registry::new();
        assert!(registry.take().is_empty());
        let mut renamed = false;
        let refused = registry.park(&temp, || {
            renamed = true;
            Ok(())
        });
        assert!(!renamed, "the rename must not run");
        assert!(refused.unwrap_err().contains("the run is ending"));
        assert!(registry.files.is_empty());
    }
}
