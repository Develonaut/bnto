// What a progressive loop already wrote, for the final write's refusal.
//
// Progressive writes copy each iteration's files as it ends, before later
// iterations' files exist, so a conflict between destinations (a folder link
// making `sub/b.txt` and `b.txt` one file) can only be seen by the final
// write, after some files are on disk. Its refusal can't say "nothing was
// written" then: it names the files iterations wrote. The progressive writer
// records each path here; the final write's paths are the same (one names
// registry spans the run), so they're matched exactly.

use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard};

static WRITTEN: Mutex<Vec<PathBuf>> = Mutex::new(Vec::new());

fn lock() -> MutexGuard<'static, Vec<PathBuf>> {
    WRITTEN.lock().unwrap_or_else(|e| e.into_inner())
}

/// A progressive write put a file at `path`.
pub fn note_progressive(path: &Path) {
    lock().push(path.to_path_buf());
}

/// A new run into `dir` starts: what earlier runs wrote there is no longer
/// this run's (the TUI runs many recipes in one process).
pub fn forget_progressive(dir: &Path) {
    lock().retain(|p| !p.starts_with(dir));
}

/// Which of `paths` progressive writes already put on disk.
pub fn already_written(paths: &[PathBuf]) -> Vec<PathBuf> {
    let written = lock();
    let found = paths.iter().filter(|p| written.contains(p));
    found.cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_only_paths_of_this_plan_are_reported() {
        let dir = tempfile::tempdir().unwrap();
        let (a, b) = (dir.path().join("a.txt"), dir.path().join("b.txt"));
        note_progressive(&a);
        let written = already_written(&[a.clone(), b.clone()]);
        assert_eq!(written, std::slice::from_ref(&a));
        forget_progressive(dir.path());
        assert!(already_written(&[a, b]).is_empty());
    }
}
