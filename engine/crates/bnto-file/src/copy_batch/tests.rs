use super::*;
use crate::copy::FileCopy;
use bnto_core::ProcessContext;
use bnto_core::context::NoopContext;
use bnto_core::metadata::InputCardinality;
use bnto_core::processor::{BatchFile, FileData, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;
use filetime::{FileTime, set_file_mtime};
use serde_json::{Map, Value, json};
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

struct DryRun;

impl ProcessContext for DryRun {
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
        true
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
    ctx: &dyn ProcessContext,
) -> Result<NodeOutput, BntoError> {
    let input = BatchInput { files, params };
    FileCopy::new().process_batch(input, &ProgressReporter::new_noop(), ctx)
}

/// The batch was refused before any copy, naming the file it protects.
fn assert_refused(result: Result<NodeOutput, BntoError>, b: &Path) {
    let Err(BntoError::InvalidInput(why)) = result else {
        panic!("expected a refusal");
    };
    assert!(
        why.starts_with("File copy refused, nothing copied"),
        "{why}"
    );
    assert!(why.contains(&b.display().to_string()), "{why}");
    assert_eq!(fs::read_to_string(b).unwrap(), "BBBB", "{why}");
}

// --- The plan ---

#[test]
fn test_overwrite_onto_a_file_another_file_reads_is_refused() {
    // ISS-52: `a` renamed to `b.txt`, copied over `b.txt` before `b`'s own
    // copy (which leaves its file as is) reads it.
    let (dir, a, b) = two_inputs();
    let files = vec![file(&a, "b.txt"), file(&b, "b.txt")];
    let result = copy(files, params(dir.path(), "overwrite"), &NoopContext);
    assert_refused(result, &b);
}

#[test]
fn test_overwrite_onto_a_file_read_earlier_is_refused_too() {
    // `b` comes first, so its own copy has run, but every step after
    // file-copy still reads `b.txt` through the pass-through.
    let (dir, a, b) = two_inputs();
    let files = vec![file(&b, "b.txt"), file(&a, "b.txt")];
    let result = copy(files, params(dir.path(), "overwrite"), &NoopContext);
    assert_refused(result, &b);
}

#[test]
fn test_newer_onto_an_older_file_another_file_reads_is_refused() {
    let (dir, a, b) = two_inputs();
    set_file_mtime(&b, FileTime::from_unix_time(1_000, 0)).unwrap();
    let files = vec![file(&a, "b.txt"), file(&b, "b.txt")];
    let result = copy(files, params(dir.path(), "newer"), &NoopContext);
    assert_refused(result, &b);
}

#[test]
fn test_newer_onto_a_newer_file_another_file_reads_still_runs() {
    // Guard: `newer` keeps a newer target, so nothing is replaced.
    let (dir, a, b) = two_inputs();
    set_file_mtime(&a, FileTime::from_unix_time(1_000, 0)).unwrap();
    let files = vec![file(&a, "b.txt"), file(&b, "b.txt")];
    copy(files, params(dir.path(), "newer"), &NoopContext).unwrap();
    assert_eq!(fs::read_to_string(&b).unwrap(), "BBBB");
}

#[test]
fn test_a_dry_run_is_refused_too() {
    let (dir, a, b) = two_inputs();
    let files = vec![file(&a, "b.txt"), file(&b, "b.txt")];
    let result = copy(files, params(dir.path(), "overwrite"), &DryRun);
    assert_refused(result, &b);
}

#[test]
fn test_skip_and_rename_onto_a_file_another_file_reads_still_run() {
    for conflict in ["skip", "rename"] {
        let (dir, a, b) = two_inputs();
        let files = vec![file(&a, "b.txt"), file(&b, "b.txt")];
        copy(files, params(dir.path(), conflict), &NoopContext).unwrap();
        assert_eq!(fs::read_to_string(&b).unwrap(), "BBBB", "{conflict}");
    }
}

#[test]
fn test_overwrite_into_another_folder_copies_every_file() {
    let (_dir, a, b) = two_inputs();
    let out = TempDir::new().unwrap();
    fs::write(out.path().join("a.txt"), "old").unwrap();
    let files = vec![file(&a, "a.txt"), file(&b, "b.txt")];
    let output = copy(files, params(out.path(), "overwrite"), &NoopContext).unwrap();
    assert_eq!(output.files.len(), 2);
    assert_eq!(
        fs::read_to_string(out.path().join("a.txt")).unwrap(),
        "AAAA"
    );
    assert_eq!(
        fs::read_to_string(out.path().join("b.txt")).unwrap(),
        "BBBB"
    );
}

#[test]
fn test_every_file_left_at_its_own_destination_still_runs() {
    // Guard (ISS-36): each copy leaves its own file as is.
    let (dir, a, b) = two_inputs();
    let files = vec![file(&a, "a.txt"), file(&b, "b.txt")];
    copy(files, params(dir.path(), "overwrite"), &NoopContext).unwrap();
    assert_eq!(fs::read_to_string(&a).unwrap(), "AAAA");
    assert_eq!(fs::read_to_string(&b).unwrap(), "BBBB");
}

// --- Per-file params ---

/// `file(src, name)` carrying `folder` in its metadata.
fn in_folder(src: &Path, name: &str, folder: &str) -> BatchFile {
    let mut f = file(src, name);
    f.metadata.insert("folder".into(), json!(folder));
    f
}

#[test]
fn test_item_values_in_the_destination_are_filled_per_file() {
    let (_dir, a, b) = two_inputs();
    let out = TempDir::new().unwrap();
    let dest = out.path().join("{{item.folder}}");
    let files = vec![in_folder(&a, "a.txt", "x"), in_folder(&b, "b.txt", "y")];
    copy(files, params(&dest, "skip"), &NoopContext).unwrap();
    assert_eq!(
        fs::read_to_string(out.path().join("x/a.txt")).unwrap(),
        "AAAA"
    );
    assert_eq!(
        fs::read_to_string(out.path().join("y/b.txt")).unwrap(),
        "BBBB"
    );
}

#[test]
fn test_an_item_value_a_file_lacks_fails_naming_the_key_and_the_file() {
    let (_dir, a, b) = two_inputs();
    let out = TempDir::new().unwrap();
    let dest = out.path().join("{{item.folder}}");
    let files = vec![in_folder(&a, "a.txt", "x"), file(&b, "b.txt")];
    let result = copy(files, params(&dest, "skip"), &NoopContext);
    let Err(BntoError::InvalidInput(why)) = result else {
        panic!("expected invalid input");
    };
    assert!(
        why.contains("{{item.folder}}") && why.contains("b.txt"),
        "{why}"
    );
    assert!(!out.path().join("x").exists(), "nothing is copied: {why}");
}

#[test]
fn test_file_copy_runs_as_a_batch_and_fills_items_itself() {
    let node = FileCopy::new();
    assert_eq!(node.metadata().input_cardinality, InputCardinality::Batch);
    assert!(node.resolves_items());
}

#[test]
fn test_a_sole_item_placeholder_keeps_its_type() {
    // As the executor fills a whole-value placeholder: `false` stays a
    // boolean, so `createDirs` reads it.
    let (_dir, a, _) = two_inputs();
    let out = TempDir::new().unwrap();
    let missing = out.path().join("missing");
    let mut f = file(&a, "a.txt");
    f.metadata.insert("make".into(), json!(false));
    let mut p = params(&missing, "skip");
    p.insert("createDirs".into(), json!("{{item.make}}"));
    let result = copy(vec![f], p, &NoopContext);
    assert!(
        matches!(result, Err(BntoError::InvalidInput(_))),
        "no folder is made"
    );
    assert!(!missing.exists());
}

#[test]
fn test_a_sole_item_placeholder_ignores_surrounding_whitespace() {
    // As the executor does: `"{{item.dest}} "` is the value itself, not a
    // folder name ending in a space. (Only a trailing space: a leading one
    // would make a regressed run write under the working directory.)
    let (_dir, a, _) = two_inputs();
    let out = TempDir::new().unwrap();
    let mut f = file(&a, "a.txt");
    let dest = out.path().join("y");
    f.metadata
        .insert("dest".into(), json!(dest.to_str().unwrap()));
    let mut p = params(out.path(), "skip");
    p.insert("destination".into(), json!("{{item.dest}} "));
    copy(vec![f], p, &NoopContext).unwrap();
    assert!(dest.join("a.txt").exists());
}

// --- Metadata and failures ---

#[test]
fn test_each_file_keeps_its_metadata_under_the_nodes_own() {
    // RED (ISS-52 review): a per-file node's output inherited the input's
    // metadata from the executor; a batch node must carry it itself.
    let (_dir, a, b) = two_inputs();
    let out = TempDir::new().unwrap();
    let files = vec![in_folder(&a, "a.txt", "x"), in_folder(&b, "b.txt", "y")];
    let output = copy(files, params(out.path(), "newer"), &NoopContext).unwrap();
    let folders: Vec<&Value> = output.files.iter().map(|f| &f.metadata["folder"]).collect();
    assert_eq!(folders, [&json!("x"), &json!("y")]);
    assert_eq!(output.files[0].metadata["copied"], json!(1));
}

#[test]
fn test_the_nodes_own_metadata_wins_a_key_the_input_carries() {
    // The executor puts a node's metadata over the input's: a stale
    // `copied: 0` from an earlier copy step gives way to this copy's.
    let (_dir, a, _) = two_inputs();
    let out = TempDir::new().unwrap();
    let mut f = file(&a, "a.txt");
    f.metadata.insert("copied".into(), json!(0));
    let output = copy(vec![f], params(out.path(), "newer"), &NoopContext).unwrap();
    assert_eq!(output.files[0].metadata["copied"], json!(1));
}

#[test]
fn test_a_failed_copy_hands_over_the_files_already_copied() {
    // RED (ISS-52 review): a per-file node handed the files done before a
    // failure to the run's closing steps; the batch dropped them.
    let (dir, a, _) = two_inputs();
    let out = TempDir::new().unwrap();
    let gone = dir.path().join("gone.txt");
    let files = vec![file(&a, "a.txt"), file(&gone, "gone.txt")];
    let result = copy(files, params(out.path(), "overwrite"), &NoopContext);
    let Err(error) = result else {
        panic!("the missing source must fail");
    };
    let (_, handed) = error.into_parts();
    let names: Vec<String> = handed
        .unwrap()
        .files
        .into_iter()
        .map(|f| f.filename)
        .collect();
    assert_eq!(names, ["a.txt"]);
}

// --- Combined output ---

/// A one-file output whose node-level metadata is `counts`.
fn counted(name: &str, counts: &[(&str, u8)]) -> NodeOutput {
    let metadata: Map<String, Value> = counts
        .iter()
        .map(|(k, v)| (k.to_string(), json!(v)))
        .collect();
    NodeOutput {
        files: vec![OutputFile {
            data: FileData::Bytes(Vec::new()),
            filename: name.to_string(),
            mime_type: "text/plain".to_string(),
            metadata: metadata.clone(),
        }],
        metadata,
    }
}

#[test]
fn test_combined_keeps_every_file_in_order_and_adds_up_the_counts() {
    let one = counted(
        "a.txt",
        &[("copied", 1), ("up_to_date", 0), ("skipped_newer", 0)],
    );
    let two = counted(
        "b.txt",
        &[("copied", 1), ("up_to_date", 0), ("skipped_newer", 0)],
    );
    let three = counted(
        "c.txt",
        &[("copied", 0), ("up_to_date", 1), ("skipped_newer", 0)],
    );
    let out = combined(vec![one, two, three]);
    let names: Vec<&str> = out.files.iter().map(|f| f.filename.as_str()).collect();
    assert_eq!(names, ["a.txt", "b.txt", "c.txt"]);
    let expected = json!({ "copied": 2, "up_to_date": 1, "skipped_newer": 0 });
    assert_eq!(Value::Object(out.metadata), expected);
}

#[test]
fn test_combined_outputs_without_counts_have_none() {
    let out = combined(vec![counted("a.txt", &[]), counted("b.txt", &[])]);
    assert!(out.metadata.is_empty());
}
