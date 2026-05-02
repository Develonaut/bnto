// FileData — in-memory vs on-disk file content.
//
// Small outputs (images, CSVs) use `Bytes`. Large outputs from shell-command
// file mode use `Path` to avoid reading multi-GB files into memory.

use std::path::{Path, PathBuf};

/// File content — either in-memory bytes or a path on disk.
///
/// Small outputs (images, CSVs, renamed files) use `Bytes`. Large outputs
/// from shell-command file mode use `Path` to avoid reading multi-GB files
/// into memory. Consumers call `write_to()` which uses `rename()` (O(1))
/// for the `Path` variant with a copy fallback for cross-device moves.
#[derive(Debug, Clone, PartialEq)]
pub enum FileData {
    /// In-memory file content (images, CSVs, small outputs).
    Bytes(Vec<u8>),
    /// Reference to a file on disk (shell-command file-mode outputs).
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
    /// Unlike `write_to()`, the source file is never removed.
    pub fn copy_to(&self, dest: &Path) -> Result<(), std::io::Error> {
        match self {
            FileData::Bytes(data) => std::fs::write(dest, data),
            FileData::Path(src) => {
                std::fs::copy(src, dest)?;
                Ok(())
            }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filedata_bytes_write_to() {
        let dir = std::env::temp_dir().join("bnto-test-filedata-bytes");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let data = FileData::Bytes(b"hello world".to_vec());
        let dest = dir.join("output.txt");
        data.write_to(&dest).unwrap();

        assert_eq!(std::fs::read(&dest).unwrap(), b"hello world");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_filedata_path_write_to_renames() {
        let dir = std::env::temp_dir().join("bnto-test-filedata-path");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let src = dir.join("source.bin");
        std::fs::write(&src, b"file content").unwrap();

        let data = FileData::Path(src.clone());
        let dest = dir.join("moved.bin");
        data.write_to(&dest).unwrap();

        assert_eq!(std::fs::read(&dest).unwrap(), b"file content");
        // Source should be gone (renamed).
        assert!(!src.exists());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_filedata_bytes_len() {
        let data = FileData::Bytes(vec![0u8; 42]);
        assert_eq!(data.len().unwrap(), 42);
    }

    #[test]
    fn test_filedata_path_len() {
        let dir = std::env::temp_dir().join("bnto-test-filedata-len");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let path = dir.join("sized.bin");
        std::fs::write(&path, vec![0u8; 1024]).unwrap();

        let data = FileData::Path(path);
        assert_eq!(data.len().unwrap(), 1024);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_filedata_bytes_is_empty() {
        assert!(FileData::Bytes(vec![]).is_empty().unwrap());
        assert!(!FileData::Bytes(vec![1]).is_empty().unwrap());
    }

    #[test]
    fn test_filedata_bytes_copy_to() {
        let dir = std::env::temp_dir().join("bnto-test-filedata-bytes-copy");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let data = FileData::Bytes(b"copy me".to_vec());
        let dest = dir.join("copied.txt");
        data.copy_to(&dest).unwrap();

        assert_eq!(std::fs::read(&dest).unwrap(), b"copy me");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_filedata_path_copy_to_preserves_source() {
        let dir = std::env::temp_dir().join("bnto-test-filedata-path-copy");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let src = dir.join("source.bin");
        std::fs::write(&src, b"original content").unwrap();

        let data = FileData::Path(src.clone());
        let dest = dir.join("copied.bin");
        data.copy_to(&dest).unwrap();

        assert_eq!(std::fs::read(&dest).unwrap(), b"original content");
        assert!(src.exists(), "Source should still exist after copy_to");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_filedata_bytes_into_bytes() {
        let data = FileData::Bytes(b"content".to_vec());
        assert_eq!(data.into_bytes().unwrap(), b"content");
    }

    #[test]
    fn test_filedata_path_into_bytes() {
        let dir = std::env::temp_dir().join("bnto-test-filedata-into");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let path = dir.join("read-me.bin");
        std::fs::write(&path, b"disk content").unwrap();

        let data = FileData::Path(path);
        assert_eq!(data.into_bytes().unwrap(), b"disk content");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
