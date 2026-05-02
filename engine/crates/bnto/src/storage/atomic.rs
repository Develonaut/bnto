// Atomic file writes — write to temp file, then rename.
//
// Prevents corruption from partial writes (crash, power loss, etc.).
// Rename is atomic on POSIX and NTFS filesystems.

use std::io::Write;
use std::path::Path;

/// Write data to a file atomically.
///
/// Creates a temp file in the same directory as the target, writes all
/// data, flushes, then renames over the target. The rename is atomic
/// on all POSIX filesystems and NTFS — a crash mid-write never
/// corrupts the target file.
pub fn atomic_write(path: &Path, data: &[u8]) -> Result<(), std::io::Error> {
    let dir = path.parent().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "path has no parent")
    })?;
    std::fs::create_dir_all(dir)?;

    let mut tmp = tempfile::NamedTempFile::new_in(dir)?;
    tmp.write_all(data)?;
    tmp.flush()?;
    tmp.persist(path).map_err(|e| e.error)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atomic_write_creates_file() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("test.txt");

        atomic_write(&path, b"hello world").unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "hello world");
    }

    #[test]
    fn atomic_write_creates_parent_dirs() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("deep").join("nested").join("file.txt");

        atomic_write(&path, b"nested data").unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "nested data");
    }

    #[test]
    fn atomic_write_overwrites_existing() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("overwrite.txt");

        atomic_write(&path, b"original").unwrap();
        atomic_write(&path, b"replaced").unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "replaced");
    }
}
