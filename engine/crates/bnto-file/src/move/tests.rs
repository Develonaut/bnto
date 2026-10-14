use std::path::{Path, PathBuf};

use bnto_core::NoopContext;
use bnto_core::processor::{BatchFile, BatchInput, FileData, NodeOutput, NodeProcessor};
use bnto_core::progress::ProgressReporter;
use bnto_core::{BntoError, ProcessContext};
use serde_json::json;

use super::FileMove;

struct DryRun;

impl ProcessContext for DryRun {
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
        NoopContext.work_dir()
    }
    fn dry_run(&self) -> bool {
        true
    }
}

/// A temp dir holding `files` (relative path → content), folders included.
fn dir_with(files: &[(&str, &str)]) -> tempfile::TempDir {
    let tmp = tempfile::tempdir().unwrap();
    for (name, content) in files {
        write(tmp.path(), name, content);
    }
    tmp
}

fn write(dir: &Path, name: &str, content: &str) {
    let path = dir.join(name);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, content).unwrap();
}

fn read(dir: &Path, name: &str) -> String {
    std::fs::read_to_string(dir.join(name)).unwrap_or_else(|e| panic!("{name}: {e}"))
}

/// Every file under `dir`, as sorted `/`-joined relative paths.
fn tree(dir: &Path) -> Vec<String> {
    let mut found = Vec::new();
    walk(dir, dir, &mut found);
    found.sort();
    found
}

fn walk(base: &Path, dir: &Path, found: &mut Vec<String>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            walk(base, &path, found);
        } else {
            found.push(bnto_core::slash_path(path.strip_prefix(base).unwrap()));
        }
    }
}

/// Files in `dir`, each `(source, name)` given its pipeline name.
fn batch(dir: &Path, moves: &[(&str, &str)]) -> Vec<BatchFile> {
    moves
        .iter()
        .map(|(src, name)| BatchFile {
            data: FileData::Path(dir.join(src)),
            filename: name.to_string(),
            mime_type: Some("audio/wav".to_string()),
            metadata: serde_json::Map::new(),
        })
        .collect()
}

fn run_with(
    files: Vec<BatchFile>,
    params: serde_json::Value,
    ctx: &dyn ProcessContext,
) -> Result<NodeOutput, BntoError> {
    let input = BatchInput {
        files,
        params: params.as_object().unwrap().clone(),
    };
    FileMove::new().process_batch(input, &ProgressReporter::new_noop(), ctx)
}

/// Move `(source, name)` pairs inside `dir` to names under `dir/dest`.
fn run(dir: &Path, moves: &[(&str, &str)]) -> Result<NodeOutput, BntoError> {
    let dest = dir.join("dest");
    let params = json!({ "destination": dest.to_string_lossy() });
    run_with(batch(dir, moves), params, &NoopContext)
}

fn dry_run(dir: &Path, moves: &[(&str, &str)]) -> Result<NodeOutput, BntoError> {
    let dest = dir.join("dest");
    let params = json!({ "destination": dest.to_string_lossy() });
    run_with(batch(dir, moves), params, &DryRun)
}

fn error(result: Result<NodeOutput, BntoError>) -> String {
    match result {
        Ok(_) => panic!("expected the run to be refused"),
        Err(e) => e.to_string(),
    }
}

// --- Node basics ---

#[test]
fn test_name_cardinality_and_platforms() {
    let p = FileMove::new();
    assert_eq!(p.name(), "file-move");
    let meta = p.metadata();
    assert_eq!(
        meta.input_cardinality,
        bnto_core::metadata::InputCardinality::Batch
    );
    assert!(!meta.platforms.contains(&"browser".to_string()));
}

#[test]
fn test_validate_requires_a_destination() {
    let p = FileMove::new();
    assert_eq!(p.validate(json!({}).as_object().unwrap()).len(), 1);
    assert!(
        p.validate(json!({ "destination": "d" }).as_object().unwrap())
            .is_empty()
    );
}

// --- Moves that run ---

#[test]
fn test_moves_files_into_new_folders_by_their_names() {
    let tmp = dir_with(&[("Pack A/shot.wav", "SHOT"), ("Pack B/rack.wav", "RACK")]);
    let out = run(
        tmp.path(),
        &[
            ("Pack A/shot.wav", "Pistol/Shot/shot.wav"),
            ("Pack B/rack.wav", "Pistol/Slide/rack.wav"),
        ],
    )
    .unwrap();
    let dest = tmp.path().join("dest");
    assert_eq!(
        tree(&dest),
        vec!["Pistol/Shot/shot.wav", "Pistol/Slide/rack.wav"]
    );
    assert_eq!(read(&dest, "Pistol/Slide/rack.wav"), "RACK");
    assert!(!tmp.path().join("Pack A/shot.wav").exists());

    assert_eq!(out.files[1].filename, "Pistol/Slide/rack.wav");
    let FileData::Path(at) = &out.files[1].data else {
        panic!("a moved file stays a path");
    };
    assert_eq!(std::fs::read_to_string(at).unwrap(), "RACK");
}

#[test]
fn test_a_second_file_joins_a_folder_the_first_created() {
    let tmp = dir_with(&[("a.wav", "A"), ("b.wav", "B")]);
    run(tmp.path(), &[("a.wav", "Pistol/Slide/a.wav")]).unwrap();
    run(tmp.path(), &[("b.wav", "Pistol/Slide/b.wav")]).unwrap();
    let dest = tmp.path().join("dest");
    assert_eq!(
        tree(&dest),
        vec!["Pistol/Slide/a.wav", "Pistol/Slide/b.wav"]
    );
}

#[test]
fn test_backslashes_separate_folders_too() {
    let tmp = dir_with(&[("a.wav", "A")]);
    let out = run(tmp.path(), &[("a.wav", r"Pistol\Slide\a.wav")]).unwrap();
    assert_eq!(read(&tmp.path().join("dest"), "Pistol/Slide/a.wav"), "A");
    assert_eq!(out.files[0].filename, "Pistol/Slide/a.wav");
}

#[test]
fn test_a_file_already_in_place_stays() {
    let tmp = dir_with(&[("dest/Pistol/a.wav", "A")]);
    let out = run(tmp.path(), &[("dest/Pistol/a.wav", "Pistol/a.wav")]).unwrap();
    assert_eq!(out.files.len(), 1);
    assert_eq!(read(&tmp.path().join("dest"), "Pistol/a.wav"), "A");
}

#[test]
fn test_metadata_passes_through() {
    let tmp = dir_with(&[("a.wav", "A")]);
    let mut files = batch(tmp.path(), &[("a.wav", "Pistol/a.wav")]);
    files[0].metadata.insert("subject".into(), json!("Pistol"));
    let dest = tmp.path().join("dest");
    let params = json!({ "destination": dest.to_string_lossy() });
    let out = run_with(files, params, &NoopContext).unwrap();
    assert_eq!(out.files[0].metadata["subject"], json!("Pistol"));
}

#[test]
fn test_files_swapping_folders_both_land() {
    let tmp = dir_with(&[("dest/A/x.wav", "FROM A"), ("dest/B/x.wav", "FROM B")]);
    run(
        tmp.path(),
        &[("dest/A/x.wav", "B/x.wav"), ("dest/B/x.wav", "A/x.wav")],
    )
    .unwrap();
    let dest = tmp.path().join("dest");
    assert_eq!(read(&dest, "B/x.wav"), "FROM A");
    assert_eq!(read(&dest, "A/x.wav"), "FROM B");
    assert_eq!(tree(&dest).len(), 2, "no temporary file left");
}

#[test]
fn test_the_table_read_the_other_way_undoes_the_move() {
    let tmp = dir_with(&[("dest/Pack/a.wav", "A"), ("dest/Pack/b.wav", "B")]);
    let table = [
        ("Pack/a.wav", "Pistol/Slide/a.wav"),
        ("Pack/b.wav", "Rifle/b.wav"),
    ];
    let forward: Vec<(String, &str)> = table
        .iter()
        .map(|(old, new)| (format!("dest/{old}"), *new))
        .collect();
    let forward: Vec<(&str, &str)> = forward.iter().map(|(s, n)| (s.as_str(), *n)).collect();
    run(tmp.path(), &forward).unwrap();

    let back: Vec<(String, &str)> = table
        .iter()
        .map(|(old, new)| (format!("dest/{new}"), *old))
        .collect();
    let back: Vec<(&str, &str)> = back.iter().map(|(s, n)| (s.as_str(), *n)).collect();
    run(tmp.path(), &back).unwrap();

    let dest = tmp.path().join("dest");
    assert!(tree(&dest).ends_with(&["Pack/a.wav".to_string(), "Pack/b.wav".to_string()]));
    assert_eq!(read(&dest, "Pack/a.wav"), "A");
    assert_eq!(read(&dest, "Pack/b.wav"), "B");
}

#[test]
fn test_a_missing_destination_is_created() {
    let tmp = dir_with(&[("a.wav", "A")]);
    let dest = tmp.path().join("new").join("tree");
    let params = json!({ "destination": dest.to_string_lossy() });
    run_with(
        batch(tmp.path(), &[("a.wav", "x/a.wav")]),
        params,
        &NoopContext,
    )
    .unwrap();
    assert_eq!(read(&dest, "x/a.wav"), "A");
}

// --- Names that are refused ---

#[test]
fn test_absolute_names_are_refused_and_nothing_moves() {
    let tmp = dir_with(&[("a.wav", "A"), ("b.wav", "B")]);
    let abs = tmp.path().join("elsewhere.wav");
    let abs = abs.to_string_lossy();
    for bad in ["/etc/a.wav", r"\a.wav", abs.as_ref()] {
        let err = error(run(tmp.path(), &[("b.wav", "ok/b.wav"), ("a.wav", bad)]));
        assert!(
            err.contains("nothing moved") && err.contains("absolute"),
            "{bad}: {err}"
        );
        assert_eq!(tree(tmp.path()), vec!["a.wav", "b.wav"], "{bad}");
    }
}

#[test]
fn test_drive_letters_are_refused_on_every_os() {
    let tmp = dir_with(&[("a.wav", "A")]);
    for bad in ["D:/Pistol/a.wav", r"d:.wav", "C:a.wav"] {
        let err = error(run(tmp.path(), &[("a.wav", bad)]));
        assert!(err.contains("absolute"), "{bad}: {err}");
    }
    assert_eq!(tree(tmp.path()), vec!["a.wav"]);
}

#[test]
fn test_dot_dot_is_refused() {
    let tmp = dir_with(&[("a.wav", "A")]);
    let err = error(run(tmp.path(), &[("a.wav", "Pistol/../../a.wav")]));
    assert!(err.contains("\"..\""), "{err}");
    assert_eq!(tree(tmp.path()), vec!["a.wav"]);
}

#[test]
fn test_every_bad_name_is_listed() {
    let tmp = dir_with(&[("a.wav", "A"), ("b.wav", "B")]);
    let err = error(run(
        tmp.path(),
        &[("a.wav", "../a.wav"), ("b.wav", "/b.wav")],
    ));
    assert!(err.contains("2 problems"), "{err}");
    assert!(err.contains("../a.wav") && err.contains("/b.wav"), "{err}");
}

#[test]
fn test_an_empty_name_is_refused() {
    let tmp = dir_with(&[("a.wav", "A")]);
    assert!(error(run(tmp.path(), &[("a.wav", "./")])).contains("no file name"));
}

#[cfg(windows)]
#[test]
fn test_names_windows_cannot_hold_are_refused() {
    let tmp = dir_with(&[("a.wav", "A")]);
    for bad in ["Pistol/CON/a.wav", "Pistol./a.wav", "Pistol/a?.wav"] {
        let err = error(run(tmp.path(), &[("a.wav", bad)]));
        assert!(err.contains("nothing moved"), "{bad}: {err}");
    }
    assert_eq!(tree(tmp.path()), vec!["a.wav"]);
}

#[test]
fn test_in_memory_data_is_refused() {
    let tmp = tempfile::tempdir().unwrap();
    let files = vec![BatchFile {
        data: FileData::Bytes(b"A".to_vec()),
        filename: "a.wav".to_string(),
        mime_type: None,
        metadata: serde_json::Map::new(),
    }];
    let params = json!({ "destination": tmp.path().to_string_lossy() });
    let err = error(run_with(files, params, &NoopContext));
    assert!(err.contains("in memory"), "{err}");
}

#[test]
fn test_a_missing_source_is_refused() {
    let tmp = dir_with(&[("a.wav", "A")]);
    let err = error(run(
        tmp.path(),
        &[("a.wav", "x/a.wav"), ("gone.wav", "x/gone.wav")],
    ));
    assert!(
        err.contains("gone.wav") && err.contains("doesn't exist"),
        "{err}"
    );
    assert_eq!(tree(tmp.path()), vec!["a.wav"]);
}

#[test]
fn test_a_source_listed_twice_is_refused() {
    let tmp = dir_with(&[("a.wav", "A")]);
    let err = error(run(
        tmp.path(),
        &[("a.wav", "x/a.wav"), ("a.wav", "y/a.wav")],
    ));
    assert!(err.contains("more than once"), "{err}");
}

// --- Moves that would replace a file ---

#[test]
fn test_two_files_onto_one_path_is_refused_on_a_dry_run_too() {
    let tmp = dir_with(&[("a/x.wav", "A"), ("b/x.wav", "B")]);
    let moves = [("a/x.wav", "Pistol/x.wav"), ("b/x.wav", "Pistol/x.wav")];
    for result in [dry_run(tmp.path(), &moves), run(tmp.path(), &moves)] {
        let err = error(result);
        assert!(
            err.contains("1 conflict") && err.contains("would all be renamed to"),
            "{err}"
        );
    }
    assert_eq!(tree(tmp.path()), vec!["a/x.wav", "b/x.wav"]);
}

#[test]
fn test_a_taken_path_is_refused_on_a_dry_run_too() {
    let tmp = dir_with(&[("a.wav", "A"), ("dest/Pistol/a.wav", "KEEP")]);
    let moves = [("a.wav", "Pistol/a.wav")];
    for result in [dry_run(tmp.path(), &moves), run(tmp.path(), &moves)] {
        assert!(error(result).contains("isn't being renamed away"));
    }
    assert_eq!(read(&tmp.path().join("dest"), "Pistol/a.wav"), "KEEP");
    assert_eq!(read(tmp.path(), "a.wav"), "A");
}

#[test]
fn test_a_file_where_a_folder_should_be_is_refused() {
    let tmp = dir_with(&[("a.wav", "A"), ("dest/Pistol", "a file")]);
    let err = error(run(tmp.path(), &[("a.wav", "Pistol/Slide/a.wav")]));
    assert!(err.contains("is a file, not a folder"), "{err}");
    assert_eq!(read(tmp.path(), "a.wav"), "A");
}

// --- Dry run ---

#[test]
fn test_dry_run_moves_nothing_and_creates_no_folder() {
    let tmp = dir_with(&[("a.wav", "A")]);
    let out = dry_run(tmp.path(), &[("a.wav", "Pistol/a.wav")]).unwrap();
    assert_eq!(tree(tmp.path()), vec!["a.wav"]);
    assert!(!tmp.path().join("dest").exists());
    assert_eq!(out.files[0].filename, "Pistol/a.wav");
    assert_eq!(out.files[0].data, FileData::Path(tmp.path().join("a.wav")));
}

// --- create_dirs off ---

#[test]
fn test_without_create_dirs_a_missing_folder_is_refused() {
    let tmp = dir_with(&[("a.wav", "A"), ("b.wav", "B"), ("dest/Pistol/keep", "")]);
    let dest = tmp.path().join("dest");
    let params = json!({ "destination": dest.to_string_lossy(), "create_dirs": false });
    let files = batch(
        tmp.path(),
        &[("a.wav", "Pistol/a.wav"), ("b.wav", "Rifle/b.wav")],
    );
    let err = error(run_with(files, params.clone(), &NoopContext));
    assert!(
        err.contains("Rifle") && err.contains("create_dirs is off"),
        "{err}"
    );
    assert_eq!(read(tmp.path(), "a.wav"), "A");

    let files = batch(tmp.path(), &[("a.wav", "Pistol/a.wav")]);
    run_with(files, params, &NoopContext).unwrap();
    assert_eq!(read(&dest, "Pistol/a.wav"), "A");
}

#[test]
fn test_without_create_dirs_a_missing_destination_is_refused() {
    let tmp = dir_with(&[("a.wav", "A")]);
    let dest = tmp.path().join("nope");
    let params = json!({ "destination": dest.to_string_lossy(), "create_dirs": false });
    let err = error(run_with(
        batch(tmp.path(), &[("a.wav", "a.wav")]),
        params,
        &NoopContext,
    ));
    assert!(err.contains("doesn't exist"), "{err}");
}

#[test]
fn test_a_destination_that_is_a_file_is_refused() {
    let tmp = dir_with(&[("a.wav", "A"), ("dest", "a file")]);
    let err = error(run(tmp.path(), &[("a.wav", "x/a.wav")]));
    assert!(err.contains("is not a folder"), "{err}");
}

// --- Links out of the destination ---

#[cfg(unix)]
#[test]
fn test_a_link_out_of_the_destination_is_refused() {
    let tmp = dir_with(&[("a.wav", "A"), ("outside/keep", "")]);
    std::fs::create_dir(tmp.path().join("dest")).unwrap();
    std::os::unix::fs::symlink(tmp.path().join("outside"), tmp.path().join("dest/out")).unwrap();
    let err = error(run(tmp.path(), &[("a.wav", "out/a.wav")]));
    assert!(err.contains("leads outside"), "{err}");
    assert!(!tmp.path().join("outside/a.wav").exists());
}
