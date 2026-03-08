use super::*;

// =========================================================================
// Real Recipe Structure Tests
// =========================================================================
//
// These tests use the EXACT JSON shapes produced by the TypeScript
// recipe definitions (with "nodes", "parameters", "version", "position",
// "metadata", "inputPorts", "outputPorts", "edges" — all the fields
// the Rust struct silently ignores plus the aliased field names).
//
// Mock processors verify orchestration — we're testing that the executor
// correctly walks container nodes, chains outputs, and skips I/O nodes.
// The actual image/CSV/file processing is tested separately in each
// node crate's own test suite.

/// Helper: JSON for compress-images recipe structure.
/// Compositional: Group → Input → Group("Batch Compress") → Loop → [image:compress] → Output
/// This mirrors how users build recipes — the batch processing logic is a
/// reusable sub-recipe (group node) that could be shared independently.
fn compress_images_json() -> &'static str {
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
                                "id": "compress-image", "type": "image", "version": "1.0.0",
                                "name": "Compress Image", "position": {"x": 0, "y": 0},
                                "metadata": {},
                                "parameters": { "operation": "compress", "quality": 80 },
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
/// Compositional: Group → Input → Group("CSV Cleaner") → [spreadsheet:clean] → Output
/// The CSV cleaner is a reusable sub-recipe containing the processor directly
/// (no loop — CSV operations process the whole file at once).
fn clean_csv_json() -> &'static str {
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
                        "id": "clean", "type": "spreadsheet", "version": "1.0.0",
                        "name": "Clean CSV", "position": {"x": 0, "y": 0},
                        "metadata": {},
                        "parameters": {
                            "operation": "clean",
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
/// Compositional: Group → Input → Group("Batch Rename") → Loop → [file-system:rename] → Output
/// Same pattern as image recipes — the batch rename logic is a reusable sub-recipe.
fn rename_files_json() -> &'static str {
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
                                "id": "rename-file", "type": "file-system", "version": "1.0.0",
                                "name": "Rename File", "position": {"x": 0, "y": 0},
                                "metadata": {},
                                "parameters": { "operation": "rename", "prefix": "renamed-" },
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
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

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
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    // Loop runs 5 times (once per file).
    assert_eq!(result.files.len(), 5);
    assert_eq!(result.files[0].name, "photo1.jpg");
    assert_eq!(result.files[4].name, "photo5.png");
}

#[test]
fn test_recipe_resize_images() {
    // Compositional: Input → Group("Batch Resize") → Loop → [image:resize] → Output
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
                                "id": "resize-image", "type": "image",
                                "parameters": { "operation": "resize", "width": 200 }
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
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    assert_eq!(result.files.len(), 3);
}

#[test]
fn test_recipe_convert_image_format() {
    // Compositional: Input → Group("Batch Convert") → Loop → [image:convert] → Output
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
                                "id": "convert-image", "type": "image",
                                "parameters": { "operation": "convert", "format": "webp" }
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
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    assert_eq!(result.files.len(), 2);
}

// --- CSV Recipe Execution ---

#[test]
fn test_recipe_clean_csv_single_file() {
    let def = parse_def(clean_csv_json());
    let registry = recipe_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![make_file("data.csv", b"name,age\nAlice,30\n")];
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    // Flat pipeline: one processor node, file passes through.
    assert_eq!(result.files.len(), 1);
    assert_eq!(result.files[0].name, "data.csv");
}

#[test]
fn test_recipe_rename_csv_columns() {
    // Compositional: Input → Group("Column Renamer") → [spreadsheet:rename] → Output
    let json = r#"{
        "nodes": [
            { "id": "input", "type": "input", "parameters": {} },
            {
                "id": "column-renamer", "type": "group", "parameters": {},
                "nodes": [
                    {
                        "id": "rename-columns", "type": "spreadsheet",
                        "parameters": { "operation": "rename", "columns": {} }
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
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

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
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    // Loop runs 4 times, UpperCaseProcessor uppercases filenames.
    assert_eq!(result.files.len(), 4);
    assert_eq!(result.files[0].name, "REPORT.PDF");
    assert_eq!(result.files[1].name, "NOTES.TXT");
    assert_eq!(result.files[2].name, "PHOTO.JPG");
    assert_eq!(result.files[3].name, "DATA.CSV");
}

// --- Nested Container Tests (Synthetic Recipes) ---

#[test]
fn test_group_containing_group_containing_loop() {
    // Group → Group → Loop → EchoProcessor. 3 levels deep.
    let json = r#"{
        "nodes": [
            {
                "id": "outer", "type": "group",
                "parameters": {},
                "nodes": [
                    {
                        "id": "inner", "type": "group",
                        "parameters": {},
                        "nodes": [
                            {
                                "id": "the-loop", "type": "loop",
                                "parameters": { "mode": "forEach" },
                                "nodes": [
                                    {
                                        "id": "proc", "type": "image",
                                        "parameters": { "operation": "compress" }
                                    }
                                ]
                            }
                        ]
                    }
                ]
            }
        ]
    }"#;

    let def = parse_def(json);
    let registry = recipe_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![
        make_file("a.jpg", b"aaa"),
        make_file("b.jpg", b"bbb"),
        make_file("c.jpg", b"ccc"),
    ];
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    // Files pass through all 3 container levels to the processor.
    assert_eq!(result.files.len(), 3);
    assert_eq!(result.files[0].name, "a.jpg");
}

#[test]
fn test_multiple_processors_inside_loop() {
    // Loop → [echo, then uppercase]. Two sequential processors per iteration.
    let json = r#"{
        "nodes": [
            {
                "id": "the-loop", "type": "loop",
                "parameters": { "mode": "forEach" },
                "nodes": [
                    { "id": "step1", "type": "test", "params": { "operation": "echo" } },
                    { "id": "step2", "type": "test", "params": { "operation": "uppercase" } }
                ]
            }
        ]
    }"#;

    let def = parse_def(json);
    let registry = mock_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![make_file("a.txt", b"aaa"), make_file("b.txt", b"bbb")];
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    // Each file goes through echo then uppercase inside the loop.
    assert_eq!(result.files.len(), 2);
    assert_eq!(result.files[0].name, "A.TXT");
    assert_eq!(result.files[1].name, "B.TXT");
}

#[test]
fn test_sequential_loops_in_pipeline() {
    // Loop1(echo) → Loop2(uppercase). Two loops in sequence.
    let json = r#"{
        "nodes": [
            {
                "id": "loop1", "type": "loop",
                "params": { "mode": "forEach" },
                "children": [
                    { "id": "echo", "type": "test", "params": { "operation": "echo" } }
                ]
            },
            {
                "id": "loop2", "type": "loop",
                "params": { "mode": "forEach" },
                "children": [
                    { "id": "upper", "type": "test", "params": { "operation": "uppercase" } }
                ]
            }
        ]
    }"#;

    let def = parse_def(json);
    let registry = mock_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![make_file("file.txt", b"data")];
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    // Passes through loop1 (echo, unchanged) then loop2 (uppercase).
    assert_eq!(result.files.len(), 1);
    assert_eq!(result.files[0].name, "FILE.TXT");
}

#[test]
fn test_four_levels_deep_nesting() {
    // Group → Group → Group → Loop → uppercase. Maximum nesting.
    let json = r#"{
        "nodes": [
            {
                "id": "g1", "type": "group", "parameters": {},
                "nodes": [
                    {
                        "id": "g2", "type": "group", "parameters": {},
                        "nodes": [
                            {
                                "id": "g3", "type": "group", "parameters": {},
                                "nodes": [
                                    {
                                        "id": "the-loop", "type": "loop",
                                        "parameters": { "mode": "forEach" },
                                        "nodes": [
                                            {
                                                "id": "proc", "type": "test",
                                                "params": { "operation": "uppercase" }
                                            }
                                        ]
                                    }
                                ]
                            }
                        ]
                    }
                ]
            }
        ]
    }"#;

    let def = parse_def(json);
    let registry = mock_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![make_file("deep.txt", b"deep")];
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    assert_eq!(result.files.len(), 1);
    assert_eq!(result.files[0].name, "DEEP.TXT");
}

// --- Edge Cases with Recipe Structures ---

#[test]
fn test_recipe_with_only_io_nodes_passthrough() {
    // Recipe with only input + output. No processing nodes.
    let json = r#"{
        "nodes": [
            {
                "id": "input", "type": "input", "version": "1.0.0",
                "name": "Input", "position": {"x": 0, "y": 0}, "metadata": {},
                "parameters": {}, "inputPorts": [], "outputPorts": []
            },
            {
                "id": "output", "type": "output", "version": "1.0.0",
                "name": "Output", "position": {"x": 0, "y": 0}, "metadata": {},
                "parameters": {}, "inputPorts": [], "outputPorts": []
            }
        ]
    }"#;

    let def = parse_def(json);
    let registry = recipe_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![make_file("test.txt", b"hello")];
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    assert_eq!(result.files.len(), 1);
    assert_eq!(result.files[0].data, b"hello");
}

#[test]
fn test_recipe_empty_files_no_error() {
    let def = parse_def(compress_images_json());
    let registry = recipe_registry();
    let reporter = PipelineReporter::new_noop();

    let result = execute_pipeline(&def, vec![], &registry, &reporter, fake_now).unwrap();
    assert!(result.files.is_empty());
}

#[test]
fn test_recipe_container_io_children_skipped() {
    // A loop containing input + output + processor. I/O children are skipped.
    let json = r#"{
        "nodes": [
            {
                "id": "the-loop", "type": "loop",
                "parameters": { "mode": "forEach" },
                "nodes": [
                    { "id": "inner-input", "type": "input", "parameters": {} },
                    {
                        "id": "proc", "type": "image",
                        "parameters": { "operation": "compress" }
                    },
                    { "id": "inner-output", "type": "output", "parameters": {} }
                ]
            }
        ]
    }"#;

    let def = parse_def(json);
    let registry = recipe_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![make_file("photo.jpg", b"data")];
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    // I/O nodes inside loop are skipped, only the processor runs.
    assert_eq!(result.files.len(), 1);
}

// --- Error Cases with Recipe Structures ---

#[test]
fn test_recipe_unregistered_operation_inside_loop() {
    // Loop contains a node with an operation that has no processor.
    let json = r#"{
        "nodes": [
            {
                "id": "the-loop", "type": "loop",
                "parameters": { "mode": "forEach" },
                "nodes": [
                    {
                        "id": "bad-node", "type": "spreadsheet",
                        "parameters": { "operation": "pivot" }
                    }
                ]
            }
        ]
    }"#;

    let def = parse_def(json);
    let registry = recipe_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![make_file("data.csv", b"csv-data")];
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now);

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("spreadsheet:pivot"),
        "Error should name the missing key: {}",
        err
    );
}

#[test]
fn test_recipe_failure_inside_nested_container() {
    // Group → Loop → FailProcessor. Error should propagate up.
    let json = r#"{
        "nodes": [
            {
                "id": "group-1", "type": "group",
                "parameters": {},
                "nodes": [
                    {
                        "id": "the-loop", "type": "loop",
                        "parameters": { "mode": "forEach" },
                        "nodes": [
                            {
                                "id": "fail-proc", "type": "test",
                                "params": { "operation": "fail" }
                            }
                        ]
                    }
                ]
            }
        ]
    }"#;

    let def = parse_def(json);
    let registry = mock_registry();
    let recorder = RecordingReporter::new();
    let reporter = recorder.reporter();

    let files = vec![make_file("test.txt", b"data")];
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now);

    assert!(result.is_err());

    let events = recorder.events();
    let has_pipeline_failed = events
        .iter()
        .any(|e| matches!(e, PipelineEvent::PipelineFailed { .. }));
    assert!(
        has_pipeline_failed,
        "Should emit PipelineFailed for nested failure"
    );
}

// --- Progress Events with Recipe Structures ---

#[test]
fn test_recipe_compress_images_event_sequence() {
    let def = parse_def(compress_images_json());
    let registry = recipe_registry();
    let recorder = RecordingReporter::new();
    let reporter = recorder.reporter();

    let files = vec![make_file("a.jpg", b"aaa"), make_file("b.jpg", b"bbb")];
    execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    let events = recorder.events();

    // Should start with PipelineStarted.
    assert!(matches!(events[0], PipelineEvent::PipelineStarted { .. }));

    // Should end with PipelineCompleted.
    assert!(matches!(
        events.last().unwrap(),
        PipelineEvent::PipelineCompleted { .. }
    ));

    // Should have NodeStarted for the batch-compress group (sub-recipe).
    let group_started = events.iter().any(
        |e| matches!(e, PipelineEvent::NodeStarted { node_id, .. } if node_id == "batch-compress"),
    );
    assert!(
        group_started,
        "Should emit NodeStarted for sub-recipe group node"
    );

    // Should have NodeStarted for the loop inside the sub-recipe.
    let loop_started = events.iter().any(
        |e| matches!(e, PipelineEvent::NodeStarted { node_id, .. } if node_id == "compress-loop"),
    );
    assert!(
        loop_started,
        "Should emit NodeStarted for loop node inside sub-recipe"
    );

    // Should have NodeStarted for the child processor (runs per file).
    let child_started_count = events
        .iter()
        .filter(|e| {
            matches!(e, PipelineEvent::NodeStarted { node_id, .. } if node_id == "compress-image")
        })
        .count();
    assert_eq!(
        child_started_count, 2,
        "Child processor should start once per file"
    );

    // Should have NodeCompleted for the child processor (runs per file).
    let child_completed_count = events
        .iter()
        .filter(|e| {
            matches!(e, PipelineEvent::NodeCompleted { node_id, .. } if node_id == "compress-image")
        })
        .count();
    assert_eq!(
        child_completed_count, 2,
        "Child processor should complete once per file"
    );
}

#[test]
fn test_recipe_clean_csv_event_sequence() {
    let def = parse_def(clean_csv_json());
    let registry = recipe_registry();
    let recorder = RecordingReporter::new();
    let reporter = recorder.reporter();

    let files = vec![make_file("data.csv", b"csv-content")];
    execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    let events = recorder.events();

    // PipelineStarted should report the csv-cleaner group as 1 processing
    // node at the top level (I/O nodes excluded). The group's children are
    // counted separately during sub-pipeline execution.
    if let PipelineEvent::PipelineStarted {
        total_nodes,
        total_files,
    } = &events[0]
    {
        assert_eq!(
            *total_nodes, 1,
            "1 top-level processing node (csv-cleaner group), I/O excluded"
        );
        assert_eq!(*total_files, 1);
    } else {
        panic!("First event should be PipelineStarted");
    }

    // NodeStarted for the csv-cleaner group + clean processor inside it = 2.
    let node_started_count = events
        .iter()
        .filter(|e| matches!(e, PipelineEvent::NodeStarted { .. }))
        .count();
    assert_eq!(
        node_started_count, 2,
        "Group + processor = 2 NodeStarted events"
    );
}

// --- Smoke Tests: All 6 Recipes Deserialize ---

#[test]
fn test_all_six_recipe_structures_deserialize() {
    // Verify every recipe structure can be parsed without error.
    // All 6 use the compositional pattern: Input → Group(sub-recipe) → Output.
    let recipes = [
        compress_images_json(),
        clean_csv_json(),
        rename_files_json(),
        // Resize: Input → Group → Loop → [image:resize] → Output
        r#"{
            "nodes": [
                { "id": "in", "type": "input", "parameters": {} },
                { "id": "batch-resize", "type": "group", "parameters": {}, "nodes": [
                    { "id": "loop", "type": "loop", "parameters": { "mode": "forEach" }, "nodes": [
                        { "id": "proc", "type": "image", "parameters": { "operation": "resize", "width": 200 } }
                    ]}
                ]},
                { "id": "out", "type": "output", "parameters": {} }
            ]
        }"#,
        // Convert: Input → Group → Loop → [image:convert] → Output
        r#"{
            "nodes": [
                { "id": "in", "type": "input", "parameters": {} },
                { "id": "batch-convert", "type": "group", "parameters": {}, "nodes": [
                    { "id": "loop", "type": "loop", "parameters": { "mode": "forEach" }, "nodes": [
                        { "id": "proc", "type": "image", "parameters": { "operation": "convert", "format": "webp" } }
                    ]}
                ]},
                { "id": "out", "type": "output", "parameters": {} }
            ]
        }"#,
        // Rename CSV columns: Input → Group → [spreadsheet:rename] → Output
        r#"{
            "nodes": [
                { "id": "in", "type": "input", "parameters": {} },
                { "id": "col-renamer", "type": "group", "parameters": {}, "nodes": [
                    { "id": "proc", "type": "spreadsheet", "parameters": { "operation": "rename", "columns": {} } }
                ]},
                { "id": "out", "type": "output", "parameters": {} }
            ]
        }"#,
    ];

    for (i, json) in recipes.iter().enumerate() {
        let result: Result<PipelineDefinition, _> = serde_json::from_str(json);
        assert!(
            result.is_ok(),
            "Recipe {} failed to deserialize: {:?}",
            i,
            result.err()
        );
    }
}

#[test]
fn test_all_six_recipes_execute_with_mocks() {
    // Run every recipe with the compositional sub-recipe pattern.
    let recipes = [
        compress_images_json(),
        clean_csv_json(),
        rename_files_json(),
        // Resize: Input → Group → Loop → [image:resize] → Output
        r#"{
            "nodes": [
                { "id": "in", "type": "input", "parameters": {} },
                { "id": "batch-resize", "type": "group", "parameters": {}, "nodes": [
                    { "id": "loop", "type": "loop", "parameters": {}, "nodes": [
                        { "id": "proc", "type": "image", "parameters": { "operation": "resize" } }
                    ]}
                ]},
                { "id": "out", "type": "output", "parameters": {} }
            ]
        }"#,
        // Convert: Input → Group → Loop → [image:convert] → Output
        r#"{
            "nodes": [
                { "id": "in", "type": "input", "parameters": {} },
                { "id": "batch-convert", "type": "group", "parameters": {}, "nodes": [
                    { "id": "loop", "type": "loop", "parameters": {}, "nodes": [
                        { "id": "proc", "type": "image", "parameters": { "operation": "convert" } }
                    ]}
                ]},
                { "id": "out", "type": "output", "parameters": {} }
            ]
        }"#,
        // Rename CSV columns: Input → Group → [spreadsheet:rename] → Output
        r#"{
            "nodes": [
                { "id": "in", "type": "input", "parameters": {} },
                { "id": "col-renamer", "type": "group", "parameters": {}, "nodes": [
                    { "id": "proc", "type": "spreadsheet", "parameters": { "operation": "rename" } }
                ]},
                { "id": "out", "type": "output", "parameters": {} }
            ]
        }"#,
    ];

    let registry = recipe_registry();
    let files = vec![make_file("test-file.dat", b"test-data")];

    for (i, json) in recipes.iter().enumerate() {
        let def = parse_def(json);
        let reporter = PipelineReporter::new_noop();
        let result = execute_pipeline(&def, files.clone(), &registry, &reporter, fake_now);
        assert!(
            result.is_ok(),
            "Recipe {} failed to execute: {:?}",
            i,
            result.err()
        );
        assert!(
            !result.unwrap().files.is_empty(),
            "Recipe {} produced no output files",
            i
        );
    }
}
