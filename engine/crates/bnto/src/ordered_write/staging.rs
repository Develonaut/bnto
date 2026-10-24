// Copies set aside so a cycle of writes (a swap) can run.
//
// The copies live in a hidden temp folder inside the output folder, made
// only when a cycle needs one, and removed when the write ends. If the write
// fails, the folder is kept: a copy may then be the only one left.

use std::path::{Path, PathBuf};

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
    pub fn keep(self) -> Option<PathBuf> {
        let dir = self.dir.filter(|_| self.count > 0)?;
        Some(dir.keep())
    }

    fn dir(&mut self) -> Result<&Path, String> {
        if self.dir.is_none() {
            let made = tempfile::Builder::new()
                .prefix(".bnto-staged-")
                .tempdir_in(&self.root)
                .map_err(|e| {
                    format!(
                        "Cannot create a staging folder in {}: {e}",
                        self.root.display()
                    )
                })?;
            self.dir = Some(made);
        }
        Ok(self.dir.as_ref().expect("made above").path())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nothing_is_created_until_a_file_is_staged() {
        let out = tempfile::tempdir().unwrap();
        let staging = Staging::new(out.path());
        assert_eq!(staging.keep(), None);
        assert_eq!(std::fs::read_dir(out.path()).unwrap().count(), 0);
    }

    #[test]
    fn test_staged_copies_are_removed_when_the_write_ends() {
        let out = tempfile::tempdir().unwrap();
        let file = out.path().join("b.txt");
        std::fs::write(&file, "BBBB").unwrap();
        let mut staging = Staging::new(out.path());
        let copy = staging.stage(&file).unwrap();
        assert_eq!(std::fs::read_to_string(&copy).unwrap(), "BBBB");
        drop(staging);
        assert!(!copy.exists());
        assert_eq!(std::fs::read_dir(out.path()).unwrap().count(), 1);
    }

    #[test]
    fn test_a_failed_write_keeps_the_staged_copies() {
        let out = tempfile::tempdir().unwrap();
        let file = out.path().join("b.txt");
        std::fs::write(&file, "BBBB").unwrap();
        let mut staging = Staging::new(out.path());
        let copy = staging.stage(&file).unwrap();
        let kept = staging.keep().unwrap();
        assert!(copy.starts_with(&kept));
        assert_eq!(std::fs::read_to_string(&copy).unwrap(), "BBBB");
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
