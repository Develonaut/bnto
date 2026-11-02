// Golden test comparison helpers — SHA-256 hash-based output verification.
//
// Normal mode: compare CLI output against committed golden files.
// Bless mode (BLESS=1): overwrite golden files with current output.
// Supports both flat and nested directory output structures.

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

/// An output file: relative path from the output root + absolute path on disk.
struct OutputFile {
    relative: String,
    absolute: PathBuf,
}

/// Recursively collect all files under `dir`, returning sorted relative paths.
fn collect_output_files(dir: &Path) -> Vec<OutputFile> {
    let mut files = Vec::new();
    walk_output_dir(dir, dir, &mut files);
    files.sort_by(|a, b| a.relative.cmp(&b.relative));
    files
}

fn walk_output_dir(root: &Path, current: &Path, results: &mut Vec<OutputFile>) {
    let Ok(entries) = std::fs::read_dir(current) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            walk_output_dir(root, &path, results);
        } else if path.is_file() {
            let relative = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .to_string_lossy()
                .to_string();
            results.push(OutputFile {
                relative,
                absolute: path,
            });
        }
    }
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
fn bless_golden_files(recipe_golden: &Path, actual_files: &[OutputFile], recipe_slug: &str) {
    if recipe_golden.exists() {
        std::fs::remove_dir_all(recipe_golden).unwrap();
    }
    std::fs::create_dir_all(recipe_golden).unwrap();

    for file in actual_files {
        let dest = recipe_golden.join(&file.relative);
        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        let bytes = std::fs::read(&file.absolute).unwrap();
        std::fs::write(&dest, &bytes).unwrap();
        eprintln!(
            "[bless] {recipe_slug}/{} ({} bytes)",
            file.relative,
            bytes.len()
        );
    }
}

/// Compare each output file against its golden counterpart via SHA-256.
fn compare_golden_files(recipe_golden: &Path, actual_files: &[OutputFile], recipe_slug: &str) {
    for file in actual_files {
        let actual_bytes = std::fs::read(&file.absolute).unwrap();
        let golden_path = recipe_golden.join(&file.relative);
        let golden_bytes = std::fs::read(&golden_path)
            .unwrap_or_else(|_| panic!("[{recipe_slug}] Missing: {}. Run BLESS=1.", file.relative));
        let (actual_hash, golden_hash) = (sha256_hex(&actual_bytes), sha256_hex(&golden_bytes));

        assert_eq!(
            actual_hash,
            golden_hash,
            "[{recipe_slug}/{}] Hash mismatch.\n  expected: {golden_hash} ({} B)\n  actual:   {actual_hash} ({} B)\n  Run BLESS=1 to update.",
            file.relative,
            golden_bytes.len(),
            actual_bytes.len(),
        );
    }
}

/// Check that every golden file was also produced as output.
fn verify_no_stale_goldens(golden_dir: &Path, actual_files: &[OutputFile], slug: &str) {
    if !golden_dir.exists() {
        return;
    }
    let golden_files = collect_output_files(golden_dir);
    let actual_names: Vec<&str> = actual_files.iter().map(|f| f.relative.as_str()).collect();
    for golden in &golden_files {
        assert!(
            actual_names.contains(&golden.relative.as_str()),
            "[{slug}] Stale golden file: {}. CLI no longer produces this output.",
            golden.relative
        );
    }
}
