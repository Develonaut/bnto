use super::*;
use bnto_core::context::NoopContext;
use bnto_core::processor::FileData;
use bnto_core::progress::ProgressReporter;

/// Helper: create a NodeInput with the given data, filename, and params.
fn make_input(
    data: &[u8],
    filename: &str,
    params: serde_json::Map<String, serde_json::Value>,
) -> NodeInput {
    NodeInput {
        data: FileData::Bytes(data.to_vec()),
        filename: filename.to_string(),
        mime_type: None,
        params,
    }
}

/// Helper: build a params map from key-value pairs.
fn params(pairs: &[(&str, serde_json::Value)]) -> serde_json::Map<String, serde_json::Value> {
    let mut map = serde_json::Map::new();
    for (k, v) in pairs {
        map.insert(k.to_string(), v.clone());
    }
    map
}

// --- Trait basics ---

#[test]
fn test_name_returns_correct_key() {
    let processor = FileFilter::new();
    assert_eq!(processor.name(), "file-filter");
}

#[test]
fn test_metadata_has_correct_category() {
    let processor = FileFilter::new();
    let meta = processor.metadata();
    assert_eq!(meta.category, NodeCategory::File);
    assert_eq!(meta.node_type, "file-filter");
}

#[test]
fn test_validate_passes_with_no_params() {
    let processor = FileFilter::new();
    let errors = processor.validate(&serde_json::Map::new());
    assert!(errors.is_empty());
}

// --- Extension filtering ---

#[test]
fn test_filter_by_extension_keeps_matching() {
    let processor = FileFilter::new();
    let progress = ProgressReporter::new_noop();
    let input = make_input(
        b"image data",
        "photo.jpg",
        params(&[("extensions", serde_json::json!("jpg,png"))]),
    );

    let output = processor.process(input, &progress, &NoopContext).unwrap();
    assert_eq!(output.files.len(), 1, "Matching file should pass through");
    assert_eq!(output.files[0].filename, "photo.jpg");
}

#[test]
fn test_filter_by_extension_drops_non_matching() {
    let processor = FileFilter::new();
    let progress = ProgressReporter::new_noop();
    let input = make_input(
        b"text data",
        "readme.txt",
        params(&[("extensions", serde_json::json!("jpg,png"))]),
    );

    let output = processor.process(input, &progress, &NoopContext).unwrap();
    assert_eq!(output.files.len(), 0, "Non-matching file should be dropped");
}

#[test]
fn test_filter_extension_case_insensitive() {
    let processor = FileFilter::new();
    let progress = ProgressReporter::new_noop();
    let input = make_input(
        b"image data",
        "photo.JPG",
        params(&[("extensions", serde_json::json!("jpg"))]),
    );

    let output = processor.process(input, &progress, &NoopContext).unwrap();
    assert_eq!(
        output.files.len(),
        1,
        "Extension matching should be case-insensitive"
    );
}

#[test]
fn test_filter_extension_with_dots() {
    // User might write ".jpg" or "jpg" — both should work.
    let processor = FileFilter::new();
    let progress = ProgressReporter::new_noop();
    let input = make_input(
        b"image data",
        "photo.jpg",
        params(&[("extensions", serde_json::json!(".jpg,.png"))]),
    );

    let output = processor.process(input, &progress, &NoopContext).unwrap();
    assert_eq!(output.files.len(), 1, "Dotted extension should still match");
}

#[test]
fn test_filter_no_extension_param_passes_all() {
    let processor = FileFilter::new();
    let progress = ProgressReporter::new_noop();
    let input = make_input(b"data", "anything.xyz", serde_json::Map::new());

    let output = processor.process(input, &progress, &NoopContext).unwrap();
    assert_eq!(output.files.len(), 1, "No filter should pass everything");
}

// --- Name pattern filtering ---

#[test]
fn test_filter_by_glob_pattern() {
    let processor = FileFilter::new();
    let progress = ProgressReporter::new_noop();

    // "photo*" should match "photo_001.jpg"
    let input = make_input(
        b"data",
        "photo_001.jpg",
        params(&[("name_pattern", serde_json::json!("photo*"))]),
    );
    let output = processor.process(input, &progress, &NoopContext).unwrap();
    assert_eq!(output.files.len(), 1, "Glob pattern should match");

    // "photo*" should NOT match "document.pdf"
    let input = make_input(
        b"data",
        "document.pdf",
        params(&[("name_pattern", serde_json::json!("photo*"))]),
    );
    let output = processor.process(input, &progress, &NoopContext).unwrap();
    assert_eq!(output.files.len(), 0, "Glob pattern should not match");
}

#[test]
fn test_filter_by_regex_pattern() {
    let processor = FileFilter::new();
    let progress = ProgressReporter::new_noop();

    // Regex: match files starting with "img" followed by digits
    let input = make_input(
        b"data",
        "img42.png",
        params(&[
            ("name_pattern", serde_json::json!(r"^img\d+\.png$")),
            ("pattern_mode", serde_json::json!("regex")),
        ]),
    );
    let output = processor.process(input, &progress, &NoopContext).unwrap();
    assert_eq!(output.files.len(), 1, "Regex should match");

    // Same regex should NOT match "photo.png"
    let input = make_input(
        b"data",
        "photo.png",
        params(&[
            ("name_pattern", serde_json::json!(r"^img\d+\.png$")),
            ("pattern_mode", serde_json::json!("regex")),
        ]),
    );
    let output = processor.process(input, &progress, &NoopContext).unwrap();
    assert_eq!(output.files.len(), 0, "Regex should not match");
}

#[test]
fn test_filter_glob_with_question_mark() {
    let processor = FileFilter::new();
    let progress = ProgressReporter::new_noop();

    // "file?.txt" should match "file1.txt" but not "file10.txt"
    let input = make_input(
        b"data",
        "file1.txt",
        params(&[("name_pattern", serde_json::json!("file?.txt"))]),
    );
    let output = processor.process(input, &progress, &NoopContext).unwrap();
    assert_eq!(output.files.len(), 1);

    let input = make_input(
        b"data",
        "file10.txt",
        params(&[("name_pattern", serde_json::json!("file?.txt"))]),
    );
    let output = processor.process(input, &progress, &NoopContext).unwrap();
    assert_eq!(output.files.len(), 0);
}

// --- Size filtering ---

#[test]
fn test_filter_by_min_size() {
    let processor = FileFilter::new();
    let progress = ProgressReporter::new_noop();

    // 5 bytes, min_size=10 → dropped
    let input = make_input(
        b"small",
        "small.txt",
        params(&[("min_size", serde_json::json!(10))]),
    );
    let output = processor.process(input, &progress, &NoopContext).unwrap();
    assert_eq!(
        output.files.len(),
        0,
        "File below min_size should be dropped"
    );

    // 20 bytes, min_size=10 → kept
    let input = make_input(
        &[0u8; 20],
        "big.txt",
        params(&[("min_size", serde_json::json!(10))]),
    );
    let output = processor.process(input, &progress, &NoopContext).unwrap();
    assert_eq!(output.files.len(), 1, "File above min_size should pass");
}

#[test]
fn test_filter_by_max_size() {
    let processor = FileFilter::new();
    let progress = ProgressReporter::new_noop();

    // 20 bytes, max_size=10 → dropped
    let input = make_input(
        &[0u8; 20],
        "big.txt",
        params(&[("max_size", serde_json::json!(10))]),
    );
    let output = processor.process(input, &progress, &NoopContext).unwrap();
    assert_eq!(
        output.files.len(),
        0,
        "File above max_size should be dropped"
    );

    // 5 bytes, max_size=10 → kept
    let input = make_input(
        b"small",
        "small.txt",
        params(&[("max_size", serde_json::json!(10))]),
    );
    let output = processor.process(input, &progress, &NoopContext).unwrap();
    assert_eq!(output.files.len(), 1, "File below max_size should pass");
}

#[test]
fn test_filter_size_zero_means_no_limit() {
    let processor = FileFilter::new();
    let progress = ProgressReporter::new_noop();

    // max_size=0 → no limit (passes)
    let input = make_input(
        &[0u8; 1000],
        "big.txt",
        params(&[("max_size", serde_json::json!(0))]),
    );
    let output = processor.process(input, &progress, &NoopContext).unwrap();
    assert_eq!(output.files.len(), 1, "max_size=0 should mean no limit");
}

// --- Combined filters (AND logic) ---

#[test]
fn test_filter_combined_extension_and_size() {
    let processor = FileFilter::new();
    let progress = ProgressReporter::new_noop();

    // File is .jpg AND > 10 bytes → passes both
    let input = make_input(
        &[0u8; 20],
        "photo.jpg",
        params(&[
            ("extensions", serde_json::json!("jpg,png")),
            ("min_size", serde_json::json!(10)),
        ]),
    );
    let output = processor.process(input, &progress, &NoopContext).unwrap();
    assert_eq!(
        output.files.len(),
        1,
        "File matching both criteria should pass"
    );

    // File is .jpg BUT < 10 bytes → fails size
    let input = make_input(
        b"tiny",
        "photo.jpg",
        params(&[
            ("extensions", serde_json::json!("jpg,png")),
            ("min_size", serde_json::json!(10)),
        ]),
    );
    let output = processor.process(input, &progress, &NoopContext).unwrap();
    assert_eq!(
        output.files.len(),
        0,
        "File failing any criterion should be dropped"
    );

    // File is .txt AND > 10 bytes → fails extension
    let input = make_input(
        &[0u8; 20],
        "readme.txt",
        params(&[
            ("extensions", serde_json::json!("jpg,png")),
            ("min_size", serde_json::json!(10)),
        ]),
    );
    let output = processor.process(input, &progress, &NoopContext).unwrap();
    assert_eq!(
        output.files.len(),
        0,
        "File failing extension should be dropped"
    );
}

#[test]
fn test_filter_combined_pattern_and_extension() {
    let processor = FileFilter::new();
    let progress = ProgressReporter::new_noop();

    // "photo*" + extensions "jpg" → photo_001.jpg passes
    let input = make_input(
        b"data",
        "photo_001.jpg",
        params(&[
            ("name_pattern", serde_json::json!("photo*")),
            ("extensions", serde_json::json!("jpg")),
        ]),
    );
    let output = processor.process(input, &progress, &NoopContext).unwrap();
    assert_eq!(output.files.len(), 1);

    // "photo*" + extensions "jpg" → document.jpg fails pattern
    let input = make_input(
        b"data",
        "document.jpg",
        params(&[
            ("name_pattern", serde_json::json!("photo*")),
            ("extensions", serde_json::json!("jpg")),
        ]),
    );
    let output = processor.process(input, &progress, &NoopContext).unwrap();
    assert_eq!(output.files.len(), 0);
}

// --- Edge cases ---

#[test]
fn test_filter_preserves_file_data() {
    let processor = FileFilter::new();
    let progress = ProgressReporter::new_noop();
    let original_data = b"original file content";

    let input = make_input(original_data, "file.txt", serde_json::Map::new());
    let output = processor.process(input, &progress, &NoopContext).unwrap();

    assert_eq!(output.files.len(), 1);
    assert_eq!(
        output.files[0].data.clone().into_bytes().unwrap(),
        original_data,
        "Data should pass through unchanged"
    );
}

#[test]
fn test_filter_file_without_extension() {
    let processor = FileFilter::new();
    let progress = ProgressReporter::new_noop();

    // A file with no extension should be dropped when extension filter is active.
    let input = make_input(
        b"data",
        "Makefile",
        params(&[("extensions", serde_json::json!("txt"))]),
    );
    let output = processor.process(input, &progress, &NoopContext).unwrap();
    assert_eq!(output.files.len(), 0);
}

#[test]
fn test_filter_invalid_regex_drops_file() {
    let processor = FileFilter::new();
    let progress = ProgressReporter::new_noop();

    let input = make_input(
        b"data",
        "file.txt",
        params(&[
            ("name_pattern", serde_json::json!("[invalid")),
            ("pattern_mode", serde_json::json!("regex")),
        ]),
    );
    let output = processor.process(input, &progress, &NoopContext).unwrap();
    assert_eq!(
        output.files.len(),
        0,
        "Invalid regex should not match anything"
    );
}

// --- match / keep ---

/// Whether `filename` survives the filter with these params.
fn passes(filename: &str, p: &serde_json::Map<String, serde_json::Value>) -> bool {
    let input = make_input(b"data", filename, p.clone());
    let progress = ProgressReporter::new_noop();
    let output = FileFilter::new()
        .process(input, &progress, &NoopContext)
        .unwrap();
    output.files.len() == 1
}

/// Drop anything under a `node_modules`, `build` or `.cache` folder at any
/// depth, and any `.tmp` file.
fn drop_folders_params() -> serde_json::Map<String, serde_json::Value> {
    params(&[
        (
            "name_pattern",
            serde_json::json!(r"(?i)(^|/)(node_modules|build|\.cache)(/|$)|\.tmp$"),
        ),
        ("pattern_mode", serde_json::json!("regex")),
        ("match", serde_json::json!("path")),
        ("keep", serde_json::json!(false)),
    ])
}

#[test]
fn test_match_path_keep_false_drops_matching_paths() {
    let p = drop_folders_params();
    assert!(!passes("a/Node_Modules/x.js", &p));
    assert!(!passes("build/y", &p));
    assert!(!passes("b/.cache/z.bin", &p));
    assert!(!passes("m.tmp", &p));
}

#[test]
fn test_match_path_keep_false_keeps_non_matching_paths() {
    let p = drop_folders_params();
    assert!(passes("m.txt", &p));
    assert!(passes("builder/x", &p));
    assert!(passes("src/build.rs", &p));
}

#[test]
fn test_match_path_tests_the_whole_relative_path() {
    let p = params(&[
        ("name_pattern", serde_json::json!("^docs/")),
        ("pattern_mode", serde_json::json!("regex")),
        ("match", serde_json::json!("path")),
    ]);
    assert!(passes("docs/readme.md", &p));
    assert!(!passes("src/readme.md", &p));
}

#[test]
fn test_match_path_with_glob() {
    let p = params(&[
        ("name_pattern", serde_json::json!("docs/*")),
        ("match", serde_json::json!("path")),
    ]);
    assert!(passes("docs/readme.md", &p));
    assert!(!passes("readme.md", &p));
}

#[test]
fn test_match_defaults_to_name() {
    let p = params(&[
        ("name_pattern", serde_json::json!("^docs/")),
        ("pattern_mode", serde_json::json!("regex")),
    ]);
    assert!(
        !passes("docs/readme.md", &p),
        "name mode sees only readme.md"
    );
}

#[test]
fn test_keep_false_drops_what_every_criterion_matches() {
    let p = params(&[
        ("extensions", serde_json::json!("jpg")),
        ("keep", serde_json::json!(false)),
    ]);
    assert!(!passes("photo.jpg", &p));
    assert!(passes("notes.txt", &p));
}

#[test]
fn test_keep_true_is_the_default() {
    let p = params(&[("extensions", serde_json::json!("jpg"))]);
    assert!(passes("photo.jpg", &p));
    assert!(!passes("notes.txt", &p));
}

#[test]
fn test_validate_refuses_unknown_match() {
    let p = params(&[("match", serde_json::json!("folder"))]);
    let errors = FileFilter::new().validate(&p);
    assert_eq!(errors.len(), 1);
    assert!(
        errors[0].contains("match"),
        "error names the param: {errors:?}"
    );
}

#[test]
fn test_validate_accepts_name_and_path() {
    for mode in ["name", "path"] {
        let p = params(&[("match", serde_json::json!(mode))]);
        assert!(FileFilter::new().validate(&p).is_empty(), "{mode}");
    }
}

#[test]
fn test_validate_refuses_a_regex_that_does_not_compile() {
    let p = params(&[
        ("name_pattern", serde_json::json!("(")),
        ("pattern_mode", serde_json::json!("regex")),
    ]);
    let errors = FileFilter::new().validate(&p);
    assert_eq!(errors.len(), 1);
    assert!(
        errors[0].contains("name_pattern"),
        "error names the param: {errors:?}"
    );
    assert!(errors[0].ends_with("unclosed group"), "{errors:?}");
    assert!(!errors[0].contains('\n'), "one line: {errors:?}");
}

#[test]
fn test_validate_accepts_a_valid_regex() {
    let p = params(&[
        ("name_pattern", serde_json::json!(r"^img\d+\.png$")),
        ("pattern_mode", serde_json::json!("regex")),
    ]);
    assert!(FileFilter::new().validate(&p).is_empty());
}

#[test]
fn test_validate_leaves_glob_patterns_alone() {
    let p = params(&[("name_pattern", serde_json::json!("("))]);
    assert!(FileFilter::new().validate(&p).is_empty());
}

#[test]
fn test_validate_skips_a_regex_that_holds_a_placeholder() {
    // The value is only known once the placeholder resolves at run time.
    let p = params(&[
        ("name_pattern", serde_json::json!("^{{vars.Prefix}}")),
        ("pattern_mode", serde_json::json!("regex")),
    ]);
    assert!(FileFilter::new().validate(&p).is_empty());
}

#[test]
fn test_metadata_declares_match_and_keep() {
    let meta = FileFilter::new().metadata();
    let names: Vec<&str> = meta.parameters.iter().map(|p| p.name.as_str()).collect();
    assert!(names.contains(&"match"));
    assert!(names.contains(&"keep"));
}

// --- Glob patterns ---

/// Whether `filename` survives a glob `pattern` tested against its name.
fn glob_passes(pattern: &str, filename: &str) -> bool {
    passes(
        filename,
        &params(&[("name_pattern", serde_json::json!(pattern))]),
    )
}

#[test]
fn test_glob_star_matches_any_run_of_characters() {
    assert!(glob_passes("*.txt", "hello.txt"));
    assert!(!glob_passes("*.txt", "hello.jpg"));
    assert!(glob_passes("photo*", "photo_001.jpg"));
    assert!(glob_passes("*", "anything"));
    assert!(glob_passes("photo*", "photo"));
    assert!(glob_passes("a*b*c", "a-b-c"));
    assert!(!glob_passes("a*b*c", "a-c-b"));
}

#[test]
fn test_glob_question_matches_one_character() {
    assert!(glob_passes("file?.txt", "file1.txt"));
    assert!(!glob_passes("file?.txt", "file12.txt"));
    assert!(!glob_passes("file?.txt", "file.txt"));
    assert!(
        glob_passes("?.txt", "é.txt"),
        "? is one character, not one byte"
    );
}

#[test]
fn test_glob_matches_the_whole_name_case_sensitively() {
    assert!(!glob_passes("*.txt", "a.txt.bak"));
    assert!(!glob_passes("a.txt", "xa.txt"));
    assert!(!glob_passes("*.TXT", "a.txt"));
}

#[test]
fn test_glob_treats_regex_characters_literally() {
    assert!(glob_passes("a.b", "a.b"));
    assert!(!glob_passes("a.b", "axb"));
    assert!(glob_passes("c++.txt", "c++.txt"));
    assert!(!glob_passes("c++.txt", "ccc.txt"));
    assert!(glob_passes("photo (1).jpg", "photo (1).jpg"));
    assert!(glob_passes("[draft].md", "[draft].md"));
    assert!(!glob_passes("[draft].md", "d.md"));
    assert!(glob_passes("{a,b}.txt", "{a,b}.txt"));
    assert!(!glob_passes("{a,b}.txt", "a.txt"));
    assert!(glob_passes("^a$|b", "^a$|b"));
    assert!(glob_passes(r"a", r"a"));
    assert!(glob_passes("#-&~ x.txt", "#-&~ x.txt"));
}

#[test]
fn test_glob_matches_a_name_with_a_newline() {
    // Unix names may hold a newline; `*` and `?` still cover it.
    assert!(glob_passes(
        "*", "a
b"
    ));
    assert!(glob_passes(
        "a*b", "a
b"
    ));
    assert!(glob_passes(
        "a?b", "a
b"
    ));
}

#[test]
fn test_glob_star_crosses_folders_when_matching_paths() {
    let p = params(&[
        ("name_pattern", serde_json::json!("docs/*.md")),
        ("match", serde_json::json!("path")),
    ]);
    assert!(passes("docs/guide/intro.md", &p));
    assert!(!passes("src/docs/intro.md", &p));
}

#[test]
fn test_regex_searches_anywhere_in_the_name() {
    let p = params(&[
        ("name_pattern", serde_json::json!(r"\d{3}")),
        ("pattern_mode", serde_json::json!("regex")),
    ]);
    assert!(passes("IMG_123.jpg", &p));
    assert!(!passes("IMG_12.jpg", &p));
}

#[test]
fn test_one_filter_follows_each_files_own_pattern() {
    // One processor serves every file of a run, and a `{{item.*}}` pattern
    // can differ per file, so a changed pattern or mode must take effect.
    let filter = FileFilter::new();
    let run = |pattern: &str, mode: &str, filename: &str| {
        let p = params(&[
            ("name_pattern", serde_json::json!(pattern)),
            ("pattern_mode", serde_json::json!(mode)),
        ]);
        let input = make_input(b"data", filename, p);
        let progress = ProgressReporter::new_noop();
        let output = filter.process(input, &progress, &NoopContext).unwrap();
        output.files.len() == 1
    };
    assert!(run("*.txt", "glob", "a.txt"));
    assert!(!run("*.md", "glob", "a.txt"));
    assert!(run("*.txt", "glob", "a.txt"));
    assert!(!run("a.b", "glob", "axb"), "glob: `.` is literal");
    assert!(run("a.b", "regex", "axb"), "regex: `.` is any character");
    assert!(!run("[bad", "regex", "a.txt"));
    assert!(run("[bad", "glob", "[bad"));
}

#[test]
fn test_matches_extension_whitespace_handling() {
    let p = params(&[("extensions", serde_json::json!(" jpg , png , svg "))]);
    assert!(matches_extension("photo.jpg", &p));
    assert!(matches_extension("icon.svg", &p));
    assert!(!matches_extension("doc.pdf", &p));
}
