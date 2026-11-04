// The file nodes resolve a relative path param against the run's working
// folder (`ProcessContext::work_dir`), never the process's own folder, and
// `file-copy` refuses a name that would leave its destination (ISS-123,
// ISS-128). Each test runs in a context whose working folder is a temp dir,
// which the test process's folder (the crate) never is.

use std::path::{Path, PathBuf};

use bnto_core::processor::{BatchFile, BatchInput, FileData, NodeOutput, NodeProcessor};
use bnto_core::progress::ProgressReporter;
use bnto_core::{BntoError, NoopContext, ProcessContext};
use serde_json::{Value, json};

use crate::{FileCopy, FileDelete, FileList, FileMove};

/// A context whose working folder is `0`.
struct InFolder(PathBuf);

impl ProcessContext for InFolder {
    fn run_command(&self, cmd: &str, args: &[&str]) -> Result<Vec<u8>, BntoError> {
        NoopContext.run_command(cmd, args)
    }
    fn temp_file(&self, suffix: &str) -> Result<PathBuf, BntoError> {
        NoopContext.temp_file(suffix)
    }
    fn env_var(&self, key: &str) -> Option<String> {
        NoopContext.env_var(key)
    }
    fn work_dir(&self) -> Result<&Path, BntoError> {
        Ok(&self.0)
    }
}

/// A working folder holding `in/a.txt`, and a context in it.
fn work() -> (tempfile::TempDir, InFolder) {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(tmp.path().join("in")).unwrap();
    std::fs::write(tmp.path().join("in/a.txt"), "a").unwrap();
    let ctx = InFolder(tmp.path().to_path_buf());
    (tmp, ctx)
}

/// `in/a.txt` as a path reference, named `name`.
fn a_txt(tmp: &tempfile::TempDir, name: &str) -> BatchFile {
    BatchFile {
        data: FileData::Path(tmp.path().join("in/a.txt")),
        filename: name.to_string(),
        mime_type: Some("text/plain".to_string()),
        metadata: serde_json::Map::new(),
    }
}

fn run(
    node: &dyn NodeProcessor,
    files: Vec<BatchFile>,
    params: Value,
    ctx: &dyn ProcessContext,
) -> Result<NodeOutput, BntoError> {
    let params = params.as_object().unwrap().clone();
    let input = BatchInput { files, params };
    node.process_batch(input, &ProgressReporter::new_noop(), ctx)
}

fn the_path(out: &NodeOutput) -> &Path {
    match &out.files[0].data {
        FileData::Path(p) => p,
        other => panic!("expected a path reference, got {other:?}"),
    }
}

// --- file-list ---

#[test]
fn test_file_list_lists_a_relative_base_under_the_work_dir() {
    let (tmp, ctx) = work();
    let params = json!({ "base": "in", "paths": ["a.txt"] });
    let out = run(&FileList::new(), vec![], params, &ctx).unwrap();
    assert_eq!(the_path(&out), tmp.path().join("in").join("a.txt"));
}

#[test]
fn test_file_list_refuses_an_absolute_or_climbing_base_naming_the_param() {
    let (tmp, ctx) = work();
    let abs = tmp.path().join("in");
    for base in [abs.to_str().unwrap(), "../x", "in/../../x", "/x"] {
        let params = json!({ "base": base, "paths": ["a.txt"] });
        let err = run(&FileList::new(), vec![], params.clone(), &ctx)
            .err()
            .unwrap();
        assert!(err.to_string().contains("'base'"), "{base}: {err}");
        let found = FileList::new().validate(params.as_object().unwrap());
        assert!(
            found.iter().any(|e| e.contains("'base'")),
            "{base}: {found:?}"
        );
    }
}

// --- file-delete ---

#[test]
fn test_file_delete_takes_a_relative_base_under_the_work_dir() {
    let (tmp, ctx) = work();
    let params = json!({ "base": "in" });
    run(&FileDelete::new(), vec![a_txt(&tmp, "a.txt")], params, &ctx).unwrap();
    assert!(!tmp.path().join("in/a.txt").exists());
}

// --- file-move ---

#[test]
fn test_file_move_takes_a_relative_destination_under_the_work_dir() {
    let (tmp, ctx) = work();
    let params = json!({ "destination": "sorted" });
    run(&FileMove::new(), vec![a_txt(&tmp, "a.txt")], params, &ctx).unwrap();
    let moved = tmp.path().join("sorted").join("a.txt");
    assert_eq!(std::fs::read_to_string(moved).unwrap(), "a");
}

// --- file-copy ---

#[test]
fn test_file_copy_takes_a_relative_destination_under_the_work_dir() {
    let (tmp, ctx) = work();
    let params = json!({ "destination": "out" });
    run(&FileCopy::new(), vec![a_txt(&tmp, "a.txt")], params, &ctx).unwrap();
    let copied = tmp.path().join("out").join("a.txt");
    assert_eq!(std::fs::read_to_string(copied).unwrap(), "a");
}

#[test]
fn test_file_copy_refuses_a_name_that_leaves_the_destination_before_copying() {
    let (tmp, ctx) = work();
    let outside = tmp.path().join("evil.txt");
    let bad = [
        "../evil.txt".to_string(),
        "sub/../../evil.txt".to_string(),
        outside.to_str().unwrap().to_string(),
        "\\evil.txt".to_string(),
    ];
    for name in &bad {
        // A good file first: the refusal comes before any copy, its own too.
        let files = vec![a_txt(&tmp, "good.txt"), a_txt(&tmp, name)];
        let params = json!({ "destination": "out/deep", "conflict": "overwrite" });
        let err = run(&FileCopy::new(), files, params, &ctx).err().unwrap();
        assert!(err.to_string().contains("would leave"), "{name}: {err}");
        assert!(!tmp.path().join("out").exists(), "{name}: copied something");
        assert!(!outside.exists(), "{name}: wrote outside");
    }
}

#[test]
fn test_file_copy_refuses_a_root_or_drive_relative_destination_naming_it() {
    // `\x` is the drive's root and `C:x` the process's folder on C, on
    // Windows; refused on every OS so a recipe means the same everywhere.
    let (tmp, ctx) = work();
    for destination in ["\\out", "C:out"] {
        let params = json!({ "destination": destination });
        let err = run(&FileCopy::new(), vec![a_txt(&tmp, "a.txt")], params, &ctx)
            .err()
            .unwrap();
        assert!(err.to_string().contains("'destination'"), "{err}");
    }
}

#[test]
fn test_file_copy_refuses_a_name_this_os_cant_hold_before_copying() {
    let (tmp, ctx) = work();
    let mut bad = vec!["sub/a\0b.txt"];
    if cfg!(windows) {
        bad.extend(["sub/CON.txt", "sub/COM1", "a./b.txt", "x/.. /y.txt"]);
    }
    for name in bad {
        let files = vec![a_txt(&tmp, "good.txt"), a_txt(&tmp, name)];
        let params = json!({ "destination": "out" });
        let err = run(&FileCopy::new(), files, params, &ctx).err().unwrap();
        assert!(err.to_string().contains("refused"), "{name:?}: {err}");
        assert!(
            !tmp.path().join("out").exists(),
            "{name:?}: copied something"
        );
    }
}

#[cfg(not(windows))]
#[test]
fn test_file_copy_keeps_a_dot_dot_space_folder_inside_the_destination() {
    // Only Windows drops a trailing space, so elsewhere `.. ` is a plain
    // folder name and stays under the destination.
    let (tmp, ctx) = work();
    let params = json!({ "destination": "out" });
    run(
        &FileCopy::new(),
        vec![a_txt(&tmp, "x/.. /y.txt")],
        params,
        &ctx,
    )
    .unwrap();
    assert!(tmp.path().join("out/x/.. /y.txt").exists());
    assert!(!tmp.path().join("y.txt").exists());
}

#[test]
fn test_file_copy_plans_a_dotted_name_as_the_file_it_names() {
    // `./a.txt` under `out` is `out/a.txt`: the replace check names it so,
    // not as a second spelling.
    let (tmp, ctx) = work();
    let out = tmp.path().join("out");
    std::fs::create_dir_all(&out).unwrap();
    std::fs::write(out.join("a.txt"), "read").unwrap();
    let reader = BatchFile {
        data: FileData::Path(out.join("a.txt")),
        ..a_txt(&tmp, "keep.txt")
    };
    let files = vec![a_txt(&tmp, "./a.txt"), reader];
    let params = json!({ "destination": "out", "conflict": "overwrite" });
    let err = run(&FileCopy::new(), files, params, &ctx).err().unwrap();
    let named = format!("{}:", out.join("a.txt").display());
    assert!(err.to_string().contains(&named), "{err}");
    assert_eq!(std::fs::read_to_string(out.join("a.txt")).unwrap(), "read");
}

/// A working folder `<tmp>/work` holding `in/a.txt` and an empty `out/`, so
/// anything written beside the working folder lands in `<tmp>`, where the
/// test sees it.
fn nested_work() -> (tempfile::TempDir, InFolder) {
    let tmp = tempfile::tempdir().unwrap();
    let work = tmp.path().join("work");
    std::fs::create_dir_all(work.join("in")).unwrap();
    std::fs::create_dir_all(work.join("out")).unwrap();
    std::fs::write(work.join("in/a.txt"), "a").unwrap();
    (tmp, InFolder(work))
}

/// Every path under `dir`, relative and sorted.
fn tree(dir: &Path) -> Vec<String> {
    let mut found = Vec::new();
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        let name = path.strip_prefix(dir).unwrap().display().to_string();
        if path.is_dir() {
            found.extend(tree(&path).into_iter().map(|p| format!("{name}/{p}")));
        }
        found.push(name);
    }
    found.sort();
    found
}

#[test]
fn test_file_copy_refuses_a_name_with_no_file_name_in_every_conflict_mode() {
    // A name of only `.` segments is the destination folder itself, so
    // `rename` once wrote `<parent>/<destination>_1`, outside it.
    for destination in [Some("out"), None] {
        for conflict in ["rename", "overwrite", "newer", "skip"] {
            for name in [".", "", "./.", "./", "/."] {
                let (tmp, ctx) = nested_work();
                let before = tree(tmp.path());
                let src = tmp.path().join("work/in/a.txt");
                let file = BatchFile {
                    data: FileData::Path(src),
                    filename: name.to_string(),
                    mime_type: None,
                    metadata: serde_json::Map::new(),
                };
                let mut params = json!({ "conflict": conflict });
                if let Some(d) = destination {
                    params["destination"] = json!(d);
                }
                let at = format!("{destination:?} {conflict} {name:?}");
                let err = run(&FileCopy::new(), vec![file], params, &ctx).err();
                let err = err.unwrap_or_else(|| panic!("{at}: copied"));
                assert!(err.to_string().contains("refused"), "{at}: {err}");
                assert_eq!(tree(tmp.path()), before, "{at}: wrote something");
            }
        }
    }
}

#[test]
fn test_file_copy_says_a_dot_name_has_no_file_name() {
    let (tmp, ctx) = nested_work();
    let file = BatchFile {
        data: FileData::Path(tmp.path().join("work/in/a.txt")),
        filename: "./.".to_string(),
        mime_type: None,
        metadata: serde_json::Map::new(),
    };
    let params = json!({ "destination": "out", "conflict": "rename" });
    let err = run(&FileCopy::new(), vec![file], params, &ctx)
        .err()
        .unwrap();
    assert!(
        err.to_string().contains("\"./.\" has no file name"),
        "{err}"
    );
}

#[test]
fn test_file_move_refuses_a_name_with_no_file_name() {
    for name in [".", "", "./."] {
        let (tmp, ctx) = nested_work();
        let before = tree(tmp.path());
        let file = BatchFile {
            data: FileData::Path(tmp.path().join("work/in/a.txt")),
            filename: name.to_string(),
            mime_type: None,
            metadata: serde_json::Map::new(),
        };
        let params = json!({ "destination": "out" });
        let err = run(&FileMove::new(), vec![file], params, &ctx)
            .err()
            .unwrap();
        assert!(
            err.to_string().contains("has no file name"),
            "{name:?}: {err}"
        );
        assert_eq!(tree(tmp.path()), before, "{name:?}: moved something");
    }
}
