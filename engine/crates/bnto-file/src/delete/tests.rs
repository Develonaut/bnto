use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use bnto_core::NoopContext;
use bnto_core::processor::{BatchFile, BatchInput, FileData, NodeOutput, NodeProcessor};
use bnto_core::progress::ProgressReporter;
use bnto_core::{BntoError, ProcessContext};
use serde_json::json;

use super::FileDelete;

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

/// A temp dir holding `base/a.txt`, `base/b.txt`, `base/sub/c.txt`, the
/// folder `base/dir/` and `outside.txt` next to `base`.
fn fixture() -> tempfile::TempDir {
    let tmp = tempfile::tempdir().unwrap();
    let base = tmp.path().join("base");
    std::fs::create_dir_all(base.join("sub")).unwrap();
    std::fs::create_dir_all(base.join("dir")).unwrap();
    std::fs::write(base.join("a.txt"), "a").unwrap();
    std::fs::write(base.join("b.txt"), "b").unwrap();
    std::fs::write(base.join("sub").join("c.txt"), "c").unwrap();
    std::fs::write(tmp.path().join("outside.txt"), "o").unwrap();
    tmp
}

fn base(tmp: &tempfile::TempDir) -> PathBuf {
    tmp.path().join("base")
}

/// An incoming item referencing `path` on disk, named `name`.
fn item(path: PathBuf, name: &str) -> BatchFile {
    let mut metadata = serde_json::Map::new();
    metadata.insert("tag".to_string(), json!("kept"));
    BatchFile {
        data: FileData::Path(path),
        filename: name.to_string(),
        mime_type: Some("text/plain".to_string()),
        metadata,
    }
}

/// Items for `names` under `base`, each named by its relative path.
fn items(base: &Path, names: &[&str]) -> Vec<BatchFile> {
    names.iter().map(|n| item(base.join(n), n)).collect()
}

fn params_for(base: &Path) -> serde_json::Map<String, serde_json::Value> {
    let mut params = serde_json::Map::new();
    params.insert("base".to_string(), json!(base.to_str().unwrap()));
    params
}

/// Run file-delete over `files` under `base`, and the summary lines it printed.
fn run_with(
    base: &Path,
    files: Vec<BatchFile>,
    ctx: &dyn ProcessContext,
) -> (Result<NodeOutput, BntoError>, Vec<String>) {
    let lines = Rc::new(RefCell::new(Vec::new()));
    let sink = Rc::clone(&lines);
    let progress =
        ProgressReporter::with_live(|_, _| {}, move |l| sink.borrow_mut().push(l.to_string()));
    let input = BatchInput {
        files,
        params: params_for(base),
    };
    let result = FileDelete::new().process_batch(input, &progress, ctx);
    let printed = lines.borrow().clone();
    (result, printed)
}

fn run(base: &Path, files: Vec<BatchFile>) -> Result<NodeOutput, BntoError> {
    run_with(base, files, &NoopContext).0
}

fn invalid(result: Result<NodeOutput, BntoError>) -> String {
    match result {
        Err(BntoError::InvalidInput(msg)) => msg,
        Err(other) => panic!("expected InvalidInput, got {other:?}"),
        Ok(_) => panic!("expected InvalidInput, the run passed"),
    }
}

// --- Trait basics ---

#[test]
fn test_name_and_metadata() {
    let node = FileDelete::new();
    assert_eq!(node.name(), "file-delete");
    let meta = node.metadata();
    assert_eq!(meta.node_type, "file-delete");
    assert_eq!(meta.category, bnto_core::metadata::NodeCategory::File);
    assert!(!meta.platforms.contains(&"browser".to_string()));
    assert!(meta.platforms.contains(&"cli".to_string()));
    assert_eq!(
        meta.input_cardinality,
        bnto_core::metadata::InputCardinality::Batch
    );
    let names: Vec<&str> = meta.parameters.iter().map(|p| p.name.as_str()).collect();
    assert_eq!(names, vec!["base"]);
}

#[test]
fn test_counts_are_declared_as_count_keys() {
    assert_eq!(FileDelete::new().count_keys(), ["deleted", "skipped"]);
}

// --- Deleting ---

#[test]
fn test_the_incoming_files_are_deleted() {
    let tmp = fixture();
    let base = base(&tmp);
    let out = run(&base, items(&base, &["a.txt", "sub/c.txt"])).unwrap();
    assert!(!base.join("a.txt").exists());
    assert!(!base.join("sub").join("c.txt").exists());
    assert!(base.join("b.txt").exists(), "files not passed in stay");
    assert!(base.join("sub").is_dir(), "the folder stays");
    assert_eq!(out.metadata["deleted"], json!(2));
    assert_eq!(out.metadata["skipped"], json!(0));
}

#[test]
fn test_each_item_passes_through_flagged_with_its_metadata_kept() {
    let tmp = fixture();
    let base = base(&tmp);
    let out = run(&base, items(&base, &["a.txt", "gone.txt"])).unwrap();
    let names: Vec<&str> = out.files.iter().map(|f| f.filename.as_str()).collect();
    assert_eq!(names, vec!["a.txt", "gone.txt"]);
    assert_eq!(out.files[0].mime_type, "text/plain");
    let flags = |i: usize| {
        let m = &out.files[i].metadata;
        (m["deleted"].clone(), m["skipped"].clone(), m["tag"].clone())
    };
    assert_eq!(flags(0), (json!(1), json!(0), json!("kept")));
    assert_eq!(flags(1), (json!(0), json!(1), json!("kept")));
}

#[test]
fn test_a_relative_item_path_is_resolved() {
    let tmp = fixture();
    let base = base(&tmp);
    let odd = base.join("sub").join("..").join("a.txt");
    run(&base, vec![item(odd, "a.txt")]).unwrap();
    assert!(!base.join("a.txt").exists());
}

#[test]
fn test_zero_items_deletes_nothing() {
    let tmp = fixture();
    let base = base(&tmp);
    let out = run(&base, vec![]).unwrap();
    assert!(out.files.is_empty());
    assert_eq!(out.metadata["deleted"], json!(0));
    assert!(base.join("a.txt").exists());
}

#[test]
fn test_the_summary_counts_deleted_and_missing() {
    let tmp = fixture();
    let base = base(&tmp);
    let (result, printed) = run_with(&base, items(&base, &["a.txt", "gone.txt"]), &NoopContext);
    result.unwrap();
    assert_eq!(printed.len(), 1, "{printed:?}");
    assert!(
        printed[0].starts_with("Deleted 1 file under"),
        "{printed:?}"
    );
    assert!(printed[0].ends_with(", 1 missing"), "{printed:?}");
}

// --- Missing files ---

#[test]
fn test_a_missing_file_is_skipped_and_counted() {
    let tmp = fixture();
    let base = base(&tmp);
    let files = items(&base, &["a.txt", "gone.txt", "nope/x.txt"]);
    let out = run(&base, files).unwrap();
    assert!(!base.join("a.txt").exists());
    assert_eq!(out.metadata["deleted"], json!(1));
    assert_eq!(out.metadata["skipped"], json!(2));
}

#[test]
fn test_the_same_file_twice_is_deleted_once_then_skipped() {
    let tmp = fixture();
    let base = base(&tmp);
    let out = run(&base, items(&base, &["a.txt", "a.txt"])).unwrap();
    assert_eq!(out.metadata["deleted"], json!(1));
    assert_eq!(out.metadata["skipped"], json!(1));
}

// --- Escaping the base ---

#[test]
fn test_a_file_outside_base_is_refused_and_nothing_is_deleted() {
    let tmp = fixture();
    let base = base(&tmp);
    let mut files = items(&base, &["a.txt"]);
    files.push(item(tmp.path().join("outside.txt"), "outside.txt"));
    let msg = invalid(run(&base, files));
    assert!(msg.contains("outside.txt"), "{msg}");
    assert!(msg.contains("outside the base folder"), "{msg}");
    assert!(tmp.path().join("outside.txt").exists());
    assert!(base.join("a.txt").exists(), "a refused run deletes nothing");
}

#[test]
fn test_climbing_out_of_base_with_dotdot_is_refused() {
    let tmp = fixture();
    let base = base(&tmp);
    let sneaky = base.join("sub").join("..").join("..").join("outside.txt");
    let msg = invalid(run(&base, vec![item(sneaky, "sneaky.txt")]));
    assert!(msg.contains("sneaky.txt"), "{msg}");
    assert!(tmp.path().join("outside.txt").exists());
}

#[test]
fn test_a_missing_file_outside_base_is_refused_not_skipped() {
    let tmp = fixture();
    let base = base(&tmp);
    let gone = base.join("nope").join("..").join("..").join("gone.txt");
    let msg = invalid(run(&base, vec![item(gone, "gone.txt")]));
    assert!(msg.contains("outside the base folder"), "{msg}");
}

#[test]
fn test_a_base_named_with_a_prefix_of_another_folder_does_not_match_it() {
    // `base-other/x` starts with the text `base` but is not under `base/`.
    let tmp = fixture();
    let base = base(&tmp);
    let other = tmp.path().join("base-other");
    std::fs::create_dir_all(&other).unwrap();
    std::fs::write(other.join("x.txt"), "x").unwrap();
    invalid(run(&base, vec![item(other.join("x.txt"), "x.txt")]));
    assert!(other.join("x.txt").exists());
}

#[cfg(unix)]
#[test]
fn test_a_link_to_a_file_outside_base_is_refused() {
    let tmp = fixture();
    let base = base(&tmp);
    let link = base.join("link.txt");
    std::os::unix::fs::symlink(tmp.path().join("outside.txt"), &link).unwrap();
    let msg = invalid(run(&base, vec![item(link.clone(), "link.txt")]));
    assert!(msg.contains("link.txt"), "{msg}");
    assert!(tmp.path().join("outside.txt").exists());
    assert!(std::fs::symlink_metadata(&link).is_ok());
}

#[cfg(unix)]
#[test]
fn test_a_file_reached_through_a_linked_folder_outside_base_is_refused() {
    let tmp = fixture();
    let base = base(&tmp);
    std::fs::create_dir_all(tmp.path().join("elsewhere")).unwrap();
    std::fs::write(tmp.path().join("elsewhere").join("e.txt"), "e").unwrap();
    std::os::unix::fs::symlink(tmp.path().join("elsewhere"), base.join("out")).unwrap();
    let path = base.join("out").join("e.txt");
    invalid(run(&base, vec![item(path, "out/e.txt")]));
    assert!(tmp.path().join("elsewhere").join("e.txt").exists());
}

#[cfg(unix)]
#[test]
fn test_a_link_inside_base_to_a_file_inside_base_deletes_the_link_only() {
    let tmp = fixture();
    let base = base(&tmp);
    let link = base.join("link.txt");
    std::os::unix::fs::symlink(base.join("b.txt"), &link).unwrap();
    run(&base, vec![item(link.clone(), "link.txt")]).unwrap();
    assert!(std::fs::symlink_metadata(&link).is_err());
    assert!(base.join("b.txt").exists());
}

// --- Folders and in-memory items ---

#[test]
fn test_a_folder_is_refused() {
    let tmp = fixture();
    let base = base(&tmp);
    let mut files = items(&base, &["a.txt"]);
    files.push(item(base.join("dir"), "dir"));
    let msg = invalid(run(&base, files));
    assert!(msg.contains("dir"), "{msg}");
    assert!(msg.contains("is a folder"), "{msg}");
    assert!(base.join("dir").is_dir());
    assert!(base.join("a.txt").exists(), "a refused run deletes nothing");
}

#[test]
fn test_an_in_memory_item_is_refused() {
    let tmp = fixture();
    let base = base(&tmp);
    let bytes = BatchFile {
        data: FileData::Bytes(b"x".to_vec()),
        filename: "made.txt".to_string(),
        mime_type: None,
        metadata: serde_json::Map::new(),
    };
    let msg = invalid(run(&base, vec![bytes]));
    assert!(msg.contains("made.txt"), "{msg}");
    assert!(msg.contains("not a file on disk"), "{msg}");
}

// --- Dry run ---

#[test]
fn test_a_dry_run_deletes_nothing_and_reports_the_same_counts() {
    let tmp = fixture();
    let base = base(&tmp);
    let names = ["a.txt", "sub/c.txt", "gone.txt"];
    let (dry, printed) = run_with(&base, items(&base, &names), &DryRun);
    let dry = dry.unwrap();
    assert!(base.join("a.txt").exists());
    assert!(base.join("sub").join("c.txt").exists());
    assert!(
        printed[0].starts_with("Would delete 2 files under"),
        "{printed:?}"
    );

    let real = run(&base, items(&base, &names)).unwrap();
    assert_eq!(dry.metadata, real.metadata);
    for (d, r) in dry.files.iter().zip(&real.files) {
        assert_eq!(d.metadata, r.metadata);
    }
}

#[test]
fn test_a_dry_run_still_refuses_a_file_outside_base() {
    let tmp = fixture();
    let base = base(&tmp);
    let files = vec![item(tmp.path().join("outside.txt"), "outside.txt")];
    invalid(run_with(&base, files, &DryRun).0);
}

// --- Params ---

#[test]
fn test_validate_requires_base() {
    let node = FileDelete::new();
    let errors = node.validate(&serde_json::Map::new());
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert!(errors[0].contains("'base'"), "{errors:?}");
    let mut empty = serde_json::Map::new();
    empty.insert("base".to_string(), json!(""));
    assert_eq!(node.validate(&empty).len(), 1);
    let mut not_text = serde_json::Map::new();
    not_text.insert("base".to_string(), json!(5));
    assert_eq!(node.validate(&not_text).len(), 1);
}

#[test]
fn test_validate_passes_with_a_base() {
    let tmp = fixture();
    assert!(
        FileDelete::new()
            .validate(&params_for(&base(&tmp)))
            .is_empty()
    );
}

#[test]
fn test_running_without_base_is_invalid_input() {
    let tmp = fixture();
    let input = BatchInput {
        files: items(&base(&tmp), &["a.txt"]),
        params: serde_json::Map::new(),
    };
    let result =
        FileDelete::new().process_batch(input, &ProgressReporter::new_noop(), &NoopContext);
    invalid(result);
    assert!(base(&tmp).join("a.txt").exists());
}

#[test]
fn test_a_missing_base_is_refused() {
    let tmp = fixture();
    let msg = invalid(run(&tmp.path().join("nope"), vec![]));
    assert!(msg.contains("base folder"), "{msg}");
}

#[test]
fn test_a_base_that_is_a_file_is_refused() {
    let tmp = fixture();
    let msg = invalid(run(&tmp.path().join("outside.txt"), vec![]));
    assert!(msg.contains("not a folder"), "{msg}");
}

#[test]
fn test_process_runs_one_file_as_a_batch() {
    let tmp = fixture();
    let base = base(&tmp);
    let input = bnto_core::processor::NodeInput {
        data: FileData::Path(base.join("a.txt")),
        filename: "a.txt".to_string(),
        mime_type: None,
        params: params_for(&base),
    };
    let out = FileDelete::new()
        .process(input, &ProgressReporter::new_noop(), &NoopContext)
        .unwrap();
    assert!(!base.join("a.txt").exists());
    assert_eq!(out.metadata["deleted"], json!(1));
}
