// file-move against the run's time limit (ISS-60): the batch asks before each
// move, and once the limit has passed it moves no more, failing with the
// run's timeout and handing over the files now under the destination. A swap
// is never left half done: no file stays under a temporary name.

use std::path::Path;

use bnto_core::processor::{BatchFile, BatchInput, FileData, NodeOutput, NodeProcessor};
use bnto_core::progress::ProgressReporter;
use bnto_core::{BntoError, ProcessContext};
use serde_json::{Map, json};

use super::FileMove;
use crate::time_runs_out::TimeRunsOut;

/// A temp dir holding `files` (relative path, content).
pub(super) fn dir_with(files: &[(&str, &str)]) -> tempfile::TempDir {
    let tmp = tempfile::tempdir().unwrap();
    for (name, content) in files {
        let path = tmp.path().join(name);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, content).unwrap();
    }
    tmp
}

/// Move `(source, name)` pairs inside `dir` to names under `dir/dest`.
pub(super) fn run(
    dir: &Path,
    moves: &[(&str, &str)],
    ctx: &dyn ProcessContext,
) -> Result<NodeOutput, BntoError> {
    let files = moves
        .iter()
        .map(|(src, name)| BatchFile {
            data: FileData::Path(dir.join(src)),
            filename: name.to_string(),
            mime_type: None,
            metadata: Map::new(),
        })
        .collect();
    let dest = dir.join("dest");
    let params = json!({ "destination": dest.to_string_lossy() });
    let params = params.as_object().unwrap().clone();
    let input = BatchInput { files, params };
    FileMove::new().process_batch(input, &ProgressReporter::new_noop(), ctx)
}

pub(super) fn names(output: &NodeOutput) -> Vec<&str> {
    output.files.iter().map(|f| f.filename.as_str()).collect()
}

pub(super) fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

const THREE: [(&str, &str); 3] = [
    ("a.wav", "Pack/a.wav"),
    ("b.wav", "Pack/b.wav"),
    ("c.wav", "Pack/c.wav"),
];

#[test]
fn test_a_run_out_of_time_after_the_first_move_moves_no_more() {
    // RED (ISS-60): the batch kept moving past the run's limit.
    let tmp = dir_with(&[("a.wav", "A"), ("b.wav", "B"), ("c.wav", "C")]);
    let err = run(tmp.path(), &THREE, &TimeRunsOut::after(1))
        .err()
        .expect("the run timed out");
    assert!(err.is_timeout(), "{err}");
    assert_eq!(err.to_string(), "Run timed out after 2s");
    let dest = tmp.path().join("dest/Pack");
    assert_eq!(read(&dest.join("a.wav")), "A");
    assert!(!tmp.path().join("a.wav").exists());
    assert_eq!(read(&tmp.path().join("b.wav")), "B", "b.wav never moved");
    assert_eq!(read(&tmp.path().join("c.wav")), "C", "c.wav never moved");
    let (_, handed) = err.into_parts();
    let handed = handed.expect("the file it moved is handed over");
    assert_eq!(names(&handed), ["Pack/a.wav"]);
    assert!(matches!(&handed.files[0].data, FileData::Path(p) if p.ends_with("Pack/a.wav")));
}

#[test]
fn test_a_run_already_out_of_time_moves_nothing() {
    let tmp = dir_with(&[("a.wav", "A"), ("b.wav", "B"), ("c.wav", "C")]);
    let err = run(tmp.path(), &THREE, &TimeRunsOut::after(0))
        .err()
        .expect("the run timed out");
    assert!(err.is_timeout(), "{err}");
    assert!(!tmp.path().join("dest/Pack").exists(), "no folder made");
    assert!(err.into_parts().1.is_none(), "nothing to hand over");
}

#[test]
fn test_a_file_already_in_place_is_handed_over_when_time_runs_out() {
    let tmp = dir_with(&[("dest/Pack/a.wav", "A"), ("b.wav", "B")]);
    let moves = [("dest/Pack/a.wav", "Pack/a.wav"), ("b.wav", "Pack/b.wav")];
    let err = run(tmp.path(), &moves, &TimeRunsOut::after(0))
        .err()
        .expect("the run timed out");
    assert!(err.is_timeout(), "{err}");
    assert_eq!(read(&tmp.path().join("b.wav")), "B", "b.wav never moved");
    let handed = err.into_parts().1.expect("a.wav is where it belongs");
    assert_eq!(names(&handed), ["Pack/a.wav"]);
}

#[test]
fn test_a_swap_started_before_the_limit_finishes() {
    // A swap parks one file under a temporary name. Stopping before its
    // final rename would leave it there, so the swap runs to the end.
    let tmp = dir_with(&[("dest/A/x.wav", "FROM A"), ("dest/B/x.wav", "FROM B")]);
    let moves = [("dest/A/x.wav", "B/x.wav"), ("dest/B/x.wav", "A/x.wav")];
    run(tmp.path(), &moves, &TimeRunsOut::after(1)).unwrap();
    let dest = tmp.path().join("dest");
    assert_eq!(read(&dest.join("B/x.wav")), "FROM A");
    assert_eq!(read(&dest.join("A/x.wav")), "FROM B");
    let left: Vec<_> = std::fs::read_dir(dest.join("A")).unwrap().collect();
    assert_eq!(left.len(), 1, "no temporary file left");
}

#[test]
fn test_a_swap_not_started_before_the_limit_moves_nothing() {
    let tmp = dir_with(&[("dest/A/x.wav", "FROM A"), ("dest/B/x.wav", "FROM B")]);
    let moves = [("dest/A/x.wav", "B/x.wav"), ("dest/B/x.wav", "A/x.wav")];
    let err = run(tmp.path(), &moves, &TimeRunsOut::after(0))
        .err()
        .expect("the run timed out");
    assert!(err.is_timeout(), "{err}");
    let dest = tmp.path().join("dest");
    assert_eq!(read(&dest.join("A/x.wav")), "FROM A");
    assert_eq!(read(&dest.join("B/x.wav")), "FROM B");
}
