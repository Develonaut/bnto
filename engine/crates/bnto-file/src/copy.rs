// file-copy — Place output files in a destination directory.
//
// CLI-only processor: writes files to the filesystem. Supports conflict
// resolution (skip/overwrite/rename) and optional directory creation.
// File data passes through unchanged — this is a placement node.

use bnto_core::metadata::{NodeCategory, NodeMetadata, OptionEntry, ParameterDef, ParameterType};
use bnto_core::processor::{NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;
use bnto_core::{BntoError, ProcessContext};
use std::path::Path;

/// File-copy processor — write files to a destination directory.
pub struct FileCopy;

impl FileCopy {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FileCopy {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeProcessor for FileCopy {
    fn name(&self) -> &str {
        "file-copy"
    }

    fn metadata(&self) -> NodeMetadata {
        NodeMetadata {
            node_type: "file-copy".to_string(),
            name: "Copy Files".to_string(),
            description: "Place output files in a destination directory with conflict handling."
                .to_string(),
            category: NodeCategory::File,
            accepts: vec![],
            platforms: vec!["cli".to_string(), "desktop".to_string()],
            parameters: build_copy_params(),
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
        progress.report(0, "Copying file...");

        // Read destination from params (required).
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

        // Build the full output path.
        let dest_path = dest_dir.join(&input.filename);

        // Handle conflicts.
        let final_path = resolve_conflict(&dest_path, conflict)?;

        // Write the file (if not skipped).
        if let Some(path) = &final_path {
            // Create parent directories if the filename contains subdirs.
            if let Some(parent) = path.parent()
                && !parent.exists()
            {
                std::fs::create_dir_all(parent).map_err(|e| {
                    BntoError::ProcessingFailed(format!(
                        "Failed to create parent dir {}: {e}",
                        parent.display()
                    ))
                })?;
            }

            std::fs::write(path, &input.data).map_err(|e| {
                BntoError::ProcessingFailed(format!("Failed to write {}: {e}", path.display()))
            })?;
        }

        progress.report(100, "Done");

        // Pass the file through unchanged (the copy is a side effect).
        let output_filename = final_path
            .as_ref()
            .map(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or(&input.filename)
                    .to_string()
            })
            .unwrap_or_else(|| input.filename.clone());

        Ok(NodeOutput {
            files: vec![OutputFile {
                data: input.data,
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

fn build_copy_params() -> Vec<ParameterDef> {
    vec![
        ParameterDef {
            name: "destination".to_string(),
            label: "Destination".to_string(),
            description: "Directory path to copy files into.".to_string(),
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
        // No conflict — write to the target path.
        return Ok(Some(dest_path.to_path_buf()));
    }

    match conflict_mode {
        "overwrite" => Ok(Some(dest_path.to_path_buf())),
        "rename" => {
            // Append a numeric suffix until we find a free name.
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
        data: &[u8],
        filename: &str,
        params: serde_json::Map<String, serde_json::Value>,
    ) -> NodeInput {
        NodeInput {
            data: data.to_vec(),
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
        assert_eq!(FileCopy::new().name(), "file-copy");
    }

    #[test]
    fn test_metadata_is_cli_only() {
        let meta = FileCopy::new().metadata();
        assert_eq!(meta.category, NodeCategory::File);
        assert!(!meta.platforms.contains(&"browser".to_string()));
        assert!(meta.platforms.contains(&"cli".to_string()));
    }

    // --- Basic copy ---

    #[test]
    fn test_copy_to_destination() {
        let dest = TempDir::new().unwrap();
        let processor = FileCopy::new();
        let progress = ProgressReporter::new_noop();

        let input = make_input(
            b"file content",
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

    // --- Create dirs ---

    #[test]
    fn test_copy_creates_destination_dir() {
        let base = TempDir::new().unwrap();
        let dest = base.path().join("new_subdir");
        let processor = FileCopy::new();
        let progress = ProgressReporter::new_noop();

        let input = make_input(
            b"data",
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
    fn test_copy_fails_without_create_dirs() {
        let processor = FileCopy::new();
        let progress = ProgressReporter::new_noop();

        let input = make_input(
            b"data",
            "file.txt",
            params(&[
                (
                    "destination",
                    serde_json::json!("/tmp/nonexistent_bnto_test_dir_12345"),
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
    fn test_copy_conflict_skip() {
        let dest = TempDir::new().unwrap();
        // Pre-create the file so there's a conflict.
        fs::write(dest.path().join("existing.txt"), b"old content").unwrap();

        let processor = FileCopy::new();
        let progress = ProgressReporter::new_noop();

        let input = make_input(
            b"new content",
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
    fn test_copy_conflict_overwrite() {
        let dest = TempDir::new().unwrap();
        fs::write(dest.path().join("existing.txt"), b"old content").unwrap();

        let processor = FileCopy::new();
        let progress = ProgressReporter::new_noop();

        let input = make_input(
            b"new content",
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
    fn test_copy_conflict_rename() {
        let dest = TempDir::new().unwrap();
        fs::write(dest.path().join("file.txt"), b"original").unwrap();

        let processor = FileCopy::new();
        let progress = ProgressReporter::new_noop();

        let input = make_input(
            b"new data",
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

    // --- Data passthrough ---

    #[test]
    fn test_copy_passes_data_through() {
        let dest = TempDir::new().unwrap();
        let processor = FileCopy::new();
        let progress = ProgressReporter::new_noop();
        let data = b"original file data";

        let input = make_input(
            data,
            "file.bin",
            params(&[(
                "destination",
                serde_json::json!(dest.path().to_str().unwrap()),
            )]),
        );

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(
            output.files[0].data, data,
            "Data should pass through unchanged"
        );
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
