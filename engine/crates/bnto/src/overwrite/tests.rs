use std::path::Path;

use bnto_core::processor::FileData;
use bnto_core::{PipelineFileResult, PipelineResult};
use bnto_file::relocate::{Move, run_in_order};

use super::overwrite_in_place;

/// A temp dir holding `files` (name → content).
fn dir_with(files: &[(&str, &str)]) -> tempfile::TempDir {
    let tmp = tempfile::tempdir().unwrap();
    for (name, content) in files {
        std::fs::write(tmp.path().join(name), content).unwrap();
    }
    tmp
}

/// A result renaming each `(from, to)` inside `dir`; `to` is a pipeline name,
/// so only its leaf counts.
fn result(dir: &Path, renames: &[(&str, &str)]) -> PipelineResult {
    PipelineResult {
        files: renames
            .iter()
            .map(|(from, to)| PipelineFileResult {
                name: format!("Pack/{to}"),
                data: FileData::Path(dir.join(from)),
                mime_type: "audio/wav".to_string(),
                metadata: serde_json::Map::new(),
            })
            .collect(),
        duration_ms: 0,
        warnings: vec![],
        node_metadata: Default::default(),
    }
}

fn run(dir: &Path, renames: &[(&str, &str)]) -> Result<usize, String> {
    overwrite_in_place(&result(dir, renames), false)
}

fn read(dir: &Path, name: &str) -> String {
    std::fs::read_to_string(dir.join(name)).unwrap_or_else(|e| panic!("{name}: {e}"))
}

fn names(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

// --- Renames that run ---

#[test]
fn test_renames_in_place_using_the_leaf_name() {
    let tmp = dir_with(&[("a.wav", "A")]);
    assert_eq!(run(tmp.path(), &[("a.wav", "x.wav")]).unwrap(), 1);
    assert_eq!(read(tmp.path(), "x.wav"), "A");
    assert_eq!(names(tmp.path()), vec!["x.wav"]);
}

#[test]
fn test_unchanged_names_count_and_stay() {
    let tmp = dir_with(&[("a.wav", "A")]);
    assert_eq!(run(tmp.path(), &[("a.wav", "a.wav")]).unwrap(), 1);
    assert_eq!(read(tmp.path(), "a.wav"), "A");
}

#[test]
fn test_a_chain_runs_in_the_order_that_keeps_every_file() {
    // a→b and b→c: b must move first, whatever order the results come in.
    let tmp = dir_with(&[("a.wav", "A"), ("b.wav", "B")]);
    run(tmp.path(), &[("a.wav", "b.wav"), ("b.wav", "c.wav")]).unwrap();
    assert_eq!(read(tmp.path(), "b.wav"), "A");
    assert_eq!(read(tmp.path(), "c.wav"), "B");
    assert_eq!(names(tmp.path()), vec!["b.wav", "c.wav"]);
}

#[test]
fn test_a_swap_goes_through_a_temporary_name() {
    let tmp = dir_with(&[("a.wav", "A"), ("b.wav", "B")]);
    run(tmp.path(), &[("a.wav", "b.wav"), ("b.wav", "a.wav")]).unwrap();
    assert_eq!(read(tmp.path(), "a.wav"), "B");
    assert_eq!(read(tmp.path(), "b.wav"), "A");
    assert_eq!(
        names(tmp.path()),
        vec!["a.wav", "b.wav"],
        "no temp file left"
    );
}

#[test]
fn test_a_three_way_rotation_keeps_every_file() {
    let tmp = dir_with(&[("a.wav", "A"), ("b.wav", "B"), ("c.wav", "C")]);
    run(
        tmp.path(),
        &[("a.wav", "b.wav"), ("b.wav", "c.wav"), ("c.wav", "a.wav")],
    )
    .unwrap();
    assert_eq!(read(tmp.path(), "b.wav"), "A");
    assert_eq!(read(tmp.path(), "c.wav"), "B");
    assert_eq!(read(tmp.path(), "a.wav"), "C");
    assert_eq!(names(tmp.path()), vec!["a.wav", "b.wav", "c.wav"]);
}

#[test]
fn test_two_cycles_and_a_chain_into_a_cycle_in_one_plan() {
    let files = [
        ("a.wav", "A"),
        ("b.wav", "B"),
        ("c.wav", "C"),
        ("d.wav", "D"),
        ("x.wav", "X"),
    ];
    let tmp = dir_with(&files);
    let renames = [
        ("a.wav", "b.wav"),
        ("b.wav", "a.wav"),
        ("c.wav", "d.wav"),
        ("d.wav", "c.wav"),
        ("x.wav", "y.wav"),
    ];
    assert_eq!(run(tmp.path(), &renames).unwrap(), 5);
    assert_eq!(read(tmp.path(), "a.wav"), "B");
    assert_eq!(read(tmp.path(), "b.wav"), "A");
    assert_eq!(read(tmp.path(), "c.wav"), "D");
    assert_eq!(read(tmp.path(), "d.wav"), "C");
    assert_eq!(read(tmp.path(), "y.wav"), "X");
    assert_eq!(names(tmp.path()).len(), 5, "no temp file left");
}

#[test]
fn test_a_chain_feeding_a_cycle() {
    // x→a waits on a, and a↔b is a swap.
    let tmp = dir_with(&[("a.wav", "A"), ("b.wav", "B"), ("x.wav", "X")]);
    let err = run(
        tmp.path(),
        &[("x.wav", "a.wav"), ("a.wav", "b.wav"), ("b.wav", "a.wav")],
    );
    // Two files onto a.wav is a shared target, refused before anything moves.
    assert!(err.unwrap_err().contains("would all be renamed to"));
    assert_eq!(names(tmp.path()), vec!["a.wav", "b.wav", "x.wav"]);

    let tmp = dir_with(&[("a.wav", "A"), ("b.wav", "B"), ("x.wav", "X")]);
    run(
        tmp.path(),
        &[("x.wav", "a.wav"), ("a.wav", "b.wav"), ("b.wav", "c.wav")],
    )
    .unwrap();
    assert_eq!(read(tmp.path(), "a.wav"), "X");
    assert_eq!(read(tmp.path(), "b.wav"), "A");
    assert_eq!(read(tmp.path(), "c.wav"), "B");
}

#[test]
fn test_an_existing_temp_name_is_skipped() {
    let tmp = dir_with(&[
        ("a.wav", "A"),
        ("b.wav", "B"),
        (".bnto-rename-0-a.wav", "OLD"),
    ]);
    run(tmp.path(), &[("a.wav", "b.wav"), ("b.wav", "a.wav")]).unwrap();
    assert_eq!(read(tmp.path(), ".bnto-rename-0-a.wav"), "OLD");
    assert_eq!(read(tmp.path(), "a.wav"), "B");
    assert_eq!(read(tmp.path(), "b.wav"), "A");
}

#[test]
fn test_a_case_only_rename_works() {
    let tmp = dir_with(&[("kick.wav", "K")]);
    run(tmp.path(), &[("kick.wav", "Kick.wav")]).unwrap();
    assert_eq!(names(tmp.path()), vec!["Kick.wav"]);
    assert_eq!(read(tmp.path(), "Kick.wav"), "K");
}

// --- Plans that are refused ---

#[test]
fn test_two_files_onto_one_name_is_refused_and_nothing_moves() {
    let tmp = dir_with(&[("a.wav", "A"), ("b.wav", "B"), ("c.wav", "C")]);
    let err = run(
        tmp.path(),
        &[("a.wav", "x.wav"), ("b.wav", "x.wav"), ("c.wav", "y.wav")],
    )
    .unwrap_err();
    assert!(err.contains("nothing renamed: 1 conflict"), "{err}");
    assert!(
        err.contains("a.wav") && err.contains("b.wav") && err.contains("x.wav"),
        "{err}"
    );
    assert_eq!(names(tmp.path()), vec!["a.wav", "b.wav", "c.wav"]);
}

#[test]
fn test_names_differing_only_in_case_collide() {
    let tmp = dir_with(&[("a.wav", "A"), ("b.wav", "B")]);
    assert!(run(tmp.path(), &[("a.wav", "X.wav"), ("b.wav", "x.wav")]).is_err());
    assert_eq!(names(tmp.path()), vec!["a.wav", "b.wav"]);
}

#[test]
fn test_replacing_a_file_outside_the_run_is_refused() {
    let tmp = dir_with(&[("a.wav", "A"), ("keep.wav", "KEEP")]);
    let err = run(tmp.path(), &[("a.wav", "keep.wav")]).unwrap_err();
    assert!(err.contains("isn't being renamed away"), "{err}");
    assert_eq!(read(tmp.path(), "keep.wav"), "KEEP");
    assert_eq!(read(tmp.path(), "a.wav"), "A");
}

#[test]
fn test_replacing_a_file_the_run_leaves_in_place_is_refused() {
    let tmp = dir_with(&[("a.wav", "A"), ("b.wav", "B")]);
    assert!(run(tmp.path(), &[("a.wav", "b.wav"), ("b.wav", "b.wav")]).is_err());
    assert_eq!(read(tmp.path(), "b.wav"), "B");
}

#[test]
fn test_dry_run_refuses_the_same_plan_and_touches_nothing() {
    let tmp = dir_with(&[("a.wav", "A"), ("keep.wav", "KEEP")]);
    let bad = result(tmp.path(), &[("a.wav", "keep.wav")]);
    assert!(overwrite_in_place(&bad, true).is_err());
    let ok = result(tmp.path(), &[("a.wav", "x.wav")]);
    assert_eq!(overwrite_in_place(&ok, true).unwrap(), 1);
    assert_eq!(names(tmp.path()), vec!["a.wav", "keep.wav"]);
}

#[test]
fn test_a_source_listed_twice_is_refused() {
    let tmp = dir_with(&[("a.wav", "A")]);
    let err = run(tmp.path(), &[("a.wav", "x.wav"), ("a.wav", "y.wav")]).unwrap_err();
    assert!(err.contains("more than once"), "{err}");
    assert_eq!(names(tmp.path()), vec!["a.wav"]);
}

#[test]
fn test_a_name_with_no_usable_leaf_is_refused() {
    let tmp = dir_with(&[("a.wav", "A")]);
    let mut r = result(tmp.path(), &[("a.wav", "x.wav")]);
    r.files[0].name = "sub/..".to_string();
    assert!(
        overwrite_in_place(&r, false)
            .unwrap_err()
            .contains("no usable file name")
    );
    assert_eq!(names(tmp.path()), vec!["a.wav"]);
}

#[cfg(windows)]
#[test]
fn test_names_windows_cannot_hold_are_refused_up_front() {
    for bad in ["x.wav.", "x.wav ", "a:b.wav", "CON.wav", "nul"] {
        let tmp = dir_with(&[("a.wav", "A"), ("b.wav", "B")]);
        let err = run(tmp.path(), &[("a.wav", "ok.wav"), ("b.wav", bad)]).unwrap_err();
        assert!(err.contains("nothing renamed"), "{bad}: {err}");
        assert_eq!(names(tmp.path()), vec!["a.wav", "b.wav"], "{bad}");
    }
}

#[test]
fn test_bytes_data_is_refused() {
    let mut r = result(Path::new("."), &[("a.wav", "b.wav")]);
    r.files[0].data = FileData::Bytes(b"x".to_vec());
    assert!(
        overwrite_in_place(&r, true)
            .unwrap_err()
            .contains("FileData::Path")
    );
}

// --- Failure partway through ---

#[test]
fn test_a_failure_partway_says_how_far_it_got() {
    // b.wav vanishes after planning, so its rename fails after a's ran.
    let tmp = dir_with(&[("a.wav", "A")]);
    let moves = vec![
        Move {
            src: tmp.path().join("a.wav"),
            dest: tmp.path().join("x.wav"),
        },
        Move {
            src: tmp.path().join("b.wav"),
            dest: tmp.path().join("y.wav"),
        },
    ];
    let err = run_in_order(moves).unwrap_err();
    assert!(err.contains("b.wav"), "{err}");
    assert!(err.contains("1 of 2 renames ran"), "{err}");
    assert_eq!(read(tmp.path(), "x.wav"), "A");
}

#[test]
fn test_a_file_that_appears_after_planning_is_never_replaced() {
    let tmp = dir_with(&[("a.wav", "A"), ("late.wav", "LATE")]);
    let moves = vec![Move {
        src: tmp.path().join("a.wav"),
        dest: tmp.path().join("late.wav"),
    }];
    let err = run_in_order(moves).unwrap_err();
    assert!(err.contains("now exists"), "{err}");
    assert_eq!(read(tmp.path(), "late.wav"), "LATE");
    assert_eq!(read(tmp.path(), "a.wav"), "A");
}

// --- Case-sensitive filesystems ---

/// `Kick.wav` and `kick.wav` can only both exist on a case-sensitive
/// filesystem; skip where they can't.
fn case_sensitive_dir() -> Option<tempfile::TempDir> {
    let tmp = dir_with(&[("Kick.wav", "UPPER")]);
    std::fs::write(tmp.path().join("kick.wav"), "lower").unwrap();
    (names(tmp.path()).len() == 2).then_some(tmp)
}

#[test]
fn test_case_sensitive_fs_never_replaces_a_case_twin() {
    let Some(tmp) = case_sensitive_dir() else {
        return;
    };
    // kick.wav stays; Kick.wav would land on it.
    let err = run(
        tmp.path(),
        &[("Kick.wav", "kick.wav"), ("kick.wav", "kick.wav")],
    )
    .unwrap_err();
    assert!(err.contains("isn't being renamed away"), "{err}");
    assert_eq!(read(tmp.path(), "kick.wav"), "lower");
    assert_eq!(read(tmp.path(), "Kick.wav"), "UPPER");
}

#[test]
fn test_case_sensitive_fs_refuses_moving_two_case_twins() {
    let Some(tmp) = case_sensitive_dir() else {
        return;
    };
    let err = run(tmp.path(), &[("Kick.wav", "a.wav"), ("kick.wav", "b.wav")]).unwrap_err();
    assert!(err.contains("differ only in case"), "{err}");
    assert_eq!(names(tmp.path()), vec!["Kick.wav", "kick.wav"]);
}

#[test]
fn test_case_sensitive_fs_refuses_landing_on_an_unrelated_case_twin() {
    let Some(tmp) = case_sensitive_dir() else {
        return;
    };
    std::fs::write(tmp.path().join("y.wav"), "Y").unwrap();
    // Kick.wav moves away; y.wav would land on kick.wav, a different file.
    let renames = [("Kick.wav", "x.wav"), ("y.wav", "kick.wav")];
    let err = run(tmp.path(), &renames).unwrap_err();
    assert!(err.contains("isn't being renamed away"), "{err}");
    assert_eq!(read(tmp.path(), "kick.wav"), "lower");
}
