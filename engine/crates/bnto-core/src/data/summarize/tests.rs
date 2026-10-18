use super::*;
use crate::FileData;
use crate::context::NoopContext;
use serde_json::{Map, Value, json};

/// An item named `name` carrying `metadata` (a JSON object).
fn item(name: &str, metadata: Value) -> BatchFile {
    BatchFile {
        data: FileData::Bytes(format!("data of {name}").into_bytes()),
        filename: name.to_string(),
        mime_type: Some("text/plain".to_string()),
        metadata: metadata.as_object().cloned().unwrap_or_default(),
    }
}

fn run(params: Value, files: Vec<BatchFile>) -> Result<NodeOutput, BntoError> {
    let input = BatchInput {
        files,
        params: params.as_object().cloned().unwrap(),
    };
    Summarize::new().process_batch(input, &ProgressReporter::new_noop(), &NoopContext)
}

/// The metadata of each summary item, in output order.
fn groups(params: Value, files: Vec<BatchFile>) -> Vec<Map<String, Value>> {
    let out = run(params, files).expect("summarize should not fail");
    out.files.into_iter().map(|f| f.metadata).collect()
}

fn failure(params: Value, files: Vec<BatchFile>) -> String {
    match run(params, files) {
        Ok(_) => panic!("expected summarize to fail"),
        Err(err) => err.to_string(),
    }
}

fn validate(params: Value) -> Vec<String> {
    Summarize::new().validate(params.as_object().unwrap())
}

/// Five files in two folders, `docs` seen first: docs has 3 (100 + 250 + 50),
/// img has 2 (1000 + 24).
fn five_in_two_folders() -> Vec<BatchFile> {
    vec![
        item(
            "a.md",
            json!({ "folder": "docs", "size": 100, "kind": "md" }),
        ),
        item(
            "b.png",
            json!({ "folder": "img", "size": 1000, "kind": "png" }),
        ),
        item(
            "c.md",
            json!({ "folder": "docs", "size": 250, "kind": "md" }),
        ),
        item(
            "d.png",
            json!({ "folder": "img", "size": 24, "kind": "png" }),
        ),
        item(
            "e.txt",
            json!({ "folder": "docs", "size": 50, "kind": "txt" }),
        ),
    ]
}

fn by_folder_sum_size() -> Value {
    json!({ "groupBy": "folder", "sum": "size" })
}

// --- Trait basics ---

#[test]
fn test_name_is_summarize() {
    assert_eq!(Summarize::new().name(), "summarize");
}

#[test]
fn test_metadata_is_a_browser_capable_batch_data_node() {
    let meta = Summarize::new().metadata();
    assert_eq!(meta.node_type, "summarize");
    assert_eq!(meta.category, NodeCategory::Data);
    assert_eq!(meta.input_cardinality, InputCardinality::Batch);
    assert!(meta.platforms.contains(&"browser".to_string()));
    assert!(meta.platforms.contains(&"cli".to_string()));
    let params: Vec<&str> = meta.parameters.iter().map(|p| p.name.as_str()).collect();
    assert_eq!(params, ["groupBy", "sum"]);
}

#[test]
fn test_does_not_fill_item_templates_itself() {
    // Its params name fields; they are not templates over an item.
    assert!(!Summarize::new().resolves_items());
}

// --- Grouping ---

#[test]
fn test_five_items_in_two_groups_give_two_groups_with_counts_and_totals() {
    let out = groups(by_folder_sum_size(), five_in_two_folders());
    assert_eq!(out.len(), 2);
    assert_eq!(
        Value::Object(out[0].clone()),
        json!({ "folder": "docs", "count": 3, "sum_size": 400 })
    );
    assert_eq!(
        Value::Object(out[1].clone()),
        json!({ "folder": "img", "count": 2, "sum_size": 1024 })
    );
}

#[test]
fn test_groups_come_out_in_the_order_their_first_item_arrived() {
    let files = vec![
        item("1", json!({ "k": "zeta" })),
        item("2", json!({ "k": "alpha" })),
        item("3", json!({ "k": "zeta" })),
        item("4", json!({ "k": "mid" })),
    ];
    let keys: Vec<Value> = groups(json!({ "groupBy": "k" }), files)
        .into_iter()
        .map(|g| g["k"].clone())
        .collect();
    assert_eq!(keys, [json!("zeta"), json!("alpha"), json!("mid")]);
}

#[test]
fn test_summary_items_carry_no_data_and_are_named_by_group_index() {
    let out = run(by_folder_sum_size(), five_in_two_folders()).unwrap();
    let names: Vec<&str> = out.files.iter().map(|f| f.filename.as_str()).collect();
    assert_eq!(names, ["group0", "group1"]);
    for file in out.files {
        assert_eq!(file.data.into_bytes().unwrap(), Vec::<u8>::new());
    }
}

#[test]
fn test_node_metadata_reports_the_group_count() {
    let out = run(by_folder_sum_size(), five_in_two_folders()).unwrap();
    assert_eq!(out.metadata["groupCount"], 2);
}

#[test]
fn test_an_empty_input_gives_no_groups() {
    let out = run(by_folder_sum_size(), vec![]).unwrap();
    assert!(out.files.is_empty());
    assert_eq!(out.metadata["groupCount"], 0);
}

#[test]
fn test_two_group_by_fields_group_by_both_values() {
    let out = groups(
        json!({ "groupBy": "folder, kind", "sum": "size" }),
        five_in_two_folders(),
    );
    let rows: Vec<Value> = out.into_iter().map(Value::Object).collect();
    assert_eq!(
        rows,
        [
            json!({ "folder": "docs", "kind": "md", "count": 2, "sum_size": 350 }),
            json!({ "folder": "img", "kind": "png", "count": 2, "sum_size": 1024 }),
            json!({ "folder": "docs", "kind": "txt", "count": 1, "sum_size": 50 }),
        ]
    );
}

#[test]
fn test_without_group_by_every_item_is_one_group() {
    let out = groups(json!({ "sum": "size" }), five_in_two_folders());
    assert_eq!(out.len(), 1);
    assert_eq!(
        Value::Object(out[0].clone()),
        json!({ "count": 5, "sum_size": 1424 })
    );
}

#[test]
fn test_without_sum_only_counts() {
    let out = groups(json!({ "groupBy": "folder" }), five_in_two_folders());
    assert_eq!(
        Value::Object(out[0].clone()),
        json!({ "folder": "docs", "count": 3 })
    );
}

#[test]
fn test_two_sum_fields_each_get_a_total() {
    let files = vec![
        item("1", json!({ "g": "x", "a": 1, "b": 10 })),
        item("2", json!({ "g": "x", "a": 2, "b": 20 })),
    ];
    let out = groups(json!({ "groupBy": "g", "sum": "a, b" }), files);
    assert_eq!(
        Value::Object(out[0].clone()),
        json!({ "g": "x", "count": 2, "sum_a": 3, "sum_b": 30 })
    );
}

#[test]
fn test_values_that_print_the_same_share_a_group() {
    // A CSV reads "2024" as text and a JSON file reads 2024 as a number.
    // The group keeps the value of its first item.
    let files = vec![
        item("1", json!({ "year": "2024" })),
        item("2", json!({ "year": 2024 })),
    ];
    let out = groups(json!({ "groupBy": "year" }), files);
    assert_eq!(out.len(), 1);
    assert_eq!(out[0]["year"], "2024");
    assert_eq!(out[0]["count"], 2);
}

// --- A missing group-by field ---

#[test]
fn test_items_missing_the_group_by_field_form_one_blank_group() {
    // Absent, null and "" all print as nothing, so they group together, in
    // first-seen order, with the field set to null.
    let files = vec![
        item("1", json!({ "folder": "docs", "size": 1 })),
        item("2", json!({ "size": 2 })),
        item("3", json!({ "folder": null, "size": 4 })),
        item("4", json!({ "folder": "", "size": 8 })),
    ];
    let out = groups(by_folder_sum_size(), files);
    assert_eq!(out.len(), 2);
    assert_eq!(
        Value::Object(out[1].clone()),
        json!({ "folder": null, "count": 3, "sum_size": 14 })
    );
}

// --- Totals ---

#[test]
fn test_numeric_text_is_summed() {
    // Every CSV value is text; a number written as text still adds up.
    let files = vec![
        item("1", json!({ "g": "x", "size": "1200" })),
        item("2", json!({ "g": "x", "size": " 300 " })),
        item("3", json!({ "g": "x", "size": 45 })),
    ];
    let out = groups(json!({ "groupBy": "g", "sum": "size" }), files);
    assert_eq!(out[0]["sum_size"], json!(1545));
}

#[test]
fn test_a_fractional_value_makes_the_total_fractional() {
    let files = vec![
        item("1", json!({ "g": "x", "v": 1 })),
        item("2", json!({ "g": "x", "v": "2.5" })),
    ];
    let out = groups(json!({ "groupBy": "g", "sum": "v" }), files);
    assert_eq!(out[0]["sum_v"], json!(3.5));
}

#[test]
fn test_negative_values_subtract() {
    let files = vec![
        item("1", json!({ "g": "x", "v": 10 })),
        item("2", json!({ "g": "x", "v": "-3" })),
    ];
    let out = groups(json!({ "groupBy": "g", "sum": "v" }), files);
    assert_eq!(out[0]["sum_v"], json!(7));
}

#[test]
fn test_blank_summed_values_are_skipped_but_the_item_is_counted() {
    // Like a spreadsheet's SUM over blank cells.
    let files = vec![
        item("1", json!({ "g": "x", "v": 5 })),
        item("2", json!({ "g": "x" })),
        item("3", json!({ "g": "x", "v": null })),
        item("4", json!({ "g": "x", "v": "" })),
    ];
    let out = groups(json!({ "groupBy": "g", "sum": "v" }), files);
    assert_eq!(out[0]["count"], 4);
    assert_eq!(out[0]["sum_v"], json!(5));
}

#[test]
fn test_a_group_with_only_blank_values_totals_zero() {
    let files = vec![item("1", json!({ "g": "x" }))];
    let out = groups(json!({ "groupBy": "g", "sum": "v" }), files);
    assert_eq!(out[0]["sum_v"], json!(0));
}

#[test]
fn test_a_non_numeric_summed_value_fails_naming_the_item_field_and_value() {
    let files = vec![
        item("ok.txt", json!({ "g": "x", "size": 10 })),
        item("bad.txt", json!({ "g": "x", "size": "12 KB" })),
    ];
    let msg = failure(json!({ "groupBy": "g", "sum": "size" }), files);
    assert!(msg.contains("bad.txt"), "{msg}");
    assert!(msg.contains("size"), "{msg}");
    assert!(msg.contains("12 KB"), "{msg}");
}

#[test]
fn test_not_a_number_and_infinity_are_not_numbers() {
    for text in ["NaN", "inf", "-infinity"] {
        let files = vec![item("x.txt", json!({ "v": text }))];
        let msg = failure(json!({ "sum": "v" }), files);
        assert!(msg.contains(text), "{text}: {msg}");
    }
}

#[test]
fn test_a_boolean_or_list_summed_value_fails() {
    for value in [json!(true), json!([1, 2]), json!({ "n": 1 })] {
        let files = vec![item("x.txt", json!({ "v": value }))];
        let msg = failure(json!({ "sum": "v" }), files);
        assert!(msg.contains("x.txt"), "{msg}");
    }
}

// --- Param validation ---

#[test]
fn test_validate_accepts_group_by_and_sum_or_neither() {
    assert!(validate(json!({})).is_empty());
    assert!(validate(json!({ "groupBy": "folder, kind", "sum": "size" })).is_empty());
    assert!(validate(json!({ "groupBy": "", "sum": "" })).is_empty());
}

#[test]
fn test_validate_refuses_group_by_that_is_not_text() {
    let errors = validate(json!({ "groupBy": ["folder"] }));
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert!(errors[0].contains("groupBy"), "{errors:?}");
}

#[test]
fn test_validate_refuses_sum_that_is_not_text() {
    let errors = validate(json!({ "sum": 3 }));
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert!(errors[0].contains("sum"), "{errors:?}");
}

#[test]
fn test_validate_refuses_an_empty_field_between_commas() {
    for params in [json!({ "groupBy": "folder, " }), json!({ "sum": ",size" })] {
        let errors = validate(params.clone());
        assert_eq!(errors.len(), 1, "{params}: {errors:?}");
        assert!(errors[0].contains("empty field"), "{errors:?}");
    }
}

#[test]
fn test_validate_refuses_a_group_field_named_like_an_output_field() {
    // `count` and `sum_<field>` are what the node writes; a group field with
    // either name would be overwritten.
    for params in [
        json!({ "groupBy": "count" }),
        json!({ "groupBy": "sum_size", "sum": "size" }),
    ] {
        let errors = validate(params.clone());
        assert_eq!(errors.len(), 1, "{params}: {errors:?}");
        assert!(errors[0].contains("collides"), "{errors:?}");
    }
}

#[test]
fn test_process_refuses_invalid_params() {
    let msg = failure(json!({ "groupBy": 7 }), five_in_two_folders());
    assert!(msg.contains("groupBy"), "{msg}");
}
