// Exclusive creation of every destination that's new to this run.
//
// Before any write, each destination that didn't exist when the run planned
// its writes is created empty with `create_new` (O_EXCL). A filesystem can
// treat two different names as one (Unicode normalization on APFS, trailing
// dots and spaces or 8.3 short names on Windows), and no name comparison
// knows them all; the exclusive create does. The second name of such a pair
// fails, and so does a file that appeared after planning, so the plan is
// refused before anything is replaced. The claims are undone then, and when a
// later write fails or stops: an empty file this run made and never wrote is
// removed, never anything else.

use std::collections::HashSet;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use bnto_core::processor::FileData;

/// The destinations this run created empty, until each is written.
pub struct Claims {
    files: Vec<(usize, PathBuf)>,
    /// Folders made for the claims, deepest last.
    dirs: Vec<PathBuf>,
    written: HashSet<usize>,
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
            written: HashSet::new(),
        };
        let new = paths.iter().enumerate().filter(|&(i, _)| missing[i]);
        for (i, path) in new {
            claims.claim_one(i, path)?;
        }
        Ok(claims)
    }

    /// Write `i` ran: its claimed file now holds its content.
    pub fn written(&mut self, i: usize) {
        self.written.insert(i);
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
                 as one), so one write would replace the other",
                earlier.display(),
                path.display()
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
    /// Undo the claims never written: empty files this run made, then the
    /// folders made for them, if empty.
    fn drop(&mut self) {
        let unwritten = self.files.iter().filter(|(i, _)| !self.written.contains(i));
        for (_, path) in unwritten {
            if std::fs::metadata(path).is_ok_and(|m| m.len() == 0) {
                let _ = std::fs::remove_file(path);
            }
        }
        for dir in self.dirs.iter().rev() {
            let _ = std::fs::remove_dir(dir);
        }
    }
}

fn is_same_file(a: &Path, b: &Path) -> bool {
    FileData::Path(a.to_path_buf()).is_at(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_destinations_are_created_and_kept_once_written() {
        let out = tempfile::tempdir().unwrap();
        let paths = [
            out.path().join("a.txt"),
            out.path().join("sub").join("b.txt"),
        ];
        let mut claims = Claims::claim(&paths, &missing(&paths)).unwrap();
        assert!(paths.iter().all(|p| p.exists()));
        std::fs::write(&paths[0], "A").unwrap();
        std::fs::write(&paths[1], "B").unwrap();
        claims.written(0);
        claims.written(1);
        drop(claims);
        assert_eq!(std::fs::read_to_string(&paths[1]).unwrap(), "B");
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

    #[test]
    fn test_an_unwritten_claim_is_removed_but_a_written_one_kept() {
        let out = tempfile::tempdir().unwrap();
        let paths = [out.path().join("a.txt"), out.path().join("b.txt")];
        let mut claims = Claims::claim(&paths, &missing(&paths)).unwrap();
        std::fs::write(&paths[0], "A").unwrap();
        claims.written(0);
        drop(claims);
        assert!(paths[0].exists());
        assert!(
            !paths[1].exists(),
            "a write that never ran leaves no empty file"
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_two_names_one_filesystem_treats_as_one_are_refused() {
        let out = tempfile::tempdir().unwrap();
        let paths = [out.path().join("caf\u{e9}"), out.path().join("cafe\u{301}")];
        let error = Claims::claim(&paths, &missing(&paths)).err().unwrap();
        assert!(error.contains("treats the two names as one"), "{error}");
        assert_eq!(std::fs::read_dir(out.path()).unwrap().count(), 0);
    }
}
