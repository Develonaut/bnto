// Destinations with different names that are one file on disk.
//
// `output_names` keeps names apart, but links can still make two
// destinations one file: an existing file through a symlink or hard link
// (matched by identity), or a new one through a folder symlink or a
// dangling link (matched by where it lands, see `landing`). Ordering can't
// save both writes then: the later copy replaces the earlier one, and a
// write that leaves its file as is (ISS-36) would be reported as kept while
// the other replaced it. Such a plan is refused, as `relocate` refuses two
// moves onto one path. Two writes that both leave the file as is are fine.

use std::path::{Path, PathBuf};

use bnto_core::processor::FileData;

use super::landing::landing_key;
use super::readers::stamp;

/// `shared_kept` with only files already at their destination left as is.
#[cfg(test)]
pub fn shared_destinations(
    files: &[bnto_core::PipelineFileResult],
    paths: &[PathBuf],
) -> Vec<(usize, usize)> {
    shared_kept(paths, &super::left_as_is(files, paths, &[]))
}

/// Each pair of writes `(i, j)`, `i < j`, whose destinations are one file
/// though their names differ, unless both leave that file as is (`kept`: its
/// own source, or a held file already holding the result's bytes). One that
/// writes would replace the other's file, kept or not.
pub fn shared_kept(paths: &[PathBuf], kept: &[bool]) -> Vec<(usize, usize)> {
    let identical = same_stamp_pairs(paths)
        .into_iter()
        .filter(|&(i, j)| same_file(&paths[i], &paths[j]));
    let mut pairs: Vec<_> = identical.chain(same_landing_pairs(paths)).collect();
    pairs.sort_unstable();
    pairs.dedup();
    pairs.retain(|&(i, j)| !(kept[i] && kept[j]));
    pairs
}

/// Pairs of existing destinations sharing a stamp: the only ones that can be
/// one file. Most runs write into an empty folder and get none.
fn same_stamp_pairs(paths: &[PathBuf]) -> Vec<(usize, usize)> {
    let stamped = paths
        .iter()
        .enumerate()
        .filter_map(|(i, p)| stamp(p).map(|s| (s, i)));
    pairs_by_key(stamped.collect())
}

/// Pairs of destinations that land on one path, new files included.
fn same_landing_pairs(paths: &[PathBuf]) -> Vec<(usize, usize)> {
    let keyed = paths.iter().enumerate().map(|(i, p)| (landing_key(p), i));
    pairs_by_key(keyed.collect())
}

/// Every pair of indexes `(i, j)`, `i < j`, that share a key.
fn pairs_by_key<K: Ord>(mut keyed: Vec<(K, usize)>) -> Vec<(usize, usize)> {
    keyed.sort();
    let groups = keyed.chunk_by(|x, y| x.0 == y.0);
    let pairs_in = |g: &[(K, usize)]| {
        let ids: Vec<usize> = g.iter().map(|(_, i)| *i).collect();
        let pairs = (0..ids.len()).flat_map(|a| (a + 1..ids.len()).map(move |b| (a, b)));
        pairs.map(|(a, b)| (ids[a], ids[b])).collect::<Vec<_>>()
    };
    groups.flat_map(pairs_in).collect()
}

fn same_file(a: &Path, b: &Path) -> bool {
    FileData::Path(a.to_path_buf()).is_at(b)
}

/// One line per pair, naming both destinations (apart, see `shown`) and how
/// they meet.
pub fn describe(paths: &[PathBuf], pairs: &[(usize, usize)]) -> Vec<String> {
    let line = |&(i, j): &(usize, usize)| {
        let (a, b) = (&paths[i], &paths[j]);
        format!(
            "{} and {} would be one file on disk ({}), so one write would replace the other",
            shown(a),
            shown(b),
            how_they_meet(a, b)
        )
    };
    pairs.iter().map(line).collect()
}

/// Why two destinations are one file: a new path both lead to, a symlink,
/// two directory entries for one file (hard links), or one entry the
/// filesystem reaches by both names (Unicode forms, Windows short names).
fn how_they_meet(a: &Path, b: &Path) -> &'static str {
    if !same_file(a, b) {
        return "both lead to one new path, through a folder link, a dangling link or `..`";
    }
    if is_symlink(a) || is_symlink(b) {
        return "one is a symlink to the other's file";
    }
    match (canonical_parent(a), canonical_parent(b)) {
        (Some(x), Some(y)) if x == y && listed(&x, a) && listed(&x, b) => {
            "they are hard links to one file"
        }
        (Some(x), Some(y)) if x == y => "the filesystem treats the two names as one file",
        _ => "both reach one existing file, through a folder link or a hard link",
    }
}

fn is_symlink(path: &Path) -> bool {
    std::fs::symlink_metadata(path).is_ok_and(|m| m.file_type().is_symlink())
}

fn canonical_parent(path: &Path) -> Option<PathBuf> {
    dunce::canonicalize(path.parent()?).ok()
}

/// Whether `dir` has an entry spelled exactly as `path`'s file name.
fn listed(dir: &Path, path: &Path) -> bool {
    let Some(name) = path.file_name() else {
        return false;
    };
    let entries = std::fs::read_dir(dir).into_iter().flatten().flatten();
    entries.into_iter().any(|e| e.file_name() == name)
}

/// `path` for a refusal: with its file name's non-ASCII characters escaped
/// after it when it has any, since two Unicode forms of one name (`café`
/// composed and decomposed) print the same.
pub fn shown(path: &Path) -> String {
    let leaf = path.file_name().map(|n| n.to_string_lossy().into_owned());
    match leaf.filter(|l| !l.is_ascii()) {
        Some(leaf) => format!("{} (\"{}\")", path.display(), escaped(&leaf)),
        None => path.display().to_string(),
    }
}

fn escaped(name: &str) -> String {
    let char_shown = |c: char| match c.is_ascii() {
        true => c.to_string(),
        false => c.escape_unicode().to_string(),
    };
    name.chars().map(char_shown).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::PipelineFileResult;

    fn from(src: &Path) -> PipelineFileResult {
        PipelineFileResult {
            name: "unused".to_string(),
            data: FileData::Path(src.to_path_buf()),
            mime_type: "text/plain".to_string(),
            metadata: serde_json::Map::new(),
        }
    }

    /// `a.txt` = `AAAA`, `b.txt` = `BBBB` and `l.txt`, a hard link to `b.txt`.
    fn linked() -> (tempfile::TempDir, PathBuf, PathBuf, PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let (a, b) = (dir.path().join("a.txt"), dir.path().join("b.txt"));
        std::fs::write(&a, "AAAA").unwrap();
        std::fs::write(&b, "BBBB").unwrap();
        let l = dir.path().join("l.txt");
        std::fs::hard_link(&b, &l).unwrap();
        (dir, a, b, l)
    }

    #[test]
    fn test_a_write_through_a_link_onto_a_file_left_as_is_is_a_conflict() {
        // a → l.txt (a link to b.txt) while b → b.txt leaves b.txt as is.
        let (_dir, a, b, l) = linked();
        let paths = [l, b.clone()];
        assert_eq!(shared_destinations(&[from(&a), from(&b)], &paths), [(0, 1)]);
    }

    #[test]
    fn test_two_writes_through_links_to_one_file_are_a_conflict() {
        // Neither destination is a source: the second copy replaces the first.
        let (dir, a, b, l) = linked();
        let other = dir.path().join("other.txt");
        std::fs::write(&other, "OTHER").unwrap();
        let paths = [b, l];
        let files = [from(&a), from(&other)];
        assert_eq!(shared_destinations(&files, &paths), [(0, 1)]);
    }

    #[test]
    fn test_two_writes_leaving_one_file_as_is_are_no_conflict() {
        let (_dir, _a, b, l) = linked();
        let paths = [b.clone(), l.clone()];
        assert!(shared_destinations(&[from(&b), from(&l)], &paths).is_empty());
    }

    #[test]
    fn test_separate_files_with_one_stamp_are_no_conflict() {
        let dir = tempfile::tempdir().unwrap();
        let (x, y) = (dir.path().join("x.txt"), dir.path().join("y.txt"));
        std::fs::write(&x, "same").unwrap();
        std::fs::copy(&x, &y).unwrap();
        let mtime = std::fs::metadata(&x).unwrap().modified().unwrap();
        std::fs::File::options()
            .write(true)
            .open(&y)
            .unwrap()
            .set_modified(mtime)
            .unwrap();
        let paths = [x.clone(), y.clone()];
        assert!(shared_destinations(&[from(&y), from(&x)], &paths).is_empty());
    }

    #[test]
    fn test_missing_destinations_are_no_conflict() {
        let dir = tempfile::tempdir().unwrap();
        let paths = [dir.path().join("x.txt"), dir.path().join("y.txt")];
        let file = from(&dir.path().join("gone.txt"));
        assert!(shared_destinations(&[file.clone(), file], &paths).is_empty());
    }

    #[cfg(unix)]
    #[test]
    fn test_a_write_through_a_symlink_onto_a_file_left_as_is_is_a_conflict() {
        let (dir, a, b, _) = linked();
        let s = dir.path().join("s.txt");
        std::os::unix::fs::symlink("b.txt", &s).unwrap();
        let paths = [s, b.clone()];
        assert_eq!(shared_destinations(&[from(&a), from(&b)], &paths), [(0, 1)]);
    }

    #[cfg(unix)]
    #[test]
    fn test_two_new_files_through_a_folder_link_are_a_conflict() {
        // ISS-48 review 2: neither exists, so only where they land tells.
        let (_dir, a, b, _) = linked();
        let out = tempfile::tempdir().unwrap();
        std::os::unix::fs::symlink(".", out.path().join("sub")).unwrap();
        let paths = [
            out.path().join("sub").join("b.txt"),
            out.path().join("b.txt"),
        ];
        assert_eq!(shared_destinations(&[from(&a), from(&b)], &paths), [(0, 1)]);
    }

    #[test]
    fn test_describe_names_both_destinations() {
        let paths = [PathBuf::from("out/l.txt"), PathBuf::from("out/b.txt")];
        let lines = describe(&paths, &[(0, 1)]);
        assert_eq!(lines.len(), 1);
        assert!(
            lines[0].contains("l.txt") && lines[0].contains("b.txt"),
            "{lines:?}"
        );
        assert!(lines[0].contains("one file on disk"), "{lines:?}");
    }

    #[test]
    fn test_hard_links_are_called_hard_links() {
        let (_dir, _a, b, l) = linked();
        assert_eq!(how_they_meet(&b, &l), "they are hard links to one file");
    }

    #[cfg(unix)]
    #[test]
    fn test_a_symlink_is_called_a_symlink() {
        let (dir, _a, b, _) = linked();
        let s = dir.path().join("s.txt");
        std::os::unix::fs::symlink("b.txt", &s).unwrap();
        assert_eq!(
            how_they_meet(&s, &b),
            "one is a symlink to the other's file"
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_two_unicode_forms_of_one_name_say_so_and_print_apart() {
        // RED (ISS-48 review 4): an existing NFC `café` and an NFD output
        // were said to meet "through a link", and both names printed alike.
        let dir = tempfile::tempdir().unwrap();
        let nfc = dir.path().join("caf\u{e9}");
        std::fs::write(&nfc, "ONE").unwrap();
        let nfd = dir.path().join("cafe\u{301}");
        let line = &describe(&[nfc, nfd], &[(0, 1)])[0];
        assert!(
            line.contains("the filesystem treats the two names as one file"),
            "{line}"
        );
        assert!(
            line.contains("caf\\u{e9}") && line.contains("cafe\\u{301}"),
            "{line}"
        );
    }

    #[test]
    fn test_an_ascii_name_is_shown_as_is() {
        assert_eq!(shown(Path::new("out/b.txt")), "out/b.txt");
        assert_eq!(
            shown(Path::new("out/caf\u{e9}")),
            "out/caf\u{e9} (\"caf\\u{e9}\")"
        );
    }
}
