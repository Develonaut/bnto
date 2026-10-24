use super::*;
use crate::FileData;
use crate::context::NoopContext;
use serde_json::{Value, json};
use std::path::PathBuf;

fn file(name: &str, metadata: Value) -> BatchFile {
    BatchFile {
        data: FileData::Bytes(format!("data of {name}").into_bytes()),
        filename: name.to_string(),
        mime_type: Some("text/csv".to_string()),
        metadata: metadata.as_object().cloned().unwrap_or_default(),
    }
}

fn run(params: Value, files: Vec<BatchFile>) -> Result<NodeOutput, BntoError> {
    let input = BatchInput {
        files,
        params: params.as_object().cloned().unwrap(),
    };
    EditFields::new().process_batch(input, &ProgressReporter::new_noop(), &NoopContext)
}

/// The metadata of the one file `values` is applied to.
fn edited(values: Value, metadata: Value) -> serde_json::Map<String, Value> {
    let out = run(json!({ "values": values }), vec![file("row.csv", metadata)]).unwrap();
    out.files.into_iter().next().unwrap().metadata
}

fn failure(params: Value, files: Vec<BatchFile>) -> String {
    match run(params, files) {
        Ok(_) => panic!("expected edit-fields to fail"),
        Err(err) => err.to_string(),
    }
}

// --- Setting fields ---

#[test]
fn test_a_field_is_set_from_a_template_over_two_others() {
    let meta = edited(
        json!({ "label": "{{item.first}} {{item.last}}" }),
        json!({ "first": "Ada", "last": "Lovelace" }),
    );
    assert_eq!(meta["label"], "Ada Lovelace");
    assert_eq!(meta["first"], "Ada", "the other fields stay");
    assert_eq!(meta["last"], "Lovelace");
}

#[test]
fn test_an_existing_field_is_overwritten() {
    let meta = edited(
        json!({ "status": "done ({{item.status}})" }),
        json!({ "status": "open" }),
    );
    assert_eq!(meta["status"], "done (open)");
}

#[test]
fn test_a_dotted_key_is_read_as_written() {
    let meta = edited(
        json!({ "link": "https://example.com/items/{{item.meta.id}}" }),
        json!({ "meta.id": "42" }),
    );
    assert_eq!(meta["link"], "https://example.com/items/42");
}

#[test]
fn test_static_values_are_set_as_given() {
    let meta = edited(json!({ "kind": "row", "rank": 3, "ok": true }), json!({}));
    assert_eq!(meta["kind"], "row");
    assert_eq!(meta["rank"], 3);
    assert_eq!(meta["ok"], true);
}

#[test]
fn test_a_sole_placeholder_keeps_the_fields_type() {
    let meta = edited(
        json!({ "copy": "{{item.size}}", "text": "{{item.size}} bytes" }),
        json!({ "size": 1024 }),
    );
    assert_eq!(meta["copy"], 1024, "a lone placeholder is the field itself");
    assert_eq!(meta["text"], "1024 bytes", "anything around it makes text");
}

#[test]
fn test_templates_inside_lists_and_maps_are_filled() {
    let meta = edited(
        json!({ "tags": ["{{item.a}}", "fixed"], "pair": { "x": "{{item.b}}" } }),
        json!({ "a": "one", "b": "two" }),
    );
    assert_eq!(meta["tags"], json!(["one", "fixed"]));
    assert_eq!(meta["pair"], json!({ "x": "two" }));
}

#[test]
fn test_templates_read_the_fields_the_item_arrived_with() {
    let meta = edited(
        json!({ "name": "new", "was": "{{item.name}}" }),
        json!({ "name": "old" }),
    );
    assert_eq!(meta["name"], "new");
    assert_eq!(meta["was"], "old", "not the value this node sets");
}

#[test]
fn test_each_file_reads_its_own_fields() {
    let files = vec![
        file("a.csv", json!({ "n": "1" })),
        file("b.csv", json!({ "n": "2" })),
    ];
    let out = run(json!({ "values": { "label": "row {{item.n}}" } }), files).unwrap();
    let labels: Vec<&Value> = out.files.iter().map(|f| &f.metadata["label"]).collect();
    assert_eq!(labels, [&json!("row 1"), &json!("row 2")]);
}

#[test]
fn test_keep_only_set_drops_every_other_field() {
    let params = json!({ "values": { "label": "{{item.first}}" }, "keepOnlySet": true });
    let files = vec![file("row.csv", json!({ "first": "Ada", "last": "L" }))];
    let out = run(params, files).unwrap();
    let meta = &out.files[0].metadata;
    assert_eq!(meta.len(), 1, "{meta:?}");
    assert_eq!(meta["label"], "Ada");
}

// --- The file's name ---

#[test]
fn test_a_field_set_to_file_is_the_files_name() {
    let meta = edited(json!({ "source": "{{file}}" }), json!({}));
    assert_eq!(meta["source"], "row.csv");
}

#[test]
fn test_file_and_item_fields_fill_in_one_value() {
    let files = vec![
        file("a.csv", json!({ "x": "1" })),
        file("b.csv", json!({ "x": "2" })),
    ];
    let values = json!({ "label": "{{file}}: {{item.x}}", "tags": ["{{file}}"] });
    let out = run(json!({ "values": values }), files).unwrap();
    assert_eq!(out.files[0].metadata["label"], "a.csv: 1");
    assert_eq!(out.files[1].metadata["label"], "b.csv: 2");
    assert_eq!(out.files[1].metadata["tags"], json!(["b.csv"]));
}

#[test]
fn test_file_is_the_name_as_it_travels_as_assert_and_report_read_it() {
    let out = run(
        json!({ "values": { "source": "{{file}}" } }),
        vec![file("sub/a.csv", json!({}))],
    )
    .unwrap();
    assert_eq!(out.files[0].metadata["source"], "sub/a.csv");
}

#[test]
fn test_a_filled_field_that_reads_like_file_is_kept_as_written() {
    let meta = edited(
        json!({ "copy": "{{item.note}}", "text": "{{item.note}}!" }),
        json!({ "note": "{{file}}" }),
    );
    assert_eq!(meta["copy"], "{{file}}");
    assert_eq!(meta["text"], "{{file}}!");
}

// --- Unknown fields ---

#[test]
fn test_an_unknown_field_fails_naming_it_the_field_set_and_the_item() {
    let files = vec![
        file("a.csv", json!({ "first": "Ada", "nick": "A" })),
        file("b.csv", json!({ "first": "Grace" })),
    ];
    let msg = failure(
        json!({ "values": { "label": "{{item.first}} ({{item.nick}})" } }),
        files,
    );
    assert!(msg.contains("\"nick\""), "names the unknown field: {msg}");
    assert!(
        msg.contains("\"label\""),
        "names the field being set: {msg}"
    );
    assert!(msg.contains("b.csv"), "names the item: {msg}");
    assert!(!msg.contains("a.csv"), "only the item that lacks it: {msg}");
    assert!(msg.contains("first"), "lists the fields it has: {msg}");
}

#[test]
fn test_an_unknown_sole_placeholder_fails_too() {
    let msg = failure(
        json!({ "values": { "copy": "{{item.gone}}" } }),
        vec![file("a.csv", json!({}))],
    );
    assert!(msg.contains("\"gone\"") && msg.contains("a.csv"), "{msg}");
}

// --- Files pass through ---

#[test]
fn test_files_pass_through_with_data_unchanged() {
    let files = vec![file("a.csv", json!({ "x": "1" }))];
    let out = run(json!({ "values": { "y": "{{item.x}}" } }), files).unwrap();
    let f = &out.files[0];
    assert_eq!(f.filename, "a.csv");
    assert_eq!(f.mime_type, "text/csv");
    let FileData::Bytes(bytes) = &f.data else {
        panic!("bytes stay bytes");
    };
    assert_eq!(bytes, b"data of a.csv");
}

#[test]
fn test_a_disk_reference_stays_a_reference() {
    let path = PathBuf::from("some/where/a.mp4");
    let input = BatchFile {
        data: FileData::Path(path.clone()),
        ..file("a.mp4", json!({}))
    };
    let out = run(json!({ "values": { "k": "v" } }), vec![input]).unwrap();
    assert!(matches!(&out.files[0].data, FileData::Path(p) if *p == path));
}

#[test]
fn test_no_files_is_no_files() {
    let out = run(json!({ "values": { "k": "{{item.x}}" } }), vec![]).unwrap();
    assert!(out.files.is_empty());
}

// --- Validate and metadata ---

#[test]
fn test_validate_refuses_missing_or_malformed_params() {
    let node = EditFields::new();
    let cases = [
        (json!({}), "values"),
        (json!({ "values": "label" }), "map"),
        (json!({ "values": ["a"] }), "map"),
        (json!({ "values": { "": "x" } }), "no name"),
        (json!({ "values": {}, "keepOnlySet": "yes" }), "keepOnlySet"),
    ];
    for (params, word) in cases {
        let errors = node.validate(params.as_object().unwrap());
        assert_eq!(errors.len(), 1, "{params}: {errors:?}");
        assert!(errors[0].contains(word), "{params}: {errors:?}");
    }
    let ok = json!({ "values": { "label": "{{item.a}}" }, "keepOnlySet": true });
    assert!(node.validate(ok.as_object().unwrap()).is_empty());
}

#[test]
fn test_a_run_with_invalid_params_fails() {
    let msg = failure(json!({ "values": 3 }), vec![file("a.csv", json!({}))]);
    assert!(msg.contains("values"), "{msg}");
}

#[test]
fn test_metadata_is_a_batch_data_node_that_fills_items_itself() {
    let node = EditFields::new();
    let meta = node.metadata();
    assert_eq!(node.name(), "edit-fields");
    assert_eq!(meta.node_type, "edit-fields");
    assert_eq!(meta.category, NodeCategory::Data);
    assert_eq!(meta.input_cardinality, InputCardinality::Batch);
    assert!(meta.platforms.contains(&"browser".to_string()));
    let params: Vec<&str> = meta.parameters.iter().map(|p| p.name.as_str()).collect();
    assert_eq!(params, ["values", "keepOnlySet"]);
    assert!(
        node.resolves_items(),
        "outside a loop {{item.*}} reaches it"
    );
}

#[test]
fn test_values_is_a_required_key_value_map_and_keep_only_set_a_switch() {
    let meta = EditFields::new().metadata();
    let values = &meta.parameters[0];
    assert_eq!(values.param_type, crate::metadata::ParameterType::Object);
    assert_eq!(values.control.as_deref(), Some("keyValue"));
    assert_eq!(values.constraints.as_ref().map(|c| c.required), Some(true));
    assert!(values.description.contains("{{item."), "current syntax");
    assert!(
        values.description.contains("{{file}}"),
        "names the file token"
    );
    let keep = &meta.parameters[1];
    assert_eq!(keep.default, Some(json!(false)));
    assert_eq!(keep.control.as_deref(), Some("switch"));
}
