// Edges with real nodes: a file-list with no incoming edge starts fresh, and
// file-join joins its fan-in in edge order, whatever order `nodes` lists.
#![cfg(feature = "native")]

use std::path::{Path, PathBuf};

use bnto_core::processor::FileData;
use bnto_core::{BntoError, NoopContext, PipelineReporter, PipelineResult, ProcessContext};
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

/// `steps` between an empty input and an output that writes nothing, wired by `edges`.
fn pipeline(steps: Vec<Value>, edges: &[(&str, &str)]) -> String {
    let mut nodes =
        vec![json!({ "id": "input", "type": "input", "parameters": { "mode": "none" } })];
    nodes.extend(steps);
    nodes.push(json!({ "id": "output", "type": "output", "parameters": { "mode": "none" } }));
    let edges: Vec<Value> = edges
        .iter()
        .enumerate()
        .map(|(i, (s, t))| json!({ "id": format!("e{i}"), "source": s, "target": t }))
        .collect();
    json!({ "settings": { "iteration": "auto" }, "nodes": nodes, "edges": edges }).to_string()
}

fn list(id: &str, root: &Path, paths: &[&str]) -> Value {
    json!({ "id": id, "type": "file-list",
            "parameters": { "base": root.to_str().unwrap(), "paths": paths } })
}

/// A file-render with no items: one file holding just `text`.
fn render(id: &str, filename: &str, text: &str) -> Value {
    json!({ "id": id, "type": "file-render",
            "parameters": { "filename": filename, "header": text, "row": "-" } })
}

fn notes_and_listing() -> TempDir {
    let root = TempDir::new().unwrap();
    std::fs::write(root.path().join("notes.md"), "# Notes\n").unwrap();
    std::fs::write(root.path().join("listing.md"), "- a.wav\n").unwrap();
    root
}

fn run(json: &str) -> PipelineResult {
    let ctx = TempCtx(TempDir::new().unwrap());
    let result = run_pipeline(json, vec![], &PipelineReporter::new_noop(), &ctx);
    // Read every file before the context's temp folder goes.
    let mut result = result.expect("pipeline should succeed");
    for f in &mut result.files {
        f.data = FileData::Bytes(f.data.to_bytes().unwrap().into_owned());
    }
    result
}

fn text(result: &PipelineResult, name: &str) -> String {
    let file = result.files.iter().find(|f| f.name == name).unwrap();
    String::from_utf8(file.data.to_bytes().unwrap().into_owned()).unwrap()
}

#[test]
fn test_a_file_list_with_no_incoming_edge_lists_only_its_own_paths() {
    let root = notes_and_listing();
    // `emit` writes a path list naming listing.md; `fresh` must not read it.
    let json = pipeline(
        vec![
            render("emit", "paths.txt", "listing.md\n"),
            list("fresh", root.path(), &["notes.md"]),
        ],
        &[("input", "emit"), ("emit", "output"), ("fresh", "output")],
    );
    let result = run(&json);
    let names: Vec<&str> = result.files.iter().map(|f| f.name.as_str()).collect();
    assert_eq!(names, vec!["paths.txt", "notes.md"]);
}

#[test]
fn test_file_join_joins_its_fan_in_in_edge_order() {
    let root = notes_and_listing();
    let join = json!({ "id": "join", "type": "file-join",
                       "parameters": { "filename": "README.md", "separator": "\n" } });
    // `listing` comes first in `nodes`, but the notes edge comes first.
    let json = pipeline(
        vec![
            // file-render ends its header with a newline of its own.
            render("listing", "listing.md", "- a.wav\n- b.wav"),
            list("notes", root.path(), &["notes.md"]),
            join,
        ],
        &[
            ("input", "listing"),
            ("notes", "join"),
            ("listing", "join"),
            ("join", "output"),
        ],
    );
    let result = run(&json);
    assert_eq!(result.files.len(), 1);
    assert_eq!(text(&result, "README.md"), "# Notes\n\n- a.wav\n- b.wav\n");
}
