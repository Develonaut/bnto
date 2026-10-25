use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use super::*;
use crate::processor::BatchInput;

// =========================================================================
// What a batch node sees of the run (ISS-59): the files a later step still
// reads outside its own (`held_elsewhere`) and the writes made so far.
// =========================================================================

/// One list per call: what the node was told.
type Log<T> = Arc<Mutex<Vec<T>>>;

/// A batch node that asks `held_elsewhere` about each of `paths`, logs the
/// answers, then records a write onto each file it was handed and logs the
/// writes it saw before its own. Passes its files on unchanged.
struct RunProbe {
    paths: Vec<PathBuf>,
    held: Log<Vec<Option<String>>>,
    writes: Log<Vec<PathBuf>>,
}

impl crate::processor::NodeProcessor for RunProbe {
    fn name(&self) -> &str {
        "run-probe"
    }

    fn metadata(&self) -> crate::metadata::NodeMetadata {
        crate::metadata::NodeMetadata {
            input_cardinality: crate::metadata::InputCardinality::Batch,
            ..crate::processor::NodeProcessor::metadata(&EchoProcessor)
        }
    }

    fn process(
        &self,
        _input: NodeInput,
        _progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        unreachable!("run-probe only runs as a batch")
    }

    fn process_batch(
        &self,
        input: BatchInput,
        _progress: &ProgressReporter,
        ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let answers = self.paths.iter().map(|p| ctx.held_elsewhere(p)).collect();
        self.held.lock().unwrap().push(answers);
        self.writes.lock().unwrap().push(ctx.recorded_writes());
        let files = input
            .files
            .into_iter()
            .map(|f| {
                if let FileData::Path(path) = &f.data {
                    ctx.record_write(path);
                }
                OutputFile {
                    data: f.data,
                    filename: f.filename,
                    mime_type: "text/plain".to_string(),
                    metadata: f.metadata,
                }
            })
            .collect();
        Ok(NodeOutput {
            files,
            metadata: serde_json::Map::new(),
        })
    }
}

/// `a.txt`, `b.txt` and `c.txt` in a fresh folder.
fn three_files() -> (tempfile::TempDir, Vec<PathBuf>) {
    let dir = tempfile::tempdir().unwrap();
    let paths: Vec<PathBuf> = ["a.txt", "b.txt", "c.txt"]
        .iter()
        .map(|name| dir.path().join(name))
        .collect();
    for (path, body) in paths.iter().zip(["AAAA", "BBBB", "CCCC"]) {
        std::fs::write(path, body).unwrap();
    }
    (dir, paths)
}

/// A run input that is a reference to the file at `path`.
fn on_disk(path: &Path) -> PipelineFile {
    PipelineFile {
        name: path.file_name().unwrap().to_string_lossy().into_owned(),
        data: FileData::Path(path.to_path_buf()),
        mime_type: "text/plain".to_string(),
        metadata: serde_json::Map::new(),
    }
}

/// What the probe saw in each call, running `json` on `paths` as inputs
/// and asking about every one of them.
fn probe(json: &str, paths: &[PathBuf]) -> (Vec<Vec<Option<String>>>, Vec<Vec<PathBuf>>) {
    let (held, writes) = (Log::default(), Log::default());
    let mut registry = mock_registry();
    let probe = RunProbe {
        paths: paths.to_vec(),
        held: Arc::clone(&held),
        writes: Arc::clone(&writes),
    };
    registry.register("run-probe", Box::new(probe));
    let files = paths.iter().map(|p| on_disk(p)).collect();
    let reporter = PipelineReporter::new_noop();
    execute_pipeline(
        &parse_def(json),
        files,
        &registry,
        &reporter,
        &NoopContext,
        fake_now,
    )
    .unwrap();
    let held = held.lock().unwrap().clone();
    let writes = writes.lock().unwrap().clone();
    (held, writes)
}

fn named(names: &[Option<&str>]) -> Vec<Option<String>> {
    names.iter().map(|n| n.map(str::to_string)).collect()
}

const LOOP: &str = r#"{
    "nodes": [
        { "id": "in", "type": "input" },
        { "id": "each", "type": "loop", "children": [ CHILDREN ] },
        { "id": "out", "type": "output" }
    ]
}"#;

fn in_loop(children: &str) -> String {
    LOOP.replace("CHILDREN", children)
}

const PROBE: &str = r#"{ "id": "p", "type": "run-probe" }"#;

#[test]
fn test_a_loop_iteration_sees_every_other_iterations_file_held() {
    // RED (ISS-59): a later iteration's file, and a finished one's (its
    // output still reads from it), are held; the iteration's own isn't.
    let (_dir, paths) = three_files();
    let (held, _) = probe(&in_loop(PROBE), &paths);
    assert_eq!(
        held,
        [
            named(&[None, Some("b.txt"), Some("c.txt")]),
            named(&[Some("a.txt"), None, Some("c.txt")]),
            named(&[Some("a.txt"), Some("b.txt"), None]),
        ]
    );
}

#[test]
fn test_a_finished_iteration_holds_its_file_under_its_output_name() {
    let (_dir, paths) = three_files();
    let children = format!(r#"{{ "id": "up", "type": "test-uppercase" }}, {PROBE}"#);
    let (held, _) = probe(&in_loop(&children), &paths[..2]);
    assert_eq!(held[1], named(&[Some("A.TXT"), None]));
}

#[test]
fn test_a_nested_loop_sees_the_outer_loops_other_files() {
    let (_dir, paths) = three_files();
    let inner = format!(r#"{{ "id": "inner", "type": "loop", "children": [ {PROBE} ] }}"#);
    let (held, _) = probe(&in_loop(&inner), &paths);
    assert_eq!(held[0], named(&[None, Some("b.txt"), Some("c.txt")]));
}

#[test]
fn test_a_flat_batch_node_sees_nothing_held() {
    // Guard: the node gets every file its level holds.
    let (_dir, paths) = three_files();
    let json = r#"{ "nodes": [
        { "id": "in", "type": "input" },
        { "id": "p", "type": "run-probe" },
        { "id": "out", "type": "output" }
    ] }"#;
    let (held, _) = probe(json, &paths);
    assert_eq!(held, [named(&[None, None, None])]);
}

#[test]
fn test_a_branch_sees_the_files_held_for_a_later_consumer() {
    // RED (ISS-59): `input` feeds the probe and the output; the output's
    // share is still to be read when the probe runs.
    let (_dir, paths) = three_files();
    let json = r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "p", "type": "run-probe" },
            { "id": "out", "type": "output" }
        ],
        "edges": [
            { "id": "e1", "source": "in", "target": "p" },
            { "id": "e2", "source": "in", "target": "out" }
        ]
    }"#;
    let (held, _) = probe(json, &paths[..2]);
    assert_eq!(held, [named(&[Some("a.txt"), Some("b.txt")])]);
}

#[test]
fn test_writes_an_earlier_iteration_recorded_are_seen_by_later_ones() {
    // RED: a refusal in a later iteration can name what earlier ones wrote.
    let (_dir, paths) = three_files();
    let (_, writes) = probe(&in_loop(PROBE), &paths[..2]);
    assert_eq!(writes, [vec![], vec![paths[0].clone()]]);
}
