use super::*;
use crate::FileData;
use crate::context::NoopContext;
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

fn file(name: &str, metadata: Value) -> BatchFile {
    BatchFile {
        data: FileData::Bytes(name.as_bytes().to_vec()),
        filename: name.to_string(),
        mime_type: Some("text/plain".to_string()),
        metadata: metadata.as_object().cloned().unwrap_or_default(),
    }
}

fn run_with(
    params: Value,
    files: Vec<BatchFile>,
    ctx: &dyn ProcessContext,
) -> Result<NodeOutput, BntoError> {
    let input = BatchInput {
        files,
        params: params.as_object().cloned().unwrap(),
    };
    Assert::new().process_batch(input, &ProgressReporter::new_noop(), ctx)
}

fn run(params: Value, files: Vec<BatchFile>) -> Result<NodeOutput, BntoError> {
    run_with(params, files, &NoopContext)
}

fn failure(params: Value, files: Vec<BatchFile>) -> String {
    match run(params, files) {
        Ok(_) => panic!("expected the assert to fail"),
        Err(BntoError::ProcessingFailed(msg)) => msg,
        Err(other) => panic!("expected ProcessingFailed, got {other}"),
    }
}

fn check(value: Value, op: &str, expected: Value) -> Value {
    json!({ "value": value, "op": op, "expected": expected })
}

// --- Each op ---

#[test]
fn test_equals_passes_and_fails() {
    assert!(run(check(json!("0"), "equals", json!("0")), vec![]).is_ok());
    let msg = failure(check(json!("2"), "equals", json!("0")), vec![]);
    assert!(msg.contains("\"2\"") && msg.contains("\"0\""), "{msg}");
}

#[test]
fn test_equals_compares_a_number_as_its_text() {
    // A sole `{{node.x.count}}` placeholder resolves to a JSON number.
    assert!(run(check(json!(0), "equals", json!("0")), vec![]).is_ok());
}

#[test]
fn test_not_equals_passes_and_fails() {
    assert!(run(check(json!("a"), "not_equals", json!("b")), vec![]).is_ok());
    let msg = failure(check(json!("a"), "not_equals", json!("a")), vec![]);
    assert!(msg.contains("\"a\""), "{msg}");
}

#[test]
fn test_after_and_before_compare_times() {
    let early = "2026-10-17T15:00:00.000Z";
    let late = "2026-10-17T15:00:00.500Z";
    assert!(run(check(json!(late), "after", json!(early)), vec![]).is_ok());
    assert!(run(check(json!(early), "before", json!(late)), vec![]).is_ok());
    let msg = failure(check(json!(early), "after", json!(late)), vec![]);
    assert!(msg.contains(early) && msg.contains(late), "{msg}");
    let msg = failure(check(json!(late), "before", json!(early)), vec![]);
    assert!(msg.contains("not before"), "{msg}");
}

#[test]
fn test_an_equal_time_is_neither_after_nor_before() {
    let t = "2026-10-17T15:00:00.000Z";
    assert!(run(check(json!(t), "after", json!(t)), vec![]).is_err());
    assert!(run(check(json!(t), "before", json!(t)), vec![]).is_err());
}

#[test]
fn test_a_value_that_is_not_a_time_fails_naming_it() {
    let msg = failure(
        check(json!("soon"), "after", json!("2026-10-17T15:00:00Z")),
        vec![],
    );
    assert!(
        msg.contains("\"soon\"") && msg.contains("RFC 3339"),
        "{msg}"
    );
}

// --- Pass-through and the failure message ---

#[test]
fn test_a_passing_assert_emits_its_input_unchanged() {
    let files = vec![file("a.txt", json!({ "k": 1 })), file("b.txt", json!({}))];
    let out = run(check(json!("x"), "equals", json!("x")), files).unwrap();
    let names: Vec<&str> = out.files.iter().map(|f| f.filename.as_str()).collect();
    assert_eq!(names, ["a.txt", "b.txt"]);
    assert_eq!(out.files[0].metadata["k"], 1);
    assert_eq!(out.files[0].mime_type, "text/plain");
}

#[test]
fn test_a_passing_assert_with_no_files_passes() {
    let out = run(check(json!("x"), "equals", json!("x")), vec![]).unwrap();
    assert!(out.files.is_empty());
}

#[test]
fn test_message_leads_the_error() {
    let mut params = check(json!("2"), "equals", json!("0"));
    params["message"] = json!("Missing references: [\"a\",\"b\"]");
    let msg = failure(params, vec![]);
    assert!(
        msg.starts_with("Missing references: [\"a\",\"b\"]: "),
        "{msg}"
    );
}

#[test]
fn test_without_a_message_the_error_says_what_failed() {
    let msg = failure(check(json!("2"), "equals", json!("0")), vec![]);
    assert!(msg.starts_with("Assertion failed: "), "{msg}");
}

// --- Per-item checks ---

#[test]
fn test_item_templates_check_each_file_and_name_every_failure() {
    let started = "2026-10-17T15:00:00.000Z";
    let files = vec![
        file(
            "old-a.txt",
            json!({ "modified": "2026-10-17T14:00:00.000Z" }),
        ),
        file("new.txt", json!({ "modified": "2026-10-17T16:00:00.000Z" })),
        file(
            "old-b.txt",
            json!({ "modified": "2026-10-17T13:00:00.000Z" }),
        ),
    ];
    let msg = failure(
        check(json!("{{item.modified}}"), "after", json!(started)),
        files,
    );
    assert!(
        msg.contains("old-a.txt") && msg.contains("old-b.txt"),
        "{msg}"
    );
    assert!(!msg.contains("new.txt"), "{msg}");
}

#[test]
fn test_item_templates_pass_when_every_file_holds() {
    let files = vec![
        file("a.txt", json!({ "tag": "x" })),
        file("b.txt", json!({ "tag": "x" })),
    ];
    let out = run(check(json!("{{item.tag}}"), "equals", json!("x")), files).unwrap();
    assert_eq!(out.files.len(), 2);
}

#[test]
fn test_an_item_key_a_file_lacks_is_a_failure_naming_it() {
    let files = vec![file("a.txt", json!({}))];
    let msg = failure(
        check(json!("{{item.modified}}"), "equals", json!("x")),
        files,
    );
    assert!(msg.contains("a.txt") && msg.contains("modified"), "{msg}");
}

#[test]
fn test_item_templates_with_no_files_check_nothing() {
    assert!(
        run(
            check(json!("{{item.modified}}"), "equals", json!("x")),
            vec![]
        )
        .is_ok()
    );
}

// --- The file token ---

#[test]
fn test_file_token_reads_each_file_name() {
    let files = vec![file("a.txt", json!({}))];
    assert!(run(check(json!("{{file}}"), "equals", json!("a.txt")), files).is_ok());
}

#[test]
fn test_file_token_checks_each_file_and_names_every_failure() {
    // "Fail if any file arrives": every file reaching the step fails, named.
    let files = vec![file("utils.ts", json!({})), file("hooks.ts", json!({}))];
    let msg = failure(check(json!("{{file}}"), "equals", json!("")), files);
    assert!(
        msg.contains("utils.ts") && msg.contains("hooks.ts"),
        "{msg}"
    );
    assert!(!msg.contains("{{file}}"), "{msg}");
}

#[test]
fn test_file_token_with_no_files_checks_nothing() {
    assert!(run(check(json!("{{file}}"), "equals", json!("")), vec![]).is_ok());
}

#[test]
fn test_file_token_in_expected_checks_per_file() {
    let files = vec![file("a.txt", json!({ "name": "a.txt" }))];
    let params = check(json!("{{item.name}}"), "equals", json!("{{file}}"));
    assert!(run(params, files).is_ok());
}

#[test]
fn test_file_token_mixes_with_item_keys() {
    let files = vec![file("a.txt", json!({ "tag": "x" }))];
    let params = check(json!("{{file}}:{{item.tag}}"), "equals", json!("a.txt:x"));
    assert!(run(params, files).is_ok());
}

#[test]
fn test_a_metadata_value_that_reads_like_the_file_token_is_kept_as_written() {
    let files = vec![file("a.txt", json!({ "note": "{{file}}" }))];
    let params = check(json!("{{item.note}}"), "equals", json!("{{file}}x"));
    let msg = failure(params, files);
    assert!(
        msg.contains("got \"{{file}}\", expected \"a.txtx\""),
        "{msg}"
    );
}

// --- Validate and dry run ---

#[test]
fn test_validate_refuses_a_missing_value_or_op() {
    let assert = Assert::new();
    let no_value = json!({ "op": "equals", "expected": "0" });
    let no_op = json!({ "value": "1", "expected": "0" });
    let bad_op = json!({ "value": "1", "op": "greater", "expected": "0" });
    for (params, word) in [(no_value, "value"), (no_op, "op"), (bad_op, "op")] {
        let errors = assert.validate(params.as_object().unwrap());
        assert_eq!(errors.len(), 1, "{errors:?}");
        assert!(errors[0].contains(word), "{errors:?}");
    }
    let ok = json!({ "value": "{{node.x.count}}", "op": "equals" });
    assert!(assert.validate(ok.as_object().unwrap()).is_empty());
}

/// A dry-run context: everything else is the no-op context.
struct DryRun;

impl ProcessContext for DryRun {
    fn run_command(&self, cmd: &str, args: &[&str]) -> Result<Vec<u8>, BntoError> {
        NoopContext.run_command(cmd, args)
    }
    fn temp_file(&self, suffix: &str) -> Result<PathBuf, BntoError> {
        NoopContext.temp_file(suffix)
    }
    fn env_var(&self, _key: &str) -> Option<String> {
        None
    }
    fn work_dir(&self) -> Result<&Path, BntoError> {
        NoopContext.work_dir()
    }
    fn dry_run(&self) -> bool {
        true
    }
}

#[test]
fn test_a_dry_run_checks_nothing_and_passes_files_through() {
    let files = vec![file("a.txt", json!({}))];
    let out = run_with(check(json!("2"), "equals", json!("0")), files, &DryRun).unwrap();
    assert_eq!(out.files.len(), 1);
}

#[test]
fn test_metadata_is_a_browser_capable_batch_control_node() {
    let meta = Assert::new().metadata();
    assert_eq!(meta.node_type, "assert");
    assert_eq!(meta.category, NodeCategory::Control);
    assert_eq!(meta.input_cardinality, InputCardinality::Batch);
    assert!(meta.platforms.contains(&"browser".to_string()));
    let params: Vec<&str> = meta.parameters.iter().map(|p| p.name.as_str()).collect();
    assert_eq!(params, ["value", "op", "expected", "message"]);
    // It fills `{{item.*}}` itself, so the executor leaves those for it.
    assert!(Assert::new().resolves_items());
}
