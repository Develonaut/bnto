// file-delete against the run's time limit (ISS-60): the batch asks before
// each delete, and once the limit has passed it deletes no more, failing with
// the run's timeout and handing over the items it finished, flagged.

use std::path::Path;

use bnto_core::processor::{BatchFile, BatchInput, FileData, NodeOutput, NodeProcessor};
use bnto_core::progress::ProgressReporter;
use bnto_core::{BntoError, ProcessContext};
use serde_json::{Map, json};

use super::FileDelete;
use crate::time_runs_out::TimeRunsOut;

/// A temp dir whose `base/` holds `a.txt`, `b.txt` and `c.txt`.
fn three_files() -> tempfile::TempDir {
    let tmp = tempfile::tempdir().unwrap();
    let base = tmp.path().join("base");
    std::fs::create_dir_all(&base).unwrap();
    for name in ["a.txt", "b.txt", "c.txt"] {
        std::fs::write(base.join(name), name).unwrap();
    }
    tmp
}

/// Delete `names` under `base`, each an item named by its relative path.
fn run(base: &Path, names: &[&str], ctx: &dyn ProcessContext) -> Result<NodeOutput, BntoError> {
    let files = names
        .iter()
        .map(|name| BatchFile {
            data: FileData::Path(base.join(name)),
            filename: name.to_string(),
            mime_type: None,
            metadata: Map::new(),
        })
        .collect();
    let mut params = Map::new();
    params.insert("base".into(), json!(base.to_str().unwrap()));
    let input = BatchInput { files, params };
    FileDelete::new().process_batch(input, &ProgressReporter::new_noop(), ctx)
}

fn names(output: &NodeOutput) -> Vec<&str> {
    output.files.iter().map(|f| f.filename.as_str()).collect()
}

#[test]
fn test_a_run_out_of_time_after_the_first_delete_deletes_no_more() {
    // RED (ISS-60): the batch kept deleting past the run's limit.
    let tmp = three_files();
    let base = tmp.path().join("base");
    let all = ["a.txt", "b.txt", "c.txt"];
    let err = run(&base, &all, &TimeRunsOut::after(1))
        .err()
        .expect("the run timed out");
    assert!(err.is_timeout(), "{err}");
    assert_eq!(err.to_string(), "Run timed out after 2s");
    assert!(!base.join("a.txt").exists());
    assert!(base.join("b.txt").exists(), "b.txt never deleted");
    assert!(base.join("c.txt").exists(), "c.txt never deleted");
    let handed = err
        .into_parts()
        .1
        .expect("the item it deleted is handed over");
    assert_eq!(names(&handed), ["a.txt"]);
    assert_eq!(handed.files[0].metadata["deleted"], json!(1));
    assert_eq!(handed.metadata["deleted"], json!(1));
    assert_eq!(handed.metadata["skipped"], json!(0));
}

#[test]
fn test_a_run_already_out_of_time_deletes_nothing() {
    let tmp = three_files();
    let base = tmp.path().join("base");
    let err = run(&base, &["a.txt", "b.txt"], &TimeRunsOut::after(0))
        .err()
        .expect("the run timed out");
    assert!(err.is_timeout(), "{err}");
    assert!(base.join("a.txt").exists() && base.join("b.txt").exists());
    assert!(err.into_parts().1.is_none(), "nothing to hand over");
}

#[test]
fn test_a_missing_item_takes_no_time_and_is_handed_over_as_skipped() {
    // A file already gone needs no delete, so it doesn't ask the run.
    let tmp = three_files();
    let base = tmp.path().join("base");
    let names_in = ["gone.txt", "a.txt", "b.txt"];
    let err = run(&base, &names_in, &TimeRunsOut::after(1))
        .err()
        .expect("the run timed out");
    assert!(err.is_timeout(), "{err}");
    assert!(!base.join("a.txt").exists());
    assert!(base.join("b.txt").exists(), "b.txt never deleted");
    let handed = err.into_parts().1.expect("finished items are handed over");
    assert_eq!(names(&handed), ["gone.txt", "a.txt"]);
    assert_eq!(handed.metadata["deleted"], json!(1));
    assert_eq!(handed.metadata["skipped"], json!(1));
}
