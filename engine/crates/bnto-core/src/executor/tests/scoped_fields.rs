// Scoped fields: a container's `fields` reach every step inside it, for
// params and `when` guards alike. The nearest declaration wins.
use super::always::{always_registry, run, started};
use super::item_templates::run_names;
use super::*;

/// A `fields` block declaring one string field `name` with `default`.
fn name_field(default: &str) -> String {
    format!(r#"{{ "name": {{ "type": "string", "label": "Name", "default": "{default}" }} }}"#)
}

/// An enum field `action` (start / stop) set to `value`.
fn action_field(value: &str) -> String {
    format!(
        r#"{{ "action": {{ "type": "enum", "label": "Action", "default": "{value}",
            "options": [{{ "value": "start", "label": "Start" }}, {{ "value": "stop", "label": "Stop" }}] }} }}"#
    )
}

/// A `test-name` step that names its file `{{fields.name}}`.
const NAME_STEP: &str =
    r#"{ "id": "name", "type": "test-name", "parameters": { "name": "{{fields.name}}" } }"#;

fn one_file() -> Vec<PipelineFile> {
    vec![make_file("a.txt", b"data")]
}

#[test]
fn a_step_reads_its_groups_field() {
    let json = format!(
        r#"{{ "nodes": [{{ "id": "g", "type": "group", "fields": {},
            "nodes": [{NAME_STEP}] }}] }}"#,
        name_field("from-group.txt")
    );
    assert_eq!(run_names(&json, one_file()), ["from-group.txt"]);
}

#[test]
fn a_step_reads_a_field_two_containers_up() {
    let json = format!(
        r#"{{ "nodes": [{{ "id": "g", "type": "group", "fields": {},
            "nodes": [{{ "id": "each", "type": "loop", "nodes": [{NAME_STEP}] }}] }}] }}"#,
        name_field("outer.txt")
    );
    assert_eq!(run_names(&json, one_file()), ["outer.txt"]);
}

#[test]
fn a_loops_field_reaches_every_iteration() {
    let json = format!(
        r#"{{ "nodes": [{{ "id": "each", "type": "loop", "fields": {},
            "nodes": [{NAME_STEP}] }}] }}"#,
        name_field("same.txt")
    );
    let files = vec![make_file("a.txt", b"1"), make_file("b.txt", b"2")];
    assert_eq!(run_names(&json, files), ["same.txt", "same.txt"]);
}

#[test]
fn a_steps_own_field_shadows_its_groups() {
    let json = format!(
        r#"{{ "nodes": [{{ "id": "g", "type": "group", "fields": {},
            "nodes": [{{ "id": "name", "type": "test-name", "fields": {},
                "parameters": {{ "name": "{{{{fields.name}}}}" }} }}] }}] }}"#,
        name_field("from-group.txt"),
        name_field("own.txt")
    );
    assert_eq!(run_names(&json, one_file()), ["own.txt"]);
}

#[test]
fn a_nearer_container_shadows_an_outer_one() {
    let json = format!(
        r#"{{ "nodes": [{{ "id": "outer", "type": "group", "fields": {},
            "nodes": [{{ "id": "inner", "type": "group", "fields": {},
                "nodes": [{NAME_STEP}] }}] }}] }}"#,
        name_field("outer.txt"),
        name_field("inner.txt")
    );
    assert_eq!(run_names(&json, one_file()), ["inner.txt"]);
}

#[test]
fn a_groups_field_stays_inside_the_group() {
    let json = format!(
        r#"{{ "nodes": [
            {{ "id": "g", "type": "group", "fields": {},
               "nodes": [{{ "id": "inside", "type": "test-echo" }}] }},
            {NAME_STEP}
        ] }}"#,
        name_field("leaked.txt")
    );
    assert_eq!(run_names(&json, one_file()), ["{{fields.name}}"]);
}

/// A group declaring `action`, holding a `start` and a `stop` step guarded
/// on it: an if/else pair on one field.
fn if_else(action: &str) -> String {
    let guard = |expected| {
        format!(
            r#""when": {{ "value": "{{{{fields.action}}}}", "op": "equals", "expected": "{expected}" }}"#
        )
    };
    format!(
        r#"{{ "nodes": [{{ "id": "session", "type": "group", "fields": {},
            "nodes": [
                {{ "id": "start", "type": "test-capture", {} }},
                {{ "id": "stop", "type": "test-capture", {} }}
            ] }}] }}"#,
        action_field(action),
        guard("start"),
        guard("stop")
    )
}

#[test]
fn guards_inside_a_group_read_its_field() {
    for action in ["start", "stop"] {
        let (registry, _) = always_registry();
        let (result, events) = run(&if_else(action), &registry);
        assert!(result.is_ok(), "{result:?}");
        assert_eq!(started(&events), ["session", action], "action={action}");
    }
}

#[test]
fn a_groups_own_guard_reads_its_own_field() {
    let json = format!(
        r#"{{ "nodes": [{{ "id": "g", "type": "group", "fields": {},
            "when": {{ "value": "{{{{fields.action}}}}", "op": "equals", "expected": "start" }},
            "nodes": [{{ "id": "keep", "type": "test-capture" }}] }}] }}"#,
        action_field("stop")
    );
    let (registry, seen) = always_registry();
    let (result, events) = run(&json, &registry);
    assert!(result.is_ok(), "{result:?}");
    assert!(
        started(&events).is_empty(),
        "action is stop, so g is skipped"
    );
    assert!(seen.lock().unwrap().is_empty());
}
