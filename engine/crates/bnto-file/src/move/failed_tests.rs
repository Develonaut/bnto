// file-move failing partway (ISS-87): a rename that fails after others ran
// hands over the files now under the destination, as the time limit does
// (time_tests.rs), so a closing step still sees what was moved.

use std::path::Path;

use bnto_core::processor::{BatchFile, BatchInput, FileData, NodeOutput, NodeProcessor};
use bnto_core::progress::ProgressReporter;
use bnto_core::{BntoError, ProcessContext};
use serde_json::{Map, json};

use super::FileMove;
use crate::changes_mid_run::ChangesMidRun;

/// Move `a.wav` and `b.wav` inside `dir` to `dir/dest/Pack/`.
fn run(dir: &Path, ctx: &dyn ProcessContext) -> Result<NodeOutput, BntoError> {
    let files = ["a.wav", "b.wav"]
        .iter()
        .map(|name| BatchFile {
            data: FileData::Path(dir.join(name)),
            filename: format!("Pack/{name}"),
            mime_type: None,
            metadata: Map::new(),
        })
        .collect();
    let dest = dir.join("dest");
    let params = json!({ "destination": dest.to_string_lossy() });
    let input = BatchInput {
        files,
        params: params.as_object().unwrap().clone(),
    };
    FileMove::new().process_batch(input, &ProgressReporter::new_noop(), ctx)
}

#[test]
fn a_rename_that_finds_its_target_taken_hands_over_the_first_file() {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::write(tmp.path().join("a.wav"), "A").unwrap();
    std::fs::write(tmp.path().join("b.wav"), "B").unwrap();
    let pack = tmp.path().join("dest/Pack");
    let taken = pack.join("b.wav");
    let ctx = ChangesMidRun::before_file(2, || std::fs::write(&taken, "X").unwrap());

    let err = run(tmp.path(), &ctx)
        .err()
        .expect("the second rename fails");

    assert!(!err.is_timeout(), "{err}");
    assert!(err.to_string().contains("now exists"), "{err}");
    assert_eq!(
        std::fs::read_to_string(&taken).unwrap(),
        "X",
        "never replaced"
    );
    let handed = err.into_parts().1.expect("the moved file is handed over");
    let names: Vec<&str> = handed.files.iter().map(|f| f.filename.as_str()).collect();
    assert_eq!(names, ["Pack/a.wav"]);
    assert!(matches!(&handed.files[0].data, FileData::Path(p) if p.ends_with("dest/Pack/a.wav")));
}

#[test]
fn a_first_rename_that_fails_hands_over_nothing() {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::write(tmp.path().join("a.wav"), "A").unwrap();
    std::fs::write(tmp.path().join("b.wav"), "B").unwrap();
    let taken = tmp.path().join("dest/Pack/a.wav");
    let ctx = ChangesMidRun::before_file(1, || {
        std::fs::create_dir_all(taken.parent().unwrap()).unwrap();
        std::fs::write(&taken, "X").unwrap();
    });

    let err = run(tmp.path(), &ctx).err().expect("the first rename fails");

    assert!(err.to_string().contains("now exists"), "{err}");
    assert!(err.into_parts().1.is_none(), "nothing was moved");
}
