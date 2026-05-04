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
    std::fs::create_dir_all(output_dir).map_err(|e| format!("Cannot create {output_dir}: {e}"))?;

    for file in &result.files {
        if file.name.is_empty() {
            return Err(
                "Pipeline produced a file with an empty name — cannot write to output directory"
                    .to_string(),
            );
        }
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
pub fn write_results_with_mode(
    result: &PipelineResult,
    mode: &str,
    output_dir: &str,
) -> Result<WriteOutcome, String> {
    match mode {
        "write" => {
            write_results(result, output_dir)?;
            Ok(WriteOutcome::Written {
                dir: output_dir.to_string(),
                count: result.files.len(),
            })
        }
        "overwrite" => overwrite_in_place(result),
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

/// Rename source files in place — the file at `FileData::Path(src)` gets
/// renamed to the pipeline's output filename in the same directory.
fn overwrite_in_place(result: &PipelineResult) -> Result<WriteOutcome, String> {
    use bnto_core::processor::FileData;

    let mut count = 0;
    for file in &result.files {
        let FileData::Path(src) = &file.data else {
            return Err("Overwrite mode requires FileData::Path (CLI-only). \
                 Cannot overwrite in-memory data."
                .to_string());
        };

        // Derive the target filename (strip any directory separators — only leaf name matters).
        let leaf_name = Path::new(&file.name)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(&file.name);

        // Target lives in the same directory as the source file.
        let src_dir = src.parent().unwrap_or(Path::new("."));
        let dest = src_dir.join(leaf_name);

        // No-op if source and dest are the same path.
        if src == &dest {
            count += 1;
            continue;
        }

        std::fs::rename(src, &dest)
            .map_err(|e| format!("Cannot rename {} → {}: {e}", src.display(), dest.display()))?;
        count += 1;
    }

    Ok(WriteOutcome::Overwritten { count })
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

    // --- write_results_with_mode tests ---

    #[test]
    fn test_write_mode_copies_files_to_output_dir() {
        let tmp = std::env::temp_dir().join("bnto-test-write-mode-copy");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();

        let src_file = tmp.join("source.txt");
        std::fs::write(&src_file, b"hello").unwrap();

        let out_dir = tmp.join("output");
        let result = make_result_with_path(src_file.clone(), "output.txt");

        let outcome = write_results_with_mode(&result, "write", out_dir.to_str().unwrap()).unwrap();

        assert!(matches!(outcome, WriteOutcome::Written { count: 1, .. }));
        assert_eq!(std::fs::read(out_dir.join("output.txt")).unwrap(), b"hello");
        // Source still exists (write = copy semantics).
        assert!(src_file.exists());

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_overwrite_mode_renames_source_file_in_place() {
        let tmp = std::env::temp_dir().join("bnto-test-overwrite-rename");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();

        let old_file = tmp.join("old-name.txt");
        std::fs::write(&old_file, b"important data").unwrap();

        let result = make_result_with_path(old_file.clone(), "new-name.txt");
        let outcome = write_results_with_mode(&result, "overwrite", "").unwrap();

        assert_eq!(outcome, WriteOutcome::Overwritten { count: 1 });
        assert!(!old_file.exists(), "Old file should be gone");
        assert_eq!(
            std::fs::read(tmp.join("new-name.txt")).unwrap(),
            b"important data"
        );

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_overwrite_mode_noop_when_name_unchanged() {
        let tmp = std::env::temp_dir().join("bnto-test-overwrite-noop");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();

        let file = tmp.join("same.txt");
        std::fs::write(&file, b"content").unwrap();

        let result = make_result_with_path(file.clone(), "same.txt");
        let outcome = write_results_with_mode(&result, "overwrite", "").unwrap();

        assert_eq!(outcome, WriteOutcome::Overwritten { count: 1 });
        assert!(file.exists(), "File should still exist");
        assert_eq!(std::fs::read(&file).unwrap(), b"content");

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_overwrite_mode_errors_on_bytes_data() {
        let result = make_result_with_bytes(b"data", "anything.txt");
        let err = write_results_with_mode(&result, "overwrite", "").unwrap_err();
        assert!(
            err.contains("Overwrite mode requires FileData::Path"),
            "Error: {err}"
        );
    }

    #[test]
    fn test_message_mode_writes_no_files() {
        let tmp = std::env::temp_dir().join("bnto-test-message-mode");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();

        let result = make_result_with_bytes(b"data", "file.txt");
        let outcome = write_results_with_mode(&result, "message", tmp.to_str().unwrap()).unwrap();

        assert!(matches!(outcome, WriteOutcome::Message { .. }));
        // No files should have been written.
        let entries: Vec<_> = std::fs::read_dir(&tmp).unwrap().collect();
        assert_eq!(entries.len(), 0);

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_none_mode_writes_no_files() {
        let tmp = std::env::temp_dir().join("bnto-test-none-mode");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();

        let result = make_result_with_bytes(b"data", "file.txt");
        let outcome = write_results_with_mode(&result, "none", tmp.to_str().unwrap()).unwrap();

        assert_eq!(outcome, WriteOutcome::None);
        let entries: Vec<_> = std::fs::read_dir(&tmp).unwrap().collect();
        assert_eq!(entries.len(), 0);

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_overwrite_mode_strips_directory_from_filename() {
        let tmp = std::env::temp_dir().join("bnto-test-overwrite-nested");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();

        let src = tmp.join("original.txt");
        std::fs::write(&src, b"nested content").unwrap();

        // Pipeline output has a directory prefix — overwrite should only use the leaf.
        let result = make_result_with_path(src.clone(), "group/renamed.txt");
        let outcome = write_results_with_mode(&result, "overwrite", "").unwrap();

        assert_eq!(outcome, WriteOutcome::Overwritten { count: 1 });
        assert!(!src.exists());
        assert_eq!(
            std::fs::read(tmp.join("renamed.txt")).unwrap(),
            b"nested content"
        );

        let _ = std::fs::remove_dir_all(&tmp);
    }

    // --- Existing tests ---

    #[test]
    fn test_read_pipeline_file_directory() {
        let dir = std::env::temp_dir().join("bnto-test-dir-input");
        let _ = std::fs::create_dir_all(&dir);

        let pf = read_pipeline_file(dir.to_str().unwrap()).unwrap();
        assert!(pf.name.contains("bnto-test-dir-input"));
        assert_eq!(pf.mime_type, "inode/directory");
        assert!(
            matches!(&pf.data, bnto_core::processor::FileData::Bytes(b) if b.is_empty()),
            "Directory input should have empty bytes"
        );

        let _ = std::fs::remove_dir_all(&dir);
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

        let tmp = std::env::temp_dir().join("bnto-test-write-preserves-src");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();

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

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_write_results_creates_subdirectories() {
        use bnto_core::PipelineFileResult;
        use bnto_core::processor::FileData;

        let dir = std::env::temp_dir().join("bnto-test-write-subdirs");
        let _ = std::fs::remove_dir_all(&dir);

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

        let _ = std::fs::remove_dir_all(&dir);
    }
}
