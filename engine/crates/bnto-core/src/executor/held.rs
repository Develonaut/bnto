// The files a run still holds outside the step that's running, for a batch
// node that would replace, move or delete one (`ProcessContext::held_elsewhere`,
// ISS-59, ISS-82).
//
// A loop holds its other iterations' files: later ones, and finished ones
// whose results still read from their sources. A graph level holds the files
// it keeps for steps that run later (fan-out). Frames nest, so a loop inside
// a branch sees both. Only disk references are held: in-memory files read
// nothing from disk.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};

use crate::file_stamp::{Stamp, stamp};
use crate::pipeline::PipelineFile;
use crate::processor::FileData;

use super::PipelineContext;

/// One level's held files, inside the frames of the levels around it.
pub(super) struct HeldFiles {
    files: Mutex<Vec<Option<Held>>>,
    outer: Option<Arc<HeldFiles>>,
}

/// A file on disk a later step reads, and the name it goes by in the run.
struct Held {
    path: PathBuf,
    name: String,
    /// Read on the first lookup that needs it, then kept: a lookup runs once
    /// per copy that would replace a file and per move or delete source, so
    /// each file is stat'd once however many lookups compare against it.
    stamp: OnceLock<Option<Stamp>>,
}

impl HeldFiles {
    /// An empty frame inside `outer`.
    pub(super) fn within(outer: Option<&Arc<HeldFiles>>) -> Arc<Self> {
        Arc::new(Self {
            files: Mutex::default(),
            outer: outer.cloned(),
        })
    }

    /// A frame inside `outer` holding `files`: file `i` is slot `i`, for `release`.
    pub(super) fn holding(outer: Option<&Arc<HeldFiles>>, files: &[PipelineFile]) -> Arc<Self> {
        let held = Self::within(outer);
        held.hold(files);
        held
    }

    /// Hold `files`, after what's held.
    pub(super) fn hold(&self, files: &[PipelineFile]) {
        self.lock().extend(files.iter().map(Held::of));
    }

    /// Stop holding the file in slot `index` (a loop hands it to its iteration).
    pub(super) fn release(&self, index: usize) {
        if let Some(slot) = self.lock().get_mut(index) {
            *slot = None;
        }
    }

    /// Hold exactly `files`, in place of what was held.
    pub(super) fn replace<'a>(&self, files: impl Iterator<Item = &'a PipelineFile>) {
        *self.lock() = files.map(Held::of).collect();
    }

    /// The name of a held file that is the file at `dest`, in this frame or
    /// an outer one. `None` when nothing is at `dest`.
    pub(super) fn reader_of(&self, dest: &Path) -> Option<String> {
        let at = stamp(dest)?;
        let mut frame = Some(self);
        while let Some(held) = frame {
            if let Some(name) = held.find(dest, at) {
                return Some(name);
            }
            frame = held.outer.as_deref();
        }
        None
    }

    fn find(&self, dest: &Path, at: Stamp) -> Option<String> {
        let files = self.lock();
        let found = files.iter().flatten().find(|held| held.is_at(dest, at));
        found.map(|held| held.name.clone())
    }

    /// The executor is single-threaded, so the lock is never contended; a
    /// panic elsewhere can't leave the list half-written.
    fn lock(&self) -> MutexGuard<'_, Vec<Option<Held>>> {
        self.files.lock().unwrap_or_else(|e| e.into_inner())
    }
}

impl<F: Fn() -> u64 + Copy> PipelineContext<'_, F> {
    /// The same context over a new layer, with a frame of its own for the
    /// files the level holds (a graph level's files kept for later steps).
    pub(super) fn layer_with_held(&self) -> PipelineContext<'_, F> {
        PipelineContext {
            held: Some(HeldFiles::within(self.held.as_ref())),
            ..self.layer()
        }
    }
}

impl Held {
    fn of(file: &PipelineFile) -> Option<Self> {
        match &file.data {
            FileData::Path(path) => Some(Self {
                path: path.clone(),
                name: file.name.clone(),
                stamp: OnceLock::new(),
            }),
            FileData::Bytes(_) => None,
        }
    }

    /// Whether this is the file at `dest`, whose stamp is `at`: the same
    /// path, or a path to the same file (a link, another case).
    fn is_at(&self, dest: &Path, at: Stamp) -> bool {
        if self.path == dest {
            return true;
        }
        let stamp = self.stamp.get_or_init(|| stamp(&self.path));
        *stamp == Some(at) && same_file::is_same_file(&self.path, dest).unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `a.txt`, `b.txt` in a fresh folder, as run files on disk.
    fn two_files() -> (tempfile::TempDir, Vec<PipelineFile>) {
        let dir = tempfile::tempdir().unwrap();
        let files = [("a.txt", "AAAA"), ("b.txt", "BB")]
            .iter()
            .map(|(name, body)| {
                let path = dir.path().join(name);
                std::fs::write(&path, body).unwrap();
                on_disk(&path, name)
            })
            .collect();
        (dir, files)
    }

    fn on_disk(path: &Path, name: &str) -> PipelineFile {
        PipelineFile {
            name: name.to_string(),
            data: FileData::Path(path.to_path_buf()),
            mime_type: "text/plain".to_string(),
            metadata: serde_json::Map::new(),
        }
    }

    fn path_of(file: &PipelineFile) -> PathBuf {
        match &file.data {
            FileData::Path(path) => path.clone(),
            FileData::Bytes(_) => unreachable!(),
        }
    }

    #[test]
    fn test_a_held_file_is_found_by_path() {
        let (_dir, files) = two_files();
        let held = HeldFiles::within(None);
        held.hold(&files);
        assert_eq!(
            held.reader_of(&path_of(&files[1])).as_deref(),
            Some("b.txt")
        );
    }

    #[test]
    fn test_a_hard_link_to_a_held_file_is_that_file() {
        let (dir, files) = two_files();
        let link = dir.path().join("link.txt");
        std::fs::hard_link(path_of(&files[0]), &link).unwrap();
        let held = HeldFiles::within(None);
        held.hold(&files);
        assert_eq!(held.reader_of(&link).as_deref(), Some("a.txt"));
    }

    #[test]
    fn test_a_released_file_is_no_longer_held() {
        let (_dir, files) = two_files();
        let held = HeldFiles::within(None);
        held.hold(&files);
        held.release(0);
        assert!(held.reader_of(&path_of(&files[0])).is_none());
        assert!(held.reader_of(&path_of(&files[1])).is_some());
    }

    #[test]
    fn test_replace_holds_only_the_new_files() {
        let (_dir, files) = two_files();
        let held = HeldFiles::within(None);
        held.hold(&files[..1]);
        held.replace(files[1..].iter());
        assert!(held.reader_of(&path_of(&files[0])).is_none());
        assert!(held.reader_of(&path_of(&files[1])).is_some());
    }

    #[test]
    fn test_an_outer_frames_files_are_found_too() {
        let (_dir, files) = two_files();
        let outer = HeldFiles::within(None);
        outer.hold(&files);
        let inner = HeldFiles::within(Some(&outer));
        assert_eq!(
            inner.reader_of(&path_of(&files[0])).as_deref(),
            Some("a.txt")
        );
    }

    #[test]
    fn test_in_memory_files_and_missing_paths_are_never_held() {
        let (dir, files) = two_files();
        let held = HeldFiles::within(None);
        let bytes = PipelineFile {
            data: FileData::Bytes(b"AAAA".to_vec()),
            ..files[0].clone()
        };
        held.hold(&[bytes]);
        assert!(held.reader_of(&path_of(&files[0])).is_none());
        assert!(held.reader_of(&dir.path().join("missing.txt")).is_none());
    }
}
