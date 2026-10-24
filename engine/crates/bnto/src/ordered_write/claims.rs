// Exclusive creation of every destination that's new to this run, as a
// probe for names a filesystem treats as one.
//
// Before any write, each destination that didn't exist when the run planned
// its writes is created empty with `create_new` (O_EXCL). A filesystem can
// treat two different names as one (Unicode normalization on APFS, trailing
// dots and spaces or 8.3 short names on Windows), and no name comparison
// knows them all; the exclusive create does. The second name of such a pair
// fails, and so does a file that appeared after planning, so the plan is
// refused before anything is replaced. Just before its own write, each
// placeholder is removed again (`release`), so the copy creates the file
// itself: copying onto an existing file can't clone it (APFS `clonefile`),
// turning an O(1) copy of a large file into a full one. Undoing only ever
// removes a placeholder this run made that is still empty, and the folders
// made for it if they're empty. SIGKILL or power loss after claiming can
// leave such empty files under output names; a rerun replaces them.

use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use bnto_core::processor::FileData;

use super::aliases::shown;

/// The empty placeholders this run created, until each write releases its own.
pub struct Claims {
    files: Vec<(usize, PathBuf)>,
    /// Folders made for the claims, deepest last.
    dirs: Vec<PathBuf>,
}

/// Which destinations don't exist now: the ones `claim` will create.
pub fn missing(paths: &[PathBuf]) -> Vec<bool> {
    let absent = |p: &PathBuf| std::fs::symlink_metadata(p).is_err();
    paths.iter().map(absent).collect()
}

impl Claims {
    /// Create each `paths[i]` with `missing[i]` exclusively, in order.
    /// On a clash, undo the claims made and say which names collided.
    pub fn claim(paths: &[PathBuf], missing: &[bool]) -> Result<Self, String> {
        let mut claims = Self {
            files: Vec::new(),
            dirs: Vec::new(),
        };
        let new = paths.iter().enumerate().filter(|&(i, _)| missing[i]);
        for (i, path) in new {
            claims.claim_one(i, path)?;
        }
        Ok(claims)
    }

    /// Write `i` is about to run: remove its placeholder, if it's still the
    /// empty file this run made, so the write creates the file itself.
    pub fn release(&mut self, i: usize) {
        if let Some(at) = self.files.iter().position(|(k, _)| *k == i) {
            let (_, path) = self.files.remove(at);
            remove_if_empty(&path);
        }
    }

    fn claim_one(&mut self, i: usize, path: &Path) -> Result<(), String> {
        self.make_parent(path)?;
        let created = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path);
        match created {
            Ok(_) => {
                self.files.push((i, path.to_path_buf()));
                Ok(())
            }
            Err(e) if e.kind() == ErrorKind::AlreadyExists => Err(self.clash(path)),
            Err(e) => Err(format!("Cannot create {}: {e}", path.display())),
        }
    }

    /// Create the folders `path` needs, remembering the ones made here.
    fn make_parent(&mut self, path: &Path) -> Result<(), String> {
        let Some(parent) = path.parent() else {
            return Ok(());
        };
        let mut made: Vec<PathBuf> = parent
            .ancestors()
            .take_while(|dir| std::fs::symlink_metadata(dir).is_err())
            .map(Path::to_path_buf)
            .collect();
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Cannot create {}: {e}", parent.display()))?;
        made.reverse();
        self.dirs.extend(made);
        Ok(())
    }

    /// The refusal for `path`, which exists though it didn't at planning.
    fn clash(&self, path: &Path) -> String {
        let same = |(_, claimed): &&(usize, PathBuf)| is_same_file(claimed, path);
        let detail = match self.files.iter().find(same) {
            Some((_, earlier)) => format!(
                "{} and {} would be one file on disk (the filesystem treats the two names \
                 as one file), so one write would replace the other",
                shown(earlier),
                shown(path)
            ),
            None => format!(
                "{} appeared after the run planned its writes",
                path.display()
            ),
        };
        format!("Output refused, nothing was written: 1 conflict\n  {detail}")
    }
}

impl Drop for Claims {
    /// Undo the claims whose write never ran (a refusal, a failed write, a
    /// stop): the placeholders, then the folders made for them, if empty.
    fn drop(&mut self) {
        for (_, path) in &self.files {
            remove_if_empty(path);
        }
        for dir in self.dirs.iter().rev() {
            let _ = std::fs::remove_dir(dir);
        }
    }
}

/// Remove a placeholder, only while it's still empty: never a file with content.
fn remove_if_empty(path: &Path) {
    if std::fs::symlink_metadata(path).is_ok_and(|m| m.is_file() && m.len() == 0) {
        let _ = std::fs::remove_file(path);
    }
}

fn is_same_file(a: &Path, b: &Path) -> bool {
    FileData::Path(a.to_path_buf()).is_at(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_released_placeholder_is_gone_so_the_write_creates_the_file() {
        // ISS-48 review 4: copying onto the placeholder can't clone (APFS),
        // so a 1 GB copy took 244 ms instead of 0.4 ms.
        let out = tempfile::tempdir().unwrap();
        let paths = [
            out.path().join("a.txt"),
            out.path().join("sub").join("b.txt"),
        ];
        let mut claims = Claims::claim(&paths, &missing(&paths)).unwrap();
        assert!(paths.iter().all(|p| p.exists()), "claimed at plan time");
        claims.release(1);
        assert!(!paths[1].exists(), "the write will create it itself");
        assert!(
            out.path().join("sub").exists(),
            "its folder stays for the write"
        );
        std::fs::write(&paths[1], "B").unwrap();
        drop(claims);
        assert_eq!(std::fs::read_to_string(&paths[1]).unwrap(), "B");
        assert!(!paths[0].exists(), "a claim never released is undone");
    }

    #[test]
    fn test_releasing_leaves_a_file_that_has_content() {
        let out = tempfile::tempdir().unwrap();
        let paths = [out.path().join("a.txt")];
        let mut claims = Claims::claim(&paths, &missing(&paths)).unwrap();
        std::fs::write(&paths[0], "NOT OURS").unwrap();
        claims.release(0);
        assert_eq!(std::fs::read_to_string(&paths[0]).unwrap(), "NOT OURS");
    }

    #[test]
    fn test_a_file_appearing_after_planning_is_refused_and_nothing_is_left() {
        // The portable stand-in for two names one filesystem treats as one:
        // the second claim finds the file already there.
        let out = tempfile::tempdir().unwrap();
        let paths = [
            out.path().join("new").join("a.txt"),
            out.path().join("b.txt"),
        ];
        let planned = missing(&paths);
        std::fs::write(&paths[1], "SOMEONE ELSE").unwrap();
        let error = Claims::claim(&paths, &planned).err().unwrap();
        assert!(
            error.starts_with("Output refused, nothing was written"),
            "{error}"
        );
        assert!(error.contains("appeared after the run planned"), "{error}");
        assert!(
            !out.path().join("new").exists(),
            "the claim and its folder are undone"
        );
        assert_eq!(std::fs::read_to_string(&paths[1]).unwrap(), "SOMEONE ELSE");
    }

    #[test]
    fn test_an_existing_destination_is_not_claimed() {
        let out = tempfile::tempdir().unwrap();
        let path = out.path().join("a.txt");
        std::fs::write(&path, "OLD").unwrap();
        let paths = [path.clone()];
        drop(Claims::claim(&paths, &missing(&paths)).unwrap());
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "OLD");
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_two_names_one_filesystem_treats_as_one_are_refused() {
        let out = tempfile::tempdir().unwrap();
        let paths = [out.path().join("caf\u{e9}"), out.path().join("cafe\u{301}")];
        let error = Claims::claim(&paths, &missing(&paths)).err().unwrap();
        assert!(error.contains("treats the two names as one"), "{error}");
        assert!(
            error.contains("caf\\u{e9}") && error.contains("cafe\\u{301}"),
            "the two names print apart: {error}"
        );
        assert_eq!(std::fs::read_dir(out.path()).unwrap().count(), 0);
    }
}
