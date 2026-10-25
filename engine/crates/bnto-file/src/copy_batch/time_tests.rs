// file-copy against the run's time limit (ISS-60): the batch asks before each
// copy, and once the limit has passed it copies no more, failing with the
// run's timeout and handing over the files it copied.

use std::fs;
use std::path::Path;

use bnto_core::processor::{BatchFile, BatchInput, FileData, NodeOutput, NodeProcessor};
use bnto_core::progress::ProgressReporter;
use bnto_core::{BntoError, NoopContext, ProcessContext};
use serde_json::{Map, json};
use tempfile::TempDir;

use crate::copy::FileCopy;
use crate::time_runs_out::TimeRunsOut;

/// `a.txt`, `b.txt` and `c.txt` in `src/`, and an empty `out/` beside it.
fn three_files() -> (TempDir, Vec<BatchFile>) {
    let dir = TempDir::new().unwrap();
    let src = dir.path().join("src");
    fs::create_dir_all(&src).unwrap();
    let files = ["a.txt", "b.txt", "c.txt"]
        .iter()
        .map(|name| {
            fs::write(src.join(name), name).unwrap();
            BatchFile {
                data: FileData::Path(src.join(name)),
                filename: name.to_string(),
                mime_type: None,
                metadata: Map::new(),
            }
        })
        .collect();
    (dir, files)
}

fn copy_into_out(
    dir: &Path,
    files: Vec<BatchFile>,
    ctx: &dyn ProcessContext,
) -> Result<NodeOutput, BntoError> {
    let mut params = Map::new();
    let out = dir.join("out");
    params.insert("destination".into(), json!(out.to_str().unwrap()));
    let input = BatchInput { files, params };
    FileCopy::new().process_batch(input, &ProgressReporter::new_noop(), ctx)
}

fn copied(dir: &Path) -> Vec<String> {
    let Ok(entries) = fs::read_dir(dir.join("out")) else {
        return Vec::new();
    };
    let mut names: Vec<String> = entries
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

fn names(output: &NodeOutput) -> Vec<&str> {
    output.files.iter().map(|f| f.filename.as_str()).collect()
}

#[test]
fn test_a_run_out_of_time_after_the_first_copy_copies_no_more() {
    // RED (ISS-60): the batch kept copying past the run's limit until the
    // CLI's watchdog ended the process, which could cut a copy off halfway.
    let (dir, files) = three_files();
    let err = copy_into_out(dir.path(), files, &TimeRunsOut::after(1))
        .err()
        .expect("the run timed out");
    assert!(err.is_timeout(), "{err}");
    assert_eq!(err.to_string(), "Run timed out after 2s");
    assert_eq!(
        copied(dir.path()),
        ["a.txt"],
        "b.txt and c.txt never copied"
    );
    let (_, handed) = err.into_parts();
    let handed = handed.expect("the file it copied is handed over");
    assert_eq!(names(&handed), ["a.txt"]);
}

#[test]
fn test_a_run_already_out_of_time_copies_nothing() {
    let (dir, files) = three_files();
    let err = copy_into_out(dir.path(), files, &TimeRunsOut::after(0))
        .err()
        .expect("the run timed out");
    assert!(err.is_timeout(), "{err}");
    assert!(copied(dir.path()).is_empty());
    assert!(err.into_parts().1.is_none(), "nothing to hand over");
}

#[test]
fn test_a_run_without_a_limit_copies_every_file() {
    let (dir, files) = three_files();
    let output = copy_into_out(dir.path(), files, &NoopContext).unwrap();
    assert_eq!(copied(dir.path()), ["a.txt", "b.txt", "c.txt"]);
    assert_eq!(names(&output), ["a.txt", "b.txt", "c.txt"]);
}
