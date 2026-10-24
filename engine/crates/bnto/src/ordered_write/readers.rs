// Which writes read the file another write's destination already holds.
//
// A result file that is a `Path` is read when its write runs, so a write
// whose destination is that file must wait. Files are matched by identity
// (`FileData::is_at`), so another spelling of the same file counts: another
// case on a case-insensitive filesystem, a hard link, a symlink.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use bnto_core::PipelineFileResult;
use bnto_core::processor::FileData;

/// Size and modified time: one file has one stamp, whatever path reaches
/// it, so only sources sharing a destination's stamp need the identity check.
pub(super) type Stamp = (u64, Option<SystemTime>);

/// For each write `i`, the other writes whose source is the file at
/// `paths[i]`: they must run before write `i` replaces it.
pub fn waits(files: &[PipelineFileResult], paths: &[PathBuf]) -> Vec<Vec<usize>> {
    let replaced = replaced_stamps(files, paths);
    // The usual run writes into an empty folder: no source stats needed.
    if replaced.iter().all(Option::is_none) {
        return vec![Vec::new(); paths.len()];
    }
    let sources = sources_by_stamp(files);
    let readers_of = |(i, at): (usize, Option<Stamp>)| {
        let candidates = at.and_then(|at| sources.get(&at)).into_iter().flatten();
        let reads_dest = |j: &&usize| **j != i && files[**j].data.is_at(&paths[i]);
        candidates.filter(reads_dest).copied().collect()
    };
    replaced.into_iter().enumerate().map(readers_of).collect()
}

/// For each write, the stamp of the file it would replace: `None` when its
/// destination is empty, or is its own source (left as is, ISS-36).
fn replaced_stamps(files: &[PipelineFileResult], paths: &[PathBuf]) -> Vec<Option<Stamp>> {
    let pairs = files.iter().zip(paths);
    pairs
        .map(|(file, dest)| {
            if file.data.is_at(dest) {
                None
            } else {
                stamp(dest)
            }
        })
        .collect()
}

/// The writes reading a file on disk, grouped by that file's stamp.
fn sources_by_stamp(files: &[PipelineFileResult]) -> HashMap<Stamp, Vec<usize>> {
    let mut by_stamp: HashMap<Stamp, Vec<usize>> = HashMap::new();
    for (j, file) in files.iter().enumerate() {
        if let FileData::Path(src) = &file.data
            && let Some(at) = stamp(src)
        {
            by_stamp.entry(at).or_default().push(j);
        }
    }
    by_stamp
}

/// The stamp of the file at `path`, following links; `None` when nothing is there.
pub(super) fn stamp(path: &Path) -> Option<Stamp> {
    let meta = std::fs::metadata(path).ok()?;
    Some((meta.len(), meta.modified().ok()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::processor::FileData;
    use std::path::Path;

    fn from(src: &Path) -> PipelineFileResult {
        PipelineFileResult {
            name: "unused".to_string(),
            data: FileData::Path(src.to_path_buf()),
            mime_type: "text/plain".to_string(),
            metadata: serde_json::Map::new(),
        }
    }

    fn none() -> Vec<usize> {
        Vec::new()
    }

    /// `a.txt` = `AAAA` and `b.txt` = `BBBB` in a fresh folder.
    fn two_inputs() -> (tempfile::TempDir, PathBuf, PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let (a, b) = (dir.path().join("a.txt"), dir.path().join("b.txt"));
        std::fs::write(&a, "AAAA").unwrap();
        std::fs::write(&b, "BBBB").unwrap();
        (dir, a, b)
    }

    #[test]
    fn test_a_destination_holding_a_later_source_waits_for_it() {
        // ISS-48: a → b.txt, b → b_1.txt.
        let (dir, a, b) = two_inputs();
        let paths = [dir.path().join("b.txt"), dir.path().join("b_1.txt")];
        assert_eq!(waits(&[from(&a), from(&b)], &paths), [vec![1], none()]);
    }

    #[test]
    fn test_a_swap_waits_both_ways() {
        let (_dir, a, b) = two_inputs();
        let paths = [b.clone(), a.clone()];
        assert_eq!(waits(&[from(&a), from(&b)], &paths), [vec![1], vec![0]]);
    }

    #[test]
    fn test_writes_into_another_folder_wait_for_nothing() {
        let (_dir, a, b) = two_inputs();
        let out = tempfile::tempdir().unwrap();
        let paths = [out.path().join("b.txt"), out.path().join("a.txt")];
        assert_eq!(waits(&[from(&a), from(&b)], &paths), [none(), none()]);
    }

    #[test]
    fn test_a_file_already_at_its_destination_waits_for_nothing() {
        // Its write leaves the file as is (ISS-36), so a fan-out reading it
        // doesn't have to go first.
        let (_dir, a, _) = two_inputs();
        let out = tempfile::tempdir().unwrap();
        let paths = [a.clone(), out.path().join("copy.txt")];
        assert_eq!(waits(&[from(&a), from(&a)], &paths), [none(), none()]);
    }

    #[test]
    fn test_a_file_in_the_output_folder_no_write_reads_waits_for_nothing() {
        let (dir, a, _) = two_inputs();
        let paths = [dir.path().join("b.txt")];
        assert_eq!(waits(&[from(&a)], &paths), [none()]);
    }

    #[test]
    fn test_every_write_reading_a_destination_is_listed() {
        let (dir, a, b) = two_inputs();
        let paths = [
            b.clone(),
            dir.path().join("x.txt"),
            dir.path().join("y.txt"),
        ];
        let files = [from(&a), from(&b), from(&b)];
        assert_eq!(waits(&files, &paths), [vec![1, 2], none(), none()]);
    }

    #[test]
    fn test_bytes_results_never_wait_to_be_read() {
        let (_dir, _, b) = two_inputs();
        let mut bytes = from(&b);
        bytes.data = FileData::Bytes(b"new".to_vec());
        let paths = [b.clone(), PathBuf::from("unused")];
        let files = [bytes.clone(), bytes];
        assert_eq!(waits(&files, &paths), [none(), none()]);
    }

    #[test]
    fn test_a_case_only_spelling_of_a_source_is_that_source_where_case_is_ignored() {
        // `B.txt` is `b.txt` on Windows and macOS (by default); on a
        // case-sensitive filesystem it's another file, and nothing waits.
        let (dir, a, b) = two_inputs();
        let upper = dir.path().join("B.txt");
        let case_insensitive = upper.exists();
        let paths = [upper, dir.path().join("b_1.txt")];
        let expected = if case_insensitive { vec![1] } else { none() };
        assert_eq!(waits(&[from(&a), from(&b)], &paths), [expected, none()]);
    }

    #[test]
    fn test_a_hard_link_to_a_source_is_that_source() {
        let (dir, a, b) = two_inputs();
        let link = dir.path().join("link.txt");
        std::fs::hard_link(&b, &link).unwrap();
        let paths = [link, dir.path().join("b_1.txt")];
        assert_eq!(waits(&[from(&a), from(&b)], &paths), [vec![1], none()]);
    }
}
