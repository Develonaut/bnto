// spreadsheet-read on a JSON array: one item per element, nested fields as dotted keys.

use bnto_core::NoopContext;
use bnto_core::errors::BntoError;
use bnto_core::processor::{FileData, NodeInput, NodeOutput, NodeProcessor};
use bnto_core::progress::ProgressReporter;
use serde_json::json;

use super::ReadSpreadsheet;

// --- Test Helpers ---

type Params = serde_json::Map<String, serde_json::Value>;

fn input(filename: &str, text: &str, params: Params) -> NodeInput {
    NodeInput {
        data: FileData::Bytes(text.as_bytes().to_vec()),
        filename: filename.to_string(),
        mime_type: None,
        params,
    }
}

fn json_input(text: &str) -> NodeInput {
    input("items.json", text, Params::new())
}

fn run(input: NodeInput) -> Result<NodeOutput, BntoError> {
    ReadSpreadsheet::new().process(input, &ProgressReporter::new_noop(), &NoopContext)
}

fn read_ok(text: &str) -> NodeOutput {
    run(json_input(text)).expect("should read")
}

fn read_err(input: NodeInput) -> String {
    match run(input) {
        Err(e) => e.to_string(),
        Ok(_) => panic!("expected an error"),
    }
}

fn format_param(format: &str) -> Params {
    let mut params = Params::new();
    params.insert("format".into(), json!(format));
    params
}

// --- One Item per Element ---

#[test]
fn test_three_element_array_gives_three_items() {
    let output = read_ok(r#"[{"id": "a"}, {"id": "b"}, {"id": "c"}]"#);
    let ids: Vec<_> = output.files.iter().map(|f| &f.metadata["id"]).collect();
    assert_eq!(ids, vec!["a", "b", "c"]);
}

#[test]
fn test_items_are_empty_and_named_after_the_file() {
    let output = read_ok(r#"[{"id": "a"}, {"id": "b"}]"#);
    assert_eq!(output.files[0].filename, "items_row0.json");
    assert_eq!(output.files[1].filename, "items_row1.json");
    assert_eq!(output.files[0].mime_type, "application/json");
    assert!(output.files[0].data.is_empty().unwrap());
}

#[test]
fn test_empty_array_gives_no_items() {
    let output = read_ok("[]");
    assert!(output.files.is_empty());
    assert_eq!(output.metadata["rowCount"], 0);
    assert_eq!(output.metadata["columnCount"], 0);
}

#[test]
fn test_bom_is_stripped() {
    let output = read_ok("\u{FEFF}[{\"id\": \"a\"}]");
    assert_eq!(output.files[0].metadata["id"], "a");
}

// --- Fields ---

#[test]
fn test_nested_field_reads_by_its_dotted_path() {
    let output = read_ok(r#"[{"meta": {"id": "x", "deep": {"tag": "t"}}, "name": "n"}]"#);
    let meta = &output.files[0].metadata;
    assert_eq!(meta["meta.id"], "x");
    assert_eq!(meta["meta.deep.tag"], "t");
    assert_eq!(meta["name"], "n");
    assert!(
        meta.get("meta").is_none(),
        "only leaves become keys: {meta:?}"
    );
}

#[test]
fn test_scalars_keep_their_json_type() {
    let output = read_ok(r#"[{"n": 3, "f": 1.5, "ok": true, "none": null}]"#);
    let meta = &output.files[0].metadata;
    assert_eq!(meta["n"], json!(3));
    assert_eq!(meta["f"], json!(1.5));
    assert_eq!(meta["ok"], json!(true));
    assert_eq!(meta["none"], json!(null));
}

#[test]
fn test_array_field_is_kept_as_json_under_its_key() {
    let output = read_ok(r#"[{"tags": ["a", {"b": 1}]}]"#);
    let meta = &output.files[0].metadata;
    assert_eq!(meta["tags"], json!(["a", {"b": 1}]));
    assert!(meta.get("tags.0").is_none());
}

#[test]
fn test_empty_object_field_is_kept_under_its_key() {
    let output = read_ok(r#"[{"meta": {}}]"#);
    assert_eq!(output.files[0].metadata["meta"], json!({}));
}

#[test]
fn test_non_object_element_is_kept_under_value() {
    let output = read_ok(r#"["a", 2, [3]]"#);
    let values: Vec<_> = output.files.iter().map(|f| &f.metadata["value"]).collect();
    assert_eq!(values, vec![&json!("a"), &json!(2), &json!([3])]);
}

#[test]
fn test_two_fields_on_one_dotted_key_fail_naming_file_and_key() {
    let msg = read_err(json_input(r#"[{"a.b": 1, "a": {"b": 2}}]"#));
    assert!(msg.contains("items.json"), "{msg}");
    assert!(msg.contains("a.b"), "{msg}");
}

// --- Node Metadata ---

#[test]
fn test_metadata_counts_rows_and_distinct_columns() {
    let output = read_ok(r#"[{"a": 1, "m": {"b": 2}}, {"a": 3, "c": 4}]"#);
    assert_eq!(output.metadata["rowCount"], 2);
    assert_eq!(output.metadata["columnCount"], 3, "a, m.b, c");
}

// --- Errors ---

#[test]
fn test_object_file_fails_naming_it() {
    let msg = read_err(json_input(r#"{"id": "a"}"#));
    assert!(msg.contains("items.json"), "{msg}");
    assert!(msg.contains("array"), "{msg}");
}

#[test]
fn test_invalid_json_fails_naming_the_file() {
    let msg = read_err(json_input("[{\"id\": "));
    assert!(msg.contains("items.json"), "{msg}");
    assert!(msg.contains("not valid JSON"), "{msg}");
}

#[test]
fn test_empty_file_fails_naming_it() {
    let msg = read_err(json_input("  \n"));
    assert!(msg.contains("items.json"), "{msg}");
}

#[test]
fn test_max_rows_applies_to_json() {
    let mut params = format_param("auto");
    params.insert("maxRows".into(), json!(2));
    let msg = read_err(input("items.json", "[1, 2, 3]", params));
    assert!(msg.contains("3"), "{msg}");
    assert!(msg.contains("maxRows"), "{msg}");
    assert!(msg.contains("items.json"), "{msg}");
}

// --- Format ---

#[test]
fn test_auto_reads_json_by_mime_type() {
    let mut input = input("items", r#"[{"id": "a"}]"#, Params::new());
    input.mime_type = Some("application/json".to_string());
    assert_eq!(run(input).unwrap().files[0].metadata["id"], "a");
}

#[test]
fn test_auto_reads_json_by_extension_in_any_case() {
    let output = run(input("sub/Items.JSON", r#"[{"id": "a"}]"#, Params::new())).unwrap();
    assert_eq!(output.files[0].metadata["id"], "a");
    assert_eq!(output.files[0].filename, "sub/Items_row0.json");
}

#[test]
fn test_format_json_reads_any_file_name_as_json() {
    let output = run(input("list.txt", r#"[{"id": "a"}]"#, format_param("json"))).unwrap();
    assert_eq!(output.files[0].metadata["id"], "a");
}

#[test]
fn test_format_csv_reads_a_json_named_file_as_csv() {
    let output = run(input("table.json", "id\na\n", format_param("csv"))).unwrap();
    assert_eq!(output.files[0].metadata["id"], "a");
    assert_eq!(output.files[0].filename, "table_row0.csv");
}

#[test]
fn test_unknown_format_fails() {
    let msg = read_err(input("items.json", "[]", format_param("yaml")));
    assert!(msg.contains("yaml"), "{msg}");
}

#[test]
fn test_validate_reports_unknown_format() {
    let processor = ReadSpreadsheet::new();
    assert!(processor.validate(&format_param("json")).is_empty());
    let errors = processor.validate(&format_param("yaml"));
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert!(errors[0].contains("yaml"));
}

#[test]
fn test_metadata_declares_format_and_accepts_json() {
    let meta = ReadSpreadsheet::new().metadata();
    let format = meta.parameters.iter().find(|p| p.name == "format");
    assert_eq!(format.expect("format param").default, Some(json!("auto")));
    assert!(meta.accepts.contains(&"application/json".to_string()));
    assert!(meta.accepts.contains(&"text/csv".to_string()));
}
