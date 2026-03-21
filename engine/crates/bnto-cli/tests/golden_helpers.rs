// Golden test comparison helpers — SHA-256 hash-based output verification.
//
// Normal mode: compare CLI output against committed golden files.
// Bless mode (BLESS=1): overwrite golden files with current output.

use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

/// Directory containing committed golden output files.
fn golden_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/golden")
}

/// Hash file contents with SHA-256, returning hex string.
fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

/// Collect and sort output files from a directory.
fn collect_output_files(dir: &Path) -> Vec<std::fs::DirEntry> {
    let mut files: Vec<_> = std::fs::read_dir(dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();
    files.sort_by_key(|e| e.file_name());
    files
}

/// Assert that every output file in `actual_dir` matches its golden counterpart.
///
/// In bless mode (`BLESS=1`), writes actual output to the golden directory instead.
pub fn assert_golden(recipe_slug: &str, actual_dir: &tempfile::TempDir) {
    let recipe_golden = golden_dir().join(recipe_slug);
    let actual_files = collect_output_files(actual_dir.path());

    assert!(
        !actual_files.is_empty(),
        "[{recipe_slug}] No output files produced"
    );

    if std::env::var("BLESS").is_ok() {
        bless_golden_files(&recipe_golden, &actual_files, recipe_slug);
    } else {
        compare_golden_files(&recipe_golden, &actual_files, recipe_slug);
        verify_no_stale_goldens(&recipe_golden, &actual_files, recipe_slug);
    }
}

/// Overwrite golden files with current output (bless mode).
fn bless_golden_files(recipe_golden: &Path, actual_files: &[std::fs::DirEntry], recipe_slug: &str) {
    if recipe_golden.exists() {
        std::fs::remove_dir_all(recipe_golden).unwrap();
    }
    std::fs::create_dir_all(recipe_golden).unwrap();

    for entry in actual_files {
        let name = entry.file_name();
        let bytes = std::fs::read(entry.path()).unwrap();
        std::fs::write(recipe_golden.join(&name), &bytes).unwrap();
        eprintln!(
            "[bless] {recipe_slug}/{} ({} bytes)",
            name.to_string_lossy(),
            bytes.len()
        );
    }
}

/// Compare each output file against its golden counterpart via SHA-256.
fn compare_golden_files(
    recipe_golden: &Path,
    actual_files: &[std::fs::DirEntry],
    recipe_slug: &str,
) {
    for entry in actual_files {
        compare_single_file(recipe_golden, entry, recipe_slug);
    }
}

/// Compare one output file against its golden counterpart.
fn compare_single_file(recipe_golden: &Path, entry: &std::fs::DirEntry, recipe_slug: &str) {
    let name = entry.file_name();
    let actual_bytes = std::fs::read(entry.path()).unwrap();
    let golden_path = recipe_golden.join(&name);

    assert!(
        golden_path.exists(),
        "[{recipe_slug}] Golden file missing: {}. Run BLESS=1 to generate.",
        golden_path.display()
    );

    let golden_bytes = std::fs::read(&golden_path).unwrap();
    let actual_hash = sha256_hex(&actual_bytes);
    let golden_hash = sha256_hex(&golden_bytes);

    assert_eq!(
        actual_hash,
        golden_hash,
        "[{recipe_slug}/{}] Hash mismatch.\n  expected: {golden_hash} ({} bytes)\n  actual:   {actual_hash} ({} bytes)\n  Run BLESS=1 to update.",
        name.to_string_lossy(),
        golden_bytes.len(),
        actual_bytes.len(),
    );
}

/// Check that every golden file was also produced as output.
fn verify_no_stale_goldens(
    golden_dir: &Path,
    actual_files: &[std::fs::DirEntry],
    recipe_slug: &str,
) {
    if !golden_dir.exists() {
        return;
    }
    let actual_names: Vec<_> = actual_files.iter().map(|e| e.file_name()).collect();
    for entry in std::fs::read_dir(golden_dir)
        .unwrap()
        .filter_map(|e| e.ok())
    {
        let name = entry.file_name();
        assert!(
            actual_names.contains(&name),
            "[{recipe_slug}] Stale golden file: {}. CLI no longer produces this output.",
            name.to_string_lossy()
        );
    }
}
