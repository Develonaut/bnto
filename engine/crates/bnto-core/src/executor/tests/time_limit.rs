// The run's time limit (`settings.timeout`): once it has passed, no step,
// file, loop iteration or closing step starts, and the run fails naming
// the limit. A run with no limit is never stopped.
use std::cell::Cell;

use super::always::{Seen, always_registry, seen_names, started};
use super::*;

thread_local! {
    /// The test's clock, which the tick processor moves forward. One per
    /// thread, so parallel tests keep their own; the executor runs every
    /// processor on its caller's thread.
    static CLOCK: Cell<u64> = const { Cell::new(0) };
}

/// The test clock, as `now_ms` (a plain `fn`, so it is `'static`).
fn now() -> u64 {
    CLOCK.with(Cell::get)
}

/// Moves the clock on by its step for every file it processes, standing in
/// for a step that takes that long.
struct TickProcessor {
    step_ms: u64,
}

impl crate::processor::NodeProcessor for TickProcessor {
    fn name(&self) -> &str {
        "tick"
    }

    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
        ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        CLOCK.with(|clock| clock.set(clock.get() + self.step_ms));
        EchoProcessor.process(input, progress, ctx)
    }
}

/// The closing-step registry plus `test-tick` (1.5s per file) and
/// `test-tick-long` (3s per file), both moving the clock.
fn tick_registry() -> (NodeRegistry, Seen) {
    let (mut registry, seen) = always_registry();
    for (key, step_ms) in [("test-tick", 1500), ("test-tick-long", 3000)] {
        registry.register(key, Box::new(TickProcessor { step_ms }));
    }
    (registry, seen)
}

/// Run `json` on `files` with the clock reset to 0.
fn run_timed(
    json: &str,
    files: Vec<PipelineFile>,
) -> (Result<PipelineResult, BntoError>, Vec<PipelineEvent>, Seen) {
    CLOCK.with(|clock| clock.set(0));
    let (registry, seen) = tick_registry();
    let recorder = RecordingReporter::new();
    let reporter = recorder.reporter();
    let def = parse_def(json);
    let result = execute_pipeline(&def, files, &registry, &reporter, &NoopContext, now);
    (result, recorder.events(), seen)
}

fn one_file() -> Vec<PipelineFile> {
    vec![make_file("input.txt", b"hello")]
}

#[test]
fn test_no_step_starts_once_the_time_limit_has_passed() {
    let (result, events, seen) = run_timed(
        r#"{ "settings": { "timeout": 2 }, "nodes": [
            { "id": "first", "type": "test-tick" },
            { "id": "second", "type": "test-tick" },
            { "id": "third", "type": "test-capture" }
        ] }"#,
        one_file(),
    );
    let err = result.expect_err("the run passed its limit");
    assert_eq!(err.to_string(), "Run timed out after 2s");
    assert_eq!(started(&events), vec!["first", "second"]);
    assert!(seen_names(&seen).is_empty(), "third never ran");
    assert!(
        events
            .iter()
            .any(|e| matches!(e, PipelineEvent::PipelineFailed { .. })),
        "the run reports its failure"
    );
}

#[test]
fn test_a_run_without_a_limit_is_never_stopped() {
    let (result, events, seen) = run_timed(
        r#"{ "nodes": [
            { "id": "first", "type": "test-tick-long" },
            { "id": "second", "type": "test-tick-long" },
            { "id": "third", "type": "test-capture" }
        ] }"#,
        one_file(),
    );
    assert!(result.is_ok(), "{:?}", result.err());
    assert_eq!(started(&events), vec!["first", "second", "third"]);
    assert_eq!(seen_names(&seen), vec!["input.txt"]);
}

#[test]
fn test_a_run_within_its_limit_passes() {
    let (result, _, seen) = run_timed(
        r#"{ "settings": { "timeout": 2 }, "nodes": [
            { "id": "first", "type": "test-tick" },
            { "id": "last", "type": "test-capture" }
        ] }"#,
        one_file(),
    );
    assert!(result.is_ok(), "{:?}", result.err());
    assert_eq!(seen_names(&seen), vec!["input.txt"]);
}

#[test]
fn test_a_run_whose_last_step_ends_past_the_limit_fails() {
    let (result, _, _) = run_timed(
        r#"{ "settings": { "timeout": 2 }, "nodes": [
            { "id": "only", "type": "test-tick-long" }
        ] }"#,
        one_file(),
    );
    let err = result.expect_err("it ended past its limit");
    assert_eq!(err.to_string(), "Run timed out after 2s");
}

#[test]
fn test_no_file_starts_once_the_time_limit_has_passed() {
    let files = vec![
        make_file("a.txt", b"a"),
        make_file("b.txt", b"b"),
        make_file("c.txt", b"c"),
    ];
    let (result, events, _) = run_timed(
        r#"{ "settings": { "timeout": 2 }, "nodes": [
            { "id": "each", "type": "test-tick" }
        ] }"#,
        files,
    );
    let err = result.expect_err("the run passed its limit");
    assert_eq!(err.to_string(), "Run timed out after 2s");
    let progressed: Vec<_> = events
        .iter()
        .filter(|e| matches!(e, PipelineEvent::FileProgress { .. }))
        .collect();
    assert!(
        !progressed.is_empty() && progressed.len() < 3 * 2,
        "c.txt never started: {progressed:?}"
    );
}

#[test]
fn test_no_closing_step_starts_once_the_time_limit_has_passed() {
    let (result, events, seen) = run_timed(
        r#"{ "settings": { "timeout": 2 }, "nodes": [
            { "id": "slow", "type": "test-tick-long" },
            { "id": "next", "type": "test-echo" },
            { "id": "cleanup", "type": "test-capture", "always": true }
        ] }"#,
        one_file(),
    );
    let err = result.expect_err("the run passed its limit");
    assert_eq!(err.to_string(), "Run timed out after 2s");
    assert_eq!(started(&events), vec!["slow"]);
    assert!(seen_names(&seen).is_empty(), "cleanup never ran");
}

#[test]
fn test_no_loop_iteration_starts_once_the_time_limit_has_passed() {
    let files = vec![
        make_file("a.txt", b"a"),
        make_file("b.txt", b"b"),
        make_file("c.txt", b"c"),
    ];
    let (result, _, seen) = run_timed(
        r#"{ "settings": { "timeout": 2 }, "nodes": [
            { "id": "each", "type": "loop", "nodes": [
                { "id": "tick", "type": "test-tick" },
                { "id": "keep", "type": "test-capture" }
            ] }
        ] }"#,
        files,
    );
    let err = result.expect_err("the run passed its limit");
    assert!(
        err.to_string().contains("Run timed out after 2s"),
        "error: {err}"
    );
    assert_eq!(seen_names(&seen), vec!["a.txt"], "b.txt's keep never ran");
}

#[test]
fn test_a_loop_that_continues_on_error_still_stops_at_the_time_limit() {
    let files = vec![
        make_file("a.txt", b"a"),
        make_file("b.txt", b"b"),
        make_file("c.txt", b"c"),
    ];
    let (result, _, seen) = run_timed(
        r#"{ "settings": { "timeout": 2 }, "nodes": [
            { "id": "each", "type": "loop", "parameters": { "onError": "continue" }, "nodes": [
                { "id": "tick", "type": "test-tick" },
                { "id": "keep", "type": "test-capture" }
            ] }
        ] }"#,
        files,
    );
    let err = result.expect_err("a timeout is never skipped like a failed item");
    assert!(
        err.to_string().contains("Run timed out after 2s"),
        "error: {err}"
    );
    assert_eq!(seen_names(&seen), vec!["a.txt"]);
}
