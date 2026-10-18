// Closing steps on a level with edges: after the first failure every step
// that hasn't run is skipped, branches that don't depend on the failure
// included, and passes its input through; `always` steps still run, in
// execution order, on what their incoming edges carry.
use super::always::{always_registry, run, seen_names, started};
use super::*;

/// The run's `PipelineFailed` events, as (node, error).
fn failed(events: &[PipelineEvent]) -> Vec<(String, String)> {
    events
        .iter()
        .filter_map(|e| match e {
            PipelineEvent::PipelineFailed { node_id, error } => {
                Some((node_id.clone(), error.clone()))
            }
            _ => None,
        })
        .collect()
}

#[test]
fn test_linear_edges_close_exactly_like_a_chain_without_edges() {
    let nodes = r#"
        { "id": "in", "type": "input" },
        { "id": "build", "type": "test-fail-with-log" },
        { "id": "skipped", "type": "test-echo" },
        { "id": "shout", "type": "test-uppercase", "always": true },
        { "id": "keep", "type": "test-capture", "always": true },
        { "id": "out", "type": "output" }"#;
    let edges = r#"
        { "id": "e1", "source": "in", "target": "build" },
        { "id": "e2", "source": "build", "target": "skipped" },
        { "id": "e3", "source": "skipped", "target": "shout" },
        { "id": "e4", "source": "shout", "target": "keep" },
        { "id": "e5", "source": "keep", "target": "out" }"#;
    let (registry, seen) = always_registry();
    let (with, with_events) = run(
        &format!(r#"{{ "nodes": [{nodes}], "edges": [{edges}] }}"#),
        &registry,
    );
    let with_seen = seen_names(&seen);
    let (registry, seen) = always_registry();
    let (without, without_events) = run(&format!(r#"{{ "nodes": [{nodes}] }}"#), &registry);
    assert_eq!(
        with.unwrap_err().to_string(),
        without.unwrap_err().to_string()
    );
    assert_eq!(format!("{with_events:?}"), format!("{without_events:?}"));
    assert_eq!(with_seen, seen_names(&seen));
    assert_eq!(with_seen, vec!["TOOL.LOG"]);
}

#[test]
fn test_a_closing_step_on_a_later_branch_runs_on_what_reaches_it() {
    // `log` fails in its own segment; `keep` (always) sits after the fan-in.
    let (registry, seen) = always_registry();
    let (result, events) = run(
        r#"{ "nodes": [
            { "id": "in", "type": "input" },
            { "id": "log", "type": "test-fail-with-log" },
            { "id": "copy", "type": "test-echo" },
            { "id": "keep", "type": "test-capture", "always": true },
            { "id": "out", "type": "output" }
        ], "edges": [
            { "id": "e1", "source": "in", "target": "copy" },
            { "id": "e2", "source": "log", "target": "keep" },
            { "id": "e3", "source": "copy", "target": "keep" },
            { "id": "e4", "source": "keep", "target": "out" }
        ] }"#,
        &registry,
    );
    assert_eq!(
        result.unwrap_err().to_string(),
        "Processing failed: tool exited 2"
    );
    assert_eq!(started(&events), vec!["log", "keep"], "copy was skipped");
    assert_eq!(
        seen_names(&seen),
        vec!["tool.log", "input.txt"],
        "the failed branch's log, then copy's input passed through, in edge order"
    );
}

#[test]
fn test_a_step_on_an_independent_branch_is_skipped_after_a_failure() {
    let (registry, seen) = always_registry();
    let (result, events) = run(
        r#"{ "nodes": [
            { "id": "in", "type": "input" },
            { "id": "build", "type": "test-fail" },
            { "id": "other", "type": "test-capture" },
            { "id": "out", "type": "output" }
        ], "edges": [
            { "id": "e1", "source": "in", "target": "build" },
            { "id": "e2", "source": "in", "target": "other" },
            { "id": "e3", "source": "build", "target": "out" },
            { "id": "e4", "source": "other", "target": "out" }
        ] }"#,
        &registry,
    );
    assert!(result.is_err());
    assert_eq!(started(&events), vec!["build"]);
    assert!(seen.lock().unwrap().is_empty(), "other never ran");
}

#[test]
fn test_closing_steps_on_branches_join_the_verdict_and_fail_once_last() {
    let (registry, seen) = always_registry();
    let (result, events) = run(
        r#"{ "nodes": [
            { "id": "in", "type": "input" },
            { "id": "build", "type": "test-fail-with-log" },
            { "id": "cleanup", "type": "test-fail", "always": true },
            { "id": "keep", "type": "test-capture", "always": true },
            { "id": "out", "type": "output" }
        ], "edges": [
            { "id": "e1", "source": "in", "target": "build" },
            { "id": "e2", "source": "build", "target": "cleanup" },
            { "id": "e3", "source": "build", "target": "keep" },
            { "id": "e4", "source": "cleanup", "target": "out" },
            { "id": "e5", "source": "keep", "target": "out" }
        ] }"#,
        &registry,
    );
    assert_eq!(
        result.unwrap_err().to_string(),
        "Processing failed: tool exited 2; closing step 'cleanup' also failed: \
         Processing failed: intentional test failure"
    );
    assert_eq!(started(&events), vec!["build", "cleanup", "keep"]);
    assert_eq!(seen_names(&seen), vec!["tool.log"], "keep got build's log");
    assert_eq!(failed(&events).len(), 1);
    assert_eq!(failed(&events)[0].0, "build");
    assert!(
        matches!(events.last(), Some(PipelineEvent::PipelineFailed { .. })),
        "the verdict comes last"
    );
}

#[test]
fn test_a_failed_group_with_edges_hands_its_output_to_the_parent() {
    // Inside `g`, `build` fails and `shout` (always) is one of two sinks;
    // the parent's closing step gets the group's output: its sinks' files.
    let (registry, seen) = always_registry();
    let (result, events) = run(
        r#"{ "nodes": [
            { "id": "g", "type": "group", "children": [
                { "id": "build", "type": "test-fail-with-log" },
                { "id": "side", "type": "test-echo" },
                { "id": "shout", "type": "test-uppercase", "always": true }
            ], "edges": [
                { "id": "c1", "source": "build", "target": "shout" },
                { "id": "c2", "source": "build", "target": "side" }
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
        vec!["tool.log", "TOOL.LOG"],
        "side passed build's log through, then shout's output, in nodes order"
    );
}

#[test]
fn test_auto_iteration_closes_a_later_branch_too() {
    let (registry, seen) = always_registry();
    let (result, events) = run(
        r#"{ "settings": { "iteration": "auto" }, "nodes": [
            { "id": "in", "type": "input" },
            { "id": "log", "type": "test-fail-with-log" },
            { "id": "copy", "type": "test-echo" },
            { "id": "keep", "type": "test-capture", "always": true },
            { "id": "out", "type": "output" }
        ], "edges": [
            { "id": "e1", "source": "in", "target": "copy" },
            { "id": "e2", "source": "log", "target": "keep" },
            { "id": "e3", "source": "copy", "target": "keep" },
            { "id": "e4", "source": "keep", "target": "out" }
        ] }"#,
        &registry,
    );
    assert!(result.is_err());
    assert_eq!(started(&events), vec!["log", "keep"]);
    assert_eq!(seen_names(&seen), vec!["tool.log", "input.txt"]);
}
