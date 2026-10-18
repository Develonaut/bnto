use super::*;
use bnto_core::context::NoopContext;
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// A context whose temp files land in its own folder, as a native run's do.
struct TempCtx {
    dir: TempDir,
    dry_run: bool,
}

impl TempCtx {
    fn new() -> Self {
        Self {
            dir: TempDir::new().unwrap(),
            dry_run: false,
        }
    }

    fn dry() -> Self {
        Self {
            dry_run: true,
            ..Self::new()
        }
    }
}

impl ProcessContext for TempCtx {
    fn run_command(&self, cmd: &str, args: &[&str]) -> Result<Vec<u8>, BntoError> {
        NoopContext.run_command(cmd, args)
    }
    fn temp_file(&self, suffix: &str) -> Result<PathBuf, BntoError> {
        let file = tempfile::Builder::new()
            .suffix(suffix)
            .tempfile_in(self.dir.path())
            .unwrap();
        Ok(file.into_temp_path().keep().unwrap())
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
}

/// An in-memory item named `name` holding `text`.
fn bytes(name: &str, text: &str) -> BatchFile {
    BatchFile {
        data: FileData::Bytes(text.as_bytes().to_vec()),
        filename: name.to_string(),
        mime_type: Some("text/plain".to_string()),
        metadata: serde_json::Map::new(),
    }
}

/// A disk item: `name` written under `dir` holding `text`.
fn on_disk(dir: &Path, name: &str, text: &str) -> BatchFile {
    let path = dir.join(name);
    std::fs::write(&path, text).unwrap();
    disk_ref(path, name)
}

fn disk_ref(path: PathBuf, name: &str) -> BatchFile {
    BatchFile {
        data: FileData::Path(path),
        filename: name.to_string(),
        mime_type: Some("application/octet-stream".to_string()),
        metadata: serde_json::Map::new(),
    }
}

fn join_with(
    params: Value,
    files: Vec<BatchFile>,
    ctx: &dyn ProcessContext,
) -> Result<NodeOutput, BntoError> {
    let input = BatchInput {
        files,
        params: params.as_object().cloned().unwrap(),
    };
    FileJoin::new().process_batch(input, &ProgressReporter::new_noop(), ctx)
}

fn join(params: Value, files: Vec<BatchFile>) -> Result<NodeOutput, BntoError> {
    join_with(params, files, &NoopContext)
}

/// The one output file's content.
fn content(out: &NodeOutput) -> String {
    assert_eq!(out.files.len(), 1, "file-join writes exactly one file");
    let bytes = out.files[0].data.to_bytes().unwrap();
    String::from_utf8(bytes.into_owned()).unwrap()
}

fn validate(params: Value) -> Vec<String> {
    FileJoin::new().validate(params.as_object().unwrap())
}

fn named(filename: &str) -> Value {
    json!({ "filename": filename })
}

// --- Trait basics ---

#[test]
fn test_metadata_is_a_browser_capable_batch_file_node() {
    let node = FileJoin::new();
    assert_eq!(node.name(), "file-join");
    let meta = node.metadata();
    assert_eq!(meta.node_type, "file-join");
    assert_eq!(meta.category, bnto_core::metadata::NodeCategory::File);
    assert_eq!(meta.input_cardinality, InputCardinality::Batch);
    assert!(meta.platforms.contains(&"browser".to_string()));
    assert!(meta.platforms.contains(&"cli".to_string()));
    let params: Vec<&str> = meta.parameters.iter().map(|p| p.name.as_str()).collect();
    assert_eq!(params, ["filename", "separator"]);
}

// --- Joining, in arrival order ---

#[test]
fn test_two_files_are_joined_in_order() {
    let files = vec![
        bytes("notes.md", "# Notes\n"),
        bytes("list.md", "- a\n- b\n"),
    ];
    let out = join(named("readme.md"), files).unwrap();
    assert_eq!(content(&out), "# Notes\n- a\n- b\n");
}

#[test]
fn test_order_is_the_order_files_arrive_in_not_their_names() {
    let files = vec![
        bytes("c.txt", "C"),
        bytes("a.txt", "A"),
        bytes("b.txt", "B"),
    ];
    let out = join(named("all.txt"), files).unwrap();
    assert_eq!(content(&out), "CAB");
}

#[test]
fn test_bytes_are_joined_as_they_are_not_as_text() {
    let raw = |name: &str, data: &[u8]| BatchFile {
        data: FileData::Bytes(data.to_vec()),
        ..bytes(name, "")
    };
    let files = vec![
        raw("a.bin", &[0xff, 0x00]),
        raw("b.bin", &[0xfe, 0x0d, 0x0a]),
    ];
    let out = join(named("ab.bin"), files).unwrap();
    let joined = out.files[0].data.to_bytes().unwrap();
    assert_eq!(&*joined, &[0xff, 0x00, 0xfe, 0x0d, 0x0a]);
}

// --- Disk references stream through a temp file ---

#[test]
fn test_disk_files_stream_into_a_temp_file_from_the_context() {
    let src = TempDir::new().unwrap();
    let ctx = TempCtx::new();
    let files = vec![
        on_disk(src.path(), "one.txt", "first\n"),
        bytes("two.txt", "second\n"),
        on_disk(src.path(), "three.txt", "third\n"),
    ];
    let out = join_with(named("all.txt"), files, &ctx).unwrap();
    let FileData::Path(path) = &out.files[0].data else {
        panic!("a join with disk files is a disk file, not loaded into memory");
    };
    assert!(path.starts_with(ctx.dir.path()), "{path:?}");
    assert_eq!(content(&out), "first\nsecond\nthird\n");
    // The sources are read, never moved or changed.
    assert_eq!(
        std::fs::read_to_string(src.path().join("one.txt")).unwrap(),
        "first\n"
    );
}

#[test]
fn test_in_memory_files_join_in_memory() {
    let files = vec![bytes("a.txt", "A"), bytes("b.txt", "B")];
    let out = join(named("ab.txt"), files).unwrap();
    assert!(matches!(out.files[0].data, FileData::Bytes(_)));
}

#[test]
fn test_a_missing_file_fails_the_run_naming_it() {
    let src = TempDir::new().unwrap();
    let ctx = TempCtx::new();
    let files = vec![
        on_disk(src.path(), "here.txt", "here"),
        disk_ref(src.path().join("gone.txt"), "gone.txt"),
        disk_ref(src.path().join("lost.txt"), "lost.txt"),
    ];
    let err = join_with(named("all.txt"), files, &ctx)
        .err()
        .unwrap()
        .to_string();
    assert!(
        err.contains("gone.txt") && err.contains("lost.txt"),
        "{err}"
    );
    assert!(err.contains("no longer exist"), "{err}");
    let written = std::fs::read_dir(ctx.dir.path()).unwrap().count();
    assert_eq!(written, 0, "nothing is written when an input is missing");
}

#[test]
fn test_a_dry_run_writes_only_its_own_temp_file() {
    let src = TempDir::new().unwrap();
    let ctx = TempCtx::dry();
    let files = vec![
        on_disk(src.path(), "a.txt", "A"),
        on_disk(src.path(), "b.txt", "B"),
    ];
    let out = join_with(named("ab.txt"), files, &ctx).unwrap();
    assert_eq!(content(&out), "AB", "a dry run joins the same");
    let mut left: Vec<_> = std::fs::read_dir(src.path())
        .unwrap()
        .map(|e| e.unwrap().file_name())
        .collect();
    left.sort();
    assert_eq!(
        left,
        ["a.txt", "b.txt"],
        "nothing written next to the sources"
    );
}

// --- Empty input ---

#[test]
fn test_zero_files_give_one_empty_file() {
    let out = join(json!({ "filename": "all.txt", "separator": "\n" }), vec![]).unwrap();
    assert_eq!(out.files[0].filename, "all.txt");
    assert_eq!(content(&out), "");
    assert_eq!(out.metadata["joined"], 0);
}

// --- Separator ---

#[test]
fn test_separator_goes_between_files_only() {
    let files = vec![bytes("a", "A"), bytes("b", "B"), bytes("c", "C")];
    let out = join(
        json!({ "filename": "x.txt", "separator": "\n---\n" }),
        files,
    )
    .unwrap();
    assert_eq!(content(&out), "A\n---\nB\n---\nC");
}

#[test]
fn test_separator_goes_between_disk_files_too() {
    let src = TempDir::new().unwrap();
    let files = vec![
        on_disk(src.path(), "a.txt", "A"),
        on_disk(src.path(), "b.txt", "B"),
    ];
    let params = json!({ "filename": "x.txt", "separator": "\n\n" });
    let ctx = TempCtx::new();
    let out = join_with(params, files, &ctx).unwrap();
    assert_eq!(content(&out), "A\n\nB");
}

#[test]
fn test_one_file_gets_no_separator() {
    let out = join(
        json!({ "filename": "x.txt", "separator": "|" }),
        vec![bytes("a", "A")],
    )
    .unwrap();
    assert_eq!(content(&out), "A");
}

#[test]
fn test_separator_defaults_to_nothing() {
    let defs = params::build_join_parameters();
    let separator = defs.iter().find(|d| d.name == "separator").unwrap();
    assert_eq!(separator.default, Some(json!("")));
    let out = join(named("x.txt"), vec![bytes("a", "A"), bytes("b", "B")]).unwrap();
    assert_eq!(content(&out), "AB");
}

// --- Output file and metadata ---

#[test]
fn test_output_is_named_by_filename_with_its_mime_type() {
    let out = join(named("data.csv"), vec![bytes("a.csv", "x\n")]).unwrap();
    assert_eq!(out.files[0].filename, "data.csv");
    assert_eq!(out.files[0].mime_type, "text/csv");
    let out = join(named("blob"), vec![bytes("a", "x")]).unwrap();
    assert_eq!(out.files[0].mime_type, "application/octet-stream");
}

#[test]
fn test_node_metadata_counts_the_files_joined() {
    let out = join(named("x.txt"), vec![bytes("a", "A"), bytes("b", "B")]).unwrap();
    assert_eq!(out.metadata["joined"], 2);
    assert_eq!(FileJoin::new().count_keys(), ["joined"]);
}

// --- Params ---

#[test]
fn test_validate_requires_a_filename() {
    assert!(validate(json!({}))[0].contains("'filename' is required"));
    assert!(validate(json!({ "filename": "" }))[0].contains("'filename' is required"));
    assert!(validate(named("{{vars.Name}}.md")).is_empty());
}

#[test]
fn test_validate_refuses_a_separator_that_is_not_text() {
    let errors = validate(json!({ "filename": "x.txt", "separator": 3 }));
    assert!(errors[0].contains("'separator' must be text"), "{errors:?}");
}

#[test]
fn test_process_refuses_invalid_params() {
    let err = join(json!({}), vec![bytes("a", "A")]).err().unwrap();
    assert!(err.to_string().contains("'filename' is required"), "{err}");
}
