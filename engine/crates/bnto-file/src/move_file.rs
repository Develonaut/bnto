// file-move — Move output files to a destination directory.
//
// CLI-only processor: moves files to the filesystem using rename() for
// zero-copy O(1) moves when source and destination are on the same
// filesystem. Falls back to copy+delete for cross-filesystem moves.
// Supports conflict resolution (skip/overwrite/rename) and optional
// directory creation.

use bnto_core::metadata::{NodeCategory, NodeMetadata, OptionEntry, ParameterDef, ParameterType};
use bnto_core::processor::{FileData, NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;
use bnto_core::{BntoError, ProcessContext};
use std::path::Path;

/// File-move processor — move files to a destination directory.
///
/// Unlike file-copy, this uses `rename()` for zero-copy moves when possible.
/// The output `FileData` is `Path(dest_path)`, pointing to the moved file.
pub struct FileMove;

impl FileMove {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FileMove {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeProcessor for FileMove {
    fn name(&self) -> &str {
        "file-move"
    }

    fn metadata(&self) -> NodeMetadata {
        NodeMetadata {
            node_type: "file-move".to_string(),
            name: "Move Files".to_string(),
            description: "Move output files to a destination directory with conflict handling."
                .to_string(),
            category: NodeCategory::File,
            accepts: vec![],
            platforms: vec!["cli".to_string(), "desktop".to_string()],
            parameters: build_move_params(),
            input_cardinality: Default::default(), // PerFile
            requires: vec![],
        }
    }

    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        progress.report(0, "Moving file...");

        let destination = input
            .params
            .get("destination")
            .and_then(|v| v.as_str())
            .unwrap_or(".");

        let create_dirs = input
            .params
            .get("create_dirs")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let conflict = input
            .params
            .get("conflict")
            .and_then(|v| v.as_str())
            .unwrap_or("skip");

        let dest_dir = Path::new(destination);

        // Create destination directory if needed.
        if !dest_dir.exists() {
            if create_dirs {
                std::fs::create_dir_all(dest_dir).map_err(|e| {
                    BntoError::ProcessingFailed(format!(
                        "Failed to create directory {}: {e}",
                        dest_dir.display()
                    ))
                })?;
            } else {
                return Err(BntoError::InvalidInput(format!(
                    "Destination directory does not exist: {}",
                    dest_dir.display()
                )));
            }
        }

        let dest_path = dest_dir.join(&input.filename);

        // Create parent directories for nested filenames (e.g. "group/video.mp4").
        if let Some(parent) = dest_path.parent()
            && !parent.exists()
        {
            std::fs::create_dir_all(parent).map_err(|e| {
                BntoError::ProcessingFailed(format!(
                    "Failed to create parent dir {}: {e}",
                    parent.display()
                ))
            })?;
        }

        let final_path = resolve_conflict(&dest_path, conflict)?;

        // Move the file (if not skipped).
        if let Some(path) = &final_path {
            input.data.write_to(path).map_err(|e| {
                BntoError::ProcessingFailed(format!("Failed to move {}: {e}", path.display()))
            })?;
        }

        progress.report(100, "Done");

        let output_filename = final_path
            .as_ref()
            .map(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or(&input.filename)
                    .to_string()
            })
            .unwrap_or_else(|| input.filename.clone());

        let output_data = final_path.map(FileData::Path).unwrap_or(input.data);

        Ok(NodeOutput {
            files: vec![OutputFile {
                data: output_data,
                filename: output_filename,
                mime_type: input
                    .mime_type
                    .unwrap_or_else(|| "application/octet-stream".to_string()),
                metadata: serde_json::Map::new(),
            }],
            metadata: serde_json::Map::new(),
        })
    }
}

// --- Parameter Definitions ---

fn build_move_params() -> Vec<ParameterDef> {
    vec![
        ParameterDef {
            name: "destination".to_string(),
            label: "Destination".to_string(),
            description: "Directory path to move files into.".to_string(),
            param_type: ParameterType::String,
            placeholder: Some("./output".to_string()),
            ..Default::default()
        },
        ParameterDef {
            name: "create_dirs".to_string(),
            label: "Create Directories".to_string(),
            description: "Automatically create the destination directory if it doesn't exist."
                .to_string(),
            param_type: ParameterType::Boolean,
            default: Some(serde_json::json!(true)),
            ..Default::default()
        },
        ParameterDef {
            name: "conflict".to_string(),
            label: "Conflict Resolution".to_string(),
            description: "What to do when a file with the same name already exists.".to_string(),
            param_type: ParameterType::Enum {
                options: vec![
                    OptionEntry {
                        value: "skip".to_string(),
                        label: "Skip".to_string(),
                    },
                    OptionEntry {
                        value: "overwrite".to_string(),
                        label: "Overwrite".to_string(),
                    },
                    OptionEntry {
                        value: "rename".to_string(),
                        label: "Rename (add suffix)".to_string(),
                    },
                ],
            },
            default: Some(serde_json::json!("skip")),
            ..Default::default()
        },
    ]
}

// --- Conflict Resolution ---

/// Resolve filename conflicts in the destination directory.
///
/// Returns `Some(path)` to write to, or `None` if the file should be skipped.
fn resolve_conflict(
    dest_path: &Path,
    conflict_mode: &str,
) -> Result<Option<std::path::PathBuf>, BntoError> {
    if !dest_path.exists() {
        return Ok(Some(dest_path.to_path_buf()));
    }

    match conflict_mode {
        "overwrite" => Ok(Some(dest_path.to_path_buf())),
        "rename" => {
            let stem = dest_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("file");
            let ext = dest_path.extension().and_then(|e| e.to_str()).unwrap_or("");
            let parent = dest_path.parent().unwrap_or(Path::new("."));

            for i in 1..1000 {
                let new_name = if ext.is_empty() {
                    format!("{stem}_{i}")
                } else {
                    format!("{stem}_{i}.{ext}")
                };
                let candidate = parent.join(&new_name);
                if !candidate.exists() {
                    return Ok(Some(candidate));
                }
            }

            Err(BntoError::ProcessingFailed(
                "Could not find a free filename after 999 attempts".to_string(),
            ))
        }
        _ => {
            // "skip" (default) — don't write, just pass through.
            Ok(None)
        }
    }
}

// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::context::NoopContext;
    use bnto_core::progress::ProgressReporter;
    use std::fs;
    use tempfile::TempDir;

    fn make_input(
        data: FileData,
        filename: &str,
        params: serde_json::Map<String, serde_json::Value>,
    ) -> NodeInput {
        NodeInput {
            data,
            filename: filename.to_string(),
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
        assert_eq!(FileMove::new().name(), "file-move");
    }

    #[test]
    fn test_metadata_is_cli_only() {
        let meta = FileMove::new().metadata();
        assert_eq!(meta.category, NodeCategory::File);
        assert!(!meta.platforms.contains(&"browser".to_string()));
        assert!(meta.platforms.contains(&"cli".to_string()));
    }

    // --- Move with Bytes data ---

    #[test]
    fn test_move_bytes_to_destination() {
        let dest = TempDir::new().unwrap();
        let processor = FileMove::new();
        let progress = ProgressReporter::new_noop();

        let input = make_input(
            FileData::Bytes(b"file content".to_vec()),
            "output.txt",
            params(&[(
                "destination",
                serde_json::json!(dest.path().to_str().unwrap()),
            )]),
        );

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files.len(), 1);

        // Verify file was written to disk.
        let written = dest.path().join("output.txt");
        assert!(written.exists(), "File should be written to destination");
        assert_eq!(fs::read(&written).unwrap(), b"file content");
    }

    // --- Move with Path data (zero-copy) ---

    #[test]
    fn test_move_path_to_destination() {
        let src_dir = TempDir::new().unwrap();
        let dest = TempDir::new().unwrap();

        // Create a source file.
        let src_file = src_dir.path().join("video.mp4");
        fs::write(&src_file, b"video data").unwrap();

        let processor = FileMove::new();
        let progress = ProgressReporter::new_noop();

        let input = make_input(
            FileData::Path(src_file.clone()),
            "video.mp4",
            params(&[(
                "destination",
                serde_json::json!(dest.path().to_str().unwrap()),
            )]),
        );

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files.len(), 1);

        // Source should be removed (moved).
        assert!(
            !src_file.exists(),
            "Source file should be removed after move"
        );

        // Destination should have the file.
        let dest_file = dest.path().join("video.mp4");
        assert!(dest_file.exists(), "File should exist at destination");
        assert_eq!(fs::read(&dest_file).unwrap(), b"video data");

        // Output data should be a Path pointing to the destination.
        assert!(
            matches!(&output.files[0].data, FileData::Path(p) if p == &dest_file),
            "Output should be FileData::Path pointing to destination"
        );
    }

    // --- Create dirs ---

    #[test]
    fn test_move_creates_destination_dir() {
        let base = TempDir::new().unwrap();
        let dest = base.path().join("new_subdir");
        let processor = FileMove::new();
        let progress = ProgressReporter::new_noop();

        let input = make_input(
            FileData::Bytes(b"data".to_vec()),
            "file.txt",
            params(&[
                ("destination", serde_json::json!(dest.to_str().unwrap())),
                ("create_dirs", serde_json::json!(true)),
            ]),
        );

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files.len(), 1);
        assert!(dest.join("file.txt").exists());
    }

    #[test]
    fn test_move_fails_without_create_dirs() {
        let processor = FileMove::new();
        let progress = ProgressReporter::new_noop();

        let input = make_input(
            FileData::Bytes(b"data".to_vec()),
            "file.txt",
            params(&[
                (
                    "destination",
                    serde_json::json!("/tmp/nonexistent_bnto_move_test_dir_12345"),
                ),
                ("create_dirs", serde_json::json!(false)),
            ]),
        );

        let result = processor.process(input, &progress, &NoopContext);
        assert!(
            result.is_err(),
            "Should fail when dir missing and create_dirs=false"
        );
    }

    // --- Conflict resolution ---

    #[test]
    fn test_move_conflict_skip() {
        let dest = TempDir::new().unwrap();
        fs::write(dest.path().join("existing.txt"), b"old content").unwrap();

        let processor = FileMove::new();
        let progress = ProgressReporter::new_noop();

        let input = make_input(
            FileData::Bytes(b"new content".to_vec()),
            "existing.txt",
            params(&[
                (
                    "destination",
                    serde_json::json!(dest.path().to_str().unwrap()),
                ),
                ("conflict", serde_json::json!("skip")),
            ]),
        );

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files.len(), 1);

        // Original file should be unchanged.
        let content = fs::read(dest.path().join("existing.txt")).unwrap();
        assert_eq!(content, b"old content", "Skip should not overwrite");
    }

    #[test]
    fn test_move_conflict_overwrite() {
        let dest = TempDir::new().unwrap();
        fs::write(dest.path().join("existing.txt"), b"old content").unwrap();

        let processor = FileMove::new();
        let progress = ProgressReporter::new_noop();

        let input = make_input(
            FileData::Bytes(b"new content".to_vec()),
            "existing.txt",
            params(&[
                (
                    "destination",
                    serde_json::json!(dest.path().to_str().unwrap()),
                ),
                ("conflict", serde_json::json!("overwrite")),
            ]),
        );

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files.len(), 1);

        let content = fs::read(dest.path().join("existing.txt")).unwrap();
        assert_eq!(content, b"new content", "Overwrite should replace file");
    }

    #[test]
    fn test_move_conflict_rename() {
        let dest = TempDir::new().unwrap();
        fs::write(dest.path().join("file.txt"), b"original").unwrap();

        let processor = FileMove::new();
        let progress = ProgressReporter::new_noop();

        let input = make_input(
            FileData::Bytes(b"new data".to_vec()),
            "file.txt",
            params(&[
                (
                    "destination",
                    serde_json::json!(dest.path().to_str().unwrap()),
                ),
                ("conflict", serde_json::json!("rename")),
            ]),
        );

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files.len(), 1);
        assert_eq!(output.files[0].filename, "file_1.txt");

        // Original should be untouched.
        assert_eq!(fs::read(dest.path().join("file.txt")).unwrap(), b"original");
        // Renamed copy should exist.
        assert_eq!(
            fs::read(dest.path().join("file_1.txt")).unwrap(),
            b"new data"
        );
    }

    // --- Nested filenames ---

    #[test]
    fn test_move_creates_parent_dirs_for_nested_filename() {
        let dest = TempDir::new().unwrap();
        let processor = FileMove::new();
        let progress = ProgressReporter::new_noop();

        let input = make_input(
            FileData::Bytes(b"nested data".to_vec()),
            "group/subdir/video.mp4",
            params(&[(
                "destination",
                serde_json::json!(dest.path().to_str().unwrap()),
            )]),
        );

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files.len(), 1);

        let nested = dest.path().join("group/subdir/video.mp4");
        assert!(nested.exists(), "Nested directories should be created");
        assert_eq!(fs::read(&nested).unwrap(), b"nested data");
    }

    // --- Pure function tests ---

    #[test]
    fn test_resolve_conflict_no_conflict() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("new.txt");

        let result = resolve_conflict(&path, "skip").unwrap();
        assert_eq!(result, Some(path));
    }

    #[test]
    fn test_resolve_conflict_skip_existing() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("existing.txt");
        fs::write(&path, b"data").unwrap();

        let result = resolve_conflict(&path, "skip").unwrap();
        assert_eq!(result, None, "Skip should return None for existing file");
    }
}
