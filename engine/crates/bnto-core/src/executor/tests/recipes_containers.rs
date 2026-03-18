// Nested container tests with recipe-style JSON structures.
use super::*;

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
