// file-join in a pipeline: file-list brings disk files in, file-sort or the
// list's own order arranges them, file-join writes them as one file.
#![cfg(feature = "native")]

use std::path::{Path, PathBuf};

use bnto_core::processor::FileData;
use bnto_core::{BntoError, NoopContext, PipelineFile, PipelineReporter, ProcessContext};
use bnto_engine::run_pipeline;
use serde_json::{Value, json};
use tempfile::TempDir;

/// A context whose temp files land in its own folder, as a native run's do.
struct TempCtx(TempDir);

impl ProcessContext for TempCtx {
    fn run_command(&self, cmd: &str, args: &[&str]) -> Result<Vec<u8>, BntoError> {
        NoopContext.run_command(cmd, args)
    }
    fn temp_file(&self, suffix: &str) -> Result<PathBuf, BntoError> {
        let file = tempfile::Builder::new()
            .suffix(suffix)
            .tempfile_in(self.0.path())
            .unwrap();
        Ok(file.into_temp_path().keep().unwrap())
    }
    fn env_var(&self, key: &str) -> Option<String> {
        NoopContext.env_var(key)
    }
    fn work_dir(&self) -> Result<&Path, BntoError> {
        NoopContext.work_dir()
    }
}

/// A pipeline of `steps` between an empty input and an output that writes nothing.
fn pipeline(steps: Vec<Value>) -> String {
    let mut nodes =
        vec![json!({ "id": "input", "type": "input", "parameters": { "mode": "none" } })];
    nodes.extend(steps);
    nodes.push(json!({ "id": "output", "type": "output", "parameters": { "mode": "none" } }));
    json!({ "settings": { "iteration": "auto" }, "nodes": nodes }).to_string()
}

fn list(root: &Path, paths: &[&str]) -> Value {
    json!({ "id": "list", "type": "file-list",
            "parameters": { "base": root.to_str().unwrap(), "paths": paths } })
}

fn join(separator: &str) -> Value {
    json!({ "id": "join", "type": "file-join",
            "parameters": { "filename": "README.md", "separator": separator } })
}

/// `root` holding notes.md and listing.md.
fn notes_and_listing() -> TempDir {
    let root = TempDir::new().unwrap();
    std::fs::write(root.path().join("notes.md"), "# Notes\n\nHand-written.\n").unwrap();
    std::fs::write(root.path().join("listing.md"), "- a.wav\n- b.wav\n").unwrap();
    root
}

fn run(json: &str, files: Vec<PipelineFile>) -> Result<bnto_core::PipelineResult, BntoError> {
    let ctx = TempCtx(TempDir::new().unwrap());
    let result = run_pipeline(json, files, &PipelineReporter::new_noop(), &ctx);
    // Read the joined file before the context's temp folder goes.
    result.map(|mut r| {
        for f in &mut r.files {
            f.data = FileData::Bytes(f.data.to_bytes().unwrap().into_owned());
        }
        r
    })
}

fn only_text(result: &bnto_core::PipelineResult) -> String {
    assert_eq!(result.files.len(), 1, "file-join gives one file");
    assert_eq!(result.files[0].name, "README.md");
    String::from_utf8(result.files[0].data.to_bytes().unwrap().into_owned()).unwrap()
}

#[test]
fn test_file_list_then_file_join_joins_in_listed_order() {
    let root = notes_and_listing();
    let json = pipeline(vec![
        list(root.path(), &["notes.md", "listing.md"]),
        join("\n"),
    ]);
    let result = run(&json, vec![]).expect("pipeline should succeed");
    assert_eq!(
        only_text(&result),
        "# Notes\n\nHand-written.\n\n- a.wav\n- b.wav\n"
    );
    assert_eq!(result.node_metadata["join"]["joined"], 2);
    // The sources are only read.
    assert!(root.path().join("notes.md").is_file());
    assert!(root.path().join("listing.md").is_file());
}

#[test]
fn test_file_sort_before_file_join_sets_the_order() {
    let root = notes_and_listing();
    let sort = json!({ "id": "sort", "type": "file-sort", "parameters": { "by": "-full_path" } });
    let json = pipeline(vec![
        list(root.path(), &["listing.md", "notes.md"]),
        sort,
        join(""),
    ]);
    let result = run(&json, vec![]).expect("pipeline should succeed");
    assert_eq!(
        only_text(&result),
        "# Notes\n\nHand-written.\n- a.wav\n- b.wav\n"
    );
}

#[test]
fn test_a_file_gone_before_the_join_fails_the_run_naming_it() {
    let root = TempDir::new().unwrap();
    let gone = root.path().join("gone.md");
    let file = PipelineFile {
        name: "gone.md".to_string(),
        data: FileData::Path(gone),
        mime_type: "text/markdown".to_string(),
        metadata: serde_json::Map::new(),
    };
    let json = json!({ "nodes": [join("")] }).to_string();
    let Err(err) = run(&json, vec![file]) else {
        panic!("the run should fail");
    };
    let message = err.to_string();
    assert!(
        message.contains("gone.md") && message.contains("no longer exists"),
        "{message}"
    );
}

#[test]
fn test_check_refuses_a_file_join_with_an_empty_filename() {
    // Left out, the filename is a note: `bnto run -p join:filename=…` gives it.
    let definition: bnto_core::PipelineDefinition = serde_json::from_value(json!({
        "nodes": [{ "id": "join", "type": "file-join", "parameters": { "filename": "" } }]
    }))
    .unwrap();
    let findings = bnto_core::check_pipeline(&definition, &bnto_engine::create_registry()).findings;
    assert_eq!(findings.len(), 1, "{findings:?}");
    assert_eq!(findings[0].node_id, "join");
    assert!(findings[0].message.contains("'filename' is required"));
}
