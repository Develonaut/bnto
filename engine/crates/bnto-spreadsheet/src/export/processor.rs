// SpreadsheetExport — NodeProcessor wiring for `spreadsheet-export`.

use std::path::Path;

use bnto_core::context::ProcessContext;
use bnto_core::errors::BntoError;
use bnto_core::processor::{
    BatchFile, BatchInput, NodeInput, NodeOutput, NodeProcessor, OutputFile,
};
use bnto_core::progress::ProgressReporter;

use super::params::{ExportParams, build_export_parameters};
use super::table::{Item, items_csv};

/// Writes the files passing through as a CSV, one row per file (its name,
/// then its metadata), and passes them on unchanged. It never reads file
/// content, and a dry run writes it too: the CSV is a report of the run.
pub struct SpreadsheetExport;

impl SpreadsheetExport {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SpreadsheetExport {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeProcessor for SpreadsheetExport {
    fn name(&self) -> &str {
        "spreadsheet-export"
    }

    fn metadata(&self) -> bnto_core::NodeMetadata {
        use bnto_core::metadata::*;
        NodeMetadata {
            node_type: "spreadsheet-export".to_string(),
            name: "Export Items as CSV".to_string(),
            description:
                "Write the files passing through, with their metadata, as one CSV row each"
                    .to_string(),
            category: NodeCategory::Spreadsheet,
            accepts: vec![],
            platforms: vec!["cli".to_string(), "desktop".to_string()],
            parameters: build_export_parameters(),
            input_cardinality: InputCardinality::Batch,
            requires: vec![],
        }
    }

    fn validate(&self, params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
        ExportParams::read(params).err().into_iter().collect()
    }

    /// Required by the trait; the executor always runs this node as a batch.
    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
        ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let file = BatchFile {
            data: input.data,
            filename: input.filename,
            mime_type: input.mime_type,
            metadata: serde_json::Map::new(),
        };
        let batch = BatchInput {
            files: vec![file],
            params: input.params,
        };
        self.process_batch(batch, progress, ctx)
    }

    fn process_batch(
        &self,
        input: BatchInput,
        progress: &ProgressReporter,
        ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let params = ExportParams::read(&input.params).map_err(BntoError::InvalidInput)?;
        write_csv(&params, &input.files)?;
        // A dry run writes the CSV too (it's a report of the run), so say so.
        let tag = if ctx.dry_run() {
            " (dry run: no file was changed)"
        } else {
            ""
        };
        let rows = input.files.len();
        progress.report_output(&format!("Wrote {}: {rows} rows{tag}", params.path));
        Ok(pass_through(input.files))
    }
}

/// Write the CSV for `files` to `params.path`.
fn write_csv(params: &ExportParams, files: &[BatchFile]) -> Result<(), BntoError> {
    check_target(params)?;
    let items: Vec<Item<'_>> = files
        .iter()
        .map(|f| Item {
            name: &f.filename,
            metadata: &f.metadata,
        })
        .collect();
    let text = items_csv(&items, &params.name_column).map_err(|e| cannot_write(params, &e))?;
    write_replacing(Path::new(&params.path), &text)
        .map_err(|e| BntoError::ProcessingFailed(format!("Cannot write {}: {e}", params.path)))
}

/// The folder must exist, and an existing file is kept unless `overwrite`:
/// it may be an earlier run's record, the data an undo needs.
fn check_target(params: &ExportParams) -> Result<(), BntoError> {
    let path = Path::new(&params.path);
    let folder = path.parent().filter(|p| !p.as_os_str().is_empty());
    if folder.is_some_and(|f| !f.is_dir()) {
        return Err(cannot_write(params, "its folder doesn't exist"));
    }
    if !params.overwrite && std::fs::symlink_metadata(path).is_ok() {
        return Err(cannot_write(
            params,
            "it already exists; set overwrite to replace it",
        ));
    }
    Ok(())
}

fn cannot_write(params: &ExportParams, why: &str) -> BntoError {
    BntoError::InvalidInput(format!("Cannot write {}: {why}", params.path))
}

/// Write through a temporary file in the same folder and rename it into
/// place, so an interrupted write never leaves a half-written CSV.
fn write_replacing(path: &Path, text: &str) -> std::io::Result<()> {
    let leaf = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("export.csv");
    let temp = path.with_file_name(format!(".{leaf}.bnto-tmp"));
    std::fs::write(&temp, text)?;
    std::fs::rename(&temp, path).inspect_err(|_| {
        let _ = std::fs::remove_file(&temp);
    })
}

/// The files, unchanged: same data (a path stays a path), name and metadata.
fn pass_through(files: Vec<BatchFile>) -> NodeOutput {
    let files = files
        .into_iter()
        .map(|f| OutputFile {
            data: f.data,
            filename: f.filename,
            mime_type: f
                .mime_type
                .unwrap_or_else(|| "application/octet-stream".to_string()),
            metadata: f.metadata,
        })
        .collect();
    NodeOutput {
        files,
        metadata: serde_json::Map::new(),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use bnto_core::NoopContext;
    use bnto_core::processor::FileData;
    use serde_json::json;

    use super::*;

    fn audio(name: &str, metadata: serde_json::Value) -> BatchFile {
        BatchFile {
            data: FileData::Path(PathBuf::from("lib").join(name)),
            filename: name.to_string(),
            mime_type: Some("audio/wav".to_string()),
            metadata: metadata.as_object().unwrap().clone(),
        }
    }

    fn export(files: Vec<BatchFile>, params: serde_json::Value) -> Result<NodeOutput, BntoError> {
        let input = BatchInput {
            files,
            params: params.as_object().unwrap().clone(),
        };
        SpreadsheetExport::new().process_batch(input, &ProgressReporter::new_noop(), &NoopContext)
    }

    #[test]
    fn test_name_cardinality_and_platforms() {
        let p = SpreadsheetExport::new();
        assert_eq!(p.name(), "spreadsheet-export");
        let meta = p.metadata();
        assert_eq!(
            meta.input_cardinality,
            bnto_core::metadata::InputCardinality::Batch
        );
        assert_eq!(meta.platforms, vec!["cli", "desktop"]);
    }

    #[test]
    fn test_writes_one_row_per_file_and_passes_files_through_unchanged() {
        let tmp = tempfile::tempdir().unwrap();
        let csv_path = tmp.path().join("record.csv");
        let files = vec![
            audio(
                "Pack/GUNMech_Glock 01.wav",
                json!({ "name": "Pack/Glock 45 Mag Out 01.wav", "pack": "Guns" }),
            ),
            audio("Pack/keep.wav", json!({ "unmatched": 1 })),
        ];
        let out = export(files, json!({ "path": csv_path.to_str().unwrap() })).unwrap();

        let text = std::fs::read_to_string(&csv_path).unwrap();
        assert_eq!(
            text,
            "file,name,pack,unmatched\n\
             Pack/GUNMech_Glock 01.wav,Pack/Glock 45 Mag Out 01.wav,Guns,\n\
             Pack/keep.wav,,,1\n"
        );
        assert_eq!(out.files.len(), 2);
        assert!(
            out.files
                .iter()
                .all(|f| matches!(f.data, FileData::Path(_)))
        );
        assert_eq!(out.files[0].filename, "Pack/GUNMech_Glock 01.wav");
        assert_eq!(out.files[0].metadata["pack"], "Guns");
    }

    #[test]
    fn test_a_missing_folder_fails_naming_the_path() {
        let tmp = tempfile::tempdir().unwrap();
        let csv_path = tmp.path().join("no-such-folder").join("record.csv");
        let err = export(
            vec![audio("a.wav", json!({}))],
            json!({ "path": csv_path.to_str().unwrap() }),
        )
        .err()
        .unwrap()
        .to_string();
        assert!(
            err.contains("record.csv") && err.contains("folder"),
            "{err}"
        );
    }

    #[test]
    fn test_a_missing_path_param_is_refused() {
        assert!(export(vec![], json!({})).is_err());
        let errors = SpreadsheetExport::new().validate(json!({}).as_object().unwrap());
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn test_the_row_count_is_reported_as_output() {
        let tmp = tempfile::tempdir().unwrap();
        let csv_path = tmp.path().join("record.csv");
        let lines = std::sync::Arc::new(std::sync::Mutex::new(Vec::<String>::new()));
        let sink = std::sync::Arc::clone(&lines);
        let progress = ProgressReporter::with_output(
            |_, _| {},
            move |l| sink.lock().unwrap().push(l.to_string()),
        );
        let input = BatchInput {
            files: vec![audio("a.wav", json!({})), audio("b.wav", json!({}))],
            params: json!({ "path": csv_path.to_str().unwrap() })
                .as_object()
                .unwrap()
                .clone(),
        };
        SpreadsheetExport::new()
            .process_batch(input, &progress, &NoopContext)
            .unwrap();
        let lines = lines.lock().unwrap();
        assert!(
            lines[0].contains("record.csv") && lines[0].ends_with("2 rows"),
            "{lines:?}"
        );
    }

    #[test]
    fn test_an_existing_csv_is_kept_unless_overwrite_is_set() {
        let tmp = tempfile::tempdir().unwrap();
        let csv_path = tmp.path().join("record.csv");
        std::fs::write(
            &csv_path,
            "earlier record
",
        )
        .unwrap();
        let path = csv_path.to_str().unwrap();

        let err = export(vec![audio("a.wav", json!({}))], json!({ "path": path }))
            .err()
            .unwrap()
            .to_string();
        assert!(
            err.contains("already exists") && err.contains("overwrite"),
            "{err}"
        );
        assert_eq!(
            std::fs::read_to_string(&csv_path).unwrap(),
            "earlier record
"
        );

        export(
            vec![audio("a.wav", json!({}))],
            json!({ "path": path, "overwrite": true }),
        )
        .unwrap();
        assert_eq!(
            std::fs::read_to_string(&csv_path).unwrap(),
            "file
a.wav
"
        );
        let names: Vec<_> = std::fs::read_dir(tmp.path())
            .unwrap()
            .map(|e| e.unwrap().file_name())
            .collect();
        assert_eq!(names, vec!["record.csv"], "no temp file left behind");
    }

    struct DryRun;

    impl ProcessContext for DryRun {
        fn run_command(&self, _cmd: &str, _args: &[&str]) -> Result<Vec<u8>, BntoError> {
            unreachable!()
        }
        fn temp_file(&self, _suffix: &str) -> Result<PathBuf, BntoError> {
            unreachable!()
        }
        fn env_var(&self, _key: &str) -> Option<String> {
            None
        }
        fn work_dir(&self) -> Result<&std::path::Path, BntoError> {
            unreachable!()
        }
        fn dry_run(&self) -> bool {
            true
        }
    }

    #[test]
    fn test_a_dry_run_writes_the_csv_and_says_so() {
        let tmp = tempfile::tempdir().unwrap();
        let csv_path = tmp.path().join("record.csv");
        let lines = std::sync::Arc::new(std::sync::Mutex::new(Vec::<String>::new()));
        let sink = std::sync::Arc::clone(&lines);
        let progress = ProgressReporter::with_output(
            |_, _| {},
            move |l| sink.lock().unwrap().push(l.to_string()),
        );
        let input = BatchInput {
            files: vec![audio("a.wav", json!({}))],
            params: json!({ "path": csv_path.to_str().unwrap() })
                .as_object()
                .unwrap()
                .clone(),
        };
        SpreadsheetExport::new()
            .process_batch(input, &progress, &DryRun)
            .unwrap();
        assert!(csv_path.exists());
        assert!(
            lines.lock().unwrap()[0].contains("(dry run"),
            "{:?}",
            lines.lock().unwrap()
        );
    }
}
