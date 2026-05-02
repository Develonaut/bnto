// Recipe integration tests -- CSV and file system processors.

mod common;

use bnto_core::{NoopContext, PipelineReporter, execute_pipeline};
use common::{MESSY_CSV, SIMPLE_CSV, fake_now, file, file_bytes, parse, real_registry};

// --- Clean CSV -- full recipe integration ---

#[test]
fn clean_csv_recipe_produces_cleaned_output() {
    let def = parse(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            {
                "id": "csv-cleaner", "type": "group",
                "nodes": [{
                    "id": "clean", "type": "spreadsheet-clean",
                    "parameters": {
                        "trimWhitespace": true,
                        "removeEmptyRows": true,
                        "removeDuplicates": true
                    }
                }]
            },
            { "id": "output", "type": "output" }
        ]
    }"#,
    );

    let registry = real_registry();
    let reporter = PipelineReporter::new_noop();
    let files = vec![file("data.csv", MESSY_CSV, "text/csv")];

    let result = execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now)
        .expect("clean CSV pipeline should succeed");

    assert_eq!(result.files.len(), 1);

    let bytes = file_bytes(&result.files[0].data);
    let output_str = std::str::from_utf8(&bytes).expect("cleaned CSV should be valid UTF-8");

    assert!(
        bytes.len() <= MESSY_CSV.len(),
        "cleaned CSV ({} bytes) should not be larger than input ({} bytes)",
        bytes.len(),
        MESSY_CSV.len()
    );

    assert!(
        output_str.contains(','),
        "cleaned CSV should have comma-separated values"
    );
}

// --- Rename CSV Columns -- full recipe integration ---

#[test]
fn rename_csv_columns_recipe_produces_output() {
    let def = parse(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            {
                "id": "column-renamer", "type": "group",
                "nodes": [{
                    "id": "rename-columns", "type": "spreadsheet-rename",
                    "parameters": {
                        "columns": { "name": "full_name" }
                    }
                }]
            },
            { "id": "output", "type": "output" }
        ]
    }"#,
    );

    let registry = real_registry();
    let reporter = PipelineReporter::new_noop();
    let files = vec![file("data.csv", SIMPLE_CSV, "text/csv")];

    let result = execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now)
        .expect("rename columns pipeline should succeed");

    assert_eq!(result.files.len(), 1);

    let bytes = file_bytes(&result.files[0].data);
    let output_str = std::str::from_utf8(&bytes).expect("renamed CSV should be valid UTF-8");

    let first_line = output_str.lines().next().unwrap_or("");
    assert!(
        first_line.contains("full_name"),
        "header should contain renamed column 'full_name', got: {}",
        first_line
    );
}

// --- Rename Files -- full recipe integration ---

#[test]
fn rename_files_recipe_applies_prefix() {
    let def = parse(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            {
                "id": "batch-rename", "type": "group",
                "nodes": [{
                    "id": "rename-loop", "type": "loop",
                    "parameters": { "mode": "forEach" },
                    "nodes": [{
                        "id": "rename-file", "type": "file-rename",
                        "parameters": { "prefix": "renamed-" }
                    }]
                }]
            },
            { "id": "output", "type": "output" }
        ]
    }"#,
    );

    let registry = real_registry();
    let reporter = PipelineReporter::new_noop();
    let files = vec![
        file("document.txt", b"hello world", "text/plain"),
        file("readme.md", b"# Title", "text/markdown"),
    ];

    let result = execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now)
        .expect("rename files pipeline should succeed");

    assert_eq!(result.files.len(), 2, "should output 2 renamed files");

    for f in &result.files {
        assert!(
            f.name.starts_with("renamed-"),
            "file '{}' should have 'renamed-' prefix",
            f.name
        );
    }

    // Data should be unchanged (rename only affects filenames, not content).
    assert_eq!(file_bytes(&result.files[0].data), b"hello world");
    assert_eq!(file_bytes(&result.files[1].data), b"# Title");
}
