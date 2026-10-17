use super::*;
use std::sync::atomic::{AtomicU64, Ordering};

// =========================================================================
// {{ctx.started}} — the run's start, the same for every node of a run.
// =========================================================================

/// A clock that moves forward a second on every read.
fn ticking(clock: &AtomicU64) -> impl Fn() -> u64 + Copy + '_ {
    move || clock.fetch_add(1000, Ordering::SeqCst)
}

fn run_names(def_json: &str, clock: &AtomicU64) -> Vec<String> {
    let mut registry = mock_registry();
    registry.register("test-name", Box::new(NameFromParam));
    let def = parse_def(def_json);
    let reporter = PipelineReporter::new_noop();
    let files = vec![make_file("a.txt", b"a")];
    let result = execute_pipeline(
        &def,
        files,
        &registry,
        &reporter,
        &NoopContext,
        ticking(clock),
    );
    result.unwrap().files.into_iter().map(|f| f.name).collect()
}

/// Names each file `<prefix>|<started>`: its `name` param, as resolved.
struct NameFromParam;

impl crate::processor::NodeProcessor for NameFromParam {
    fn name(&self) -> &str {
        "name-from-param"
    }

    fn process(
        &self,
        input: NodeInput,
        _progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let name = input.params["name"].as_str().unwrap_or_default();
        Ok(NodeOutput {
            files: vec![OutputFile {
                data: input.data,
                filename: format!("{}|{name}", input.filename),
                mime_type: "application/octet-stream".to_string(),
                metadata: serde_json::Map::new(),
            }],
            metadata: serde_json::Map::new(),
        })
    }
}

#[test]
fn test_first_and_last_node_read_the_same_start() {
    let clock = AtomicU64::new(1_700_000_000_000);
    let names = run_names(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "first", "type": "test-name", "parameters": { "name": "{{ctx.started}}" } },
            { "id": "mid", "type": "test-name", "parameters": { "name": "x" } },
            { "id": "last", "type": "test-name", "parameters": { "name": "{{ctx.started}}" } },
            { "id": "out", "type": "output" }
        ]
    }"#,
        &clock,
    );
    let start = "2023-11-14T22:13:20.000Z";
    assert_eq!(names, vec![format!("a.txt|{start}|x|{start}")]);
}

#[test]
fn test_a_container_child_reads_the_run_start() {
    let clock = AtomicU64::new(1_700_000_000_000);
    let names = run_names(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "grp", "type": "group", "nodes": [
                { "id": "child", "type": "test-name", "parameters": { "name": "{{ctx.started}}" } }
            ] },
            { "id": "out", "type": "output" }
        ]
    }"#,
        &clock,
    );
    assert_eq!(names, vec!["a.txt|2023-11-14T22:13:20.000Z".to_string()]);
}
