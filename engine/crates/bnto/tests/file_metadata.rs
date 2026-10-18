// file-metadata through the built binary — file-list picks a file on disk,
// file-metadata reports its size and modified time without reading it, and
// file-move (which needs a file on disk, not bytes) moves it.

mod helpers;

use std::process::Command;

use helpers::{bnto_bin, custom_recipe_path, temp_output_dir};

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
