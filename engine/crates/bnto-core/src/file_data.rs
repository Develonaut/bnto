// FileData — in-memory vs on-disk file content.
//
// Small outputs (images, CSVs) use `Bytes`. Large outputs from shell-run
// file mode use `Path` to avoid reading multi-GB files into memory.

use std::path::{Path, PathBuf};

/// File content — either in-memory bytes or a path on disk.
///
/// Small outputs (images, CSVs) use `Bytes`. Files on disk (CLI argument
/// files, `file-collect` results, shell-run file-mode outputs) use `Path`
/// to avoid reading multi-GB files into memory. A `Path` may be the user's
/// own file: write it out with `copy_to()`; `write_to()` moves it.
#[derive(Debug, Clone, PartialEq)]
pub enum FileData {
    /// In-memory file content (images, CSVs, small outputs).
    Bytes(Vec<u8>),
    /// Reference to a file on disk (CLI inputs, collected files, shell outputs).
    /// The file is NOT read into memory until `into_bytes()` is called.
    Path(PathBuf),
}

impl FileData {
    /// Move or write file content to a destination path.
    ///
    /// `Path` variant attempts `rename()` first (O(1) on same filesystem),
    /// falling back to copy+delete for cross-device moves.
    /// `Bytes` variant writes data directly.
    pub fn write_to(&self, dest: &Path) -> Result<(), std::io::Error> {
        match self {
            FileData::Bytes(data) => std::fs::write(dest, data),
            FileData::Path(src) => {
                // Try rename first (O(1) on same filesystem).
                match std::fs::rename(src, dest) {
                    Ok(()) => Ok(()),
                    // Already there: copy + delete would empty, then remove it.
                    Err(_) if self.is_at(dest) => Ok(()),
                    Err(_) => {
                        // Cross-device: fall back to copy + delete.
                        std::fs::copy(src, dest)?;
                        let _ = std::fs::remove_file(src);
                        Ok(())
                    }
                }
            }
        }
    }

    /// Byte length without loading into memory.
    /// `Bytes` returns the vec length. `Path` reads file metadata.
    pub fn len(&self) -> Result<u64, std::io::Error> {
        match self {
            FileData::Bytes(data) => Ok(data.len() as u64),
            FileData::Path(path) => std::fs::metadata(path).map(|m| m.len()),
        }
    }

    /// Whether the file data is empty (zero bytes).
    pub fn is_empty(&self) -> Result<bool, std::io::Error> {
        self.len().map(|n| n == 0)
    }

    /// Copy file content to a destination path, preserving the source.
    ///
    /// `Bytes` writes data directly. `Path` copies the file on disk.
    /// Unlike `write_to()`, the source file is never removed. A `Path`
    /// already at `dest` (see `is_at`) is left as is: copying a file onto
    /// itself opens it for writing first, which empties it.
    pub fn copy_to(&self, dest: &Path) -> Result<(), std::io::Error> {
        match self {
            FileData::Bytes(data) => std::fs::write(dest, data),
            FileData::Path(_) if self.is_at(dest) => Ok(()),
            FileData::Path(src) => {
                std::fs::copy(src, dest)?;
                Ok(())
            }
        }
    }

    /// True when this is a `Path` whose file `dest` already is, by identity
    /// rather than by name: the same path, or one reaching the same file
    /// through `..`, a symlink, a hard link or another case on a
    /// case-insensitive filesystem. False for `Bytes` or a missing `dest`.
    pub fn is_at(&self, dest: &Path) -> bool {
        match self {
            FileData::Path(src) => same_file::is_same_file(src, dest).unwrap_or(false),
            FileData::Bytes(_) => false,
        }
    }

    /// Load file content into memory. Avoid for large files.
    /// `Bytes` is a no-op move. `Path` reads the file from disk.
    pub fn into_bytes(self) -> Result<Vec<u8>, std::io::Error> {
        match self {
            FileData::Bytes(data) => Ok(data),
            FileData::Path(path) => std::fs::read(path),
        }
    }

    /// Read file content without consuming it: borrowed for `Bytes`, read
    /// from disk for `Path`. For processors that hold files by reference
    /// (batch input) and need the bytes of some of them.
    pub fn to_bytes(&self) -> Result<std::borrow::Cow<'_, [u8]>, std::io::Error> {
        match self {
            FileData::Bytes(data) => Ok(std::borrow::Cow::Borrowed(data)),
            FileData::Path(path) => std::fs::read(path).map(std::borrow::Cow::Owned),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_bytes_borrows_bytes_and_reads_a_path() {
        let data = FileData::Bytes(b"in memory".to_vec());
        assert!(matches!(
            data.to_bytes().unwrap(),
            std::borrow::Cow::Borrowed(b"in memory")
        ));

        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("on-disk.txt");
        std::fs::write(&path, b"on disk").unwrap();
        let data = FileData::Path(path);
        assert_eq!(&*data.to_bytes().unwrap(), b"on disk");
        assert!(
            matches!(data, FileData::Path(_)),
            "to_bytes leaves the reference in place"
        );
    }

    #[test]
    fn test_to_bytes_names_a_missing_file_error() {
        let temp = tempfile::tempdir().unwrap();
        assert!(
            FileData::Path(temp.path().join("gone.txt"))
                .to_bytes()
                .is_err()
        );
    }

    #[test]
    fn test_filedata_bytes_write_to() {
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().to_path_buf();

        let data = FileData::Bytes(b"hello world".to_vec());
        let dest = dir.join("output.txt");
        data.write_to(&dest).unwrap();

        assert_eq!(std::fs::read(&dest).unwrap(), b"hello world");
    }

    #[test]
    fn test_filedata_path_write_to_renames() {
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().to_path_buf();

        let src = dir.join("source.bin");
        std::fs::write(&src, b"file content").unwrap();

        let data = FileData::Path(src.clone());
        let dest = dir.join("moved.bin");
        data.write_to(&dest).unwrap();

        assert_eq!(std::fs::read(&dest).unwrap(), b"file content");
        // Source should be gone (renamed).
        assert!(!src.exists());
    }

    #[test]
    fn test_filedata_bytes_len() {
        let data = FileData::Bytes(vec![0u8; 42]);
        assert_eq!(data.len().unwrap(), 42);
    }

    #[test]
    fn test_filedata_path_len() {
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().to_path_buf();

        let path = dir.join("sized.bin");
        std::fs::write(&path, vec![0u8; 1024]).unwrap();

        let data = FileData::Path(path);
        assert_eq!(data.len().unwrap(), 1024);
    }

    #[test]
    fn test_filedata_bytes_is_empty() {
        assert!(FileData::Bytes(vec![]).is_empty().unwrap());
        assert!(!FileData::Bytes(vec![1]).is_empty().unwrap());
    }

    #[test]
    fn test_filedata_bytes_copy_to() {
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().to_path_buf();

        let data = FileData::Bytes(b"copy me".to_vec());
        let dest = dir.join("copied.txt");
        data.copy_to(&dest).unwrap();

        assert_eq!(std::fs::read(&dest).unwrap(), b"copy me");
    }

    #[test]
    fn test_filedata_path_copy_to_preserves_source() {
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().to_path_buf();

        let src = dir.join("source.bin");
        std::fs::write(&src, b"original content").unwrap();

        let data = FileData::Path(src.clone());
        let dest = dir.join("copied.bin");
        data.copy_to(&dest).unwrap();

        assert_eq!(std::fs::read(&dest).unwrap(), b"original content");
        assert!(src.exists(), "Source should still exist after copy_to");
    }

    #[test]
    fn test_filedata_bytes_into_bytes() {
        let data = FileData::Bytes(b"content".to_vec());
        assert_eq!(data.into_bytes().unwrap(), b"content");
    }

    #[test]
    fn test_filedata_path_into_bytes() {
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().to_path_buf();

        let path = dir.join("read-me.bin");
        std::fs::write(&path, b"disk content").unwrap();

        let data = FileData::Path(path);
        assert_eq!(data.into_bytes().unwrap(), b"disk content");
    }

    // --- Never copy a file onto itself (ISS-36) ---

    const KEPT: &[u8] = b"important";

    /// `dir/a.txt` holding `KEPT`, the source every same-file test copies.
    fn source(dir: &Path) -> PathBuf {
        let src = dir.join("a.txt");
        std::fs::write(&src, KEPT).unwrap();
        src
    }

    /// Copy `src` to `dest`, which names `src` itself, and check it survived.
    fn assert_copy_leaves(src: &Path, dest: &Path) {
        let data = FileData::Path(src.to_path_buf());
        data.copy_to(dest).unwrap();
        assert_eq!(std::fs::read(src).unwrap(), KEPT, "copy onto itself");
        assert!(data.is_at(dest), "{} is {}", dest.display(), src.display());
    }

    #[test]
    fn test_copy_to_the_same_path_leaves_the_file_as_is() {
        let temp = tempfile::tempdir().unwrap();
        let src = source(temp.path());
        assert_copy_leaves(&src, &src);
    }

    #[test]
    fn test_copy_to_the_same_file_through_dot_dot_leaves_it_as_is() {
        let temp = tempfile::tempdir().unwrap();
        let src = source(temp.path());
        std::fs::create_dir(temp.path().join("sub")).unwrap();
        assert_copy_leaves(&src, &temp.path().join("sub").join("..").join("a.txt"));
    }

    #[test]
    fn test_copy_to_a_hard_link_of_the_source_leaves_it_as_is() {
        let temp = tempfile::tempdir().unwrap();
        let src = source(temp.path());
        let link = temp.path().join("link.txt");
        std::fs::hard_link(&src, &link).unwrap();
        assert_copy_leaves(&src, &link);
    }

    #[test]
    fn test_copy_to_the_same_file_in_another_case_leaves_it_as_is() {
        let temp = tempfile::tempdir().unwrap();
        let src = source(temp.path());
        let other_case = temp.path().join("A.TXT");
        if !other_case.exists() {
            return; // case-sensitive filesystem: `A.TXT` is another file.
        }
        assert_copy_leaves(&src, &other_case);
    }

    #[cfg(unix)]
    #[test]
    fn test_copy_to_the_same_file_through_a_symlinked_folder_leaves_it_as_is() {
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().join("real");
        std::fs::create_dir(&dir).unwrap();
        let src = source(&dir);
        let linked = temp.path().join("linked");
        std::os::unix::fs::symlink(&dir, &linked).unwrap();
        assert_copy_leaves(&src, &linked.join("a.txt"));
    }

    #[cfg(unix)]
    #[test]
    fn test_copy_to_a_symlink_pointing_at_the_source_leaves_it_as_is() {
        let temp = tempfile::tempdir().unwrap();
        let src = source(temp.path());
        let link = temp.path().join("link.txt");
        std::os::unix::fs::symlink(&src, &link).unwrap();
        assert_copy_leaves(&src, &link);
    }

    #[test]
    fn test_is_at_only_for_the_source_file_itself() {
        let temp = tempfile::tempdir().unwrap();
        let src = source(temp.path());
        let twin = temp.path().join("twin.txt");
        std::fs::write(&twin, KEPT).unwrap();
        let data = FileData::Path(src.clone());
        assert!(!data.is_at(&twin), "same bytes, different file");
        assert!(!data.is_at(&temp.path().join("missing.txt")));
        assert!(!FileData::Bytes(KEPT.to_vec()).is_at(&src));
    }

    #[test]
    fn test_write_to_the_same_path_keeps_the_file() {
        // Guard: a move onto itself is a no-op, never copy-then-delete.
        let temp = tempfile::tempdir().unwrap();
        let src = source(temp.path());
        FileData::Path(src.clone()).write_to(&src).unwrap();
        assert_eq!(std::fs::read(&src).unwrap(), KEPT);
    }
}
