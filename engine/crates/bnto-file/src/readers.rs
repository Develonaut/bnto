// Which files of a batch are read from a file another write would replace.
//
// A file that is a `Path` is read when its step runs, and by every step after,
// so a write that replaces that file first loses its bytes. Files are matched
// by identity (`FileData::is_at`), so another spelling of the same file
// counts: another case on a case-insensitive filesystem, a hard link, a
// symlink. Shared by file-copy and the CLI's output write.

use std::collections::HashMap;
use std::path::Path;

use bnto_core::file_stamp::{Stamp, stamp};
use bnto_core::processor::FileData;

/// For each write `i`, the other files whose source is the file at
/// `replaced[i]`, the file write `i` would replace. `None` (the write replaces
/// nothing) or a missing file has no readers.
pub fn readers_of(sources: &[&FileData], replaced: &[Option<&Path>]) -> Vec<Vec<usize>> {
    let stamps: Vec<Option<Stamp>> = replaced.iter().map(|p| p.and_then(stamp)).collect();
    // The usual run writes into an empty folder: no source stats needed.
    if stamps.iter().all(Option::is_none) {
        return vec![Vec::new(); replaced.len()];
    }
    let by_stamp = sources_by_stamp(sources);
    let readers = |(i, at): (usize, Option<Stamp>)| {
        let candidates = at.and_then(|at| by_stamp.get(&at)).into_iter().flatten();
        let dest = replaced[i];
        let reads_dest = |j: &&usize| **j != i && dest.is_some_and(|d| sources[**j].is_at(d));
        candidates.filter(reads_dest).copied().collect()
    };
    stamps.into_iter().enumerate().map(readers).collect()
}

/// The files read from disk, grouped by that file's stamp.
fn sources_by_stamp(sources: &[&FileData]) -> HashMap<Stamp, Vec<usize>> {
    let mut by_stamp: HashMap<Stamp, Vec<usize>> = HashMap::new();
    for (j, data) in sources.iter().enumerate() {
        if let FileData::Path(src) = data
            && let Some(at) = stamp(src)
        {
            by_stamp.entry(at).or_default().push(j);
        }
    }
    by_stamp
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// `a.txt` = `AAAA` and `b.txt` = `BBBB` in a fresh folder.
    fn two_inputs() -> (tempfile::TempDir, PathBuf, PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let (a, b) = (dir.path().join("a.txt"), dir.path().join("b.txt"));
        std::fs::write(&a, "AAAA").unwrap();
        std::fs::write(&b, "BBBB").unwrap();
        (dir, a, b)
    }

    fn path(p: &Path) -> FileData {
        FileData::Path(p.to_path_buf())
    }

    #[test]
    fn test_a_replaced_file_another_file_reads_lists_that_file() {
        let (_dir, a, b) = two_inputs();
        let (fa, fb) = (path(&a), path(&b));
        let readers = readers_of(&[&fa, &fb], &[Some(&b), None]);
        assert_eq!(readers, [vec![1], vec![]]);
    }

    #[test]
    fn test_a_write_replacing_nothing_has_no_readers() {
        let (dir, a, b) = two_inputs();
        let (fa, fb) = (path(&a), path(&b));
        let missing = dir.path().join("new.txt");
        let readers = readers_of(&[&fa, &fb], &[Some(&missing), None]);
        assert_eq!(readers, [Vec::<usize>::new(), vec![]]);
    }

    #[test]
    fn test_a_write_onto_its_own_source_is_not_its_own_reader() {
        let (_dir, a, _) = two_inputs();
        let fa = path(&a);
        assert_eq!(readers_of(&[&fa], &[Some(&a)]), [Vec::<usize>::new()]);
    }

    #[test]
    fn test_bytes_are_never_read_from_disk() {
        let (_dir, _, b) = two_inputs();
        let bytes = FileData::Bytes(b"BBBB".to_vec());
        let fb = path(&b);
        let readers = readers_of(&[&fb, &bytes], &[None, Some(&b)]);
        assert_eq!(readers, [vec![], vec![0]]);
    }

    #[test]
    fn test_a_hard_link_to_a_source_is_that_source() {
        let (dir, a, b) = two_inputs();
        let link = dir.path().join("link.txt");
        std::fs::hard_link(&b, &link).unwrap();
        let (fa, fb) = (path(&a), path(&b));
        let readers = readers_of(&[&fa, &fb], &[Some(&link), None]);
        assert_eq!(readers, [vec![1], vec![]]);
    }
}
