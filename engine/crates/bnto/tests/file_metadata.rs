// file-metadata through the built binary — file-list picks a file on disk,
// file-metadata reports its size and modified time without reading it, and
// file-move (which needs a file on disk, not bytes) moves it.

mod helpers;

use std::process::Command;

use helpers::{bnto_bin, custom_recipe_path, run_custom_recipe_ok, temp_output_dir};

#[test]
fn test_each_listed_file_reports_its_folder_under_the_list_root() {
    let root = temp_output_dir();
    std::fs::create_dir_all(root.path().join("shots").join("day1")).unwrap();
    for name in ["top.txt", "shots/mid.txt", "shots/day1/deep.txt"] {
        std::fs::write(root.path().join(name), name).unwrap();
    }
    let vars_dir = temp_output_dir();
    let vars_file = vars_dir.path().join("vars.json");
    let vars = serde_json::json!({ "Root": root.path().to_str().unwrap() });
    std::fs::write(&vars_file, vars.to_string()).unwrap();

    let output = Command::new(bnto_bin())
        .args([
            "run",
            &custom_recipe_path("file-list-metadata-folder"),
            "--yes",
        ])
        .args(["--vars", vars_file.to_str().unwrap()])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    for line in [
        "folder: [] top.txt",
        "folder: [shots] shots/mid.txt",
        "folder: [shots/day1] shots/day1/deep.txt",
    ] {
        assert!(
            stderr.contains(line),
            "missing {line:?} in stderr: {stderr}"
        );
    }
}

#[test]
fn test_a_file_passed_on_the_command_line_has_an_empty_folder() {
    let dir = temp_output_dir();
    let nested = dir.path().join("inbox");
    std::fs::create_dir_all(&nested).unwrap();
    let file = nested.join("note.txt");
    std::fs::write(&file, b"hi").unwrap();

    let (_out, stderr) =
        run_custom_recipe_ok("file-metadata-folder-report", file.to_str().unwrap());

    // The argument's own disk folder isn't a folder under any list root.
    assert!(stderr.contains("folder: [] note.txt"), "stderr: {stderr}");
}

#[test]
fn test_size_and_modified_are_reported_and_the_file_stays_on_disk() {
    let root = temp_output_dir();
    let dest = temp_output_dir();
    let clip = root.path().join("clip.bin");
    std::fs::write(&clip, vec![7u8; 1234]).unwrap();
    let mtime = filetime::FileTime::from_unix_time(1_700_000_000, 123_000_000);
    filetime::set_file_mtime(&clip, mtime).unwrap();
    let vars_dir = temp_output_dir();
    let vars_file = vars_dir.path().join("vars.json");
    let vars = serde_json::json!({
        "Root": root.path().to_str().unwrap(),
        "Dest": dest.path().to_str().unwrap(),
    });
    std::fs::write(&vars_file, vars.to_string()).unwrap();

    let output = Command::new(bnto_bin())
        .args([
            "run",
            &custom_recipe_path("file-list-metadata-move"),
            "--yes",
        ])
        .args(["--vars", vars_file.to_str().unwrap()])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    assert!(
        stderr.contains("meta: clip.bin 1234 bytes, modified 2023-11-14T22:13:20.123Z"),
        "stderr: {stderr}"
    );
    // file-move refuses in-memory bytes, so the move proves it's still a path.
    assert!(!clip.exists());
    assert_eq!(
        std::fs::metadata(dest.path().join("clip.bin"))
            .unwrap()
            .len(),
        1234
    );
}
