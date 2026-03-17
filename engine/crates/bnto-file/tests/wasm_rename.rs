// WASM integration tests -- rename pattern, prefix/suffix, find/replace, case transforms.

mod common;

use wasm_bindgen_test::*;

use bnto_file::wasm_bridge::*;
use common::{TEST_FILE_DATA, extract_filename, extract_metadata, init_panic_hook, noop_callback};

wasm_bindgen_test_configure!(run_in_node_experimental);

// --- Pattern Rename Tests ---

#[wasm_bindgen_test]
fn test_rename_combined_with_pattern_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = rename_file_combined(
        TEST_FILE_DATA,
        "photo.jpg",
        r#"{"pattern": "{{name}}-compressed.{{ext}}"}"#,
        callback,
    );

    assert!(result.is_ok(), "rename_file_combined should succeed");

    let result_obj = result.unwrap();
    let metadata = extract_metadata(&result_obj);
    assert!(!metadata.is_empty(), "Metadata JSON should not be empty");

    assert!(
        metadata.contains("photo-compressed.jpg"),
        "Metadata should contain the new filename: got '{}'",
        metadata
    );

    let filename = extract_filename(&result_obj);
    assert_eq!(
        filename, "photo-compressed.jpg",
        "Top-level filename should be the renamed file"
    );
}

#[wasm_bindgen_test]
fn test_rename_combined_with_pattern_index_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = rename_file_combined(
        TEST_FILE_DATA,
        "document.pdf",
        r#"{"pattern": "file-{{index}}.{{ext}}", "index": "7"}"#,
        callback,
    );

    assert!(
        result.is_ok(),
        "rename_file_combined should succeed with index"
    );

    let result_obj = result.unwrap();
    let metadata = extract_metadata(&result_obj);
    assert!(
        metadata.contains("file-7.pdf"),
        "Metadata should contain indexed filename: got '{}'",
        metadata
    );
}

// --- Prefix / Suffix Tests ---

#[wasm_bindgen_test]
fn test_rename_combined_with_prefix_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = rename_file_combined(
        TEST_FILE_DATA,
        "report.csv",
        r#"{"prefix": "final-"}"#,
        callback,
    );

    assert!(
        result.is_ok(),
        "rename_file_combined with prefix should succeed"
    );

    let result_obj = result.unwrap();
    let metadata = extract_metadata(&result_obj);
    assert!(
        metadata.contains("final-report.csv"),
        "Metadata should contain prefixed filename: got '{}'",
        metadata
    );
}

#[wasm_bindgen_test]
fn test_rename_combined_with_suffix_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = rename_file_combined(
        TEST_FILE_DATA,
        "data.json",
        r#"{"suffix": "-backup"}"#,
        callback,
    );

    assert!(
        result.is_ok(),
        "rename_file_combined with suffix should succeed"
    );

    let result_obj = result.unwrap();
    let metadata = extract_metadata(&result_obj);
    assert!(
        metadata.contains("data-backup.json"),
        "Metadata should contain suffixed filename: got '{}'",
        metadata
    );
}

// --- Find / Replace Tests ---

#[wasm_bindgen_test]
fn test_rename_combined_find_replace_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = rename_file_combined(
        TEST_FILE_DATA,
        "old-report.txt",
        r#"{"find": "old", "replace": "new"}"#,
        callback,
    );

    assert!(
        result.is_ok(),
        "rename_file_combined with find/replace should succeed"
    );

    let result_obj = result.unwrap();
    let metadata = extract_metadata(&result_obj);
    assert!(
        metadata.contains("new-report.txt"),
        "Metadata should contain replaced filename: got '{}'",
        metadata
    );
}

// --- Case Transformation Tests ---

#[wasm_bindgen_test]
fn test_rename_combined_case_lower_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = rename_file_combined(
        TEST_FILE_DATA,
        "IMPORTANT.TXT",
        r#"{"case": "lower"}"#,
        callback,
    );

    assert!(
        result.is_ok(),
        "rename_file_combined with case=lower should succeed"
    );

    let result_obj = result.unwrap();
    let metadata = extract_metadata(&result_obj);
    // Case transforms the stem only -- extension stays as-is from the original.
    assert!(
        metadata.contains("important.TXT"),
        "Metadata should contain lowercased filename: got '{}'",
        metadata
    );
}
