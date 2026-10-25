// Which writes read the file another write's destination already holds.
//
// A result file that is a `Path` is read when its write runs, so a write
// whose destination is that file must wait. The lookup is shared with
// file-copy (`bnto_file::readers`); this adds which writes replace nothing.

use std::path::{Path, PathBuf};

use bnto_core::PipelineFileResult;
use bnto_core::processor::FileData;
use bnto_file::readers::readers_of;

/// `waits_kept` with only files already at their destination left as is.
#[cfg(test)]
pub fn waits(files: &[PipelineFileResult], paths: &[PathBuf]) -> Vec<Vec<usize>> {
    waits_kept(files, paths, &super::left_as_is(files, paths, &[]))
}

/// For each write `i`, the other writes whose source is the file at
/// `paths[i]`: they must run before write `i` replaces it. A write with
/// `kept[i]` replaces nothing (left as is, or held) and waits for nothing.
pub fn waits_kept(
    files: &[PipelineFileResult],
    paths: &[PathBuf],
    kept: &[bool],
) -> Vec<Vec<usize>> {
    let sources: Vec<&FileData> = files.iter().map(|f| &f.data).collect();
    let replaced: Vec<Option<&Path>> = paths
        .iter()
        .zip(kept)
        .map(|(dest, &kept)| (!kept).then_some(dest.as_path()))
        .collect();
    readers_of(&sources, &replaced)
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
