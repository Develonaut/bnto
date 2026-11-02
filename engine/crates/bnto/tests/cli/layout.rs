// Keeps the CLI's integration tests in one binary (ISS-120): a file in
// `tests/cli/` that `main.rs` doesn't name never compiles, so its tests would
// silently never run, and a new top-level `tests/*.rs` or `tests/*/main.rs`
// is a binary of its own.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

fn tests_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests")
}

/// File stems of the `.rs` files directly in `dir`.
fn rust_stems(dir: &Path) -> BTreeSet<String> {
    std::fs::read_dir(dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_file() && path.extension().is_some_and(|ext| ext == "rs"))
        .map(|path| path.file_stem().unwrap().to_string_lossy().into_owned())
        .collect()
}

/// Modules `main.rs` declares as `mod name;` (the `#[path]` helpers aside).
fn declared_modules(main_rs: &str) -> BTreeSet<String> {
    main_rs
        .lines()
        .filter_map(|line| line.strip_prefix("mod ")?.strip_suffix(';'))
        .map(str::to_owned)
        .collect()
}

#[test]
fn every_file_in_tests_cli_is_a_module_of_main() {
    let dir = tests_dir().join("cli");
    let main_rs = std::fs::read_to_string(dir.join("main.rs")).unwrap();
    let mut files = rust_stems(&dir);
    files.remove("main");
    let mut declared = declared_modules(&main_rs);
    declared.remove("helpers");
    assert_eq!(
        files, declared,
        "every .rs file in tests/cli/ needs a `mod` line in tests/cli/main.rs, and every `mod` line a file"
    );
}

/// Folders in `dir` that hold a `main.rs`, each of which cargo builds as a
/// test binary.
fn binary_folders(dir: &Path) -> BTreeSet<String> {
    std::fs::read_dir(dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.join("main.rs").is_file())
        .map(|path| path.file_name().unwrap().to_string_lossy().into_owned())
        .collect()
}

#[test]
fn only_the_listed_suites_are_test_binaries_of_their_own() {
    let reason = "a new CLI test file goes in tests/cli/ as a module of main.rs, \
                  not in tests/ as its own binary (tests/cli/main.rs says which suites keep one)";
    let files = rust_stems(&tests_dir());
    let expected_files = BTreeSet::from(["tui_journeys".to_owned(), "upgrade".to_owned()]);
    assert_eq!(files, expected_files, "{reason}");
    let folders = binary_folders(&tests_dir());
    let expected_folders = BTreeSet::from(["cli".to_owned(), "shell_security".to_owned()]);
    assert_eq!(folders, expected_folders, "{reason}");
}
