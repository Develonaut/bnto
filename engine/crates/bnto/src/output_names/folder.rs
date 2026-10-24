// Whether a suffixed name bnto made up may land in the output folder.
//
// Free when no entry there folds to it, or when the one that does has exactly
// that name and already holds exactly the bytes about to be written: this
// run's own progressive copy, or an earlier run's identical result. Writing
// those again loses nothing, and taking them keeps a run's names the same
// whether its files went out progressively, in the final write, or a rerun.
// Any other entry, a user's file or a folder, sends the name to the next
// number. The folder is listed for every check, never cached, so a file a
// step wrote there mid-run counts too.

use std::fs::File;
use std::io::Read;
use std::path::Path;

use bnto_core::processor::FileData;

use super::fold;

/// Bytes compared per read when checking a file already there.
const CHUNK: usize = 64 * 1024;

/// Whether writing `data` as `name` in `dir` replaces nothing but a copy of
/// itself. `name` may hold subfolders; the folder it lands in is the one read.
pub fn may_take(dir: &Path, name: &str, data: &FileData) -> bool {
    let path = dir.join(name);
    let (Some(folder), Some(leaf)) = (path.parent(), path.file_name()) else {
        return false;
    };
    let leaf = leaf.to_string_lossy();
    match folded_matches(folder, &leaf).as_slice() {
        [] => true,
        [only] if *only == *leaf => holds(&path, data),
        _ => false,
    }
}

/// The entries of `folder` whose names fold to `leaf`'s. A folder that can't
/// be read (not made yet) has none.
fn folded_matches(folder: &Path, leaf: &str) -> Vec<String> {
    let key = fold(leaf);
    let Ok(entries) = std::fs::read_dir(folder) else {
        return Vec::new();
    };
    entries
        .filter_map(|entry| Some(entry.ok()?.file_name().to_string_lossy().into_owned()))
        .filter(|name| fold(name) == key)
        .collect()
}

/// Whether the regular file at `path` holds exactly `data`'s bytes.
fn holds(path: &Path, data: &FileData) -> bool {
    let Ok(meta) = std::fs::metadata(path) else {
        return false;
    };
    if !meta.is_file() || data.len().ok() != Some(meta.len()) {
        return false;
    }
    if data.is_at(path) {
        return true;
    }
    let source: Box<dyn Read> = match data {
        FileData::Bytes(bytes) => Box::new(bytes.as_slice()),
        FileData::Path(src) => match File::open(src) {
            Ok(file) => Box::new(file),
            Err(_) => return false,
        },
    };
    File::open(path).is_ok_and(|on_disk| same_stream(source, on_disk, meta.len()))
}

/// Whether two readers of `len` bytes each yield the same bytes, read a
/// chunk at a time so a large file is never loaded whole.
fn same_stream(mut a: impl Read, mut b: impl Read, len: u64) -> bool {
    let (mut x, mut y) = (vec![0; CHUNK], vec![0; CHUNK]);
    let mut left = len;
    while left > 0 {
        let n = usize::try_from(left).map_or(CHUNK, |l| l.min(CHUNK));
        let read = a.read_exact(&mut x[..n]).and(b.read_exact(&mut y[..n]));
        if read.is_err() || x[..n] != y[..n] {
            return false;
        }
        left -= n as u64;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_streams_longer_than_a_chunk_compare_every_chunk() {
        // The difference sits in the second chunk, past the first read.
        let a = vec![7u8; CHUNK + 10];
        let mut b = a.clone();
        b[CHUNK + 5] = 8;
        let len = a.len() as u64;
        assert!(same_stream(a.as_slice(), a.as_slice(), len));
        assert!(!same_stream(a.as_slice(), b.as_slice(), len));
    }

    #[test]
    fn test_a_file_shorter_than_its_reported_length_is_not_the_same() {
        assert!(!same_stream(&b"abc"[..], &b"ab"[..], 3));
    }
}
