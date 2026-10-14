// File I/O helpers — read input files and write pipeline results.

use std::path::Path;

use bnto_core::{PipelineFile, PipelineResult};

/// Read a file (or directory) from disk into a PipelineFile.
///
/// For regular files, reads bytes into memory. For directories, creates a
/// synthetic entry with the absolute path as the name and empty data —
/// processors like `file-collect` use the filename as the directory to traverse.
pub fn read_pipeline_file(path: &str) -> Result<PipelineFile, String> {
    let p = Path::new(path);

    if p.is_dir() {
        let abs = p
            .canonicalize()
            .map_err(|e| format!("{e}"))?
            .to_string_lossy()
            .to_string();
        return Ok(PipelineFile {
            name: abs,
            data: bnto_core::processor::FileData::Bytes(vec![]),
            mime_type: "inode/directory".to_string(),
            metadata: serde_json::Map::new(),
        });
    }

    let data = std::fs::read(path).map_err(|e| format!("{e}"))?;
    let name = p
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.to_string());
    let mime_type = guess_mime(path);

    Ok(PipelineFile {
        name,
        data: bnto_core::processor::FileData::Bytes(data),
        mime_type,
        metadata: serde_json::Map::new(),
    })
}

/// Write all result files to an output directory.
///
/// File names may contain subdirectory separators (e.g. `"subdir/video.mp4"`).
/// Parent directories are created automatically before writing.
///
/// Uses `FileData::copy_to()` which copies for path-referenced files
/// and writes bytes for in-memory files. Source files are never modified.
pub fn write_results(result: &PipelineResult, output_dir: &str) -> Result<(), String> {
    check_file_names(result)?;
    std::fs::create_dir_all(output_dir).map_err(|e| format!("Cannot create {output_dir}: {e}"))?;

    for file in &result.files {
        let out_path = Path::new(output_dir).join(&file.name);
        if let Some(parent) = out_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Cannot create {}: {e}", parent.display()))?;
        }
        file.data
            .copy_to(&out_path)
            .map_err(|e| format!("Cannot write {}: {e}", out_path.display()))?;
    }

    Ok(())
}

/// Fail on a result file with no name, for a real write and a dry run alike.
fn check_file_names(result: &PipelineResult) -> Result<(), String> {
    if result.files.iter().any(|f| f.name.is_empty()) {
        return Err(
            "Pipeline produced a file with an empty name — cannot write to output directory"
                .to_string(),
        );
    }
    Ok(())
}

// =============================================================================
// Mode-aware output
// =============================================================================

/// Outcome of a mode-aware write operation.
#[derive(Debug, PartialEq)]
pub enum WriteOutcome {
    /// Files were copied to an output directory.
    Written { dir: String, count: usize },
    /// Files were renamed in place (overwrite mode).
    Overwritten { count: usize },
    /// No files written — summary message only.
    Message { summary: String },
    /// No output action taken.
    None,
}

/// Write pipeline results based on the output node's mode.
///
/// - `write`: Copy files to `output_dir` (default behavior).
/// - `overwrite`: Rename source files in place (requires `FileData::Path`).
/// - `message`: No file writes, returns a summary.
/// - `none`: No output action.
///
/// On a `dry_run` nothing is written or renamed; the outcome is what a real
/// run would report.
pub fn write_results_with_mode(
    result: &PipelineResult,
    mode: &str,
    output_dir: &str,
    dry_run: bool,
) -> Result<WriteOutcome, String> {
    match mode {
        "write" => {
            check_file_names(result)?;
            if !dry_run {
                write_results(result, output_dir)?;
            }
            Ok(WriteOutcome::Written {
                dir: output_dir.to_string(),
                count: result.files.len(),
            })
        }
        "overwrite" => crate::overwrite::overwrite_in_place(result, dry_run)
            .map(|count| WriteOutcome::Overwritten { count }),
        "message" => {
            let summary = if result.files.is_empty() {
                "No output files produced.".to_string()
            } else {
                let names: Vec<&str> = result.files.iter().map(|f| f.name.as_str()).collect();
                format!("{} file(s): {}", names.len(), names.join(", "))
            };
            Ok(WriteOutcome::Message { summary })
        }
        _ => Ok(WriteOutcome::None),
    }
}

/// Names of result files that `file-copy` left because the destination copy
/// was newer (its `skipped_newer` metadata flag is 1).
pub fn files_left_as_newer(result: &PipelineResult) -> Vec<&str> {
    result
        .files
        .iter()
        .filter(|f| f.metadata.get("skipped_newer").and_then(|v| v.as_i64()) == Some(1))
        .map(|f| f.name.as_str())
        .collect()
}

/// Per-file `file-copy` outcomes summed: (copied, up to date, newer at the
/// destination). `None` when no file carries any of the three flags.
pub fn copy_totals(result: &PipelineResult) -> Option<(usize, usize, usize)> {
    let keys = ["copied", "up_to_date", "skipped_newer"];
    let count = |key: &str| {
        result
            .files
            .iter()
            .filter(|f| f.metadata.get(key).and_then(|v| v.as_i64()) == Some(1))
            .count()
    };
    let seen = result
        .files
        .iter()
        .any(|f| keys.iter().any(|k| f.metadata.contains_key(*k)));
    seen.then(|| (count(keys[0]), count(keys[1]), count(keys[2])))
}

/// Print one `Copy:` line from `copy_totals`; a dry run says `would be copied`.
pub fn print_copy_totals(result: &PipelineResult, dry_run: bool) {
    if let Some((copied, up_to_date, newer)) = copy_totals(result) {
        let verb = if dry_run { "would be copied" } else { "copied" };
        eprintln!(
            "  Copy: {copied} {verb}, {up_to_date} up to date, {newer} newer at the destination"
        );
    }
}

/// Print one warning line naming the files `files_left_as_newer` finds.
pub fn warn_files_left_as_newer(result: &PipelineResult) {
    use colored::Colorize;

    let newer = files_left_as_newer(result);
    if !newer.is_empty() {
        let warning = "Warning:".yellow().bold();
        eprintln!("  {warning} {}", newer_warning_text(&newer));
    }
}

/// How many names the warning lists before folding the rest into `…and N more`.
const MAX_NAMES_LISTED: usize = 10;

/// `N file(s) newer at the destination, left: a, b`, names capped.
fn newer_warning_text(newer: &[&str]) -> String {
    let noun = if newer.len() == 1 { "file" } else { "files" };
    let mut names = newer
        .iter()
        .take(MAX_NAMES_LISTED)
        .copied()
        .collect::<Vec<_>>()
        .join(", ");
    if newer.len() > MAX_NAMES_LISTED {
        names.push_str(&format!(", …and {} more", newer.len() - MAX_NAMES_LISTED));
    }
    format!(
        "{} {noun} newer at the destination, left: {names}",
        newer.len()
    )
}

/// Guess MIME type from file extension.
fn guess_mime(path: &str) -> String {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "webp" => "image/webp",
        "gif" => "image/gif",
        "csv" => "text/csv",
        "json" => "application/json",
        "txt" => "text/plain",
        _ => "application/octet-stream",
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::PipelineFileResult;
    use bnto_core::processor::FileData;

    fn make_result_with_path(src: std::path::PathBuf, name: &str) -> PipelineResult {
        PipelineResult {
            files: vec![PipelineFileResult {
                name: name.to_string(),
                data: FileData::Path(src),
                mime_type: "text/plain".to_string(),
                metadata: serde_json::Map::new(),
            }],
            duration_ms: 0,
            warnings: vec![],
        }
    }

    fn make_result_with_bytes(data: &[u8], name: &str) -> PipelineResult {
        PipelineResult {
            files: vec![PipelineFileResult {
                name: name.to_string(),
                data: FileData::Bytes(data.to_vec()),
                mime_type: "text/plain".to_string(),
                metadata: serde_json::Map::new(),
            }],
            duration_ms: 0,
            warnings: vec![],
        }
    }

    // --- files_left_as_newer tests ---

    fn result_with_flags(files: &[(&str, Option<i64>)]) -> PipelineResult {
        let mut result = make_result_with_bytes(b"", "unused");
        result.files = files
            .iter()
            .map(|(name, flag)| {
                let mut f = make_result_with_bytes(b"", name).files.remove(0);
                if let Some(v) = flag {
                    f.metadata.insert("skipped_newer".into(), (*v).into());
                }
                f
            })
            .collect();
        result
    }

    #[test]
    fn test_copy_totals_none_without_flags() {
        assert_eq!(copy_totals(&result_with_flags(&[("a", None)])), None);
        assert_eq!(copy_totals(&result_with_flags(&[])), None);
    }

    #[test]
    fn test_copy_totals_sums_mixed_flags() {
        let mut result = result_with_flags(&[("a", Some(1)), ("b", Some(0)), ("c", None)]);
        result.files[1].metadata.insert("copied".into(), 1.into());
        result.files[2].metadata.insert("copied".into(), 1.into());
        result.files[2]
            .metadata
            .insert("up_to_date".into(), 1.into());
        assert_eq!(copy_totals(&result), Some((2, 1, 1)));
    }

    #[test]
    fn test_dry_run_write_mode_errors_on_empty_filename() {
        let result = make_result_with_bytes(b"data", "");
        let err = write_results_with_mode(&result, "write", "out", true).unwrap_err();
        assert!(err.contains("empty name"), "Error: {err}");
    }

    #[test]
    fn test_files_left_as_newer_empty_result() {
        assert!(files_left_as_newer(&result_with_flags(&[])).is_empty());
    }

    #[test]
    fn test_files_left_as_newer_one_flagged() {
        let r = result_with_flags(&[("a.txt", Some(0)), ("b.txt", Some(1))]);
        assert_eq!(files_left_as_newer(&r), vec!["b.txt"]);
    }

    #[test]
    fn test_files_left_as_newer_several_flagged_in_order() {
        let r = result_with_flags(&[("a.txt", Some(1)), ("b.txt", Some(0)), ("c.txt", Some(1))]);
        assert_eq!(files_left_as_newer(&r), vec!["a.txt", "c.txt"]);
    }

    #[test]
    fn test_files_left_as_newer_ignores_files_without_the_key() {
        let r = result_with_flags(&[("a.txt", None), ("b.txt", None)]);
        assert!(files_left_as_newer(&r).is_empty());
    }

    #[test]
    fn test_newer_warning_text_singular_and_plural() {
        assert_eq!(
            newer_warning_text(&["a.txt"]),
            "1 file newer at the destination, left: a.txt"
        );
        assert_eq!(
            newer_warning_text(&["a.txt", "b.txt"]),
            "2 files newer at the destination, left: a.txt, b.txt"
        );
    }

    #[test]
    fn test_newer_warning_text_caps_the_name_list() {
        let names: Vec<String> = (1..=12).map(|i| format!("f{i}.txt")).collect();
        let refs: Vec<&str> = names.iter().map(String::as_str).collect();
        let text = newer_warning_text(&refs);
        assert!(text.starts_with("12 files newer at the destination, left: f1.txt"));
        assert!(text.contains("f10.txt"));
        assert!(!text.contains("f11.txt"));
        assert!(text.ends_with(", …and 2 more"));
    }

    #[test]
    fn test_newer_warning_text_cap_boundary() {
        let names: Vec<String> = (1..=11).map(|i| format!("f{i}.txt")).collect();
        let refs: Vec<&str> = names.iter().map(String::as_str).collect();
        assert!(newer_warning_text(&refs[..10]).ends_with("f10.txt"));
        assert!(newer_warning_text(&refs).ends_with("f10.txt, …and 1 more"));
    }

    // --- write_results_with_mode tests ---

    #[test]
    fn test_write_mode_copies_files_to_output_dir() {
        let temp = tempfile::tempdir().unwrap();
        let tmp = temp.path().to_path_buf();

        let src_file = tmp.join("source.txt");
        std::fs::write(&src_file, b"hello").unwrap();

        let out_dir = tmp.join("output");
        let result = make_result_with_path(src_file.clone(), "output.txt");

        let outcome =
            write_results_with_mode(&result, "write", out_dir.to_str().unwrap(), false).unwrap();

        assert!(matches!(outcome, WriteOutcome::Written { count: 1, .. }));
        assert_eq!(std::fs::read(out_dir.join("output.txt")).unwrap(), b"hello");
        // Source still exists (write = copy semantics).
        assert!(src_file.exists());
    }

    #[test]
    fn test_overwrite_mode_renames_source_file_in_place() {
        let temp = tempfile::tempdir().unwrap();
        let tmp = temp.path().to_path_buf();

        let old_file = tmp.join("old-name.txt");
        std::fs::write(&old_file, b"important data").unwrap();

        let result = make_result_with_path(old_file.clone(), "new-name.txt");
        let outcome = write_results_with_mode(&result, "overwrite", "", false).unwrap();

        assert_eq!(outcome, WriteOutcome::Overwritten { count: 1 });
        assert!(!old_file.exists(), "Old file should be gone");
        assert_eq!(
            std::fs::read(tmp.join("new-name.txt")).unwrap(),
            b"important data"
        );
    }

    #[test]
    fn test_overwrite_mode_noop_when_name_unchanged() {
        let temp = tempfile::tempdir().unwrap();
        let tmp = temp.path().to_path_buf();

        let file = tmp.join("same.txt");
        std::fs::write(&file, b"content").unwrap();

        let result = make_result_with_path(file.clone(), "same.txt");
        let outcome = write_results_with_mode(&result, "overwrite", "", false).unwrap();

        assert_eq!(outcome, WriteOutcome::Overwritten { count: 1 });
        assert!(file.exists(), "File should still exist");
        assert_eq!(std::fs::read(&file).unwrap(), b"content");
    }

    #[test]
    fn test_overwrite_mode_errors_on_bytes_data() {
        let result = make_result_with_bytes(b"data", "anything.txt");
        let err = write_results_with_mode(&result, "overwrite", "", false).unwrap_err();
        assert!(
            err.contains("Overwrite mode requires FileData::Path"),
            "Error: {err}"
        );
    }

    #[test]
    fn test_message_mode_writes_no_files() {
        let temp = tempfile::tempdir().unwrap();
        let tmp = temp.path().to_path_buf();

        let result = make_result_with_bytes(b"data", "file.txt");
        let outcome =
            write_results_with_mode(&result, "message", tmp.to_str().unwrap(), false).unwrap();

        assert!(matches!(outcome, WriteOutcome::Message { .. }));
        // No files should have been written.
        let entries: Vec<_> = std::fs::read_dir(&tmp).unwrap().collect();
        assert_eq!(entries.len(), 0);
    }

    #[test]
    fn test_none_mode_writes_no_files() {
        let temp = tempfile::tempdir().unwrap();
        let tmp = temp.path().to_path_buf();

        let result = make_result_with_bytes(b"data", "file.txt");
        let outcome =
            write_results_with_mode(&result, "none", tmp.to_str().unwrap(), false).unwrap();

        assert_eq!(outcome, WriteOutcome::None);
        let entries: Vec<_> = std::fs::read_dir(&tmp).unwrap().collect();
        assert_eq!(entries.len(), 0);
    }

    #[test]
    fn test_overwrite_mode_strips_directory_from_filename() {
        let temp = tempfile::tempdir().unwrap();
        let tmp = temp.path().to_path_buf();

        let src = tmp.join("original.txt");
        std::fs::write(&src, b"nested content").unwrap();

        // Pipeline output has a directory prefix — overwrite should only use the leaf.
        let result = make_result_with_path(src.clone(), "group/renamed.txt");
        let outcome = write_results_with_mode(&result, "overwrite", "", false).unwrap();

        assert_eq!(outcome, WriteOutcome::Overwritten { count: 1 });
        assert!(!src.exists());
        assert_eq!(
            std::fs::read(tmp.join("renamed.txt")).unwrap(),
            b"nested content"
        );
    }

    #[test]
    fn test_dry_run_write_mode_writes_nothing_and_counts() {
        let temp = tempfile::tempdir().unwrap();
        let tmp = temp.path().to_path_buf();
        let src = tmp.join("source.txt");
        std::fs::write(&src, b"hello").unwrap();
        let out_dir = tmp.join("output");
        let result = make_result_with_path(src, "output.txt");

        let outcome =
            write_results_with_mode(&result, "write", out_dir.to_str().unwrap(), true).unwrap();

        assert!(matches!(outcome, WriteOutcome::Written { count: 1, .. }));
        assert!(!out_dir.exists(), "dry run must not create the output dir");
    }

    #[test]
    fn test_dry_run_overwrite_mode_leaves_source_and_counts() {
        let temp = tempfile::tempdir().unwrap();
        let tmp = temp.path().to_path_buf();
        let old_file = tmp.join("old-name.txt");
        std::fs::write(&old_file, b"data").unwrap();
        let result = make_result_with_path(old_file.clone(), "new-name.txt");

        let outcome = write_results_with_mode(&result, "overwrite", "", true).unwrap();

        assert_eq!(outcome, WriteOutcome::Overwritten { count: 1 });
        assert!(old_file.exists(), "dry run must not rename the source");
        assert!(!tmp.join("new-name.txt").exists());
    }

    #[test]
    fn test_dry_run_overwrite_mode_still_errors_on_bytes_data() {
        let result = make_result_with_bytes(b"data", "anything.txt");
        let err = write_results_with_mode(&result, "overwrite", "", true).unwrap_err();
        assert!(
            err.contains("Overwrite mode requires FileData::Path"),
            "Error: {err}"
        );
    }

    // --- Existing tests ---

    #[test]
    fn test_read_pipeline_file_directory() {
        let temp = tempfile::Builder::new()
            .prefix("bnto-test-dir-input")
            .tempdir()
            .unwrap();
        let dir = temp.path().to_path_buf();

        let pf = read_pipeline_file(dir.to_str().unwrap()).unwrap();
        assert!(pf.name.contains("bnto-test-dir-input"));
        assert_eq!(pf.mime_type, "inode/directory");
        assert!(
            matches!(&pf.data, bnto_core::processor::FileData::Bytes(b) if b.is_empty()),
            "Directory input should have empty bytes"
        );
    }

    #[test]
    fn test_guess_mime_jpeg() {
        assert_eq!(guess_mime("photo.jpg"), "image/jpeg");
        assert_eq!(guess_mime("photo.jpeg"), "image/jpeg");
        assert_eq!(guess_mime("photo.JPG"), "image/jpeg");
    }

    #[test]
    fn test_guess_mime_png() {
        assert_eq!(guess_mime("image.png"), "image/png");
    }

    #[test]
    fn test_guess_mime_csv() {
        assert_eq!(guess_mime("data.csv"), "text/csv");
    }

    #[test]
    fn test_guess_mime_unknown() {
        assert_eq!(guess_mime("file.xyz"), "application/octet-stream");
        assert_eq!(guess_mime("noext"), "application/octet-stream");
    }

    #[test]
    fn test_write_results_preserves_source_for_path_refs() {
        use bnto_core::PipelineFileResult;
        use bnto_core::processor::FileData;

        let temp = tempfile::tempdir().unwrap();
        let tmp = temp.path().to_path_buf();

        // Create a source file that the pipeline references by path.
        let src_file = tmp.join("original.txt");
        std::fs::write(&src_file, b"keep me").unwrap();

        let out_dir = tmp.join("output");
        let result = PipelineResult {
            files: vec![PipelineFileResult {
                name: "renamed.txt".to_string(),
                data: FileData::Path(src_file.clone()),
                mime_type: "text/plain".to_string(),
                metadata: serde_json::Map::new(),
            }],
            duration_ms: 0,
            warnings: vec![],
        };

        write_results(&result, out_dir.to_str().unwrap()).unwrap();

        // Output file exists with correct content.
        assert_eq!(
            std::fs::read(out_dir.join("renamed.txt")).unwrap(),
            b"keep me"
        );
        // Source file is preserved — not moved or deleted.
        assert!(src_file.exists(), "Source file must survive write_results");
    }

    #[test]
    fn test_write_results_creates_subdirectories() {
        use bnto_core::PipelineFileResult;
        use bnto_core::processor::FileData;

        // write_results must create the output dir itself, so it starts absent.
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().join("out");

        let result = PipelineResult {
            files: vec![
                PipelineFileResult {
                    name: "top.txt".to_string(),
                    data: FileData::Bytes(b"top-level".to_vec()),
                    mime_type: "text/plain".to_string(),
                    metadata: serde_json::Map::new(),
                },
                PipelineFileResult {
                    name: "group/nested.mp4".to_string(),
                    data: FileData::Bytes(b"video-data".to_vec()),
                    mime_type: "video/mp4".to_string(),
                    metadata: serde_json::Map::new(),
                },
                PipelineFileResult {
                    name: "a/b/deep.txt".to_string(),
                    data: FileData::Bytes(b"deep-data".to_vec()),
                    mime_type: "text/plain".to_string(),
                    metadata: serde_json::Map::new(),
                },
            ],
            duration_ms: 0,
            warnings: vec![],
        };

        write_results(&result, dir.to_str().unwrap()).unwrap();

        assert!(dir.join("top.txt").exists());
        assert!(dir.join("group").join("nested.mp4").exists());
        assert!(dir.join("a").join("b").join("deep.txt").exists());
        assert_eq!(
            std::fs::read(dir.join("group/nested.mp4")).unwrap(),
            b"video-data"
        );
    }
}
