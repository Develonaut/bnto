use super::*;
use bnto_core::context::NoopContext;
use bnto_core::progress::{LiveProgress, ProgressReporter};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Helper: create a test directory with known files.
fn setup_test_dir() -> TempDir {
    let dir = TempDir::new().unwrap();
    let root = dir.path();

    // Create files in root.
    fs::write(root.join("photo.jpg"), b"jpeg data").unwrap();
    fs::write(root.join("document.txt"), b"text data").unwrap();
    fs::write(root.join("data.csv"), b"a,b\n1,2\n").unwrap();

    // Create a subdirectory with files.
    let sub = root.join("subfolder");
    fs::create_dir(&sub).unwrap();
    fs::write(sub.join("image.png"), b"png data").unwrap();
    fs::write(sub.join("readme.txt"), b"readme").unwrap();

    dir
}

/// Helper: create a NodeInput pointing to a directory.
fn dir_input(dir: &Path, params: serde_json::Map<String, serde_json::Value>) -> NodeInput {
    NodeInput {
        data: FileData::Bytes(vec![]), // No data for directory input.
        filename: dir.to_string_lossy().to_string(),
        mime_type: None,
        params,
    }
}

fn params(pairs: &[(&str, serde_json::Value)]) -> serde_json::Map<String, serde_json::Value> {
    let mut map = serde_json::Map::new();
    for (k, v) in pairs {
        map.insert(k.to_string(), v.clone());
    }
    map
}

// --- Trait basics ---

#[test]
fn test_name_returns_correct_key() {
    assert_eq!(FileCollect::new().name(), "file-collect");
}

#[test]
fn test_metadata_is_cli_only() {
    let meta = FileCollect::new().metadata();
    assert_eq!(meta.category, NodeCategory::File);
    assert!(!meta.platforms.contains(&"browser".to_string()));
    assert!(meta.platforms.contains(&"cli".to_string()));
}

// --- Glob matching ---

#[test]
fn test_collect_all_files() {
    let test_dir = setup_test_dir();
    let processor = FileCollect::new();
    let progress = ProgressReporter::new_noop();

    let input = dir_input(
        test_dir.path(),
        params(&[("pattern", serde_json::json!("*"))]),
    );
    let output = processor.process(input, &progress, &NoopContext).unwrap();

    // Should find all 5 files (3 root + 2 subfolder), recursive by default.
    assert_eq!(
        output.files.len(),
        5,
        "Should collect all files recursively"
    );
}

#[test]
fn test_collect_glob_extension_filter() {
    let test_dir = setup_test_dir();
    let processor = FileCollect::new();
    let progress = ProgressReporter::new_noop();

    let input = dir_input(
        test_dir.path(),
        params(&[("pattern", serde_json::json!("*.txt"))]),
    );
    let output = processor.process(input, &progress, &NoopContext).unwrap();

    // Should find document.txt + readme.txt
    assert_eq!(output.files.len(), 2, "Should match only .txt files");
    let names: Vec<&str> = output.files.iter().map(|f| f.filename.as_str()).collect();
    assert!(names.contains(&"document.txt"));
    assert!(names.contains(&"readme.txt"));
}

// --- Live progress ---

type Reports = std::rc::Rc<std::cell::RefCell<Vec<(LiveProgress, String)>>>;

/// Run a collect over the test dir, recording every live update.
fn collect_reports(throttle_clock: Option<fn() -> u64>) -> Reports {
    let test_dir = setup_test_dir();
    let reports: Reports = Default::default();
    let sink = std::rc::Rc::clone(&reports);
    let mut progress = ProgressReporter::with_live(
        move |p, m| sink.borrow_mut().push((p, m.to_string())),
        |_| {},
    );
    if let Some(clock) = throttle_clock {
        progress = progress.throttled(clock, 100);
    }
    let input = dir_input(test_dir.path(), params(&[]));
    FileCollect::new()
        .process(input, &progress, &NoopContext)
        .unwrap();
    reports
}

fn counts(reports: &Reports) -> Vec<u64> {
    reports
        .borrow()
        .iter()
        .map(|(p, _)| match p {
            LiveProgress::Count(n) => *n,
            LiveProgress::Percent(p) => panic!("expected counts only, got {p}%"),
        })
        .collect()
}

#[test]
fn test_collect_reports_a_growing_count_while_it_walks() {
    let counts = counts(&collect_reports(None));
    assert!(counts.windows(2).all(|w| w[0] <= w[1]), "{counts:?}");
    assert!(
        counts.contains(&1),
        "reports before the walk ends: {counts:?}"
    );
    assert_eq!(counts.last(), Some(&5));
}

#[test]
fn test_collect_progress_names_the_folder_being_walked() {
    let reports = collect_reports(None);
    let messages: Vec<String> = reports.borrow().iter().map(|r| r.1.clone()).collect();
    assert!(
        messages.iter().any(|m| m.contains("subfolder")),
        "{messages:?}"
    );
    assert!(
        messages.iter().all(|m| m.starts_with("Found ")),
        "{messages:?}"
    );
}

#[test]
fn test_collect_progress_is_throttled_by_the_reporter() {
    // A clock that never moves lets only the first report through.
    assert_eq!(counts(&collect_reports(Some(|| 0))).len(), 1);
}

// --- Recursive vs flat ---

#[test]
fn test_collect_non_recursive() {
    let test_dir = setup_test_dir();
    let processor = FileCollect::new();
    let progress = ProgressReporter::new_noop();

    let input = dir_input(
        test_dir.path(),
        params(&[
            ("pattern", serde_json::json!("*")),
            ("recursive", serde_json::json!(false)),
        ]),
    );
    let output = processor.process(input, &progress, &NoopContext).unwrap();

    // Should find only 3 root files, not the 2 in subfolder.
    assert_eq!(
        output.files.len(),
        3,
        "Non-recursive should only find root files"
    );
}

// --- Flatten behavior ---

#[test]
fn test_collect_flatten_strips_paths() {
    let test_dir = setup_test_dir();
    let processor = FileCollect::new();
    let progress = ProgressReporter::new_noop();

    let input = dir_input(
        test_dir.path(),
        params(&[
            ("pattern", serde_json::json!("*.png")),
            ("flatten", serde_json::json!(true)),
        ]),
    );
    let output = processor.process(input, &progress, &NoopContext).unwrap();

    assert_eq!(output.files.len(), 1);
    assert_eq!(
        output.files[0].filename, "image.png",
        "Flatten should strip directory"
    );
}

#[test]
fn test_collect_no_flatten_preserves_paths() {
    let test_dir = setup_test_dir();
    let processor = FileCollect::new();
    let progress = ProgressReporter::new_noop();

    let input = dir_input(
        test_dir.path(),
        params(&[
            ("pattern", serde_json::json!("*.png")),
            ("flatten", serde_json::json!(false)),
        ]),
    );
    let output = processor.process(input, &progress, &NoopContext).unwrap();

    assert_eq!(output.files.len(), 1);
    // `/` on every OS, so name filters downstream split it the same way.
    assert_eq!(output.files[0].filename, "subfolder/image.png");
}

// --- No matches ---

#[test]
fn test_collect_no_matches() {
    let test_dir = setup_test_dir();
    let processor = FileCollect::new();
    let progress = ProgressReporter::new_noop();

    let input = dir_input(
        test_dir.path(),
        params(&[("pattern", serde_json::json!("*.xyz"))]),
    );
    let output = processor.process(input, &progress, &NoopContext).unwrap();

    assert_eq!(
        output.files.len(),
        0,
        "No matches should produce empty output"
    );
}

// --- Error handling ---

#[test]
fn test_collect_nonexistent_dir_fails() {
    let processor = FileCollect::new();
    let progress = ProgressReporter::new_noop();

    let input = dir_input(
        Path::new("/nonexistent/path/that/does/not/exist"),
        serde_json::Map::new(),
    );
    let result = processor.process(input, &progress, &NoopContext);

    assert!(result.is_err(), "Nonexistent directory should fail");
}

// --- Disk-reference input ---

/// An input that references `dir` on disk under a different name, as
/// `file-list` emits a listed folder.
fn reference_input(
    dir: &Path,
    name: &str,
    params: serde_json::Map<String, serde_json::Value>,
) -> NodeInput {
    NodeInput {
        data: FileData::Path(dir.to_path_buf()),
        filename: name.to_string(),
        mime_type: None,
        params,
    }
}

#[test]
fn test_collect_walks_the_referenced_dir() {
    let test_dir = setup_test_dir();
    let input = reference_input(
        test_dir.path(),
        "Some/Folder",
        params(&[("flatten", serde_json::json!(false))]),
    );
    let output = FileCollect::new()
        .process(input, &ProgressReporter::new_noop(), &NoopContext)
        .unwrap();

    let mut names: Vec<&str> = output.files.iter().map(|f| f.filename.as_str()).collect();
    names.sort_unstable();
    assert_eq!(
        names,
        [
            "data.csv",
            "document.txt",
            "photo.jpg",
            "subfolder/image.png",
            "subfolder/readme.txt"
        ],
        "names are relative to the walked folder, as for a CLI dir"
    );
}

#[test]
fn test_collect_missing_referenced_dir_fails() {
    let test_dir = setup_test_dir();
    let gone = test_dir.path().join("gone");
    let input = reference_input(&gone, "gone", serde_json::Map::new());
    let Err(err) = FileCollect::new().process(input, &ProgressReporter::new_noop(), &NoopContext)
    else {
        panic!("a missing referenced dir should fail");
    };

    assert!(
        err.to_string().contains(&gone.display().to_string()),
        "the error names the walked path: {err}"
    );
}

// --- File data reading ---

#[test]
fn test_collect_outputs_path_references() {
    let test_dir = setup_test_dir();
    let processor = FileCollect::new();
    let progress = ProgressReporter::new_noop();

    let input = dir_input(
        test_dir.path(),
        params(&[("pattern", serde_json::json!("*.csv"))]),
    );
    let output = processor.process(input, &progress, &NoopContext).unwrap();

    assert_eq!(output.files.len(), 1);
    assert_eq!(output.files[0].filename, "data.csv");

    // Verify output is a path reference, not in-memory bytes.
    assert!(
        matches!(&output.files[0].data, FileData::Path(_)),
        "Collect should output FileData::Path for zero-copy moves"
    );

    // Path should resolve to actual file content.
    let bytes = output.files[0].data.clone().into_bytes().unwrap();
    let content = String::from_utf8_lossy(&bytes);
    assert!(
        content.contains("a,b"),
        "Path should resolve to file content"
    );
}

// --- MIME type guessing ---

#[test]
fn test_mime_type_guessing() {
    assert_eq!(guess_mime_type("photo.jpg"), "image/jpeg");
    assert_eq!(guess_mime_type("icon.png"), "image/png");
    assert_eq!(guess_mime_type("data.csv"), "text/csv");
    assert_eq!(guess_mime_type("config.json"), "application/json");
    assert_eq!(guess_mime_type("unknown.xyz"), "application/octet-stream");
}

// --- Walk shape ---

fn collect_names(root: &Path, pairs: &[(&str, serde_json::Value)]) -> Vec<String> {
    let input = dir_input(root, params(pairs));
    let output = FileCollect::new()
        .process(input, &ProgressReporter::new_noop(), &NoopContext)
        .unwrap();
    let mut names: Vec<String> = output.files.into_iter().map(|f| f.filename).collect();
    names.sort_unstable();
    names
}

#[test]
fn test_collect_walks_every_depth_and_skips_folders() {
    let dir = setup_test_dir();
    let deep = dir.path().join("subfolder").join("deeper").join("deepest");
    fs::create_dir_all(&deep).unwrap();
    fs::create_dir(dir.path().join("empty")).unwrap();
    fs::write(deep.join("notes.txt"), b"n").unwrap();

    let names = collect_names(dir.path(), &[("flatten", serde_json::json!(false))]);
    assert_eq!(
        names,
        [
            "data.csv",
            "document.txt",
            "photo.jpg",
            "subfolder/deeper/deepest/notes.txt",
            "subfolder/image.png",
            "subfolder/readme.txt",
        ]
    );
}

#[test]
fn test_collect_skips_a_folder_of_staged_copies() {
    // RED: an interrupted write leaves its swap copies in a `.bnto-staged-*`
    // folder inside the output folder; a later run over that folder must not
    // take them as inputs. Other hidden folders are still walked.
    let dir = setup_test_dir();
    let staged = dir
        .path()
        .join(format!("{}abc", crate::STAGED_COPIES_PREFIX));
    fs::create_dir(&staged).unwrap();
    fs::write(staged.join("0-b.txt"), b"copy").unwrap();
    fs::create_dir(dir.path().join(".hidden")).unwrap();
    fs::write(dir.path().join(".hidden").join("kept.txt"), b"k").unwrap();

    let names = collect_names(dir.path(), &[("pattern", serde_json::json!("*.txt"))]);
    assert_eq!(names, ["document.txt", "kept.txt", "readme.txt"]);
}

#[test]
fn test_collect_pattern_applies_at_every_depth() {
    let dir = setup_test_dir();
    let deep = dir.path().join("subfolder").join("deeper");
    fs::create_dir(&deep).unwrap();
    fs::write(deep.join("notes.txt"), b"n").unwrap();
    // A folder whose name matches is walked, never collected.
    fs::create_dir(dir.path().join("folder.txt")).unwrap();

    let names = collect_names(dir.path(), &[("pattern", serde_json::json!("*.txt"))]);
    assert_eq!(names, ["document.txt", "notes.txt", "readme.txt"]);
}

// --- Symlinks ---

/// Link `link` to `target`; `None` where the OS won't let this user
/// (Windows without Developer Mode or admin).
#[cfg(any(unix, windows))]
fn symlink(target: &Path, link: &Path) -> Option<()> {
    #[cfg(unix)]
    let made = std::os::unix::fs::symlink(target, link);
    #[cfg(windows)]
    let made = if target.is_dir() {
        std::os::windows::fs::symlink_dir(target, link)
    } else {
        std::os::windows::fs::symlink_file(target, link)
    };
    match made {
        Ok(()) => Some(()),
        // ERROR_PRIVILEGE_NOT_HELD
        Err(e) if cfg!(windows) && e.raw_os_error() == Some(1314) => None,
        Err(e) => panic!("symlink failed: {e}"),
    }
}

#[cfg(any(unix, windows))]
#[test]
fn test_collect_follows_symlinked_files_and_folders() {
    let outside = TempDir::new().unwrap();
    fs::write(outside.path().join("linked.txt"), b"l").unwrap();
    let linked_dir = outside.path().join("shared");
    fs::create_dir(&linked_dir).unwrap();
    fs::write(linked_dir.join("inner.txt"), b"i").unwrap();

    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("plain.txt"), b"p").unwrap();
    let file_link = symlink(
        &outside.path().join("linked.txt"),
        &dir.path().join("file-link.txt"),
    );
    let dir_link = symlink(&linked_dir, &dir.path().join("dir-link"));
    if file_link.is_none() || dir_link.is_none() {
        eprintln!("skipped: this user can't make symlinks");
        return;
    }

    let names = collect_names(dir.path(), &[("flatten", serde_json::json!(false))]);
    assert_eq!(names, ["dir-link/inner.txt", "file-link.txt", "plain.txt"]);
    let names = collect_names(dir.path(), &[("recursive", serde_json::json!(false))]);
    assert_eq!(
        names,
        ["file-link.txt", "plain.txt"],
        "a linked folder is still a folder"
    );
}

#[cfg(any(unix, windows))]
#[test]
fn test_collect_skips_broken_symlinks() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("plain.txt"), b"p").unwrap();
    let gone = dir.path().join("gone.txt");
    fs::write(&gone, b"g").unwrap();
    if symlink(&gone, &dir.path().join("dangling.txt")).is_none() {
        eprintln!("skipped: this user can't make symlinks");
        return;
    }
    fs::remove_file(&gone).unwrap();

    assert_eq!(collect_names(dir.path(), &[]), ["plain.txt"]);
}

/// pnpm links `node_modules` folders with junctions, which any Windows
/// user can make, so this one never skips.
#[cfg(windows)]
#[test]
fn test_collect_follows_windows_junctions() {
    let outside = TempDir::new().unwrap();
    fs::write(outside.path().join("inner.txt"), b"i").unwrap();
    let dir = TempDir::new().unwrap();
    let made = std::process::Command::new("cmd")
        .args(["/c", "mklink", "/J"])
        .arg(dir.path().join("junction"))
        .arg(outside.path())
        .output()
        .unwrap();
    assert!(made.status.success(), "{made:?}");

    let names = collect_names(dir.path(), &[("flatten", serde_json::json!(false))]);
    assert_eq!(names, ["junction/inner.txt"]);
}
