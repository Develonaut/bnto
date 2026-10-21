// `when` guards: a step runs only when its comparison holds; otherwise it's
// skipped, its input files pass through untouched, and it leaves no record.
use super::always::{always_registry, run, seen_names, started};
use super::*;

/// What `{{ctx.platform}}` reads on the machine running the tests.
fn host_platform() -> &'static str {
    if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else {
        "linux"
    }
}

/// `(node_id, reason)` of every skipped node, in order.
fn skipped(events: &[PipelineEvent]) -> Vec<(String, String)> {
    events
        .iter()
        .filter_map(|e| match e {
            PipelineEvent::NodeSkipped {
                node_id, reason, ..
            } => Some((node_id.clone(), reason.clone())),
            _ => None,
        })
        .collect()
}

/// A recipe of one capture step guarded by `when`.
fn guarded(value: &str, op: &str, expected: &str) -> String {
    format!(
        r#"{{ "nodes": [
            {{ "id": "in", "type": "input" }},
            {{ "id": "keep", "type": "test-capture",
               "when": {{ "value": "{value}", "op": "{op}", "expected": "{expected}" }} }},
            {{ "id": "out", "type": "output" }}
        ] }}"#
    )
}

#[test]
fn a_holding_guard_runs_the_step() {
    let (registry, seen) = always_registry();
    let (result, events) = run(&guarded("a", "equals", "a"), &registry);
    assert!(result.is_ok(), "{result:?}");
    assert_eq!(seen_names(&seen), ["input.txt"]);
    assert!(skipped(&events).is_empty());
}

#[test]
fn a_failing_guard_skips_the_step_and_passes_its_files_through() {
    let (registry, seen) = always_registry();
    let (result, events) = run(&guarded("a", "equals", "b"), &registry);
    let result = result.expect("a skip isn't a failure");
    assert!(seen_names(&seen).is_empty(), "the step never ran");
    assert!(
        !started(&events).contains(&"keep".to_string()),
        "never started"
    );
    let skips = skipped(&events);
    assert_eq!(skips.len(), 1);
    assert_eq!(skips[0].0, "keep");
    assert!(skips[0].1.contains("expected \"b\""), "{}", skips[0].1);
    let names: Vec<&str> = result.files.iter().map(|f| f.name.as_str()).collect();
    assert_eq!(names, ["input.txt"], "the input passed through");
}

#[test]
fn a_platform_guard_reads_the_host() {
    let (registry, seen) = always_registry();
    let (_, events) = run(
        &guarded("{{ctx.platform}}", "equals", host_platform()),
        &registry,
    );
    assert_eq!(seen_names(&seen), ["input.txt"], "runs on its own platform");
    assert!(skipped(&events).is_empty());

    let (registry, seen) = always_registry();
    let (_, events) = run(
        &guarded("{{ctx.platform}}", "not-equals", host_platform()),
        &registry,
    );
    assert!(seen_names(&seen).is_empty(), "skipped everywhere else");
    assert_eq!(skipped(&events).len(), 1);
}

#[test]
fn a_guard_reads_its_nodes_fields() {
    let (registry, seen) = always_registry();
    let (_, events) = run(
        r#"{ "nodes": [
            { "id": "keep", "type": "test-capture",
              "fields": { "action": { "type": "enum", "label": "Action",
                "options": [{ "value": "start", "label": "Start" }, { "value": "stop", "label": "Stop" }],
                "default": "stop" } },
              "when": { "value": "{{fields.action}}", "op": "equals", "expected": "start" } }
        ] }"#,
        &registry,
    );
    assert!(seen_names(&seen).is_empty(), "action is stop, not start");
    assert_eq!(skipped(&events).len(), 1);
}

#[test]
fn a_skipped_step_leaves_no_node_record() {
    let (registry, _) = always_registry();
    let (result, _) = run(&guarded("a", "equals", "b"), &registry);
    let records = result.unwrap().node_metadata;
    assert!(!records.contains_key("keep"), "{records:?}");
}

#[test]
fn a_false_guard_skips_even_an_always_step() {
    let (registry, seen) = always_registry();
    let (result, events) = run(
        r#"{ "nodes": [
            { "id": "build", "type": "test-fail" },
            { "id": "keep", "type": "test-capture", "always": true,
              "when": { "value": "a", "op": "equals", "expected": "b" } }
        ] }"#,
        &registry,
    );
    assert!(result.is_err(), "the run still fails on build");
    assert!(seen_names(&seen).is_empty());
    assert_eq!(started(&events), ["build"]);
    assert_eq!(skipped(&events).len(), 1);
}

#[test]
fn a_skipped_group_never_starts_its_children() {
    let (registry, seen) = always_registry();
    let (_, events) = run(
        r#"{ "nodes": [
            { "id": "branch", "type": "group",
              "when": { "value": "{{ctx.platform}}", "op": "equals", "expected": "plan9" },
              "nodes": [{ "id": "keep", "type": "test-capture" }] }
        ] }"#,
        &registry,
    );
    assert!(seen_names(&seen).is_empty());
    assert!(started(&events).is_empty(), "{:?}", started(&events));
    assert_eq!(
        skipped(&events)
            .iter()
            .map(|s| s.0.as_str())
            .collect::<Vec<_>>(),
        ["branch"]
    );
}

#[test]
fn a_skipped_step_in_a_graph_hands_its_input_to_the_next_step() {
    let (registry, seen) = always_registry();
    let (_, events) = run(
        r#"{ "nodes": [
            { "id": "in", "type": "input" },
            { "id": "shout", "type": "test-uppercase",
              "when": { "value": "a", "op": "equals", "expected": "b" } },
            { "id": "keep", "type": "test-capture" }
        ],
        "edges": [
            { "id": "e1", "source": "in", "target": "shout" },
            { "id": "e2", "source": "shout", "target": "keep" }
        ] }"#,
        &registry,
    );
    assert_eq!(skipped(&events).len(), 1);
    let seen = seen.lock().unwrap();
    assert_eq!(seen.len(), 1);
    assert_eq!(seen[0].1, b"hello", "unchanged: shout was skipped");
}

#[test]
fn a_guard_inside_a_loop_reads_each_item() {
    // Only the jpg is captured; the png passes the loop through untouched.
    let (registry, seen) = always_registry();
    let recorder = RecordingReporter::new();
    let reporter = recorder.reporter();
    let mut jpg = make_file("a.jpg", b"jpg");
    jpg.metadata.insert("kind".into(), "jpg".into());
    let mut png = make_file("b.png", b"png");
    png.metadata.insert("kind".into(), "png".into());
    let def = parse_def(
        r#"{ "nodes": [
            { "id": "each", "type": "loop", "nodes": [
                { "id": "keep", "type": "test-capture",
                  "when": { "value": "{{item.kind}}", "op": "equals", "expected": "jpg" } }
            ] }
        ] }"#,
    );
    let result = execute_pipeline(
        &def,
        vec![jpg, png],
        &registry,
        &reporter,
        &NoopContext,
        fake_now,
    );
    assert_eq!(seen_names(&seen), ["a.jpg"]);
    let mut names: Vec<String> = result
        .unwrap()
        .files
        .iter()
        .map(|f| f.name.clone())
        .collect();
    names.sort();
    assert_eq!(names, ["a.jpg", "b.png"]);
}

#[test]
fn an_unknown_op_fails_the_run() {
    let (registry, _) = always_registry();
    let (result, _) = run(&guarded("a", "is", "a"), &registry);
    let error = result.expect_err("an op that doesn't exist");
    assert!(error.to_string().contains("\"is\""), "{error}");
}

#[test]
fn a_guard_reads_an_earlier_nodes_record() {
    for (expected, runs) in [("5", true), ("6", false)] {
        let (registry, seen) = always_registry();
        let (_, events) = run(
            &format!(
                r#"{{ "nodes": [
                    {{ "id": "squash", "type": "test-metadata" }},
                    {{ "id": "keep", "type": "test-capture",
                       "when": {{ "value": "{{{{node.squash.originalSize}}}}", "op": "equals", "expected": "{expected}" }} }}
                ] }}"#
            ),
            &registry,
        );
        assert_eq!(
            seen_names(&seen).len(),
            usize::from(runs),
            "expected {expected}: {:?}",
            skipped(&events)
        );
        assert_eq!(skipped(&events).is_empty(), runs, "expected {expected}");
    }
}
