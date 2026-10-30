use super::*;
use crate::FileData;
use crate::context::NoopContext;
use crate::control::in_loop::in_loop;
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

fn file(name: &str, metadata: Value) -> BatchFile {
    BatchFile {
        data: FileData::Bytes(name.as_bytes().to_vec()),
        filename: name.to_string(),
        mime_type: Some("text/plain".to_string()),
        metadata: metadata.as_object().cloned().unwrap_or_default(),
    }
}

/// Run report with `params` over `files`, returning its result and every
/// line it printed.
fn run_with(
    params: Value,
    files: Vec<BatchFile>,
    ctx: &dyn ProcessContext,
) -> (Result<NodeOutput, BntoError>, Vec<String>) {
    let lines = Arc::new(Mutex::new(Vec::new()));
    let sink = Arc::clone(&lines);
    let progress = ProgressReporter::with_live(
        |_, _| {},
        move |line| sink.lock().unwrap().push(line.to_string()),
    );
    let input = BatchInput {
        files,
        params: params.as_object().cloned().unwrap(),
    };
    let result = Report::new().process_batch(input, &progress, ctx);
    let printed = lines.lock().unwrap().clone();
    (result, printed)
}

/// The lines a run that must succeed prints.
fn printed(params: Value, files: Vec<BatchFile>) -> Vec<String> {
    let (result, lines) = run_with(params, files, &NoopContext);
    result.expect("report should not fail");
    lines
}

/// The message of a run that must be refused as a recipe mistake.
fn refused(params: Value, files: Vec<BatchFile>) -> String {
    match run_with(params, files, &NoopContext).0 {
        Ok(_) => panic!("expected report to refuse its params"),
        Err(BntoError::InvalidInput(msg)) => msg,
        Err(other) => panic!("expected InvalidInput, got {other}"),
    }
}

fn validate(params: Value) -> Vec<String> {
    Report::new().validate(params.as_object().unwrap())
}

// --- Once ---

#[test]
fn test_a_plain_message_prints_once_whatever_the_files() {
    let files = vec![file("a.txt", json!({})), file("b.txt", json!({}))];
    let lines = printed(json!({ "message": "Listed 3 paths" }), files);
    assert_eq!(lines, ["Listed 3 paths"]);
}

#[test]
fn test_a_plain_message_prints_once_with_no_files() {
    let lines = printed(json!({ "message": "nothing came in" }), vec![]);
    assert_eq!(lines, ["nothing came in"]);
}

#[test]
fn test_a_resolved_number_or_list_prints_as_its_text() {
    // A sole `{{node.x.count}}` placeholder resolves to a JSON number.
    assert_eq!(printed(json!({ "message": 3 }), vec![]), ["3"]);
}

// --- Per file ---

#[test]
fn test_item_templates_print_one_line_per_file_in_order() {
    let files = vec![
        file("a.txt", json!({ "size": 10 })),
        file("b.txt", json!({ "size": 20 })),
    ];
    let lines = printed(json!({ "message": "size {{item.size}}" }), files);
    assert_eq!(lines, ["size 10", "size 20"]);
}

#[test]
fn test_item_templates_with_no_files_print_nothing() {
    let lines = printed(json!({ "message": "{{item.size}}" }), vec![]);
    assert!(lines.is_empty(), "{lines:?}");
}

#[test]
fn test_an_item_key_a_file_lacks_is_refused_naming_key_and_file() {
    let files = vec![
        file("a.txt", json!({ "size": 1 })),
        file("b.txt", json!({})),
    ];
    let msg = refused(json!({ "message": "{{item.size}}" }), files);
    assert!(msg.contains("b.txt") && msg.contains("size"), "{msg}");
}

#[test]
fn test_the_file_token_prints_one_line_per_file_named_in_order() {
    let files = vec![file("utils.ts", json!({})), file("hooks.ts", json!({}))];
    let lines = printed(json!({ "message": "grab bag: {{file}}" }), files);
    assert_eq!(lines, ["grab bag: utils.ts", "grab bag: hooks.ts"]);
}

#[test]
fn test_the_file_token_and_item_keys_fill_from_the_same_file() {
    let files = vec![
        file("a.txt", json!({ "size": 10 })),
        file("b.txt", json!({ "size": 20 })),
    ];
    let lines = printed(json!({ "message": "{{file}}: {{item.size}}" }), files);
    assert_eq!(lines, ["a.txt: 10", "b.txt: 20"]);
}

#[test]
fn test_the_file_token_with_no_files_prints_nothing() {
    let lines = printed(json!({ "message": "{{file}}" }), vec![]);
    assert!(lines.is_empty(), "{lines:?}");
}

#[test]
fn test_a_file_name_that_reads_like_a_token_prints_as_written() {
    let files = vec![file("{{item.size}}.txt", json!({}))];
    let lines = printed(json!({ "message": "{{file}}" }), files);
    assert_eq!(lines, ["{{item.size}}.txt"]);
}

// --- Per entry ---

fn entries() -> Value {
    json!([
        { "path": "a.txt", "status": "found" },
        { "path": "gone.txt", "status": "missing" },
        { "path": "sub/b.txt", "status": "found" },
    ])
}

#[test]
fn test_each_prints_one_line_per_entry_in_order() {
    let params = json!({ "each": entries(), "message": "{{entry.status}}: {{entry.path}}" });
    let lines = printed(params, vec![file("x.txt", json!({}))]);
    assert_eq!(
        lines,
        ["found: a.txt", "missing: gone.txt", "found: sub/b.txt"]
    );
}

#[test]
fn test_a_plain_value_entry_is_read_whole() {
    let params = json!({ "each": ["a.txt", 2], "message": "- {{entry}}" });
    assert_eq!(printed(params, vec![]), ["- a.txt", "- 2"]);
}

#[test]
fn test_an_empty_each_prints_nothing() {
    let params = json!({ "each": [], "message": "{{entry.path}}" });
    assert!(printed(params, vec![]).is_empty());
}

#[test]
fn test_an_entry_key_an_entry_lacks_is_refused_naming_it() {
    let params = json!({ "each": [{ "path": "a" }, "b"], "message": "{{entry.path}}" });
    let msg = refused(params, vec![]);
    assert!(msg.contains("entry 2") && msg.contains("path"), "{msg}");
}

#[test]
fn test_each_that_is_not_a_list_is_refused() {
    // An unfilled `{{node.x.list}}` reaches the node as its text.
    for each in [json!("{{node.gone.entries}}"), json!(3), json!({ "a": 1 })] {
        let params = json!({ "each": each, "message": "{{entry}}" });
        let msg = refused(params, vec![]);
        assert!(msg.contains("'each'") && msg.contains("list"), "{msg}");
    }
}

// --- Pass-through ---

#[test]
fn test_files_pass_through_unchanged() {
    let disk = BatchFile {
        data: FileData::Path(PathBuf::from("folder/big.wav")),
        filename: "big.wav".to_string(),
        mime_type: Some("audio/wav".to_string()),
        metadata: json!({ "k": 1 }).as_object().cloned().unwrap(),
    };
    let params = json!({ "each": entries(), "message": "{{entry.path}}" });
    let (result, _) = run_with(params, vec![disk, file("a.txt", json!({}))], &NoopContext);
    let out = result.unwrap();
    let names: Vec<&str> = out.files.iter().map(|f| f.filename.as_str()).collect();
    assert_eq!(names, ["big.wav", "a.txt"]);
    assert!(matches!(&out.files[0].data, FileData::Path(p) if p.ends_with("big.wav")));
    assert_eq!(out.files[0].metadata["k"], 1);
    assert_eq!(out.files[0].mime_type, "audio/wav");
    assert!(out.metadata.is_empty());
}

// --- Validate ---

#[test]
fn test_validate_refuses_a_missing_or_empty_message() {
    for params in [
        json!({}),
        json!({ "message": "" }),
        json!({ "message": null }),
    ] {
        let errors = validate(params);
        assert_eq!(errors.len(), 1, "{errors:?}");
        assert!(errors[0].contains("message"), "{errors:?}");
    }
    assert!(validate(json!({ "message": "hi" })).is_empty());
}

#[test]
fn test_validate_accepts_each_as_a_list_or_a_placeholder() {
    let list = json!({ "each": [1], "message": "{{entry}}" });
    let placeholder = json!({ "each": "{{node.opt.entries}}", "message": "{{entry.path}}" });
    assert!(validate(list).is_empty());
    assert!(validate(placeholder).is_empty());
}

#[test]
fn test_validate_refuses_each_that_can_never_be_a_list() {
    let errors = validate(json!({ "each": 3, "message": "{{entry}}" }));
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert!(errors[0].contains("'each'"), "{errors:?}");
}

#[test]
fn test_validate_refuses_entry_placeholders_without_each() {
    let errors = validate(json!({ "message": "{{entry.path}}" }));
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert!(errors[0].contains("each"), "{errors:?}");
}

#[test]
fn test_entry_and_item_together_are_refused_outside_a_loop() {
    // Inside a loop `{{item.*}}` is the loop's item, so validate can't refuse it.
    let params = json!({ "each": [1], "message": "{{entry}} {{item.size}}" });
    assert!(validate(params.clone()).is_empty());
    let msg = refused(params, vec![]);
    assert!(
        msg.contains("{{item.*}}") && msg.contains("outside a loop"),
        "{msg}"
    );
}

// --- Inside a loop ---

#[test]
fn test_in_a_loop_a_file_line_reads_the_loop_item_on_top_of_the_files() {
    // RED (ISS-58): a key the file gained in the iteration (`size`) and the
    // loop's own (`row`) both fill; the loop's item wins a shared key.
    let ctx = in_loop(json!({ "row": "r1", "k": "loop" }));
    let files = vec![file("a.txt", json!({ "size": 3, "k": "file" }))];
    let params = json!({ "message": "{{item.row}} {{item.size}} {{item.k}}" });
    let (result, lines) = run_with(params, files, &ctx);
    result.unwrap();
    assert_eq!(lines, ["r1 3 loop"]);
}

#[test]
fn test_in_a_loop_each_reads_a_list_from_the_loop_item() {
    let ctx = in_loop(json!({ "tags": ["a", "b"], "who": "{{entry}}" }));
    let params = json!({ "each": "{{item.tags}}", "message": "{{item.who}}={{entry}}" });
    let (result, lines) = run_with(params, vec![], &ctx);
    result.unwrap();
    assert_eq!(lines, ["{{entry}}=a", "{{entry}}=b"]);
}

#[test]
fn test_in_a_loop_a_key_neither_has_is_refused_listing_both() {
    let ctx = in_loop(json!({ "row": "r1" }));
    let files = vec![file("a.txt", json!({ "size": 3 }))];
    let (result, _) = run_with(json!({ "message": "{{item.gone}}" }), files, &ctx);
    let msg = result.err().unwrap().to_string();
    assert!(msg.contains("gone") && msg.contains("a.txt"), "{msg}");
    assert!(msg.contains("row") && msg.contains("size"), "{msg}");
}

#[test]
fn test_validate_refuses_entry_and_file_together() {
    let params = json!({ "each": [1], "message": "{{entry}} {{file}}" });
    let errors = validate(params);
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert!(errors[0].contains("{{file}}"), "{errors:?}");
}

// --- Dry run and metadata ---

/// A dry-run context: everything else is the no-op context.
///
/// Its upstream nodes didn't run: a `{{node.*}}` the run can't fill answers
/// as the executor does, kept as written when it is the whole value and
/// empty among other text.
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
    fn run_placeholder(&self, _inner: &str, whole: bool) -> Option<Value> {
        (!whole).then(|| json!(""))
    }
}

#[test]
fn test_a_dry_run_prints_each_line_it_would_report() {
    let files = vec![file("a.txt", json!({})), file("b.txt", json!({}))];
    let (result, lines) = run_with(json!({ "message": "saw {{file}}" }), files, &DryRun);
    result.unwrap();
    assert_eq!(
        lines,
        ["Would report: saw a.txt", "Would report: saw b.txt"]
    );
}

#[test]
fn test_a_dry_run_prints_what_it_would_report_and_passes_files_through() {
    // Upstream nodes may not have run, so `each` can still be its text.
    let params = json!({ "each": "{{node.opt.entries}}", "message": "{{entry.path}}" });
    let (result, lines) = run_with(params, vec![file("a.txt", json!({}))], &DryRun);
    assert_eq!(result.unwrap().files.len(), 1);
    assert_eq!(lines, ["Would report: {{entry.path}}"]);
}

#[test]
fn test_metadata_is_a_browser_capable_batch_control_node() {
    let report = Report::new();
    let meta = report.metadata();
    assert_eq!(meta.node_type, "report");
    assert_eq!(report.name(), "report");
    assert_eq!(meta.category, NodeCategory::Control);
    assert_eq!(meta.input_cardinality, InputCardinality::Batch);
    assert!(meta.platforms.contains(&"browser".to_string()));
    let params: Vec<&str> = meta.parameters.iter().map(|p| p.name.as_str()).collect();
    assert_eq!(params, ["message", "each"]);
    assert!(report.resolves_items());
}

#[test]
fn test_metadata_marks_message_required() {
    // validate refuses a report without one, so the editor must not show it as optional.
    let meta = Report::new().metadata();
    let required = |name: &str| {
        let param = meta.parameters.iter().find(|p| p.name == name).unwrap();
        param.constraints.as_ref().is_some_and(|c| c.required)
    };
    assert!(required("message"));
    assert!(!required("each"));
}
