// Edge equivalence: every bundled recipe and test fixture wires its nodes in
// a linear chain in `nodes` order, so honoring its edges must change nothing.
// Each definition runs twice, with its edges and with every level's edges
// stripped, and the two runs must emit the same events and the same output.
#![cfg(feature = "native")]

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use bnto_core::processor::FileData;
use bnto_core::{
    BntoError, PipelineDefinition, PipelineFile, PipelineReporter, PipelineResult, ProcessContext,
    execute_pipeline,
};
use bnto_engine::{create_registry, recipes::builtin_recipes};
use serde_json::Value;
use tempfile::TempDir;

/// Params naming a place on disk; relative ones are moved into the sandbox.
const PATH_PARAMS: &[&str] = &["base", "destination", "path", "table"];

/// Files the fixtures list, move, copy or delete under their `Root` var.
const SANDBOX_FILES: &[&str] = &[
    "a.txt",
    "b.txt",
    "c.txt",
    "sub/b.txt",
    "docs/drafts/draft.md",
    "a.log",
    "keep.txt",
    "logs/b.log",
    "notes.md",
    "listing.md",
    "top.txt",
    "shots/mid.txt",
    "shots/day1/deep.txt",
    "clip.bin",
    "leftover.txt",
];

/// A context that runs no commands and keeps everything in one folder, with
/// temp files numbered so two runs name theirs the same.
struct Sandbox {
    root: PathBuf,
    temps: AtomicUsize,
}

impl ProcessContext for Sandbox {
    fn run_command(&self, cmd: &str, _args: &[&str]) -> Result<Vec<u8>, BntoError> {
        Err(BntoError::ProcessingFailed(format!(
            "no commands here: {cmd}"
        )))
    }
    fn temp_file(&self, suffix: &str) -> Result<PathBuf, BntoError> {
        let n = self.temps.fetch_add(1, Ordering::SeqCst);
        let path = self.root.join("tmp").join(format!("t{n}{suffix}"));
        std::fs::write(&path, b"").unwrap();
        Ok(path)
    }
    fn env_var(&self, _key: &str) -> Option<String> {
        None
    }
    fn work_dir(&self) -> Result<&Path, BntoError> {
        Ok(&self.root)
    }
}

impl Sandbox {
    /// Put the sandbox back to its starting files.
    fn reset(&self) {
        let _ = std::fs::remove_dir_all(&self.root);
        std::fs::create_dir_all(self.root.join("tmp")).unwrap();
        for name in SANDBOX_FILES {
            let path = self.root.join(name);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(&path, format!("contents of {name}\n")).unwrap();
            pin_mtime(&path);
        }
        self.temps.store(0, Ordering::SeqCst);
    }
}

/// Give a file the same modified time in every run (file-inspect reports it).
fn pin_mtime(path: &Path) {
    let time = filetime::FileTime::from_unix_time(1_700_000_000, 0);
    filetime::set_file_mtime(path, time).unwrap();
}

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../bnto/tests/fixtures")
}

fn test_asset(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../test-fixtures")
        .join(rel)
}

/// Every `.bnto.json` under `dir`, recursively, as (name, JSON).
fn fixtures_in(dir: &Path, out: &mut Vec<(String, String)>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            fixtures_in(&path, out);
        } else if path.to_string_lossy().ends_with(".bnto.json") {
            let json = std::fs::read_to_string(&path).unwrap();
            out.push((path.display().to_string(), json));
        }
    }
}

/// Every bundled recipe and every CLI test fixture.
fn definitions() -> Vec<(String, String)> {
    let mut all: Vec<(String, String)> = builtin_recipes()
        .into_iter()
        .map(|r| (r.slug, r.definition_json.to_string()))
        .collect();
    fixtures_in(&fixture_dir(), &mut all);
    all
}

/// Remove `edges` from a node and every container level under it.
fn strip_edges(value: &mut Value) {
    let Some(object) = value.as_object_mut() else {
        return;
    };
    object.remove("edges");
    if let Some(Value::Array(nodes)) = object.get_mut("nodes") {
        nodes.iter_mut().for_each(strip_edges);
    }
}

/// Whether every level's edges are absent or exactly the chain its nodes
/// already form, the definitions this test's claim is about. A fixture that
/// branches, or is broken on purpose for `bnto check`, is skipped.
fn edges_follow_node_order(level: &Value) -> bool {
    let Some(nodes) = level["nodes"].as_array().or(level["children"].as_array()) else {
        return true;
    };
    let ids: Vec<&str> = nodes.iter().filter_map(|n| n["id"].as_str()).collect();
    let chain: Vec<(&str, &str)> = ids.windows(2).map(|w| (w[0], w[1])).collect();
    let edges: Vec<(&str, &str)> = level["edges"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|e| {
            (
                e["source"].as_str().unwrap_or(""),
                e["target"].as_str().unwrap_or(""),
            )
        })
        .collect();
    (edges.is_empty() || edges == chain) && nodes.iter().all(edges_follow_node_order)
}

fn has_edges(value: &Value) -> bool {
    let own = value["edges"].as_array().is_some_and(|e| !e.is_empty());
    let nested = value["nodes"]
        .as_array()
        .is_some_and(|nodes| nodes.iter().any(has_edges));
    own || nested
}

/// Point every var and relative path param into the sandbox. `None` for a
/// fixture that isn't JSON (a `bnto check` fixture can be broken on purpose).
fn sandboxed(json: &str, root: &Path) -> Option<Value> {
    let mut value: Value = serde_json::from_str(json).ok()?;
    let root_str = root.to_string_lossy().into_owned();
    let _ = bnto_core::vars::apply_vars(&mut value, &|_| Some(root_str.clone()));
    move_paths(&mut value, root);
    Some(value)
}

fn move_paths(value: &mut Value, root: &Path) {
    match value {
        Value::Object(map) => {
            for (key, v) in map.iter_mut() {
                match v {
                    Value::String(s) if PATH_PARAMS.contains(&key.as_str()) => {
                        if Path::new(s.as_str()).is_relative() {
                            *s = root.join(s.as_str()).to_string_lossy().into_owned();
                        }
                    }
                    _ => move_paths(v, root),
                }
            }
        }
        Value::Array(items) => items.iter_mut().for_each(|v| move_paths(v, root)),
        _ => {}
    }
}

/// The input files for one run: one test asset copied into the sandbox, or none.
fn inputs(sandbox: &Sandbox, asset: Option<(&str, &str)>) -> Vec<PipelineFile> {
    let Some((rel, mime)) = asset else {
        return vec![];
    };
    let name = Path::new(rel).file_name().unwrap().to_string_lossy();
    let dest = sandbox.root.join("in").join(name.as_ref());
    std::fs::create_dir_all(dest.parent().unwrap()).unwrap();
    std::fs::copy(test_asset(rel), &dest).unwrap();
    pin_mtime(&dest);
    vec![PipelineFile {
        name: name.into_owned(),
        data: FileData::Path(dest),
        mime_type: mime.to_string(),
        metadata: serde_json::Map::new(),
    }]
}

/// One run's events and outcome, as comparable text.
fn run_once(sandbox: &Sandbox, def: &Value, asset: Option<(&str, &str)>) -> (Vec<String>, String) {
    sandbox.reset();
    let files = inputs(sandbox, asset);
    let events = Arc::new(Mutex::new(Vec::new()));
    let sink = events.clone();
    let reporter = PipelineReporter::new(move |e| sink.lock().unwrap().push(format!("{e:?}")));
    let outcome = match serde_json::from_value::<PipelineDefinition>(def.clone()) {
        Ok(def) => {
            let registry = create_registry();
            let result = execute_pipeline(&def, files, &registry, &reporter, sandbox, || 1000);
            describe(result)
        }
        Err(e) => format!("parse error: {e}"),
    };
    let events = events.lock().unwrap().clone();
    (events, outcome)
}

fn describe(result: Result<PipelineResult, BntoError>) -> String {
    match result {
        Ok(r) => {
            let files: Vec<String> = r
                .files
                .iter()
                .map(|f| {
                    let bytes = f.data.to_bytes().map(|b| b.into_owned());
                    format!(
                        "{} {} {:?} {:?}",
                        f.name,
                        f.mime_type,
                        f.metadata,
                        bytes.ok()
                    )
                })
                .collect();
            format!("ok {files:?} {:?} {:?}", r.node_metadata, r.warnings)
        }
        Err(e) => format!("error {e}"),
    }
}

/// Run every definition with and without its edges on `asset`; returns how
/// many definitions had edges and how many of their runs succeeded.
fn assert_equivalent_on(asset: Option<(&str, &str)>) -> (usize, usize) {
    let tmp = TempDir::new().unwrap();
    let sandbox = Sandbox {
        root: tmp.path().join("box"),
        temps: AtomicUsize::new(0),
    };
    let (mut with_edges, mut succeeded) = (0, 0);
    for (name, json) in definitions() {
        let Some(def) = sandboxed(&json, &sandbox.root) else {
            continue;
        };
        if !edges_follow_node_order(&def) {
            continue;
        }
        let mut stripped = def.clone();
        strip_edges(&mut stripped);
        let edged = run_once(&sandbox, &def, asset);
        let plain = run_once(&sandbox, &stripped, asset);
        assert_eq!(edged.0, plain.0, "{name} on {asset:?}: events differ");
        assert_eq!(edged.1, plain.1, "{name} on {asset:?}: outcome differs");
        with_edges += usize::from(has_edges(&def));
        succeeded += usize::from(has_edges(&def) && edged.1.starts_with("ok"));
    }
    eprintln!("{asset:?}: {with_edges} definitions with edges, {succeeded} succeeded");
    (with_edges, succeeded)
}

#[test]
fn test_edges_change_nothing_with_no_input() {
    let (with_edges, succeeded) = assert_equivalent_on(None);
    assert!(with_edges >= 80, "only {with_edges} definitions have edges");
    assert!(succeeded >= 60, "only {succeeded} runs succeeded");
}

#[test]
fn test_edges_change_nothing_on_a_png() {
    let (_, succeeded) = assert_equivalent_on(Some(("images/small.png", "image/png")));
    assert!(succeeded >= 30, "only {succeeded} runs succeeded");
}

#[test]
fn test_edges_change_nothing_on_a_jpeg() {
    let (_, succeeded) = assert_equivalent_on(Some(("images/small.jpg", "image/jpeg")));
    assert!(succeeded >= 30, "only {succeeded} runs succeeded");
}

#[test]
fn test_edges_change_nothing_on_an_svg() {
    let (_, succeeded) = assert_equivalent_on(Some(("images/small.svg", "image/svg+xml")));
    assert!(succeeded >= 30, "only {succeeded} runs succeeded");
}

#[test]
fn test_edges_change_nothing_on_a_csv() {
    let (_, succeeded) = assert_equivalent_on(Some(("csv/simple.csv", "text/csv")));
    assert!(succeeded >= 20, "only {succeeded} runs succeeded");
}
