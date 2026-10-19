use std::path::{Path, PathBuf};

use bnto_core::NoopContext;
use serde_json::json;

use super::*;

/// A context with a work dir and temp files, both under one temp folder.
struct TempContext(tempfile::TempDir);

impl TempContext {
    fn new() -> Self {
        Self(tempfile::tempdir().unwrap())
    }
}

impl ProcessContext for TempContext {
    fn run_command(&self, _cmd: &str, _args: &[&str]) -> Result<Vec<u8>, BntoError> {
        unreachable!("expansion never runs a command")
    }
    fn temp_file(&self, suffix: &str) -> Result<PathBuf, BntoError> {
        let file = tempfile::Builder::new()
            .suffix(suffix)
            .tempfile_in(self.0.path())
            .unwrap();
        Ok(file.into_temp_path().keep().unwrap())
    }
    fn env_var(&self, _key: &str) -> Option<String> {
        None
    }
    fn work_dir(&self) -> Result<&Path, BntoError> {
        Ok(self.0.path())
    }
}

fn path_file(name: &str, path: &str) -> BatchFile {
    BatchFile {
        data: FileData::Path(PathBuf::from(path)),
        filename: name.to_string(),
        mime_type: None,
        metadata: serde_json::Map::new(),
    }
}

fn strings(values: Vec<Value>) -> Vec<String> {
    values
        .into_iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect()
}

/// An absolute path that means the same on every OS's `is_absolute`.
fn abs(name: &str) -> String {
    std::env::temp_dir()
        .join(name)
        .to_string_lossy()
        .into_owned()
}

#[test]
fn test_a_group_repeats_once_per_file_in_order() {
    let (a, b) = (abs("a.dll"), abs("b.dll"));
    let files = [path_file("a.dll", &a), path_file("b.dll", &b)];
    let args = [
        json!("report"),
        json!({ "each": ["-object={{path}}"] }),
        json!("-x"),
    ];
    let out = expand_each(&args, &files, &NoopContext).unwrap();
    assert_eq!(
        strings(out),
        [
            "report",
            &format!("-object={a}"),
            &format!("-object={b}"),
            "-x"
        ]
    );
}

#[test]
fn test_a_group_of_several_templates_keeps_them_together_per_file() {
    let files = [
        path_file("a.txt", &abs("a.txt")),
        path_file("b.txt", &abs("b.txt")),
    ];
    let args = [json!({ "each": ["--name", "{{file}}"] })];
    let out = expand_each(&args, &files, &NoopContext).unwrap();
    assert_eq!(strings(out), ["--name", "a.txt", "--name", "b.txt"]);
}

#[test]
fn test_with_no_files_a_group_adds_nothing() {
    let args = [json!("report"), json!({ "each": ["-object={{path}}"] })];
    let out = expand_each(&args, &[], &NoopContext).unwrap();
    assert_eq!(strings(out), ["report"]);
}

#[test]
fn test_args_without_a_group_are_kept_as_they_are() {
    let files = [path_file("a.txt", &abs("a.txt"))];
    let args = [json!("--version"), json!(["--flag", "on"])];
    let out = expand_each(&args, &files, &NoopContext).unwrap();
    assert_eq!(out, args);
}

#[test]
fn test_item_reads_the_files_metadata() {
    let mut file = path_file("a.txt", &abs("a.txt"));
    file.metadata.insert("size".into(), json!(12));
    file.metadata.insert("kind".into(), json!("text"));
    let args = [json!({ "each": ["{{item.kind}}:{{item.size}}"] })];
    let out = expand_each(&args, &[file], &NoopContext).unwrap();
    assert_eq!(strings(out), ["text:12"]);
}

#[test]
fn test_a_missing_item_key_names_the_key_file_and_keys_it_has() {
    let mut file = path_file("a.txt", &abs("a.txt"));
    file.metadata.insert("size".into(), json!(12));
    let args = [json!({ "each": ["{{item.kind}}"] })];
    let err = expand_each(&args, &[file], &NoopContext).unwrap_err();
    assert_eq!(
        err.to_string(),
        "Invalid input: 'args' reads {{item.kind}}, which a.txt doesn't have (it has: size)"
    );
}

#[test]
fn test_a_value_that_reads_like_a_token_is_written_as_it_is() {
    let mut file = path_file("a.txt", &abs("a.txt"));
    file.metadata.insert("note".into(), json!("{{file}}"));
    let args = [json!({ "each": ["{{item.note}} {{other}}"] })];
    let out = expand_each(&args, &[file], &NoopContext).unwrap();
    assert_eq!(strings(out), ["{{file}} {{other}}"]);
}

#[test]
fn test_a_relative_reference_is_under_the_work_dir() {
    let ctx = TempContext::new();
    let files = [path_file("a.dll", "bin/a.dll")];
    let args = [json!({ "each": ["{{path}}"] })];
    let out = strings(expand_each(&args, &files, &ctx).unwrap());
    let expected = ctx.0.path().join("bin/a.dll");
    assert_eq!(out, [expected.to_string_lossy()]);
}

#[test]
fn test_in_memory_bytes_are_written_to_a_temp_file_keeping_the_extension() {
    let ctx = TempContext::new();
    let file = BatchFile {
        data: FileData::Bytes(b"-object=x\n".to_vec()),
        filename: "objects.rsp".to_string(),
        mime_type: None,
        metadata: serde_json::Map::new(),
    };
    let args = [json!({ "each": ["@{{path}}"] })];
    let out = strings(expand_each(&args, &[file], &ctx).unwrap());
    let path = out[0].strip_prefix('@').unwrap();
    assert!(path.ends_with(".rsp"), "{path}");
    assert_eq!(std::fs::read(path).unwrap(), b"-object=x\n");
}

#[test]
fn test_bytes_are_not_written_when_no_group_reads_path() {
    let file = BatchFile {
        data: FileData::Bytes(b"x".to_vec()),
        filename: "a.txt".to_string(),
        mime_type: None,
        metadata: serde_json::Map::new(),
    };
    // NoopContext refuses temp files, so this only passes if none is made.
    let args = [json!({ "each": ["{{file}}"] })];
    let out = expand_each(&args, &[file], &NoopContext).unwrap();
    assert_eq!(strings(out), ["a.txt"]);
}

#[test]
fn test_malformed_groups_are_refused() {
    for bad in [
        json!({ "each": "{{path}}" }),
        json!({ "each": [1] }),
        json!({ "each": ["{{path}}"], "sep": "," }),
        json!({ "for": ["{{path}}"] }),
    ] {
        let err = check_groups(Some(&json!([bad]))).expect_err(&bad.to_string());
        assert!(err.contains("{\"each\": [text, ...]}"), "{err}");
    }
}

#[test]
fn test_well_formed_or_absent_groups_pass_the_check() {
    assert_eq!(check_groups(None), Ok(()));
    assert_eq!(check_groups(Some(&json!(["a", ["b"]]))), Ok(()));
    assert_eq!(check_groups(Some(&json!([{ "each": [] }]))), Ok(()));
}
