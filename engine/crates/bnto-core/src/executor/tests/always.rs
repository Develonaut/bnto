// Closing steps: nodes marked `always` run after an earlier step failed,
// on the files the failed step handed over, and the run keeps the first
// failure as its verdict.
use std::sync::{Arc, Mutex};

use super::*;

/// What a capture node received: each file's name and bytes, per call.
pub(super) type Seen = Arc<Mutex<Vec<(String, Vec<u8>)>>>;

/// Records every file it receives and passes it on unchanged.
struct CaptureProcessor(Seen);

impl crate::processor::NodeProcessor for CaptureProcessor {
    fn name(&self) -> &str {
        "capture"
    }

    fn process(
        &self,
        input: NodeInput,
        _progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let bytes = input.data.into_bytes().unwrap_or_default();
        let entry = (input.filename.clone(), bytes.clone());
        self.0.lock().unwrap().push(entry);
        Ok(NodeOutput {
            files: vec![OutputFile {
                data: FileData::Bytes(bytes),
                filename: input.filename,
                mime_type: "text/plain".to_string(),
                metadata: serde_json::Map::new(),
            }],
            metadata: serde_json::Map::new(),
        })
    }
}

/// A source step that fails but hands over the log it wrote, the way a
/// command that exits non-zero hands over its stdout.
struct FailWithLogProcessor;

impl crate::processor::NodeProcessor for FailWithLogProcessor {
    fn name(&self) -> &str {
        "fail-with-log"
    }

    fn metadata(&self) -> crate::metadata::NodeMetadata {
        crate::metadata::NodeMetadata {
            input_cardinality: crate::metadata::InputCardinality::Source,
            ..crate::processor::NodeProcessor::metadata(&EchoProcessor)
        }
    }

    fn process(
        &self,
        _input: NodeInput,
        _progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let log = OutputFile {
            data: FileData::Bytes(b"step 1 ok\nstep 2 broke\n".to_vec()),
            filename: "tool.log".to_string(),
            mime_type: "text/plain".to_string(),
            metadata: serde_json::Map::new(),
        };
        let output = NodeOutput {
            files: vec![log],
            metadata: serde_json::Map::new(),
        };
        let error = BntoError::ProcessingFailed("tool exited 2".to_string());
        Err(BntoError::with_output(error, output))
    }
}

/// The mock registry plus `test-capture` (recording into the returned
/// list) and `test-fail-with-log`.
pub(super) fn always_registry() -> (NodeRegistry, Seen) {
    let seen = Seen::default();
    let mut registry = mock_registry();
    registry.register("test-capture", Box::new(CaptureProcessor(seen.clone())));
    registry.register("test-fail-with-log", Box::new(FailWithLogProcessor));
    (registry, seen)
}

/// Run `json` on one `input.txt` file, returning the result and events.
pub(super) fn run(
    json: &str,
    registry: &NodeRegistry,
) -> (Result<PipelineResult, BntoError>, Vec<PipelineEvent>) {
    let recorder = RecordingReporter::new();
    let reporter = recorder.reporter();
    let files = vec![make_file("input.txt", b"hello")];
    let def = parse_def(json);
    let result = execute_pipeline(&def, files, registry, &reporter, &NoopContext, fake_now);
    (result, recorder.events())
}

/// Ids of the nodes that started, in order.
pub(super) fn started(events: &[PipelineEvent]) -> Vec<String> {
    events
        .iter()
        .filter_map(|e| match e {
            PipelineEvent::NodeStarted { node_id, .. } => Some(node_id.clone()),
            _ => None,
        })
        .collect()
}

/// Names of the files the capture node saw.
pub(super) fn seen_names(seen: &Seen) -> Vec<String> {
    seen.lock()
        .unwrap()
        .iter()
        .map(|(n, _)| n.clone())
        .collect()
}

// --- Top-level chains ---

#[test]
fn test_always_step_runs_after_a_failure_and_the_first_error_is_the_verdict() {
    let (registry, seen) = always_registry();
    let (result, events) = run(
        r#"{ "nodes": [
            { "id": "in", "type": "input" },
            { "id": "build", "type": "test-fail" },
            { "id": "keep", "type": "test-capture", "always": true },
            { "id": "out", "type": "output" }
        ] }"#,
        &registry,
    );
    let err = result.expect_err("the run still fails");
    assert_eq!(
        err.to_string(),
        "Processing failed: intentional test failure"
    );
    assert_eq!(started(&events), vec!["build", "keep"], "keep ran once");
    assert!(
        seen.lock().unwrap().is_empty(),
        "build handed over no files"
    );
}

#[test]
fn test_failed_step_hands_its_output_files_to_the_closing_step() {
    let (registry, seen) = always_registry();
    let (result, _) = run(
        r#"{ "nodes": [
            { "id": "build", "type": "test-fail-with-log" },
            { "id": "keep", "type": "test-capture", "always": true }
        ] }"#,
        &registry,
    );
    assert_eq!(
        result.unwrap_err().to_string(),
        "Processing failed: tool exited 2"
    );
    let seen = seen.lock().unwrap();
    assert_eq!(seen.len(), 1);
    assert_eq!(seen[0].0, "tool.log");
    assert_eq!(seen[0].1, b"step 1 ok\nstep 2 broke\n");
}

#[test]
fn test_failed_step_without_output_hands_over_no_files() {
    let (registry, seen) = always_registry();
    let (result, events) = run(
        r#"{ "nodes": [
            { "id": "build", "type": "test-fail" },
            { "id": "keep", "type": "test-capture", "always": true }
        ] }"#,
        &registry,
    );
    assert!(result.is_err());
    assert!(seen.lock().unwrap().is_empty(), "no files to capture");
    assert_eq!(started(&events), vec!["build", "keep"], "it still ran");
}

#[test]
fn test_non_always_step_after_a_failure_is_skipped() {
    let (registry, seen) = always_registry();
    let (result, events) = run(
        r#"{ "nodes": [
            { "id": "build", "type": "test-fail-with-log" },
            { "id": "publish", "type": "test-capture" },
            { "id": "keep", "type": "test-uppercase", "always": true }
        ] }"#,
        &registry,
    );
    assert!(result.is_err());
    assert_eq!(started(&events), vec!["build", "keep"]);
    assert!(seen.lock().unwrap().is_empty(), "publish never ran");
    let skipped_reported = events.iter().any(|e| match e {
        PipelineEvent::NodeCompleted { node_id, .. }
        | PipelineEvent::NodeFailed { node_id, .. } => node_id == "publish",
        _ => false,
    });
    assert!(!skipped_reported, "a skipped node is not reported as run");
}

#[test]
fn test_passing_run_runs_an_always_step_exactly_once() {
    let (registry, seen) = always_registry();
    let (result, events) = run(
        r#"{ "nodes": [
            { "id": "in", "type": "input" },
            { "id": "first", "type": "test-echo" },
            { "id": "keep", "type": "test-capture", "always": true },
            { "id": "last", "type": "test-uppercase" },
            { "id": "out", "type": "output" }
        ] }"#,
        &registry,
    );
    let result = result.expect("the run passes");
    assert_eq!(result.files[0].name, "INPUT.TXT", "it chains like any node");
    assert_eq!(started(&events), vec!["first", "keep", "last"]);
    assert_eq!(seen_names(&seen), vec!["input.txt"]);
}

#[test]
fn test_failing_closing_step_is_named_after_the_first_failure() {
    let (registry, _) = always_registry();
    let (result, _) = run(
        r#"{ "nodes": [
            { "id": "build", "type": "test-fail-with-log" },
            { "id": "cleanup", "type": "test-fail", "always": true }
        ] }"#,
        &registry,
    );
    assert_eq!(
        result.unwrap_err().to_string(),
        "Processing failed: tool exited 2; closing step 'cleanup' also failed: \
         Processing failed: intentional test failure"
    );
}

#[test]
fn test_closing_steps_chain_and_keep_running_after_one_fails() {
    let (registry, seen) = always_registry();
    let (result, events) = run(
        r#"{ "nodes": [
            { "id": "build", "type": "test-fail-with-log" },
            { "id": "shout", "type": "test-uppercase", "always": true },
            { "id": "keep", "type": "test-capture", "always": true },
            { "id": "cleanup", "type": "test-fail", "always": true },
            { "id": "after", "type": "test-capture", "always": true }
        ] }"#,
        &registry,
    );
    assert_eq!(
        started(&events),
        vec!["build", "shout", "keep", "cleanup", "after"]
    );
    assert_eq!(
        seen_names(&seen),
        vec!["TOOL.LOG"],
        "keep saw shout's output"
    );
    let err = result.unwrap_err().to_string();
    assert!(
        err.starts_with("Processing failed: tool exited 2; closing step 'cleanup'"),
        "{err}"
    );
}

#[test]
fn test_pipeline_failed_is_emitted_once_after_the_closing_steps() {
    let (registry, _) = always_registry();
    let (_, events) = run(
        r#"{ "nodes": [
            { "id": "build", "type": "test-fail" },
            { "id": "keep", "type": "test-capture", "always": true }
        ] }"#,
        &registry,
    );
    let failed: Vec<_> = events
        .iter()
        .filter_map(|e| match e {
            PipelineEvent::PipelineFailed { node_id, error } => {
                Some((node_id.clone(), error.clone()))
            }
            _ => None,
        })
        .collect();
    assert_eq!(
        failed,
        vec![(
            "build".to_string(),
            "Processing failed: intentional test failure".to_string()
        )]
    );
    assert!(
        matches!(events.last(), Some(PipelineEvent::PipelineFailed { .. })),
        "the verdict comes last"
    );
}

// --- Containers ---

#[test]
fn test_group_children_follow_the_same_rule_and_hand_files_out() {
    let (registry, seen) = always_registry();
    let (result, events) = run(
        r#"{ "nodes": [
            { "id": "g", "type": "group", "children": [
                { "id": "build", "type": "test-fail-with-log" },
                { "id": "skipped", "type": "test-echo" },
                { "id": "shout", "type": "test-uppercase", "always": true }
            ] },
            { "id": "publish", "type": "test-echo" },
            { "id": "keep", "type": "test-capture", "always": true }
        ] }"#,
        &registry,
    );
    assert_eq!(
        result.unwrap_err().to_string(),
        "Processing failed: tool exited 2"
    );
    assert_eq!(started(&events), vec!["g", "build", "shout", "keep"]);
    assert_eq!(
        seen_names(&seen),
        vec!["TOOL.LOG"],
        "the group's last files"
    );
}

#[test]
fn test_always_container_runs_as_a_closing_step() {
    let (registry, seen) = always_registry();
    let (result, events) = run(
        r#"{ "nodes": [
            { "id": "build", "type": "test-fail-with-log" },
            { "id": "g", "type": "group", "always": true, "children": [
                { "id": "keep", "type": "test-capture" }
            ] }
        ] }"#,
        &registry,
    );
    assert!(result.is_err());
    assert_eq!(started(&events), vec!["build", "g", "keep"]);
    assert_eq!(seen_names(&seen), vec!["tool.log"]);
}

#[test]
fn test_loop_iteration_runs_its_closing_step_and_continue_still_applies() {
    let (registry, seen) = always_registry();
    let def = parse_def(
        r#"{ "nodes": [
            { "id": "each", "type": "loop", "params": { "onError": "continue" }, "children": [
                { "id": "build", "type": "test-fail-with-log" },
                { "id": "keep", "type": "test-capture", "always": true }
            ] }
        ] }"#,
    );
    let files = vec![make_file("a.txt", b"a"), make_file("b.txt", b"b")];
    let reporter = PipelineReporter::new_noop();
    let result = execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now);
    assert!(result.is_err(), "every iteration failed");
    assert_eq!(seen.lock().unwrap().len(), 2, "each iteration closed");
}

// --- Auto-iteration ---

#[test]
fn test_auto_iteration_runs_closing_steps_after_a_failed_source() {
    let (registry, seen) = always_registry();
    let (result, events) = run(
        r#"{ "settings": { "iteration": "auto" }, "nodes": [
            { "id": "in", "type": "input" },
            { "id": "build", "type": "test-fail-with-log" },
            { "id": "publish", "type": "test-echo" },
            { "id": "keep", "type": "test-capture", "always": true },
            { "id": "out", "type": "output" }
        ] }"#,
        &registry,
    );
    assert_eq!(
        result.unwrap_err().to_string(),
        "Processing failed: tool exited 2"
    );
    assert_eq!(started(&events), vec!["build", "keep"]);
    assert_eq!(seen_names(&seen), vec!["tool.log"]);
}

#[test]
fn test_auto_iteration_closes_past_a_batch_barrier() {
    let (registry, seen) = always_registry();
    let (result, events) = run(
        r#"{ "settings": { "iteration": "auto" }, "nodes": [
            { "id": "build", "type": "test-fail" },
            { "id": "shout", "type": "test-uppercase", "always": true },
            { "id": "merge", "type": "test-batch" },
            { "id": "keep", "type": "test-capture", "always": true }
        ] }"#,
        &registry,
    );
    assert!(result.is_err());
    assert_eq!(started(&events), vec!["build", "shout", "keep"]);
    assert!(
        seen.lock().unwrap().is_empty(),
        "build handed over no files"
    );
}
