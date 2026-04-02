// Recipe JSON helpers and basic execution tests (image, CSV, file recipes).
use super::*;

/// Helper: JSON for compress-images recipe structure.
/// Shared across recipe test modules via `super::recipes::compress_images_json`.
pub(super) fn compress_images_json() -> &'static str {
    r#"{
        "nodes": [
            {
                "id": "input", "type": "input", "version": "1.0.0",
                "name": "Input Files", "position": {"x": 0, "y": 100},
                "metadata": {},
                "parameters": { "mode": "file-upload" },
                "inputPorts": [], "outputPorts": [{"id": "out-1", "name": "files"}]
            },
            {
                "id": "batch-compress", "type": "group", "version": "1.0.0",
                "name": "Batch Compress", "position": {"x": 250, "y": 100},
                "metadata": { "description": "Reusable sub-recipe: loops over files and compresses each one." },
                "parameters": {},
                "inputPorts": [{"id": "in-1", "name": "files"}],
                "outputPorts": [{"id": "out-1", "name": "files"}],
                "nodes": [
                    {
                        "id": "compress-loop", "type": "loop", "version": "1.0.0",
                        "name": "Compress Each Image", "position": {"x": 0, "y": 0},
                        "metadata": {},
                        "parameters": { "mode": "forEach" },
                        "inputPorts": [{"id": "in-1", "name": "items"}], "outputPorts": [],
                        "nodes": [
                            {
                                "id": "compress-image", "type": "image-compress", "version": "1.0.0",
                                "name": "Compress Image", "position": {"x": 0, "y": 0},
                                "metadata": {},
                                "parameters": { "quality": 80 },
                                "inputPorts": [], "outputPorts": []
                            }
                        ],
                        "edges": []
                    }
                ],
                "edges": []
            },
            {
                "id": "output", "type": "output", "version": "1.0.0",
                "name": "Compressed Images", "position": {"x": 500, "y": 100},
                "metadata": {},
                "parameters": { "mode": "download", "zip": true },
                "inputPorts": [{"id": "in-1", "name": "files"}], "outputPorts": []
            }
        ],
        "edges": [
            {"id": "e1", "source": "input", "target": "batch-compress"},
            {"id": "e2", "source": "batch-compress", "target": "output"}
        ]
    }"#
}

/// Helper: JSON for clean-csv recipe structure.
/// Shared across recipe test modules via `super::recipes::clean_csv_json`.
pub(super) fn clean_csv_json() -> &'static str {
    r#"{
        "nodes": [
            {
                "id": "input", "type": "input", "version": "1.0.0",
                "name": "Input Files", "position": {"x": 0, "y": 100},
                "metadata": {},
                "parameters": { "mode": "file-upload" },
                "inputPorts": [], "outputPorts": [{"id": "out-1", "name": "files"}]
            },
            {
                "id": "csv-cleaner", "type": "group", "version": "1.0.0",
                "name": "CSV Cleaner", "position": {"x": 250, "y": 100},
                "metadata": { "description": "Reusable sub-recipe: trims whitespace, removes empty rows, deduplicates." },
                "parameters": {},
                "inputPorts": [{"id": "in-1", "name": "files"}],
                "outputPorts": [{"id": "out-1", "name": "files"}],
                "nodes": [
                    {
                        "id": "clean", "type": "spreadsheet-clean", "version": "1.0.0",
                        "name": "Clean CSV", "position": {"x": 0, "y": 0},
                        "metadata": {},
                        "parameters": {
                            "trimWhitespace": true,
                            "removeEmptyRows": true,
                            "removeDuplicates": true
                        },
                        "inputPorts": [{"id": "in-1", "name": "files"}],
                        "outputPorts": [{"id": "out-1", "name": "files"}]
                    }
                ],
                "edges": []
            },
            {
                "id": "output", "type": "output", "version": "1.0.0",
                "name": "Cleaned CSV", "position": {"x": 500, "y": 100},
                "metadata": {},
                "parameters": { "mode": "download" },
                "inputPorts": [{"id": "in-1", "name": "files"}], "outputPorts": []
            }
        ],
        "edges": [
            {"id": "e1", "source": "input", "target": "csv-cleaner"},
            {"id": "e2", "source": "csv-cleaner", "target": "output"}
        ]
    }"#
}

/// Helper: JSON for rename-files recipe structure.
/// Shared across recipe test modules via `super::recipes::rename_files_json`.
pub(super) fn rename_files_json() -> &'static str {
    r#"{
        "nodes": [
            {
                "id": "input", "type": "input", "version": "1.0.0",
                "name": "Input Files", "position": {"x": 0, "y": 100},
                "metadata": {},
                "parameters": { "mode": "file-upload" },
                "inputPorts": [], "outputPorts": [{"id": "out-1", "name": "files"}]
            },
            {
                "id": "batch-rename", "type": "group", "version": "1.0.0",
                "name": "Batch Rename", "position": {"x": 250, "y": 100},
                "metadata": { "description": "Reusable sub-recipe: loops over files and renames each one." },
                "parameters": {},
                "inputPorts": [{"id": "in-1", "name": "files"}],
                "outputPorts": [{"id": "out-1", "name": "files"}],
                "nodes": [
                    {
                        "id": "rename-loop", "type": "loop", "version": "1.0.0",
                        "name": "Rename Each File", "position": {"x": 0, "y": 0},
                        "metadata": {},
                        "parameters": { "mode": "forEach" },
                        "inputPorts": [{"id": "in-1", "name": "items"}], "outputPorts": [],
                        "nodes": [
                            {
                                "id": "rename-file", "type": "file-rename", "version": "1.0.0",
                                "name": "Rename File", "position": {"x": 0, "y": 0},
                                "metadata": {},
                                "parameters": { "prefix": "renamed-" },
                                "inputPorts": [], "outputPorts": []
                            }
                        ],
                        "edges": []
                    }
                ],
                "edges": []
            },
            {
                "id": "output", "type": "output", "version": "1.0.0",
                "name": "Renamed Files", "position": {"x": 500, "y": 100},
                "metadata": {},
                "parameters": { "mode": "download", "zip": true },
                "inputPorts": [{"id": "in-1", "name": "files"}], "outputPorts": []
            }
        ],
        "edges": [
            {"id": "e1", "source": "input", "target": "batch-rename"},
            {"id": "e2", "source": "batch-rename", "target": "output"}
        ]
    }"#
}

// --- Image Recipe Execution ---

#[test]
fn test_recipe_compress_images_single_file() {
    let def = parse_def(compress_images_json());
    let registry = recipe_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![make_file("photo.jpg", b"jpeg-data")];
    let result =
        execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    // Loop runs once (1 file), EchoProcessor passes it through.
    assert_eq!(result.files.len(), 1);
    assert_eq!(result.files[0].name, "photo.jpg");
    assert_eq!(result.files[0].data, b"jpeg-data");
}

#[test]
fn test_recipe_compress_images_multiple_files() {
    let def = parse_def(compress_images_json());
    let registry = recipe_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![
        make_file("photo1.jpg", b"data1"),
        make_file("photo2.png", b"data2"),
        make_file("photo3.webp", b"data3"),
        make_file("photo4.jpg", b"data4"),
        make_file("photo5.png", b"data5"),
    ];
    let result =
        execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    // Loop runs 5 times (once per file).
    assert_eq!(result.files.len(), 5);
    assert_eq!(result.files[0].name, "photo1.jpg");
    assert_eq!(result.files[4].name, "photo5.png");
}

#[test]
fn test_recipe_resize_images() {
    // Compositional: Input → Group("Batch Resize") → Loop → [image-resize] → Output
    let json = r#"{
        "nodes": [
            { "id": "input", "type": "input", "parameters": {} },
            {
                "id": "batch-resize", "type": "group", "parameters": {},
                "nodes": [
                    {
                        "id": "resize-loop", "type": "loop",
                        "parameters": { "mode": "forEach" },
                        "nodes": [
                            {
                                "id": "resize-image", "type": "image-resize",
                                "parameters": { "width": 200 }
                            }
                        ]
                    }
                ]
            },
            { "id": "output", "type": "output", "parameters": {} }
        ]
    }"#;

    let def = parse_def(json);
    let registry = recipe_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![
        make_file("a.jpg", b"img-a"),
        make_file("b.jpg", b"img-b"),
        make_file("c.jpg", b"img-c"),
    ];
    let result =
        execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    assert_eq!(result.files.len(), 3);
}

#[test]
fn test_recipe_convert_image_format() {
    // Compositional: Input → Group("Batch Convert") → Loop → [image-convert] → Output
    let json = r#"{
        "nodes": [
            { "id": "input", "type": "input", "parameters": {} },
            {
                "id": "batch-convert", "type": "group", "parameters": {},
                "nodes": [
                    {
                        "id": "convert-loop", "type": "loop",
                        "parameters": { "mode": "forEach" },
                        "nodes": [
                            {
                                "id": "convert-image", "type": "image-convert",
                                "parameters": { "format": "webp" }
                            }
                        ]
                    }
                ]
            },
            { "id": "output", "type": "output", "parameters": {} }
        ]
    }"#;

    let def = parse_def(json);
    let registry = recipe_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![
        make_file("photo.jpg", b"jpeg"),
        make_file("icon.png", b"png"),
    ];
    let result =
        execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    assert_eq!(result.files.len(), 2);
}

// --- CSV Recipe Execution ---

#[test]
fn test_recipe_clean_csv_single_file() {
    let def = parse_def(clean_csv_json());
    let registry = recipe_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![make_file("data.csv", b"name,age\nAlice,30\n")];
    let result =
        execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    // Flat pipeline: one processor node, file passes through.
    assert_eq!(result.files.len(), 1);
    assert_eq!(result.files[0].name, "data.csv");
}

#[test]
fn test_recipe_rename_csv_columns() {
    // Compositional: Input → Group("Column Renamer") → [spreadsheet-rename] → Output
    let json = r#"{
        "nodes": [
            { "id": "input", "type": "input", "parameters": {} },
            {
                "id": "column-renamer", "type": "group", "parameters": {},
                "nodes": [
                    {
                        "id": "rename-columns", "type": "spreadsheet-rename",
                        "parameters": { "columns": {} }
                    }
                ]
            },
            { "id": "output", "type": "output", "parameters": {} }
        ]
    }"#;

    let def = parse_def(json);
    let registry = recipe_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![make_file("data.csv", b"old_name\nvalue\n")];
    let result =
        execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    assert_eq!(result.files.len(), 1);
}

// --- File System Recipe Execution ---

#[test]
fn test_recipe_rename_files() {
    let def = parse_def(rename_files_json());
    let registry = recipe_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![
        make_file("report.pdf", b"pdf-data"),
        make_file("notes.txt", b"text-data"),
        make_file("photo.jpg", b"img-data"),
        make_file("data.csv", b"csv-data"),
    ];
    let result =
        execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    // Loop runs 4 times, UpperCaseProcessor uppercases filenames.
    assert_eq!(result.files.len(), 4);
    assert_eq!(result.files[0].name, "REPORT.PDF");
    assert_eq!(result.files[1].name, "NOTES.TXT");
    assert_eq!(result.files[2].name, "PHOTO.JPG");
    assert_eq!(result.files[3].name, "DATA.CSV");
}
