// file-copy against the rest of the run (ISS-59): a copy over a file a later
// step still reads, outside this batch, is refused, and every copy made is
// recorded so a later refusal can name it.

use super::*;
use crate::copy::FileCopy;
use bnto_core::ProcessContext;
use bnto_core::context::NoopContext;
use bnto_core::processor::{BatchFile, FileData, NodeProcessor};
use bnto_core::progress::ProgressReporter;
use filetime::{FileTime, set_file_mtime};
use serde_json::{Map, Value, json};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tempfile::TempDir;

/// A run as a batch node sees it: `held` files a later step reads (path,
/// reader's name), the writes recorded, and whether it's a dry run.
#[derive(Default)]
struct Run {
    held: Vec<(PathBuf, String)>,
    writes: Mutex<Vec<PathBuf>>,
    dry_run: bool,
}

impl Run {
    fn holding(path: &Path, name: &str) -> Self {
        Self {
            held: vec![(path.to_path_buf(), name.to_string())],
            ..Self::default()
        }
    }

    fn writes(&self) -> Vec<PathBuf> {
        self.writes.lock().unwrap().clone()
    }
}

impl ProcessContext for Run {
    fn run_command(&self, cmd: &str, args: &[&str]) -> Result<Vec<u8>, BntoError> {
        NoopContext.run_command(cmd, args)
    }
    fn temp_file(&self, suffix: &str) -> Result<PathBuf, BntoError> {
        NoopContext.temp_file(suffix)
    }
    fn env_var(&self, key: &str) -> Option<String> {
        NoopContext.env_var(key)
    }
    fn work_dir(&self) -> Result<&Path, BntoError> {
        NoopContext.work_dir()
    }
    fn dry_run(&self) -> bool {
        self.dry_run
    }
    fn held_elsewhere(&self, path: &Path) -> Option<String> {
        let found = self.held.iter().find(|(held, _)| held == path);
        found.map(|(_, name)| name.clone())
    }
    fn record_write(&self, path: &Path) {
        self.writes.lock().unwrap().push(path.to_path_buf());
    }
    fn recorded_writes(&self) -> Vec<PathBuf> {
        self.writes()
    }
}

/// `a.txt` = `AAAA` and `b.txt` = `BBBB` in a fresh folder.
fn two_inputs() -> (TempDir, PathBuf, PathBuf) {
    let dir = TempDir::new().unwrap();
    let (a, b) = (dir.path().join("a.txt"), dir.path().join("b.txt"));
    fs::write(&a, "AAAA").unwrap();
    fs::write(&b, "BBBB").unwrap();
    (dir, a, b)
}

/// The file at `src`, named `name` in the pipeline.
fn file(src: &Path, name: &str) -> BatchFile {
    BatchFile {
        data: FileData::Path(src.to_path_buf()),
        filename: name.to_string(),
        mime_type: None,
        metadata: Map::new(),
    }
}

fn params(dest: &Path, conflict: &str) -> Map<String, Value> {
    let mut map = Map::new();
    map.insert("destination".into(), json!(dest.to_str().unwrap()));
    map.insert("conflict".into(), json!(conflict));
    map
}

fn copy(
    files: Vec<BatchFile>,
    params: Map<String, Value>,
    run: &Run,
) -> Result<NodeOutput, BntoError> {
    let input = BatchInput { files, params };
    FileCopy::new().process_batch(input, &ProgressReporter::new_noop(), run)
}

/// The refusal's text; panics on anything else.
fn refusal(result: Result<NodeOutput, BntoError>) -> String {
    match result {
        Err(BntoError::InvalidInput(why)) => why,
        Err(other) => panic!("expected a refusal, got {other}"),
        Ok(_) => panic!("expected a refusal, the copy ran"),
    }
}

// --- Refused: a file a later step reads ---

#[test]
fn test_overwrite_onto_a_file_held_elsewhere_is_refused() {
    // RED (ISS-59): inside a loop, `b` is another iteration's file. `a`
    // renamed to `b.txt` was copied over it: BBBB was lost.
    let (dir, a, b) = two_inputs();
    let run = Run::holding(&b, "b.txt");
    let why = refusal(copy(
        vec![file(&a, "b.txt")],
        params(dir.path(), "overwrite"),
        &run,
    ));
    assert!(
        why.starts_with("File copy refused, nothing copied"),
        "{why}"
    );
    assert!(why.contains(&b.display().to_string()), "{why}");
    assert!(why.contains("the file b.txt is read from"), "{why}");
    assert_eq!(fs::read_to_string(&b).unwrap(), "BBBB");
}

#[test]
fn test_newer_onto_an_older_file_held_elsewhere_is_refused() {
    let (dir, a, b) = two_inputs();
    set_file_mtime(&b, FileTime::from_unix_time(1_577_836_800, 0)).unwrap();
    let run = Run::holding(&b, "b.txt");
    let why = refusal(copy(
        vec![file(&a, "b.txt")],
        params(dir.path(), "newer"),
        &run,
    ));
    assert!(
        why.starts_with("File copy refused, nothing copied"),
        "{why}"
    );
    assert_eq!(fs::read_to_string(&b).unwrap(), "BBBB");
}

#[test]
fn test_a_dry_run_onto_a_file_held_elsewhere_is_refused_too() {
    let (dir, a, b) = two_inputs();
    let run = Run {
        dry_run: true,
        ..Run::holding(&b, "b.txt")
    };
    refusal(copy(
        vec![file(&a, "b.txt")],
        params(dir.path(), "overwrite"),
        &run,
    ));
}

#[test]
fn test_a_refusal_names_the_copies_made_before_it() {
    // RED: in a loop, earlier iterations' copies have run; the refusal says
    // which, instead of "nothing copied".
    let (dir, a, b) = two_inputs();
    let earlier = dir.path().join("c.txt");
    let run = Run::holding(&b, "b.txt");
    run.record_write(&earlier);
    let why = refusal(copy(
        vec![file(&a, "b.txt")],
        params(dir.path(), "overwrite"),
        &run,
    ));
    assert!(
        why.starts_with("File copy refused, nothing more copied"),
        "{why}"
    );
    assert!(why.contains("Copied earlier in this run: 1 file"), "{why}");
    assert!(why.contains(&earlier.display().to_string()), "{why}");
}

// --- Not refused ---

#[test]
fn test_skip_and_rename_onto_a_file_held_elsewhere_still_run() {
    for conflict in ["skip", "rename"] {
        let (dir, a, b) = two_inputs();
        let run = Run::holding(&b, "b.txt");
        let result = copy(vec![file(&a, "b.txt")], params(dir.path(), conflict), &run);
        assert!(result.is_ok(), "{conflict}");
        assert_eq!(fs::read_to_string(&b).unwrap(), "BBBB", "{conflict}");
    }
}

#[test]
fn test_a_file_left_at_its_own_destination_is_not_refused() {
    // The same file held by a later step and copied onto itself: left as is.
    let (dir, _, b) = two_inputs();
    let run = Run::holding(&b, "b.txt");
    let result = copy(
        vec![file(&b, "b.txt")],
        params(dir.path(), "overwrite"),
        &run,
    );
    assert!(result.is_ok());
    assert!(run.writes().is_empty(), "nothing was copied");
}

#[test]
fn test_overwriting_a_file_nothing_else_reads_still_runs() {
    // Guard: an in-place overwrite of new bytes over the file's own original.
    let (dir, a, _) = two_inputs();
    let new = BatchFile {
        data: FileData::Bytes(b"aaaa".to_vec()),
        ..file(&a, "a.txt")
    };
    let run = Run::default();
    copy(vec![new], params(dir.path(), "overwrite"), &run).unwrap();
    assert_eq!(fs::read_to_string(&a).unwrap(), "aaaa");
}

// --- Recorded writes ---

#[test]
fn test_each_copy_made_is_recorded() {
    let (_dir, a, b) = two_inputs();
    let out = TempDir::new().unwrap();
    let run = Run::default();
    let files = vec![file(&a, "a.txt"), file(&b, "b.txt")];
    copy(files, params(out.path(), "overwrite"), &run).unwrap();
    let expected = [out.path().join("a.txt"), out.path().join("b.txt")];
    assert_eq!(run.writes(), expected);
}

#[test]
fn test_a_newer_copy_made_is_recorded() {
    let (_dir, a, _) = two_inputs();
    let out = TempDir::new().unwrap();
    let run = Run::default();
    copy(vec![file(&a, "a.txt")], params(out.path(), "newer"), &run).unwrap();
    assert_eq!(run.writes(), [out.path().join("a.txt")]);
}

#[test]
fn test_skipped_and_dry_run_copies_are_not_recorded() {
    let (dir, a, _) = two_inputs();
    let run = Run::default();
    copy(vec![file(&a, "a.txt")], params(dir.path(), "skip"), &run).unwrap();
    let out = TempDir::new().unwrap();
    let dry = Run {
        dry_run: true,
        ..Run::default()
    };
    copy(
        vec![file(&a, "a.txt")],
        params(out.path(), "overwrite"),
        &dry,
    )
    .unwrap();
    assert!(run.writes().is_empty());
    assert!(dry.writes().is_empty());
}
