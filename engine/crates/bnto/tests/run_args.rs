// Files passed as `bnto run` arguments enter the pipeline as disk references,
// not bytes: nodes that need content read it, and the source is never moved.

mod helpers;

use std::process::Command;

use helpers::{
    bnto_bin, custom_recipe_path, fixture_image, fixture_vector, output_files, recipe_path,
};

/// Run `recipe` on `inputs` with extra args; returns (success, stderr).
fn run(recipe: &str, inputs: &[&std::path::Path], extra: &[&str]) -> (bool, String) {
    let output = Command::new(bnto_bin())
        .args(["run", recipe, "--yes"])
        .args(inputs)
        .args(extra)
        .output()
        .unwrap();
    (
        output.status.success(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    )
}

#[test]
fn test_overwrite_renames_an_argument_file_in_place() {
    // Guards: an argument file read into memory had no disk path, so
    // overwrite mode refused it ("requires FileData::Path").
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("take.wav");
    std::fs::write(&src, "audio").unwrap();

    let recipe = custom_recipe_path("file-rename-overwrite-args");
    let (ok, stderr) = run(&recipe, &[&src], &["-p", "rename:pattern=final.{{ext}}"]);

    assert!(ok, "stderr: {stderr}");
    assert!(!src.exists(), "source should be renamed away");
    let renamed = dir.path().join("final.wav");
    assert_eq!(std::fs::read_to_string(renamed).unwrap(), "audio");
}

#[test]
fn test_compress_reads_an_argument_file_and_leaves_it_in_place() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("photo.jpg");
    std::fs::copy(fixture_image("small.jpg"), &src).unwrap();
    let before = std::fs::read(&src).unwrap();
    let out = tempfile::tempdir().unwrap();

    let out_arg = out.path().to_str().unwrap();
    let (ok, stderr) = run(&recipe_path("compress-images"), &[&src], &["-o", out_arg]);

    assert!(ok, "stderr: {stderr}");
    assert_eq!(std::fs::read(&src).unwrap(), before, "source unchanged");
    let files = output_files(&out);
    assert_eq!(files.len(), 1);
    let compressed = std::fs::read(files[0].path()).unwrap();
    assert_eq!(&compressed[..3], &[0xFF, 0xD8, 0xFF], "JPEG magic bytes");
}

#[test]
fn test_pass_through_copies_an_argument_file_and_keeps_the_source() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("notes.txt");
    std::fs::write(&src, "keep me").unwrap();
    let out = tempfile::tempdir().unwrap();

    let out_arg = out.path().to_str().unwrap();
    let (ok, stderr) = run(&recipe_path("rename-files"), &[&src], &["-o", out_arg]);

    assert!(ok, "stderr: {stderr}");
    assert_eq!(std::fs::read_to_string(&src).unwrap(), "keep me");
    let files = output_files(&out);
    assert_eq!(files.len(), 1);
    assert_eq!(std::fs::read_to_string(files[0].path()).unwrap(), "keep me");
}

#[test]
fn test_files_whose_names_differ_only_in_case_all_land_in_the_output() {
    // Guards: `favicon/Logo.svg` and `logo.svg` were one file on Windows, so
    // optimize-svg reported "2 files written" with only one on disk. Names
    // are case-folded on every OS, so Linux writes the same two files.
    let dir = tempfile::tempdir().unwrap();
    let (upper, lower) = (dir.path().join("favicon"), dir.path().join("site"));
    std::fs::create_dir_all(&upper).unwrap();
    std::fs::create_dir_all(&lower).unwrap();
    let (a, b) = (upper.join("Logo.svg"), lower.join("logo.svg"));
    std::fs::copy(fixture_vector("verbose.svg"), &a).unwrap();
    std::fs::write(
        &b,
        r#"<svg xmlns="http://www.w3.org/2000/svg"><rect width="1"/></svg>"#,
    )
    .unwrap();
    let out = tempfile::tempdir().unwrap();

    let out_arg = out.path().to_str().unwrap();
    let (ok, stderr) = run(&recipe_path("optimize-svg"), &[&a, &b], &["-o", out_arg]);

    assert!(ok, "stderr: {stderr}");
    let mut names: Vec<String> = output_files(&out)
        .iter()
        .map(|f| f.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    assert_eq!(names, ["Logo.svg", "logo_1.svg"]);
    assert!(stderr.contains("2 files written"), "stderr: {stderr}");
    assert!(stderr.contains("logo.svg → logo_1.svg"), "stderr: {stderr}");
    let small = std::fs::read_to_string(out.path().join("logo_1.svg")).unwrap();
    assert!(
        small.contains("rect"),
        "the second file kept its own content"
    );
}
