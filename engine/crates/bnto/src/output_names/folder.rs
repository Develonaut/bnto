// What a suffixed name bnto made up would land on in the output folder.
//
// Free when no entry there folds to it. Held when the one that does has
// exactly that name and already holds exactly the bytes the result has: this
// run's own progressive copy, or an earlier run's identical result. A held
// file is the result already, so it's left as is, and taking it keeps a run's
// names the same whether its files went out progressively, in the final
// write, or in a rerun. Any other entry (a user's file, a link, a folder)
// takes the name. The listing folds names as the run's own names are folded;
// the filesystem is then asked too, since it may hold one file under a name
// the listing spells differently. Nothing is cached here, so a file a step
// wrote there mid-run counts too.

use std::fs::File;
use std::io::Read;
use std::path::Path;

use bnto_core::processor::FileData;

use super::fold;

/// Bytes compared per read when checking a file already there.
const CHUNK: usize = 64 * 1024;

/// What a name would land on in the output folder.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Spot {
    /// Nothing there: the result is written.
    Free,
    /// A regular file by exactly that name already holds the result's
    /// bytes: it's left as is.
    Held,
    /// Something else is there: the name moves on.
    Taken,
}

/// What `data` written as `name` in `dir` would land on. `name` may hold
/// subfolders; the folder it lands in is the one read.
pub fn spot(dir: &Path, name: &str, data: &FileData) -> Spot {
    let path = dir.join(name);
    let (Some(folder), Some(leaf)) = (path.parent(), path.file_name()) else {
        return Spot::Taken;
    };
    let leaf = leaf.to_string_lossy();
    match folded_matches(folder, &leaf).as_slice() {
        [] => unlisted(&path, data),
        [only] if *only == *leaf => held_or_taken(&path, data),
        _ => Spot::Taken,
    }
}

/// The entries of `folder` whose names fold to `leaf`'s. A folder that can't
/// be listed (not made yet, or no read permission) gives none, and
/// `unlisted` decides.
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

/// With no entry listed under `path`'s name, ask the filesystem. It can know
/// a file the listing doesn't show under that spelling (APFS takes an NFC
/// and an NFD `é` as one name) or in a folder it won't list (write but no
/// read permission); what it finds is held or taken like a listed file. A
/// lookup that fails, `NotFound` or any other way (no search permission on
/// the folder), is free: creating the file then fails the same way, so
/// nothing is replaced and the write reports the error. Calling those taken
/// would take every number, and the claim would never end.
fn unlisted(path: &Path, data: &FileData) -> Spot {
    match std::fs::symlink_metadata(path) {
        Err(_) => Spot::Free,
        Ok(_) => held_or_taken(path, data),
    }
}

fn held_or_taken(path: &Path, data: &FileData) -> Spot {
    if holds(path, data) {
        Spot::Held
    } else {
        Spot::Taken
    }
}

/// Whether `path` is a regular file (not a link to one) holding exactly
/// `data`'s bytes.
fn holds(path: &Path, data: &FileData) -> bool {
    let Ok(meta) = std::fs::symlink_metadata(path) else {
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
