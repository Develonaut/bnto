// SpreadsheetLookup — NodeProcessor wiring for `spreadsheet-lookup`.

use bnto_core::context::ProcessContext;
use bnto_core::errors::BntoError;
use bnto_core::processor::{NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;

use super::params::{LookupParams, MatchOn, Unmatched, build_lookup_parameters};
use super::source::load_table;
use super::table::Row;

/// Joins each file to the table row whose key equals its name, attaching
/// that row's columns as the file's metadata. File data passes through.
pub struct SpreadsheetLookup;

impl SpreadsheetLookup {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SpreadsheetLookup {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeProcessor for SpreadsheetLookup {
    fn name(&self) -> &str {
        "spreadsheet-lookup"
    }

    fn metadata(&self) -> bnto_core::NodeMetadata {
        use bnto_core::metadata::*;
        NodeMetadata {
            node_type: "spreadsheet-lookup".to_string(),
            name: "Look Up Rows".to_string(),
            description: "Attach the CSV row whose key matches each file's name to that file"
                .to_string(),
            category: NodeCategory::Spreadsheet,
            accepts: vec![],
            platforms: vec!["cli".to_string(), "desktop".to_string()],
            parameters: build_lookup_parameters(),
            input_cardinality: InputCardinality::PerFile,
            requires: vec![],
        }
    }

    fn validate(&self, params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
        LookupParams::read(params).err().unwrap_or_default()
    }

    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let params = LookupParams::read(&input.params)
            .map_err(|errors| BntoError::InvalidInput(errors.join("; ")))?;
        let table = load_table(&params.table, &params.key_column)?;

        progress.report(50, "Looking up row...");
        let output = match table.get(lookup_key(&input.filename, params.match_on)) {
            Some(row) => pass_through(input, row.clone()),
            None => no_row_output(input, &params)?,
        };
        progress.report(100, "Done");
        Ok(output)
    }
}

/// What a file with no row becomes, per the `unmatched` param.
fn no_row_output(input: NodeInput, params: &LookupParams) -> Result<NodeOutput, BntoError> {
    match params.unmatched {
        Unmatched::Keep => Ok(pass_through(input, unmatched_flag())),
        Unmatched::Drop => Ok(empty_output()),
        Unmatched::Fail => Err(no_row_error(&input.filename, params)),
    }
}

/// The part of `filename` compared with the key column.
fn lookup_key(filename: &str, match_on: MatchOn) -> &str {
    match match_on {
        MatchOn::Path => filename,
        MatchOn::Filename => filename.rsplit('/').next().unwrap_or(filename),
    }
}

fn unmatched_flag() -> Row {
    let mut flag = Row::new();
    flag.insert("unmatched".to_string(), 1.into());
    flag
}

fn no_row_error(filename: &str, params: &LookupParams) -> BntoError {
    BntoError::InvalidInput(format!(
        "{filename} has no row in {} (key column '{}')",
        params.table, params.key_column
    ))
}

fn empty_output() -> NodeOutput {
    NodeOutput {
        files: Vec::new(),
        metadata: serde_json::Map::new(),
    }
}

fn pass_through(input: NodeInput, metadata: Row) -> NodeOutput {
    NodeOutput {
        files: vec![OutputFile {
            data: input.data,
            filename: input.filename,
            mime_type: input
                .mime_type
                .unwrap_or_else(|| "application/octet-stream".to_string()),
            metadata,
        }],
        metadata: serde_json::Map::new(),
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use bnto_core::NoopContext;
    use bnto_core::processor::FileData;
    use serde_json::json;
    use tempfile::TempDir;

    use super::*;

    fn table_file(text: &str) -> (TempDir, PathBuf) {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("mapping.csv");
        std::fs::write(&path, text).unwrap();
        (tmp, path)
    }

    const MAPPING: &str = "name,new_name,pack\n\
                           Pack/sub/Glock 45 Mag Out 01.wav,GUNMech_Glock 01.wav,Guns\n";

    fn lookup(filename: &str, params: serde_json::Value) -> Result<NodeOutput, BntoError> {
        let input = NodeInput {
            data: FileData::Path(PathBuf::from("lib").join(filename)),
            filename: filename.to_string(),
            mime_type: Some("audio/wav".to_string()),
            params: params.as_object().unwrap().clone(),
        };
        SpreadsheetLookup::new().process(input, &ProgressReporter::new_noop(), &NoopContext)
    }

    fn table_param(path: &Path, extra: serde_json::Value) -> serde_json::Value {
        let mut params = extra.as_object().unwrap().clone();
        params.insert("table".to_string(), json!(path.to_str().unwrap()));
        serde_json::Value::Object(params)
    }

    #[test]
    fn test_name_and_platforms() {
        let p = SpreadsheetLookup::new();
        assert_eq!(p.name(), "spreadsheet-lookup");
        assert_eq!(p.metadata().platforms, vec!["cli", "desktop"]);
    }

    #[test]
    fn test_matched_file_carries_the_rows_columns_and_keeps_its_name() {
        let (_tmp, path) = table_file(MAPPING);
        let out = lookup(
            "Pack/sub/Glock 45 Mag Out 01.wav",
            table_param(&path, json!({})),
        )
        .unwrap();
        let file = &out.files[0];
        assert_eq!(file.filename, "Pack/sub/Glock 45 Mag Out 01.wav");
        assert_eq!(file.metadata["new_name"], "GUNMech_Glock 01.wav");
        assert_eq!(file.metadata["pack"], "Guns");
        assert_eq!(file.metadata["name"], "Pack/sub/Glock 45 Mag Out 01.wav");
        assert_eq!(file.mime_type, "audio/wav");
        assert!(!file.metadata.contains_key("unmatched"));
    }

    #[test]
    fn test_path_data_stays_a_path() {
        let (_tmp, path) = table_file(MAPPING);
        let out = lookup(
            "Pack/sub/Glock 45 Mag Out 01.wav",
            table_param(&path, json!({})),
        )
        .unwrap();
        assert!(matches!(out.files[0].data, FileData::Path(_)));
    }

    #[test]
    fn test_key_column_and_filename_matching_find_the_undo_row() {
        // After a rename the file sits in the same folder under its new leaf name.
        let (_tmp, path) = table_file(MAPPING);
        let params = table_param(
            &path,
            json!({ "key_column": "new_name", "match_on": "filename" }),
        );
        let out = lookup("Pack/sub/GUNMech_Glock 01.wav", params).unwrap();
        assert_eq!(
            out.files[0].metadata["name"],
            "Pack/sub/Glock 45 Mag Out 01.wav"
        );
    }

    #[test]
    fn test_path_matching_needs_the_whole_path() {
        let (_tmp, path) = table_file(MAPPING);
        let out = lookup("Glock 45 Mag Out 01.wav", table_param(&path, json!({}))).unwrap();
        assert_eq!(out.files[0].metadata["unmatched"], 1);
    }

    #[test]
    fn test_unmatched_keep_flags_the_file() {
        let (_tmp, path) = table_file(MAPPING);
        let out = lookup("Pack/other.wav", table_param(&path, json!({}))).unwrap();
        assert_eq!(out.files[0].filename, "Pack/other.wav");
        assert_eq!(out.files[0].metadata.len(), 1);
        assert_eq!(out.files[0].metadata["unmatched"], 1);
    }

    #[test]
    fn test_unmatched_drop_leaves_the_file_out() {
        let (_tmp, path) = table_file(MAPPING);
        let out = lookup(
            "Pack/other.wav",
            table_param(&path, json!({ "unmatched": "drop" })),
        )
        .unwrap();
        assert!(out.files.is_empty());
    }

    #[test]
    fn test_unmatched_fail_names_the_file_and_table() {
        let (_tmp, path) = table_file(MAPPING);
        let err = lookup(
            "Pack/other.wav",
            table_param(&path, json!({ "unmatched": "fail" })),
        )
        .err()
        .unwrap()
        .to_string();
        assert!(err.contains("Pack/other.wav"), "{err}");
        assert!(err.contains("mapping.csv"), "{err}");
    }

    #[test]
    fn test_missing_table_fails_naming_it() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("missing.csv");
        let err = lookup("a.wav", table_param(&path, json!({})))
            .err()
            .unwrap()
            .to_string();
        assert!(err.contains("missing.csv"), "{err}");
    }

    #[test]
    fn test_invalid_params_fail_and_validate_reports_them() {
        let params = json!({ "unmatched": "skip" });
        assert!(lookup("a.wav", params.clone()).is_err());
        let errors = SpreadsheetLookup::new().validate(params.as_object().unwrap());
        assert_eq!(errors.len(), 2, "{errors:?}");
    }

    #[test]
    fn test_validate_passes_with_a_table() {
        let errors =
            SpreadsheetLookup::new().validate(json!({ "table": "t.csv" }).as_object().unwrap());
        assert!(errors.is_empty());
    }
}
