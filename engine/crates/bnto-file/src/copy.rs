// file-copy — Place output files in a destination directory.
//
// CLI-only processor: writes files to the filesystem. Supports conflict
// resolution (skip/overwrite/rename/newer) and optional directory creation.
// File data passes through unchanged — this is a placement node. It runs as a
// batch, so every copy is checked before any runs (`copy_batch`).

use crate::copy_batch::{copy_each, per_file, refuse_replacing_read_files};
use crate::copy_newer::{DEFAULT_TOLERANCE_MS, OUTCOME_KEYS, process_newer};
use bnto_core::metadata::{
    Constraints, InputCardinality, NodeCategory, NodeMetadata, OptionEntry, ParamCondition,
    ParamConditionEntry, ParameterDef, ParameterType,
};
use bnto_core::processor::{
    BatchInput, FileData, NodeInput, NodeOutput, NodeProcessor, OutputFile,
};
use bnto_core::progress::ProgressReporter;
use bnto_core::{BntoError, ProcessContext};
use std::path::{Path, PathBuf};

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
            description: "Copy output files into a destination folder, with conflict handling."
                .to_string(),
            category: NodeCategory::File,
            accepts: vec![],
            platforms: vec!["cli".to_string(), "desktop".to_string()],
            parameters: build_copy_params(),
            input_cardinality: InputCardinality::Batch,
            requires: vec![],
        }
    }

    /// `newer` mode's per-file outcome flags, added up over the node's files.
    fn count_keys(&self) -> &'static [&'static str] {
        &OUTCOME_KEYS
    }

    /// `{{item.*}}` in its params is filled per file (`copy_batch`), as it
    /// was when this was a per-file node.
    fn resolves_items(&self) -> bool {
        true
    }

    /// Check every copy, then run them in order (`process` copies one file).
    fn process_batch(
        &self,
        input: BatchInput,
        progress: &ProgressReporter,
        ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let files = per_file(input, ctx)?;
        refuse_replacing_read_files(&files, ctx)?;
        copy_each(files, ctx, |input| self.process(input, progress, ctx))
    }

    #[expect(clippy::too_many_lines)]
    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
        ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        progress.report(0, "Copying file...");

        let create_dirs = input
            .params
            .get("createDirs")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let conflict = conflict_mode(&input.params);
        let dest_path = dest_path(&input);
        let dest_dir = Path::new(destination(&input.params));

        // A dry run decides each file against the destination as it stands, so two
        // inputs sharing a destination name both decide against the empty slot
        // (a real run sees the first one's file).
        let dry_run = ctx.dry_run();

        // Create destination directory if needed (a dry run creates nothing).
        if !dest_dir.exists() {
            if create_dirs {
                if !dry_run {
                    std::fs::create_dir_all(dest_dir).map_err(|e| {
                        BntoError::ProcessingFailed(format!(
                            "Failed to create directory {}: {e}",
                            dest_dir.display()
                        ))
                    })?;
                }
            } else {
                return Err(BntoError::InvalidInput(format!(
                    "Destination directory does not exist: {}",
                    dest_dir.display()
                )));
            }
        }

        if conflict == "newer" {
            let output = process_newer(input, &dest_path, ctx)?;
            progress.report(100, "Done");
            return Ok(output);
        }

        // Handle conflicts.
        let final_path = resolve_conflict(&dest_path, conflict)?;

        // Write the file (if not skipped).
        if let Some(path) = final_path.as_ref().filter(|_| !dry_run) {
            write_copy(&input, path, ctx)?;
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

/// Copy `input` to `path`, creating the folders a name with subdirs needs,
/// and record the copy with the run. A file already at `path` is left as
/// is (ISS-36): no copy is made, so none is recorded.
fn write_copy(input: &NodeInput, path: &Path, ctx: &dyn ProcessContext) -> Result<(), BntoError> {
    if let Some(parent) = path.parent()
        && !parent.exists()
    {
        std::fs::create_dir_all(parent).map_err(|e| {
            let at = parent.display();
            BntoError::ProcessingFailed(format!("Failed to create parent dir {at}: {e}"))
        })?;
    }
    let in_place = input.data.is_at(path);
    input
        .data
        .copy_to(path)
        .map_err(|e| copy_error(&input.data, path, e))?;
    if !in_place {
        ctx.record_write(path);
    }
    Ok(())
}

/// Why copying `data` to `path` failed: the source, when it's the file
/// that's gone (ISS-83), else the destination.
fn copy_error(data: &FileData, path: &Path, e: std::io::Error) -> BntoError {
    let message = match data {
        FileData::Path(src) if !src.exists() => format!("{} doesn't exist", src.display()),
        _ => format!("Failed to write {}: {e}", path.display()),
    };
    BntoError::ProcessingFailed(message)
}

/// The `destination` folder, `.` when unset.
fn destination(params: &serde_json::Map<String, serde_json::Value>) -> &str {
    params
        .get("destination")
        .and_then(|v| v.as_str())
        .unwrap_or(".")
}

/// Where `input` lands before conflict handling: its name under `destination`.
pub(crate) fn dest_path(input: &NodeInput) -> PathBuf {
    Path::new(destination(&input.params)).join(&input.filename)
}

/// The `conflict` mode, `skip` when unset.
pub(crate) fn conflict_mode(params: &serde_json::Map<String, serde_json::Value>) -> &str {
    params
        .get("conflict")
        .and_then(|v| v.as_str())
        .unwrap_or("skip")
}

// --- Parameter Definitions ---

/// The `conflict` param's description, kept out of `build_copy_params`.
const CONFLICT_DESCRIPTION: &str = "What to do when a file with the same name already \
    exists. \"newer\" copies only when the destination is missing or older than the source \
    (path input only), keeps the source's modified time, and never deletes. A copy that would \
    replace a file the run still reads (another incoming file's, another loop iteration's or \
    another branch's) is refused before the step copies anything.";

#[expect(clippy::too_many_lines)]
fn build_copy_params() -> Vec<ParameterDef> {
    vec![
        ParameterDef {
            name: "destination".to_string(),
            label: "Destination".to_string(),
            description: "Folder to copy files into.".to_string(),
            param_type: ParameterType::String,
            placeholder: Some("./output".to_string()),
            ..Default::default()
        },
        ParameterDef {
            name: "createDirs".to_string(),
            label: "Create Folders".to_string(),
            description: "Automatically create the destination folder if it doesn't exist."
                .to_string(),
            param_type: ParameterType::Boolean,
            default: Some(serde_json::json!(true)),
            ..Default::default()
        },
        ParameterDef {
            name: "conflict".to_string(),
            label: "Conflict Resolution".to_string(),
            description: CONFLICT_DESCRIPTION.to_string(),
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
                    OptionEntry {
                        value: "newer".to_string(),
                        label: "Newer only (keep time)".to_string(),
                    },
                ],
            },
            default: Some(serde_json::json!("skip")),
            ..Default::default()
        },
        ParameterDef {
            name: "toleranceMs".to_string(),
            label: "Time Tolerance".to_string(),
            description: "Modified-time gap treated as equal in \"newer\" mode; FAT and Google \
                          Drive round times."
                .to_string(),
            param_type: ParameterType::Number,
            default: Some(serde_json::json!(DEFAULT_TOLERANCE_MS)),
            constraints: Some(Constraints {
                min: Some(0.0),
                max: None,
                required: false,
            }),
            suffix: Some("ms".to_string()),
            visible_when: Some(ParamCondition::Single(ParamConditionEntry {
                param: "conflict".to_string(),
                equals: "newer".to_string(),
            })),
            ..Default::default()
        },
    ]
}

// --- Conflict Resolution ---

/// Resolve filename conflicts in the destination directory.
///
/// Returns `Some(path)` to write to, or `None` if the file should be skipped.
#[expect(clippy::too_many_lines)]
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
            data: FileData::Bytes(data.to_vec()),
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

    // --- A source that is gone (ISS-83) ---

    #[test]
    fn a_source_that_is_gone_is_named_not_the_destination() {
        let tmp = TempDir::new().unwrap();
        let dest = tmp.path().join("out");
        fs::create_dir(&dest).unwrap();
        let gone = tmp.path().join("a.txt");
        let dest_param = serde_json::json!(dest.to_str().unwrap());
        let mut input = make_input(b"", "a.txt", params(&[("destination", dest_param)]));
        input.data = FileData::Path(gone.clone());

        let progress = ProgressReporter::new_noop();
        let Err(err) = FileCopy::new().process(input, &progress, &NoopContext) else {
            panic!("copying a missing source succeeded");
        };
        let err = err.to_string();

        assert!(
            err.contains(&format!("{} doesn't exist", gone.display())),
            "{err}"
        );
        assert!(!err.contains("Failed to write"), "{err}");
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
                ("createDirs", serde_json::json!(true)),
            ]),
        );

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files.len(), 1);
        assert!(dest.join("file.txt").exists());
    }

    #[test]
    fn test_copy_fails_without_create_dirs() {
        let base = TempDir::new().unwrap();
        let dest = base.path().join("missing");
        let processor = FileCopy::new();
        let progress = ProgressReporter::new_noop();

        let input = make_input(
            b"data",
            "file.txt",
            params(&[
                ("destination", serde_json::json!(dest.to_str().unwrap())),
                ("createDirs", serde_json::json!(false)),
            ]),
        );

        let result = processor.process(input, &progress, &NoopContext);
        assert!(
            result.is_err(),
            "Should fail when dir missing and createDirs=false"
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
            output.files[0].data.clone().into_bytes().unwrap(),
            data,
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

    // --- conflict: "newer" ---

    use filetime::{FileTime, set_file_mtime};
    use std::path::{Path, PathBuf};

    fn ft(ms: i64) -> FileTime {
        FileTime::from_unix_time(ms / 1000, ((ms % 1000) * 1_000_000) as u32)
    }

    fn mtime_ms(p: &Path) -> i64 {
        let t = FileTime::from_last_modification_time(&fs::metadata(p).unwrap());
        t.unix_seconds() * 1000 + (t.nanoseconds() / 1_000_000) as i64
    }

    /// Source with `src_ms` mtime, plus a destination dir (file lands at `dest/name`).
    fn newer_fixture(src_ms: i64) -> (TempDir, PathBuf, PathBuf) {
        let root = TempDir::new().unwrap();
        let src = root.path().join("src.txt");
        fs::write(&src, b"new bytes").unwrap();
        set_file_mtime(&src, ft(src_ms)).unwrap();
        let dest = root.path().join("dest");
        (root, src, dest)
    }

    fn run_newer(src: &Path, dest: &Path) -> Result<NodeOutput, BntoError> {
        let input = NodeInput {
            data: FileData::Path(src.to_path_buf()),
            filename: "out.txt".to_string(),
            mime_type: None,
            params: params(&[
                ("destination", serde_json::json!(dest.to_str().unwrap())),
                ("conflict", serde_json::json!("newer")),
            ]),
        };
        FileCopy::new().process(input, &ProgressReporter::new_noop(), &NoopContext)
    }

    fn flags(out: &NodeOutput) -> (i64, i64, i64) {
        let m = &out.files[0].metadata;
        let g = |k: &str| m.get(k).and_then(|v| v.as_i64()).unwrap();
        (g("copied"), g("up_to_date"), g("skipped_newer"))
    }

    fn put_target(dest: &Path, ms: i64) -> PathBuf {
        fs::create_dir_all(dest).unwrap();
        let t = dest.join("out.txt");
        fs::write(&t, b"old bytes").unwrap();
        set_file_mtime(&t, ft(ms)).unwrap();
        t
    }

    #[test]
    fn test_newer_reports_its_outcome_as_node_level_counts() {
        let (_root, src, dest) = newer_fixture(10_000);
        put_target(&dest, 12_001);
        let out = run_newer(&src, &dest).unwrap();
        assert_eq!(out.metadata, out.files[0].metadata);
        assert_eq!(flags(&out), (0, 0, 1));
    }

    #[test]
    fn test_copy_declares_its_outcome_flags_as_count_keys() {
        let keys = FileCopy::new().count_keys();
        assert_eq!(keys, ["copied", "up_to_date", "skipped_newer"]);
    }

    #[test]
    fn test_newer_missing_target_is_copied_and_folder_made() {
        let (_root, src, dest) = newer_fixture(10_000);
        assert!(!dest.exists());
        let out = run_newer(&src, &dest).unwrap();
        assert_eq!(fs::read(dest.join("out.txt")).unwrap(), b"new bytes");
        assert_eq!(flags(&out), (1, 0, 0));
    }

    #[test]
    fn test_newer_source_overwrites_and_target_takes_source_mtime() {
        let (_root, src, dest) = newer_fixture(100_000);
        let t = put_target(&dest, 10_000);
        let out = run_newer(&src, &dest).unwrap();
        assert_eq!(fs::read(&t).unwrap(), b"new bytes");
        assert_eq!(mtime_ms(&t) / 1000, 100);
        assert_eq!(flags(&out), (1, 0, 0));
    }

    #[test]
    fn test_newer_tolerance_boundary() {
        // Target older than the source by 2001 ms copies; 2000 and 1999 do not.
        for (older_by, copies) in [(2001, true), (2000, false), (1999, false), (0, false)] {
            let (_root, src, dest) = newer_fixture(10_000);
            let t = put_target(&dest, 10_000 - older_by);
            let out = run_newer(&src, &dest).unwrap();
            let want: &[u8] = if copies { b"new bytes" } else { b"old bytes" };
            assert_eq!(fs::read(&t).unwrap(), want, "older_by={older_by}");
            let expect = if copies { (1, 0, 0) } else { (0, 1, 0) };
            assert_eq!(flags(&out), expect, "older_by={older_by}");
        }
    }

    #[test]
    fn test_newer_target_newer_than_source_is_kept() {
        let (_root, src, dest) = newer_fixture(10_000);
        let t = put_target(&dest, 12_001);
        let out = run_newer(&src, &dest).unwrap();
        assert_eq!(fs::read(&t).unwrap(), b"old bytes");
        assert_eq!(flags(&out), (0, 0, 1));
        // Within tolerance the other way is up_to_date, not skipped_newer.
        set_file_mtime(&t, ft(12_000)).unwrap();
        assert_eq!(flags(&run_newer(&src, &dest).unwrap()), (0, 1, 0));
    }

    #[test]
    fn test_newer_rejects_a_negative_or_fractional_tolerance() {
        for bad in [serde_json::json!(-5000), serde_json::json!(1.5)] {
            let (_root, src, dest) = newer_fixture(10_000);
            let t = put_target(&dest, 13_000);
            let input = NodeInput {
                data: FileData::Path(src),
                filename: "out.txt".to_string(),
                mime_type: None,
                params: params(&[
                    ("destination", serde_json::json!(dest.to_str().unwrap())),
                    ("conflict", serde_json::json!("newer")),
                    ("toleranceMs", bad.clone()),
                ]),
            };
            let result =
                FileCopy::new().process(input, &ProgressReporter::new_noop(), &NoopContext);
            assert!(
                matches!(result, Err(BntoError::InvalidInput(_))),
                "tolerance {bad}"
            );
            assert_eq!(fs::read(&t).unwrap(), b"old bytes", "tolerance {bad}");
        }
    }

    #[test]
    fn test_newer_tolerance_param_is_honoured() {
        // 500 and 500.0 (what the TUI and `{{fields.x}}` defaults emit) behave alike.
        for tolerance in [serde_json::json!(500), serde_json::json!(500.0)] {
            let (_root, src, dest) = newer_fixture(10_000);
            let t = put_target(&dest, 9_000);
            let input = NodeInput {
                data: FileData::Path(src),
                filename: "out.txt".to_string(),
                mime_type: None,
                params: params(&[
                    ("destination", serde_json::json!(dest.to_str().unwrap())),
                    ("conflict", serde_json::json!("newer")),
                    ("toleranceMs", tolerance.clone()),
                ]),
            };
            let out = FileCopy::new()
                .process(input, &ProgressReporter::new_noop(), &NoopContext)
                .unwrap();
            assert_eq!(fs::read(&t).unwrap(), b"new bytes", "tolerance {tolerance}");
            assert_eq!(flags(&out), (1, 0, 0), "tolerance {tolerance}");
        }
    }

    #[test]
    fn test_newer_float_tolerance_of_2000_is_accepted_and_acts_as_2000() {
        // Older by 2000 ms is inside a 2000 ms tolerance, so it is left alone.
        let (_root, src, dest) = newer_fixture(10_000);
        let t = put_target(&dest, 8_000);
        let input = NodeInput {
            data: FileData::Path(src),
            filename: "out.txt".to_string(),
            mime_type: None,
            params: params(&[
                ("destination", serde_json::json!(dest.to_str().unwrap())),
                ("conflict", serde_json::json!("newer")),
                ("toleranceMs", serde_json::json!(2000.0)),
            ]),
        };
        let out = FileCopy::new()
            .process(input, &ProgressReporter::new_noop(), &NoopContext)
            .unwrap();
        assert_eq!(fs::read(&t).unwrap(), b"old bytes");
        assert_eq!(flags(&out), (0, 1, 0));
    }

    #[test]
    fn test_newer_directory_at_destination_is_invalid_input() {
        let (_root, src, dest) = newer_fixture(10_000);
        fs::create_dir_all(dest.join("out.txt")).unwrap();
        let err = run_newer(&src, &dest).err().expect("a directory must fail");
        let BntoError::InvalidInput(msg) = err else {
            panic!("expected InvalidInput, got {err:?}");
        };
        assert!(msg.contains("is a directory, not a file"), "{msg}");
    }

    #[test]
    fn test_newer_tolerance_is_only_visible_for_newer() {
        let meta = FileCopy::new().metadata();
        let param = meta
            .parameters
            .iter()
            .find(|p| p.name == "toleranceMs")
            .unwrap();
        assert_eq!(
            param.visible_when,
            Some(ParamCondition::Single(ParamConditionEntry {
                param: "conflict".to_string(),
                equals: "newer".to_string(),
            }))
        );
    }

    #[test]
    fn test_newer_replaces_read_only_target() {
        let (_root, src, dest) = newer_fixture(100_000);
        let t = put_target(&dest, 10_000);
        let mut perms = fs::metadata(&t).unwrap().permissions();
        perms.set_readonly(true);
        fs::set_permissions(&t, perms).unwrap();
        run_newer(&src, &dest).unwrap();
        assert_eq!(fs::read(&t).unwrap(), b"new bytes");
    }

    #[cfg(unix)]
    #[test]
    fn test_newer_adds_owner_write_only_when_a_group_may_write() {
        use std::os::unix::fs::PermissionsExt;
        let (_root, src, dest) = newer_fixture(100_000);
        let t = put_target(&dest, 10_000);
        fs::set_permissions(&t, fs::Permissions::from_mode(0o464)).unwrap();
        run_newer(&src, &dest).unwrap();
        assert_eq!(fs::read(&t).unwrap(), b"new bytes");
        assert_eq!(
            fs::metadata(&t).unwrap().permissions().mode() & 0o777,
            0o664
        );
    }

    #[test]
    fn test_newer_source_error_names_the_source() {
        let (root, _src, dest) = newer_fixture(10_000);
        let missing = root.path().join("gone.txt");
        let Err(err) = run_newer(&missing, &dest) else {
            panic!("a missing source should fail");
        };
        let err = err.to_string();
        assert!(
            err.contains("gone.txt"),
            "error should name the source: {err}"
        );
    }

    #[test]
    fn test_newer_overwrites_in_place_through_a_hard_link() {
        let (_root, src, dest) = newer_fixture(100_000);
        let t = put_target(&dest, 10_000);
        let link = dest.join("link.txt");
        fs::hard_link(&t, &link).unwrap();
        run_newer(&src, &dest).unwrap();
        assert_eq!(fs::read(&link).unwrap(), b"new bytes");
    }

    #[test]
    fn test_overwrite_into_the_source_folder_leaves_the_file_as_is() {
        // ISS-36: the destination is the source itself, so a copy would
        // truncate it to 0 bytes.
        let dir = TempDir::new().unwrap();
        let src = dir.path().join("a.txt");
        fs::write(&src, b"important").unwrap();
        let input = NodeInput {
            data: FileData::Path(src.clone()),
            filename: "a.txt".to_string(),
            mime_type: None,
            params: params(&[
                (
                    "destination",
                    serde_json::json!(dir.path().to_str().unwrap()),
                ),
                ("conflict", serde_json::json!("overwrite")),
            ]),
        };
        FileCopy::new()
            .process(input, &ProgressReporter::new_noop(), &NoopContext)
            .unwrap();
        assert_eq!(fs::read(&src).unwrap(), b"important");
    }

    #[test]
    fn test_newer_into_the_source_folder_is_up_to_date_and_unchanged() {
        // Guard: the same file has the same time, so `newer` never copies.
        let (_root, src, _dest) = newer_fixture(100_000);
        let dir = src.parent().unwrap().to_path_buf();
        let input = NodeInput {
            data: FileData::Path(src.clone()),
            filename: "src.txt".to_string(),
            mime_type: None,
            params: params(&[
                ("destination", serde_json::json!(dir.to_str().unwrap())),
                ("conflict", serde_json::json!("newer")),
            ]),
        };
        let out = FileCopy::new()
            .process(input, &ProgressReporter::new_noop(), &NoopContext)
            .unwrap();
        assert_eq!(flags(&out), (0, 1, 0));
        assert_eq!(fs::read(&src).unwrap(), b"new bytes");
    }

    #[test]
    fn test_newer_never_deletes_destination_only_files() {
        let (_root, src, dest) = newer_fixture(100_000);
        put_target(&dest, 10_000);
        let extra = dest.join("only-here.txt");
        fs::write(&extra, b"keep").unwrap();
        run_newer(&src, &dest).unwrap();
        assert_eq!(fs::read(&extra).unwrap(), b"keep");
    }

    #[test]
    fn test_newer_with_bytes_data_is_invalid_input() {
        let dest = TempDir::new().unwrap();
        let input = make_input(
            b"data",
            "out.txt",
            params(&[
                (
                    "destination",
                    serde_json::json!(dest.path().to_str().unwrap()),
                ),
                ("conflict", serde_json::json!("newer")),
            ]),
        );
        let err = FileCopy::new()
            .process(input, &ProgressReporter::new_noop(), &NoopContext)
            .err()
            .expect("bytes + newer must fail");
        assert!(matches!(err, BntoError::InvalidInput(_)), "{err:?}");
    }

    #[test]
    fn test_other_modes_emit_no_newer_metadata() {
        let dest = TempDir::new().unwrap();
        let input = make_input(
            b"x",
            "a.txt",
            params(&[(
                "destination",
                serde_json::json!(dest.path().to_str().unwrap()),
            )]),
        );
        let out = FileCopy::new()
            .process(input, &ProgressReporter::new_noop(), &NoopContext)
            .unwrap();
        assert!(out.files[0].metadata.is_empty());
    }

    // --- dry run: same decision, nothing written ---

    struct DryRun;

    impl ProcessContext for DryRun {
        fn run_command(&self, cmd: &str, args: &[&str]) -> Result<Vec<u8>, BntoError> {
            NoopContext.run_command(cmd, args)
        }
        fn temp_file(&self, suffix: &str) -> Result<PathBuf, BntoError> {
            NoopContext.temp_file(suffix)
        }
        fn env_var(&self, key: &str) -> Option<String> {
            NoopContext.env_var(key)
        }
        fn work_dir(&self) -> Result<&Path, BntoError> {
            NoopContext.work_dir()
        }
        fn dry_run(&self) -> bool {
            true
        }
    }

    /// Where the destination stands before the copy.
    #[derive(Clone, Copy)]
    enum Dest {
        NoDir,
        EmptyDir,
        /// A target `out.txt` with this mtime in ms.
        Target(i64),
    }

    /// Every entry of a destination dir as (name, bytes, mtime); `None` if the dir is missing.
    type Snapshot = Option<Vec<(String, Vec<u8>, i64)>>;

    fn snapshot(dir: &Path) -> Snapshot {
        let entries = fs::read_dir(dir).ok()?;
        let mut all: Vec<_> = entries
            .map(|e| {
                let p = e.unwrap().path();
                let name = p.file_name().unwrap().to_str().unwrap().to_string();
                (name, fs::read(&p).unwrap(), mtime_ms(&p))
            })
            .collect();
        all.sort();
        Some(all)
    }

    /// Run one twin (a fresh fixture) and return the output with the destination before and after.
    fn run_twin(
        mode: &str,
        dest: Dest,
        ctx: &dyn ProcessContext,
    ) -> (NodeOutput, Snapshot, Snapshot) {
        let (_root, src, dest_dir) = newer_fixture(100_000);
        match dest {
            Dest::NoDir => {}
            Dest::EmptyDir => fs::create_dir_all(&dest_dir).unwrap(),
            Dest::Target(ms) => {
                put_target(&dest_dir, ms);
            }
        }
        let before = snapshot(&dest_dir);
        let input = NodeInput {
            data: FileData::Path(src),
            filename: "out.txt".to_string(),
            mime_type: None,
            params: params(&[
                ("destination", serde_json::json!(dest_dir.to_str().unwrap())),
                ("conflict", serde_json::json!(mode)),
            ]),
        };
        let out = FileCopy::new()
            .process(input, &ProgressReporter::new_noop(), ctx)
            .unwrap();
        (out, before, snapshot(&dest_dir))
    }

    #[test]
    fn test_dry_run_decides_like_a_real_run_and_writes_nothing() {
        let cases = [
            ("skip", Dest::NoDir),
            ("skip", Dest::Target(10_000)),
            ("overwrite", Dest::NoDir),
            ("overwrite", Dest::Target(10_000)),
            ("rename", Dest::EmptyDir),
            ("rename", Dest::Target(10_000)),
            ("newer", Dest::NoDir),
            ("newer", Dest::Target(10_000)),
            ("newer", Dest::Target(100_000)),
            ("newer", Dest::Target(500_000)),
        ];
        for (mode, dest) in cases {
            let (real, _, _) = run_twin(mode, dest, &NoopContext);
            let (dry, before, after) = run_twin(mode, dest, &DryRun);
            assert_eq!(before, after, "{mode}: dry run changed the destination");
            assert_eq!(dry.files[0].filename, real.files[0].filename, "{mode}");
            assert_eq!(dry.files[0].metadata, real.files[0].metadata, "{mode}");
        }
    }

    #[test]
    fn test_dry_run_leaves_a_missing_destination_dir_missing() {
        let (_, _, after) = run_twin("skip", Dest::NoDir, &DryRun);
        assert!(after.is_none());
    }

    #[test]
    fn test_dry_run_still_errors_on_a_missing_dir_without_create_dirs() {
        let root = TempDir::new().unwrap();
        let missing = root.path().join("missing");
        let input = make_input(
            b"x",
            "a.txt",
            params(&[
                ("destination", serde_json::json!(missing.to_str().unwrap())),
                ("createDirs", serde_json::json!(false)),
            ]),
        );
        let err = FileCopy::new()
            .process(input, &ProgressReporter::new_noop(), &DryRun)
            .err()
            .expect("missing dir must fail");
        assert!(matches!(err, BntoError::InvalidInput(_)), "{err:?}");
    }
}
