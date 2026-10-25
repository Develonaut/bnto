// Files a run of moves has parked under a temporary name right now.
//
// A swap parks one file under a `PARKED_NAME_PREFIX` name until its final
// rename. A host that ends the process mid-run (the CLI's Ctrl+C runs
// `process::exit`, skipping cleanup) takes this list and names what is still
// there; the parked file is the only copy of its content, so it is never
// deleted. A failed run keeps its entries: its error names them, but the
// process can end before that error is shown (a duplicate line at worst).

use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard};

static PARKED: Mutex<Vec<PathBuf>> = Mutex::new(Vec::new());

fn lock() -> MutexGuard<'static, Vec<PathBuf>> {
    PARKED.lock().unwrap_or_else(|e| e.into_inner())
}

pub(super) fn register(path: &Path) {
    lock().push(path.to_path_buf());
}

pub(super) fn unregister(path: &Path) {
    lock().retain(|p| p != path);
}

/// Every file a move parked and hasn't renamed since that is still on disk
/// (the user may have dealt with it already), emptying the list. For
/// whatever ends the process once a run is stopped.
pub fn take_parked() -> Vec<PathBuf> {
    still_there(std::mem::take(&mut *lock()))
}

fn still_there(files: Vec<PathBuf>) -> Vec<PathBuf> {
    files.into_iter().filter(|f| super::occupied(f)).collect()
}

/// Whether `path` is registered as parked.
#[cfg(test)]
pub(super) fn is_parked(path: &Path) -> bool {
    lock().iter().any(|p| p == path)
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
}
