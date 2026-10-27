// file-delete failing partway (ISS-87): a delete that fails after others ran
// hands over the items finished before it, flagged, as the time limit does
// (time_tests.rs), so a closing step still sees what was deleted.

use std::path::Path;

use bnto_core::processor::{BatchFile, BatchInput, FileData, NodeOutput, NodeProcessor};
use bnto_core::progress::ProgressReporter;
use bnto_core::{BntoError, ProcessContext};
use serde_json::{Map, json};

use super::FileDelete;
use crate::changes_mid_run::ChangesMidRun;

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

/// Delete `a.txt`, `b.txt` and `c.txt` under `base`.
fn run(base: &Path, ctx: &dyn ProcessContext) -> Result<NodeOutput, BntoError> {
    let files = ["a.txt", "b.txt", "c.txt"]
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

#[test]
fn a_delete_that_fails_hands_over_the_items_before_it() {
    let tmp = three_files();
    let base = tmp.path().join("base");
    let b = base.join("b.txt");
    let ctx = ChangesMidRun::before_file(2, || std::fs::remove_file(&b).unwrap());

    let err = run(&base, &ctx).err().expect("deleting b.txt fails");

    assert!(!err.is_timeout(), "{err}");
    assert!(err.to_string().contains("Failed to delete b.txt"), "{err}");
    assert!(
        base.join("c.txt").exists(),
        "nothing deleted after the failure"
    );
    let handed = err.into_parts().1.expect("the deleted item is handed over");
    let names: Vec<&str> = handed.files.iter().map(|f| f.filename.as_str()).collect();
    assert_eq!(names, ["a.txt"]);
    assert_eq!(handed.files[0].metadata["deleted"], json!(1));
    assert_eq!(handed.metadata["deleted"], json!(1));
}

#[test]
fn a_first_delete_that_fails_hands_over_nothing() {
    let tmp = three_files();
    let base = tmp.path().join("base");
    let a = base.join("a.txt");
    let ctx = ChangesMidRun::before_file(1, || std::fs::remove_file(&a).unwrap());

    let err = run(&base, &ctx).err().expect("deleting a.txt fails");

    assert!(err.to_string().contains("Failed to delete a.txt"), "{err}");
    assert!(err.into_parts().1.is_none(), "nothing was deleted");
}
