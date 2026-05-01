// CSV Row Reader — explode a CSV into one OutputFile per row.
//
// Each row becomes an OutputFile with empty `data` and column values
// in `metadata`. Designed for loop containers: iterate over rows and
// access per-row columns via `{{item.column_name}}` templates.

use bnto_core::context::ProcessContext;
use bnto_core::errors::BntoError;
use bnto_core::processor::{NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;

const DEFAULT_MAX_ROWS: u64 = 100_000;

/// The spreadsheet-read node processor. Stateless — config comes from `NodeInput.params`.
pub struct ReadSpreadsheet;

impl ReadSpreadsheet {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ReadSpreadsheet {
    fn default() -> Self {
        Self::new()
    }
}

// --- NodeProcessor Implementation ---

impl NodeProcessor for ReadSpreadsheet {
    fn name(&self) -> &str {
        "spreadsheet-read"
    }

    fn metadata(&self) -> bnto_core::NodeMetadata {
        use bnto_core::metadata::*;
        NodeMetadata {
            node_type: "spreadsheet-read".to_string(),
            name: "Read CSV Rows".to_string(),
            description: "Explode a CSV into one item per row for loop iteration".to_string(),
            category: NodeCategory::Spreadsheet,
            accepts: vec!["text/csv".to_string()],
            platforms: vec!["browser".to_string()],
            parameters: build_read_parameters(),
            input_cardinality: InputCardinality::PerFile,
            requires: vec![],
        }
    }

    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        progress.report(0, "Parsing CSV...");
        let config = ReadConfig::from_params(&input.params);
        let csv_text = parse_and_strip_bom(&input.data)?;

        progress.report(10, "Reading CSV...");
        let delimiter = resolve_delimiter(&config.delimiter);
        let stem = file_stem(&input.filename);

        let (headers, rows) = read_csv(csv_text, delimiter, config.has_headers)?;

        if rows.len() as u64 > config.max_rows {
            return Err(BntoError::InvalidInput(format!(
                "CSV has {} rows, exceeds maxRows limit of {}. \
                 Increase the maxRows parameter to process larger files.",
                rows.len(),
                config.max_rows
            )));
        }

        progress.report(50, &format!("Building {} row files...", rows.len()));
        let files = build_row_files(&stem, &headers, &rows);

        progress.report(90, "Building result...");
        let metadata = build_read_metadata(headers.len(), rows.len());

        progress.report(100, "Done!");
        Ok(NodeOutput { files, metadata })
    }
}

// --- Configuration ---

struct ReadConfig {
    has_headers: bool,
    delimiter: String,
    max_rows: u64,
}

impl ReadConfig {
    fn from_params(params: &serde_json::Map<String, serde_json::Value>) -> Self {
        Self {
            has_headers: params
                .get("hasHeaders")
                .and_then(|v| v.as_bool())
                .unwrap_or(true),
            delimiter: params
                .get("delimiter")
                .and_then(|v| v.as_str())
                .unwrap_or("comma")
                .to_string(),
            max_rows: params
                .get("maxRows")
                .and_then(|v| v.as_u64())
                .unwrap_or(DEFAULT_MAX_ROWS),
        }
    }
}

// --- CSV Parsing (shared patterns from convert.rs) ---

/// Validate UTF-8 and strip BOM if present.
fn parse_and_strip_bom(data: &[u8]) -> Result<&str, BntoError> {
    let data = if data.starts_with(&[0xEF, 0xBB, 0xBF]) {
        &data[3..]
    } else {
        data
    };

    let text = std::str::from_utf8(data).map_err(|e| {
        BntoError::InvalidInput(format!(
            "File is not valid UTF-8 text (is this really a CSV?): {e}"
        ))
    })?;

    if text.trim().is_empty() {
        return Err(BntoError::InvalidInput(
            "CSV file is empty — no data to read".to_string(),
        ));
    }

    Ok(text)
}

/// Map delimiter name to byte.
fn resolve_delimiter(delimiter: &str) -> u8 {
    match delimiter {
        "comma" | "," => b',',
        "semicolon" | ";" => b';',
        "tab" | "\t" => b'\t',
        "pipe" | "|" => b'|',
        other if other.len() == 1 => other.as_bytes()[0],
        _ => b',',
    }
}

/// Read CSV text into headers + rows. When `has_headers` is false,
/// generates synthetic column names (col0, col1, ...).
fn read_csv(
    csv_text: &str,
    delimiter: u8,
    has_headers: bool,
) -> Result<(Vec<String>, Vec<Vec<String>>), BntoError> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(has_headers)
        .delimiter(delimiter)
        .flexible(true)
        .from_reader(csv_text.as_bytes());

    let headers = if has_headers {
        let raw = reader
            .headers()
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to read CSV headers: {e}")))?
            .clone();
        raw.iter().map(|h| h.trim().to_string()).collect()
    } else {
        Vec::new() // filled after first record
    };

    let mut rows = Vec::new();
    let mut headers = headers;

    for result in reader.records() {
        let record = result
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to read CSV row: {e}")))?;

        // Generate synthetic headers from the first record when no headers
        if !has_headers && headers.is_empty() {
            headers = (0..record.len()).map(|i| format!("col{i}")).collect();
        }

        let row: Vec<String> = (0..headers.len())
            .map(|i| record.get(i).unwrap_or("").to_string())
            .collect();
        rows.push(row);
    }

    Ok((headers, rows))
}

// --- Output Construction ---

fn build_row_files(stem: &str, headers: &[String], rows: &[Vec<String>]) -> Vec<OutputFile> {
    rows.iter()
        .enumerate()
        .map(|(i, row)| {
            let mut metadata = serde_json::Map::new();
            for (col_idx, header) in headers.iter().enumerate() {
                let value = row.get(col_idx).cloned().unwrap_or_default();
                metadata.insert(header.clone(), serde_json::Value::String(value));
            }
            OutputFile {
                data: Vec::new(),
                filename: format!("{stem}_row{i}.csv"),
                mime_type: "text/csv".to_string(),
                metadata,
            }
        })
        .collect()
}

fn build_read_metadata(
    column_count: usize,
    row_count: usize,
) -> serde_json::Map<String, serde_json::Value> {
    let mut metadata = serde_json::Map::new();
    metadata.insert(
        "columnCount".to_string(),
        serde_json::Value::Number(column_count.into()),
    );
    metadata.insert(
        "rowCount".to_string(),
        serde_json::Value::Number(row_count.into()),
    );
    metadata
}

fn file_stem(filename: &str) -> String {
    if let Some(dot_pos) = filename.rfind('.') {
        filename[..dot_pos].to_string()
    } else {
        filename.to_string()
    }
}

// --- Metadata Parameter Definitions ---

fn build_read_parameters() -> Vec<bnto_core::metadata::ParameterDef> {
    use bnto_core::metadata::{Constraints, OptionEntry, ParameterDef, ParameterType};
    vec![
        ParameterDef {
            name: "hasHeaders".to_string(),
            label: "Has Headers".to_string(),
            description: "First row contains column headers".to_string(),
            param_type: ParameterType::Boolean,
            default: Some(serde_json::json!(true)),
            ..Default::default()
        },
        ParameterDef {
            name: "delimiter".to_string(),
            label: "Delimiter".to_string(),
            description: "Column separator character".to_string(),
            param_type: ParameterType::Enum {
                options: vec![
                    OptionEntry {
                        value: "comma".to_string(),
                        label: "Comma".to_string(),
                    },
                    OptionEntry {
                        value: "semicolon".to_string(),
                        label: "Semicolon".to_string(),
                    },
                    OptionEntry {
                        value: "tab".to_string(),
                        label: "Tab".to_string(),
                    },
                    OptionEntry {
                        value: "pipe".to_string(),
                        label: "Pipe".to_string(),
                    },
                ],
            },
            default: Some(serde_json::json!("comma")),
            ..Default::default()
        },
        ParameterDef {
            name: "maxRows".to_string(),
            label: "Max Rows".to_string(),
            description: "Maximum rows to process. Error if CSV exceeds this limit.".to_string(),
            param_type: ParameterType::Number,
            default: Some(serde_json::json!(100000)),
            constraints: Some(Constraints {
                min: Some(1.0),
                max: Some(10_000_000.0),
                required: false,
            }),
            ..Default::default()
        },
    ]
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::NoopContext;

    // --- Test Helpers ---

    fn make_input(csv_text: &str) -> NodeInput {
        NodeInput {
            data: csv_text.as_bytes().to_vec(),
            filename: "data.csv".to_string(),
            mime_type: Some("text/csv".to_string()),
            params: serde_json::Map::new(),
        }
    }

    fn make_input_with_params(
        csv_text: &str,
        params: serde_json::Map<String, serde_json::Value>,
    ) -> NodeInput {
        NodeInput {
            data: csv_text.as_bytes().to_vec(),
            filename: "data.csv".to_string(),
            mime_type: Some("text/csv".to_string()),
            params,
        }
    }

    fn make_input_bytes(data: Vec<u8>) -> NodeInput {
        NodeInput {
            data,
            filename: "data.csv".to_string(),
            mime_type: Some("text/csv".to_string()),
            params: serde_json::Map::new(),
        }
    }

    fn process_ok(input: NodeInput) -> NodeOutput {
        let processor = ReadSpreadsheet::new();
        let progress = ProgressReporter::new_noop();
        processor
            .process(input, &progress, &NoopContext)
            .expect("should succeed")
    }

    fn process_err(input: NodeInput) -> BntoError {
        let processor = ReadSpreadsheet::new();
        let progress = ProgressReporter::new_noop();
        match processor.process(input, &progress, &NoopContext) {
            Err(e) => e,
            Ok(_) => panic!("expected an error"),
        }
    }

    // --- Trait Basics ---

    #[test]
    fn test_name() {
        assert_eq!(ReadSpreadsheet::new().name(), "spreadsheet-read");
    }

    #[test]
    fn test_metadata_node_type() {
        let meta = ReadSpreadsheet::new().metadata();
        assert_eq!(meta.node_type, "spreadsheet-read");
        assert_eq!(
            meta.category,
            bnto_core::metadata::NodeCategory::Spreadsheet
        );
    }

    #[test]
    fn test_metadata_has_expected_params() {
        let meta = ReadSpreadsheet::new().metadata();
        let names: Vec<&str> = meta.parameters.iter().map(|p| p.name.as_str()).collect();
        assert!(names.contains(&"hasHeaders"));
        assert!(names.contains(&"delimiter"));
        assert!(names.contains(&"maxRows"));
    }

    #[test]
    fn test_metadata_accepts_csv() {
        let meta = ReadSpreadsheet::new().metadata();
        assert_eq!(meta.accepts, vec!["text/csv"]);
    }

    #[test]
    fn test_metadata_browser_platform() {
        let meta = ReadSpreadsheet::new().metadata();
        assert_eq!(meta.platforms, vec!["browser"]);
    }

    // --- Happy Path ---

    #[test]
    fn test_simple_csv_produces_per_row_files() {
        let csv = "group,url\nAlpha,https://a.com\nBeta,https://b.com\nGamma,https://c.com\n";
        let output = process_ok(make_input(csv));
        assert_eq!(output.files.len(), 3, "3 rows should produce 3 files");
    }

    #[test]
    fn test_row_metadata_has_column_values() {
        let csv = "group,url,title\nAlpha,https://a.com,First\n";
        let output = process_ok(make_input(csv));

        let meta = &output.files[0].metadata;
        assert_eq!(meta.get("group").unwrap(), "Alpha");
        assert_eq!(meta.get("url").unwrap(), "https://a.com");
        assert_eq!(meta.get("title").unwrap(), "First");
    }

    #[test]
    fn test_output_data_is_empty() {
        let csv = "name\nAlice\nBob\n";
        let output = process_ok(make_input(csv));

        for file in &output.files {
            assert!(file.data.is_empty(), "row files should have empty data");
        }
    }

    #[test]
    fn test_output_filename_includes_row_index() {
        let csv = "name\nAlice\nBob\n";
        let output = process_ok(make_input(csv));

        assert_eq!(output.files[0].filename, "data_row0.csv");
        assert_eq!(output.files[1].filename, "data_row1.csv");
    }

    #[test]
    fn test_output_mime_type() {
        let csv = "name\nAlice\n";
        let output = process_ok(make_input(csv));
        assert_eq!(output.files[0].mime_type, "text/csv");
    }

    #[test]
    fn test_headers_only_produces_no_files() {
        let csv = "name,age,city\n";
        let output = process_ok(make_input(csv));
        assert_eq!(
            output.files.len(),
            0,
            "headers-only CSV should produce 0 files"
        );
    }

    #[test]
    fn test_single_row() {
        let csv = "x\n42\n";
        let output = process_ok(make_input(csv));

        assert_eq!(output.files.len(), 1);
        assert_eq!(output.files[0].metadata.get("x").unwrap(), "42");
    }

    // --- Empty Cells ---

    #[test]
    fn test_empty_cells_produce_empty_strings() {
        let csv = "name,age,city\nAlice,,NYC\n";
        let output = process_ok(make_input(csv));

        assert_eq!(output.files[0].metadata.get("age").unwrap(), "");
    }

    #[test]
    fn test_short_row_gets_empty_strings() {
        let csv = "name,age,city\nAlice\n";
        let output = process_ok(make_input(csv));

        assert_eq!(output.files[0].metadata.get("name").unwrap(), "Alice");
        assert_eq!(output.files[0].metadata.get("age").unwrap(), "");
        assert_eq!(output.files[0].metadata.get("city").unwrap(), "");
    }

    // --- Delimiter Configuration ---

    #[test]
    fn test_semicolon_delimiter() {
        let csv = "name;age\nAlice;30\n";
        let mut params = serde_json::Map::new();
        params.insert("delimiter".into(), serde_json::json!("semicolon"));
        let output = process_ok(make_input_with_params(csv, params));

        assert_eq!(output.files[0].metadata.get("name").unwrap(), "Alice");
        assert_eq!(output.files[0].metadata.get("age").unwrap(), "30");
    }

    #[test]
    fn test_tab_delimiter() {
        let csv = "name\tage\nAlice\t30\n";
        let mut params = serde_json::Map::new();
        params.insert("delimiter".into(), serde_json::json!("tab"));
        let output = process_ok(make_input_with_params(csv, params));

        assert_eq!(output.files[0].metadata.get("name").unwrap(), "Alice");
    }

    // --- BOM Handling ---

    #[test]
    fn test_bom_stripped() {
        let csv_with_bom = b"\xEF\xBB\xBFname,age\nAlice,30\n";
        let output = process_ok(make_input_bytes(csv_with_bom.to_vec()));

        assert_eq!(output.files[0].metadata.get("name").unwrap(), "Alice");
        assert!(output.files[0].metadata.get("\u{FEFF}name").is_none());
    }

    // --- No Headers Mode ---

    #[test]
    fn test_no_headers_uses_synthetic_names() {
        let csv = "Alice,30,NYC\nBob,25,LA\n";
        let mut params = serde_json::Map::new();
        params.insert("hasHeaders".into(), serde_json::json!(false));
        let output = process_ok(make_input_with_params(csv, params));

        assert_eq!(output.files.len(), 2);
        assert_eq!(output.files[0].metadata.get("col0").unwrap(), "Alice");
        assert_eq!(output.files[0].metadata.get("col1").unwrap(), "30");
        assert_eq!(output.files[0].metadata.get("col2").unwrap(), "NYC");
    }

    // --- maxRows Guard ---

    #[test]
    fn test_max_rows_rejects_large_csv() {
        let mut csv = String::from("id\n");
        for i in 0..10 {
            csv.push_str(&format!("{i}\n"));
        }
        let mut params = serde_json::Map::new();
        params.insert("maxRows".into(), serde_json::json!(5));
        let err = process_err(make_input_with_params(&csv, params));

        let msg = err.to_string();
        assert!(
            msg.contains("10 rows"),
            "should mention actual count: {msg}"
        );
        assert!(msg.contains("5"), "should mention limit: {msg}");
    }

    #[test]
    fn test_max_rows_at_limit_passes() {
        let csv = "id\n1\n2\n3\n";
        let mut params = serde_json::Map::new();
        params.insert("maxRows".into(), serde_json::json!(3));
        let output = process_ok(make_input_with_params(csv, params));
        assert_eq!(output.files.len(), 3);
    }

    #[test]
    fn test_default_max_rows() {
        // Verify the default is 100k — just check the config
        let params = serde_json::Map::new();
        let config = ReadConfig::from_params(&params);
        assert_eq!(config.max_rows, 100_000);
    }

    // --- Error Cases ---

    #[test]
    fn test_empty_input() {
        let err = process_err(make_input(""));
        assert!(err.to_string().contains("empty"));
    }

    #[test]
    fn test_whitespace_only_input() {
        let err = process_err(make_input("   \n\n  "));
        assert!(err.to_string().contains("empty"));
    }

    #[test]
    fn test_non_utf8_input() {
        let err = process_err(make_input_bytes(vec![0xFF, 0xFE, 0x00, 0x41]));
        assert!(err.to_string().contains("UTF-8"));
    }

    // --- Output Metadata ---

    #[test]
    fn test_metadata_counts() {
        let csv = "name,age,city\nAlice,30,NYC\nBob,25,LA\n";
        let output = process_ok(make_input(csv));

        assert_eq!(output.metadata["columnCount"], 3);
        assert_eq!(output.metadata["rowCount"], 2);
    }

    // --- Fixture Files ---

    #[test]
    fn test_fixture_manifest() {
        let data = include_bytes!("../../../../test-fixtures/csv/manifest.csv");
        let output = process_ok(make_input_bytes(data.to_vec()));

        assert_eq!(output.files.len(), 3);
        assert_eq!(output.files[0].metadata.get("group").unwrap(), "Alpha");
        assert_eq!(
            output.files[0].metadata.get("url").unwrap(),
            "https://example.com/video1.mp4"
        );
        assert_eq!(output.files[2].metadata.get("group").unwrap(), "Beta");
    }

    #[test]
    fn test_fixture_simple() {
        let data = include_bytes!("../../../../test-fixtures/csv/simple.csv");
        let output = process_ok(make_input_bytes(data.to_vec()));

        assert_eq!(output.files.len(), 5);
        assert_eq!(output.files[0].metadata.get("name").unwrap(), "Alice");
    }

    #[test]
    fn test_fixture_headers_only() {
        let data = include_bytes!("../../../../test-fixtures/csv/headers-only.csv");
        let output = process_ok(make_input_bytes(data.to_vec()));
        assert_eq!(output.files.len(), 0);
    }

    // --- Large CSV ---

    #[test]
    fn test_large_csv_1000_rows() {
        let mut csv = String::from("id,name,value\n");
        for i in 0..1000 {
            csv.push_str(&format!("{i},item_{i},{}\n", i * 10));
        }
        let output = process_ok(make_input(&csv));

        assert_eq!(output.files.len(), 1000);
        assert_eq!(output.metadata["rowCount"], 1000);
        assert_eq!(output.metadata["columnCount"], 3);
        // Verify first and last rows
        assert_eq!(output.files[0].metadata.get("id").unwrap(), "0");
        assert_eq!(output.files[999].metadata.get("id").unwrap(), "999");
    }
}
