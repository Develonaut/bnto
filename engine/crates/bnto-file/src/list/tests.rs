use super::*;
use bnto_core::context::NoopContext;
use serde_json::json;
use std::path::Path;

/// A base folder holding `a.txt`, `sub/b.txt` and the folder `dir/`.
fn base_dir() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("a.txt"), "a").unwrap();
    std::fs::create_dir_all(dir.path().join("sub")).unwrap();
    std::fs::write(dir.path().join("sub").join("b.txt"), "b").unwrap();
    std::fs::create_dir_all(dir.path().join("dir")).unwrap();
    dir
}

fn list_file(content: &[u8]) -> BatchFile {
    BatchFile {
        data: FileData::Bytes(content.to_vec()),
        filename: "list.txt".to_string(),
        mime_type: None,
        metadata: serde_json::Map::new(),
    }
}

fn run(
    base: &Path,
    files: Vec<BatchFile>,
    extra: serde_json::Value,
) -> Result<NodeOutput, BntoError> {
    let mut params = extra.as_object().cloned().unwrap_or_default();
    params.insert("base".to_string(), json!(base.to_str().unwrap()));
    let input = BatchInput { files, params };
    FileList::new().process_batch(input, &ProgressReporter::new_noop(), &NoopContext)
}

fn names(output: &NodeOutput) -> Vec<&str> {
    output.files.iter().map(|f| f.filename.as_str()).collect()
}

// --- Reading lists ---

#[test]
fn test_nul_separated_input_gives_one_path_file_per_entry() {
    let base = base_dir();
    let out = run(
        base.path(),
        vec![list_file(b"a.txt\0sub/b.txt\0")],
        json!({}),
    )
    .unwrap();
    assert_eq!(names(&out), ["a.txt", "sub/b.txt"]);
    match &out.files[1].data {
        FileData::Path(p) => assert_eq!(p, &base.path().join("sub/b.txt")),
        other => panic!("expected a path reference, got {other:?}"),
    }
}

#[test]
fn test_newline_separated_input_works_the_same() {
    let base = base_dir();
    let out = run(
        base.path(),
        vec![list_file(b"a.txt\r\nsub/b.txt\n\n")],
        json!({}),
    )
    .unwrap();
    assert_eq!(names(&out), ["a.txt", "sub/b.txt"]);
}

#[test]
fn test_an_empty_input_gives_no_files() {
    let base = base_dir();
    let out = run(base.path(), vec![list_file(b"")], json!({})).unwrap();
    assert!(out.files.is_empty());
}

#[test]
fn test_paths_param_lists_with_no_input() {
    let base = base_dir();
    let out = run(base.path(), vec![], json!({ "paths": ["sub/b.txt"] })).unwrap();
    assert_eq!(names(&out), ["sub/b.txt"]);
}

#[test]
fn test_input_lists_and_paths_combine_without_repeats() {
    let base = base_dir();
    let extra = json!({ "paths": ["sub/b.txt", "a.txt"] });
    let out = run(base.path(), vec![list_file(b"a.txt\n")], extra).unwrap();
    assert_eq!(names(&out), ["a.txt", "sub/b.txt"]);
}

#[test]
fn test_backslash_separators_become_slashes() {
    let base = base_dir();
    let out = run(base.path(), vec![], json!({ "paths": ["sub\\b.txt"] })).unwrap();
    assert_eq!(names(&out), ["sub/b.txt"]);
}

#[test]
fn test_a_directory_is_listed() {
    let base = base_dir();
    let out = run(base.path(), vec![], json!({ "paths": ["dir"] })).unwrap();
    assert_eq!(names(&out), ["dir"]);
}

#[test]
fn test_list_that_is_not_utf8_is_an_error_naming_the_file() {
    let base = base_dir();
    let err = run(base.path(), vec![list_file(&[0xff, 0xfe, b'a'])], json!({}))
        .err()
        .expect("the run should fail");
    assert!(err.to_string().contains("list.txt"), "{err}");
}

// --- Escaping and missing paths ---

#[test]
fn test_paths_that_escape_the_base_fail_naming_each() {
    let base = base_dir();
    let extra = json!({ "paths": ["../x", "C:/x", "/x", "a.txt"] });
    let err = run(base.path(), vec![], extra)
        .err()
        .expect("the run should fail")
        .to_string();
    for bad in ["../x", "C:/x", "/x"] {
        assert!(err.contains(bad), "{bad} not named: {err}");
    }
    assert!(!err.contains("\"a.txt\""), "{err}");
}

#[test]
fn test_a_missing_path_is_skipped_and_counted() {
    let base = base_dir();
    let extra = json!({ "paths": ["a.txt", "gone.txt", "sub/gone"] });
    let out = run(base.path(), vec![], extra).unwrap();
    assert_eq!(names(&out), ["a.txt"]);
    assert_eq!(out.metadata["missing"], json!(["gone.txt", "sub/gone"]));
    assert_eq!(out.metadata["missing_count"], json!(2));
}

#[test]
fn test_nothing_missing_counts_zero() {
    let base = base_dir();
    let out = run(base.path(), vec![], json!({ "paths": ["a.txt"] })).unwrap();
    assert_eq!(out.metadata["missing"], json!([]));
    assert_eq!(out.metadata["missing_count"], json!(0));
}

// --- Params and metadata ---

#[test]
fn test_validate_refuses_paths_that_are_not_a_list_of_text() {
    let p = json!({ "paths": [1, 2] });
    let errors = FileList::new().validate(p.as_object().unwrap());
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert!(errors[0].contains("paths"), "{errors:?}");
    assert!(FileList::new().validate(&serde_json::Map::new()).is_empty());
}

#[test]
fn test_metadata_is_a_native_batch_node() {
    let meta = FileList::new().metadata();
    assert_eq!(meta.node_type, "file-list");
    assert_eq!(meta.input_cardinality, InputCardinality::Batch);
    assert!(!meta.platforms.contains(&"browser".to_string()));
    let params: Vec<&str> = meta.parameters.iter().map(|p| p.name.as_str()).collect();
    assert_eq!(params, ["paths", "base"]);
}
