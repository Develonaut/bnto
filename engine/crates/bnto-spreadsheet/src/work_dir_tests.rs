// `spreadsheet-export`'s `path` and `spreadsheet-lookup`'s `table` resolve a
// relative value against the run's working folder, never the process's own.
// Each test runs in a context whose working folder is a temp dir, which the
// test process's folder (the crate) never is.

use std::path::{Path, PathBuf};

use bnto_core::processor::{BatchFile, BatchInput, FileData, NodeInput, NodeProcessor};
use bnto_core::progress::ProgressReporter;
use bnto_core::{BntoError, NoopContext, ProcessContext};
use serde_json::json;

use crate::{SpreadsheetExport, SpreadsheetLookup};

/// A context whose working folder is `0`.
struct InFolder(PathBuf);

impl ProcessContext for InFolder {
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
        Ok(&self.0)
    }
}

#[test]
fn test_export_writes_a_relative_path_under_the_work_dir() {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir(tmp.path().join("reports")).unwrap();
    let file = BatchFile {
        data: FileData::Bytes(b"x".to_vec()),
        filename: "a.wav".to_string(),
        mime_type: None,
        metadata: serde_json::Map::new(),
    };
    let params = json!({ "path": "reports/run.csv" });
    let input = BatchInput {
        files: vec![file],
        params: params.as_object().unwrap().clone(),
    };
    let ctx = InFolder(tmp.path().to_path_buf());
    let progress = ProgressReporter::new_noop();
    let written = SpreadsheetExport::new().process_batch(input, &progress, &ctx);
    assert!(written.is_ok(), "{:?}", written.err());
    let csv = std::fs::read_to_string(tmp.path().join("reports/run.csv")).unwrap();
    assert!(csv.contains("a.wav"), "{csv}");
}

#[test]
fn test_lookup_reads_a_relative_table_under_the_work_dir() {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir(tmp.path().join("data")).unwrap();
    std::fs::write(tmp.path().join("data/map.csv"), "name,tag\na.wav,kick\n").unwrap();
    let input = NodeInput {
        data: FileData::Bytes(b"x".to_vec()),
        filename: "a.wav".to_string(),
        mime_type: None,
        params: json!({ "table": "data/map.csv" })
            .as_object()
            .unwrap()
            .clone(),
    };
    let ctx = InFolder(tmp.path().to_path_buf());
    let progress = ProgressReporter::new_noop();
    let out = SpreadsheetLookup::new().process(input, &progress, &ctx);
    let out = out.map_err(|e| e.to_string()).unwrap();
    assert_eq!(out.files[0].metadata["tag"], "kick");
}
