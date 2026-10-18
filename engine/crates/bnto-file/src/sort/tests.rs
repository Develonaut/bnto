use super::*;
use bnto_core::context::NoopContext;
use bnto_core::processor::FileData;
use serde_json::{Value, json};
use std::path::PathBuf;

/// An item named `name` carrying `metadata` (a JSON object).
fn item(name: &str, metadata: Value) -> BatchFile {
    BatchFile {
        data: FileData::Bytes(name.as_bytes().to_vec()),
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
    FileSort::new().process_batch(input, &ProgressReporter::new_noop(), &NoopContext)
}

/// The names of `files` after sorting them by `by`.
fn sorted(by: &str, files: Vec<BatchFile>) -> Vec<String> {
    names(run(json!({ "by": by }), files).expect("sort should not fail"))
}

fn names(output: NodeOutput) -> Vec<String> {
    output.files.into_iter().map(|f| f.filename).collect()
}

fn validate(params: Value) -> Vec<String> {
    FileSort::new().validate(params.as_object().unwrap())
}

/// Items `a`..: one per title, in the order given.
fn titled(titles: &[Value]) -> Vec<BatchFile> {
    titles
        .iter()
        .enumerate()
        .map(|(i, t)| {
            item(
                &char::from(b'a' + i as u8).to_string(),
                json!({ "title": t }),
            )
        })
        .collect()
}

// --- Trait basics ---

#[test]
fn test_name_is_file_sort() {
    assert_eq!(FileSort::new().name(), "file-sort");
}

#[test]
fn test_metadata_is_a_browser_capable_batch_node() {
    let meta = FileSort::new().metadata();
    assert_eq!(meta.node_type, "file-sort");
    assert_eq!(meta.input_cardinality, InputCardinality::Batch);
    assert!(meta.platforms.contains(&"browser".to_string()));
    assert!(meta.platforms.contains(&"cli".to_string()));
    let params: Vec<&str> = meta.parameters.iter().map(|p| p.name.as_str()).collect();
    assert_eq!(params, ["by", "case"]);
}

// --- One key ---

#[test]
fn test_sorts_by_one_field_ascending() {
    let files = titled(&[json!("cherry"), json!("apple"), json!("banana")]);
    assert_eq!(sorted("title", files), ["b", "c", "a"]);
}

#[test]
fn test_a_leading_dash_sorts_descending() {
    let files = titled(&[json!("cherry"), json!("apple"), json!("banana")]);
    assert_eq!(sorted("-title", files), ["a", "c", "b"]);
}

#[test]
fn test_equal_keys_keep_their_incoming_order() {
    let files = vec![
        item("first", json!({ "group": "x" })),
        item("second", json!({ "group": "y" })),
        item("third", json!({ "group": "x" })),
        item("fourth", json!({ "group": "y" })),
    ];
    assert_eq!(
        sorted("group", files.clone_items()),
        ["first", "third", "second", "fourth"]
    );
    // Descending reverses the keys, not the incoming order of ties.
    assert_eq!(
        sorted("-group", files),
        ["second", "fourth", "first", "third"]
    );
}

#[test]
fn test_numbers_sort_numerically_not_as_text() {
    let files = titled(&[json!(10), json!(2), json!(33), json!(-1.5)]);
    assert_eq!(sorted("title", files), ["d", "b", "a", "c"]);
}

#[test]
fn test_strings_compare_by_code_point_by_default() {
    // Upper case (U+0041..) sorts before lower case; `é` (U+00E9) after `z`.
    let files = titled(&[
        json!("apple"),
        json!("Zebra"),
        json!("éclair"),
        json!("zoo"),
    ]);
    assert_eq!(sorted("title", files), ["b", "a", "d", "c"]);
}

#[test]
fn test_case_insensitive_compares_lowercased_text() {
    let files = titled(&[
        json!("apple"),
        json!("Zebra"),
        json!("Apple"),
        json!("banana"),
    ]);
    let out = run(json!({ "by": "title", "case": "insensitive" }), files).unwrap();
    // `apple` and `Apple` are equal, so they keep their incoming order.
    assert_eq!(names(out), ["a", "c", "d", "b"]);
}

#[test]
fn test_a_dotted_key_reads_a_flattened_nested_field() {
    let files = vec![
        item("one", json!({ "meta.id": "b" })),
        item("two", json!({ "meta.id": "a" })),
    ];
    assert_eq!(sorted("meta.id", files), ["two", "one"]);
}

// --- Several keys ---

#[test]
fn test_a_second_field_breaks_ties_in_the_first() {
    let files = vec![
        item("r1", json!({ "source": "web", "title": "b" })),
        item("r2", json!({ "source": "app", "title": "z" })),
        item("r3", json!({ "source": "web", "title": "a" })),
        item("r4", json!({ "source": "app", "title": "c" })),
    ];
    assert_eq!(sorted("source, title", files), ["r4", "r2", "r3", "r1"]);
}

#[test]
fn test_each_key_has_its_own_direction() {
    let files = vec![
        item("r1", json!({ "source": "web", "year": 2019 })),
        item("r2", json!({ "source": "app", "year": 2020 })),
        item("r3", json!({ "source": "web", "year": 2024 })),
        item("r4", json!({ "source": "app", "year": 2018 })),
    ];
    assert_eq!(sorted("source,-year", files), ["r2", "r4", "r3", "r1"]);
}

// --- Missing values ---

#[test]
fn test_a_missing_field_sorts_last_ascending() {
    let files = vec![
        item("none", json!({})),
        item("b", json!({ "title": "b" })),
        item("null", json!({ "title": null })),
        item("a", json!({ "title": "a" })),
        item("blank", json!({ "title": "" })),
    ];
    assert_eq!(sorted("title", files), ["a", "b", "none", "null", "blank"]);
}

#[test]
fn test_a_missing_field_sorts_last_descending_too() {
    let files = vec![
        item("none", json!({})),
        item("a", json!({ "title": "a" })),
        item("blank", json!({ "title": "" })),
        item("b", json!({ "title": "b" })),
    ];
    assert_eq!(sorted("-title", files), ["b", "a", "none", "blank"]);
}

#[test]
fn test_a_later_key_breaks_ties_between_missing_values() {
    let files = vec![
        item("x", json!({ "title": "z" })),
        item("y", json!({ "title": "a" })),
        item("z", json!({ "source": "web" })),
    ];
    assert_eq!(sorted("source, title", files), ["z", "y", "x"]);
}

// --- Mixed types ---

#[test]
fn test_mixed_types_order_numbers_then_text_then_booleans_then_the_rest() {
    let files = titled(&[
        json!(true),
        json!("text"),
        json!([1]),
        json!(5),
        json!(false),
        json!({ "k": 1 }),
    ]);
    assert_eq!(
        sorted("title", files.clone_items()),
        ["d", "b", "e", "a", "c", "f"]
    );
    // Descending reverses the whole order of present values.
    assert_eq!(sorted("-title", files), ["f", "c", "a", "e", "b", "d"]);
}

// --- Pass-through ---

#[test]
fn test_files_keep_their_data_metadata_and_type() {
    let reference = BatchFile {
        data: FileData::Path(PathBuf::from("lib/big.wav")),
        filename: "big.wav".to_string(),
        mime_type: Some("audio/wav".to_string()),
        metadata: json!({ "rank": 2, "extra": "kept" })
            .as_object()
            .cloned()
            .unwrap(),
    };
    let files = vec![reference, item("small", json!({ "rank": 1 }))];
    let out = run(json!({ "by": "rank" }), files).unwrap();
    let big = &out.files[1];
    assert_eq!(big.filename, "big.wav");
    assert!(matches!(&big.data, FileData::Path(p) if p == &PathBuf::from("lib/big.wav")));
    assert_eq!(big.mime_type, "audio/wav");
    assert_eq!(big.metadata["extra"], "kept");
    assert!(out.metadata.is_empty());
}

#[test]
fn test_no_items_sort_to_no_items() {
    let out = run(json!({ "by": "title" }), vec![]).unwrap();
    assert!(out.files.is_empty());
}

// --- Params ---

#[test]
fn test_validate_passes_one_or_more_keys() {
    assert!(validate(json!({ "by": "title" })).is_empty());
    assert!(validate(json!({ "by": " source , -title " })).is_empty());
    assert!(validate(json!({ "by": "title", "case": "insensitive" })).is_empty());
}

#[test]
fn test_validate_requires_by() {
    let errors = validate(json!({}));
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("'by' is required"), "{errors:?}");
    assert!(!validate(json!({ "by": "  " })).is_empty());
}

#[test]
fn test_validate_refuses_an_empty_key() {
    for by in ["title,,source", "title,", "-", "title, - "] {
        let errors = validate(json!({ "by": by }));
        assert!(
            errors.iter().any(|e| e.contains("empty sort key")),
            "{by:?}: {errors:?}"
        );
    }
}

#[test]
fn test_validate_refuses_by_that_is_not_text() {
    let errors = validate(json!({ "by": ["title"] }));
    assert!(
        errors.iter().any(|e| e.contains("'by' must be text")),
        "{errors:?}"
    );
}

#[test]
fn test_validate_refuses_an_unknown_case() {
    let errors = validate(json!({ "by": "title", "case": "upper" }));
    assert!(errors.iter().any(|e| e.contains("'case'")), "{errors:?}");
}

#[test]
fn test_a_run_with_bad_params_is_refused() {
    match run(json!({}), vec![item("a", json!({}))]) {
        Err(BntoError::InvalidInput(msg)) => assert!(msg.contains("'by'"), "{msg}"),
        other => panic!("expected InvalidInput, got {:?}", other.map(names)),
    }
}

/// Copies of test items (BatchFile isn't Clone).
trait CloneItems {
    fn clone_items(&self) -> Vec<BatchFile>;
}

impl CloneItems for Vec<BatchFile> {
    fn clone_items(&self) -> Vec<BatchFile> {
        self.iter()
            .map(|f| BatchFile {
                data: f.data.clone(),
                filename: f.filename.clone(),
                mime_type: f.mime_type.clone(),
                metadata: f.metadata.clone(),
            })
            .collect()
    }
}
