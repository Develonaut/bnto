// Rename CSV Columns — rename column headers based on a user-provided mapping.
// Data rows are preserved unchanged; only the header row is modified.

use std::collections::HashMap;

use bnto_core::context::ProcessContext;
use bnto_core::errors::BntoError;
use bnto_core::processor::{NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;

/// The rename-csv-columns node processor. Stateless — config comes from `NodeInput.params`.
pub struct RenameCsvColumns;

impl RenameCsvColumns {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RenameCsvColumns {
    fn default() -> Self {
        Self::new()
    }
}

// --- NodeProcessor Implementation ---

impl NodeProcessor for RenameCsvColumns {
    fn name(&self) -> &str {
        "rename-csv-columns"
    }

    /// Self-describing metadata. Parameters: columns (object mapping old->new names).
    fn metadata(&self) -> bnto_core::NodeMetadata {
        use bnto_core::metadata::*;
        NodeMetadata {
            node_type: "spreadsheet-rename".to_string(),
            name: "Rename CSV Columns".to_string(),
            description: "Rename column headers in a CSV file".to_string(),
            category: NodeCategory::Spreadsheet,
            accepts: vec!["text/csv".to_string()],
            platforms: vec!["browser".to_string()],
            parameters: vec![ParameterDef {
                name: "columns".to_string(),
                label: "Column Mapping".to_string(),
                description:
                    "Map of old column names to new names (e.g., {\"Name\": \"full_name\"})"
                        .to_string(),
                param_type: ParameterType::Object,
                ..Default::default()
            }],
            input_cardinality: InputCardinality::PerFile,
            requires: vec![],
        }
    }

    /// Rename column headers based on the `columns` parameter mapping.
    /// Missing or non-matching columns are silently preserved.
    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        progress.report(0, "Starting column rename...");
        let csv_text = parse_utf8(&input.data)?;
        let column_mapping = extract_column_mapping(&input.params);

        progress.report(20, "Parsed parameters...");
        let (headers, mut reader) = read_headers(csv_text)?;

        progress.report(40, "Read headers...");
        let (new_headers, columns_renamed) = apply_column_mapping(&headers, &column_mapping);

        progress.report(60, "Renamed headers...");
        let (output_bytes, row_count) = write_renamed_csv(&new_headers, &mut reader)?;

        progress.report(90, "Wrote output CSV...");
        let metadata = build_rename_metadata(
            columns_renamed,
            &new_headers,
            row_count,
            &column_mapping,
            &headers,
        );

        progress.report(100, "Done!");
        Ok(build_rename_output(output_bytes, &input.filename, metadata))
    }
}

fn build_rename_output(
    data: Vec<u8>,
    input_filename: &str,
    metadata: serde_json::Map<String, serde_json::Value>,
) -> NodeOutput {
    NodeOutput {
        files: vec![OutputFile {
            data,
            filename: build_output_filename(input_filename),
            mime_type: "text/csv".to_string(),
        }],
        metadata,
    }
}

// --- CSV Parsing ---

/// Validate and convert raw bytes to a UTF-8 string.
fn parse_utf8(data: &[u8]) -> Result<&str, BntoError> {
    std::str::from_utf8(data)
        .map_err(|e| BntoError::InvalidInput(format!("CSV is not valid UTF-8: {e}")))
}

/// Parse CSV text and return the header record plus a positioned reader.
fn read_headers(csv_text: &str) -> Result<(csv::StringRecord, csv::Reader<&[u8]>), BntoError> {
    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .from_reader(csv_text.as_bytes());

    let headers = reader
        .headers()
        .map_err(|e| BntoError::ProcessingFailed(format!("Failed to read CSV headers: {e}")))?
        .clone();

    Ok((headers, reader))
}

// --- Column Mapping ---

/// Apply the rename mapping to headers. Returns (new_headers, count_renamed).
fn apply_column_mapping(
    headers: &csv::StringRecord,
    mapping: &HashMap<String, String>,
) -> (Vec<String>, u64) {
    let mut count: u64 = 0;

    let new_headers: Vec<String> = headers
        .iter()
        .map(|header| {
            if let Some(new_name) = mapping.get(header) {
                count += 1;
                new_name.clone()
            } else {
                header.to_string()
            }
        })
        .collect();

    (new_headers, count)
}

// --- CSV Output ---

/// Write renamed headers followed by all data rows unchanged.
/// Returns (output_bytes, row_count).
fn write_renamed_csv(
    new_headers: &[String],
    reader: &mut csv::Reader<&[u8]>,
) -> Result<(Vec<u8>, u64), BntoError> {
    let mut writer = csv::WriterBuilder::new()
        .flexible(true)
        .from_writer(Vec::new());

    writer
        .write_record(new_headers)
        .map_err(|e| BntoError::ProcessingFailed(format!("Failed to write headers: {e}")))?;

    let mut row_count: u64 = 0;
    for record in reader.records() {
        let record = record
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to read CSV row: {e}")))?;
        writer
            .write_record(record.iter())
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to write CSV row: {e}")))?;
        row_count += 1;
    }

    let output_bytes = writer
        .into_inner()
        .map_err(|e| BntoError::ProcessingFailed(format!("Failed to finalize CSV: {e}")))?;

    Ok((output_bytes, row_count))
}

// --- Result Metadata ---

/// Filter mapping to only columns that exist in the original headers.
fn applied_mapping(
    column_mapping: &HashMap<String, String>,
    headers: &csv::StringRecord,
) -> serde_json::Map<String, serde_json::Value> {
    column_mapping
        .iter()
        .filter(|(old, _)| headers.iter().any(|h| h == old.as_str()))
        .map(|(old, new)| (old.clone(), serde_json::Value::String(new.clone())))
        .collect()
}

/// Build metadata including rename counts and the applied mapping.
fn build_rename_metadata(
    columns_renamed: u64,
    new_headers: &[String],
    row_count: u64,
    column_mapping: &HashMap<String, String>,
    headers: &csv::StringRecord,
) -> serde_json::Map<String, serde_json::Value> {
    let mut m = serde_json::Map::new();
    m.insert(
        "columnsRenamed".to_string(),
        serde_json::Value::Number(columns_renamed.into()),
    );
    m.insert(
        "totalColumns".to_string(),
        serde_json::Value::Number((new_headers.len() as u64).into()),
    );
    m.insert(
        "dataRows".to_string(),
        serde_json::Value::Number(row_count.into()),
    );
    m.insert(
        "mapping".to_string(),
        serde_json::Value::Object(applied_mapping(column_mapping, headers)),
    );
    m
}

// --- Helper Functions ---

/// Extract `columns` param as a HashMap. Returns empty map if missing or invalid.
fn extract_column_mapping(
    params: &serde_json::Map<String, serde_json::Value>,
) -> HashMap<String, String> {
    let columns_value = match params.get("columns") {
        Some(val) => val,
        None => return HashMap::new(),
    };

    let obj = match columns_value {
        serde_json::Value::Object(obj) => obj,
        _ => return HashMap::new(),
    };

    obj.iter()
        .filter_map(|(key, value)| {
            if let serde_json::Value::String(new_name) = value {
                Some((key.clone(), new_name.clone()))
            } else {
                None
            }
        })
        .collect()
}

/// Add "-renamed" before the file extension: "data.csv" -> "data-renamed.csv"
fn build_output_filename(input_filename: &str) -> String {
    match input_filename.rfind('.') {
        Some(dot_pos) => {
            let (name, ext) = input_filename.split_at(dot_pos);
            format!("{name}-renamed{ext}")
        }
        None => format!("{input_filename}-renamed"),
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::NoopContext;

    // --- Test Helpers ---

    /// Create a NodeInput from raw CSV text and optional params.
    /// This helper makes tests cleaner by handling the boilerplate.
    fn make_csv_input(csv_text: &str, params_json: &str) -> NodeInput {
        // Parse the JSON string into a serde_json::Map.
        // If parsing fails, use an empty map (same as production behavior).
        let params: serde_json::Map<String, serde_json::Value> =
            serde_json::from_str(params_json).unwrap_or_default();

        NodeInput {
            data: csv_text.as_bytes().to_vec(),
            filename: "test.csv".to_string(),
            mime_type: Some("text/csv".to_string()),
            params,
        }
    }

    /// Extract the output CSV as a UTF-8 string from the NodeOutput.
    /// Panics if the output has no files or the data isn't valid UTF-8.
    fn output_to_string(output: &NodeOutput) -> String {
        let file = output
            .files
            .first()
            .expect("Should have at least one output file");
        String::from_utf8(file.data.clone()).expect("Output should be valid UTF-8")
    }

    // --- Core Functionality Tests ---

    #[test]
    fn test_rename_one_column() {
        // Rename a single column and verify the rest are unchanged.
        let processor = RenameCsvColumns::new();
        let progress = ProgressReporter::new_noop();

        let input = make_csv_input(
            "name,age,city\nAlice,30,NYC\nBob,25,LA\n",
            r#"{"columns": {"name": "full_name"}}"#,
        );

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        let csv_out = output_to_string(&output);

        // The header "name" should be renamed to "full_name".
        assert!(csv_out.starts_with("full_name,age,city\n"));
        // Data rows should be unchanged.
        assert!(csv_out.contains("Alice,30,NYC"));
        assert!(csv_out.contains("Bob,25,LA"));
    }

    #[test]
    fn test_rename_multiple_columns() {
        // Rename multiple columns at once.
        let processor = RenameCsvColumns::new();
        let progress = ProgressReporter::new_noop();

        let input = make_csv_input(
            "first_name,last_name,email\nJane,Doe,jane@example.com\n",
            r#"{"columns": {"first_name": "given_name", "last_name": "surname"}}"#,
        );

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        let csv_out = output_to_string(&output);

        // Both columns should be renamed; "email" stays the same.
        assert!(csv_out.starts_with("given_name,surname,email\n"));
        // Data row unchanged.
        assert!(csv_out.contains("Jane,Doe,jane@example.com"));
    }

    #[test]
    fn test_rename_nonexistent_column_ignored() {
        // If the mapping references a column that doesn't exist in the CSV,
        // it should be silently ignored — no error.
        let processor = RenameCsvColumns::new();
        let progress = ProgressReporter::new_noop();

        let input = make_csv_input(
            "name,age\nAlice,30\n",
            r#"{"columns": {"nonexistent": "something"}}"#,
        );

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        let csv_out = output_to_string(&output);

        // Headers should be unchanged because "nonexistent" isn't in the CSV.
        assert!(csv_out.starts_with("name,age\n"));
        assert!(csv_out.contains("Alice,30"));

        // Metadata should show 0 columns renamed.
        let renamed_count = output.metadata.get("columnsRenamed").unwrap();
        assert_eq!(renamed_count, &serde_json::json!(0));
    }

    #[test]
    fn test_no_columns_param_passthrough() {
        // If no "columns" param is provided, the CSV should pass through unchanged.
        let processor = RenameCsvColumns::new();
        let progress = ProgressReporter::new_noop();

        let input = make_csv_input("name,age\nAlice,30\n", "{}");

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        let csv_out = output_to_string(&output);

        assert!(csv_out.starts_with("name,age\n"));
        assert!(csv_out.contains("Alice,30"));

        // 0 columns renamed.
        let renamed_count = output.metadata.get("columnsRenamed").unwrap();
        assert_eq!(renamed_count, &serde_json::json!(0));
    }

    #[test]
    fn test_empty_mapping_passthrough() {
        // An empty columns mapping should also pass through unchanged.
        let processor = RenameCsvColumns::new();
        let progress = ProgressReporter::new_noop();

        let input = make_csv_input("name,age\nAlice,30\n", r#"{"columns": {}}"#);

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        let csv_out = output_to_string(&output);

        assert!(csv_out.starts_with("name,age\n"));
        let renamed_count = output.metadata.get("columnsRenamed").unwrap();
        assert_eq!(renamed_count, &serde_json::json!(0));
    }

    #[test]
    fn test_all_columns_renamed() {
        // Rename every column in the CSV.
        let processor = RenameCsvColumns::new();
        let progress = ProgressReporter::new_noop();

        let input = make_csv_input(
            "a,b,c\n1,2,3\n",
            r#"{"columns": {"a": "x", "b": "y", "c": "z"}}"#,
        );

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        let csv_out = output_to_string(&output);

        assert!(csv_out.starts_with("x,y,z\n"));
        assert!(csv_out.contains("1,2,3"));

        // All 3 columns renamed.
        let renamed_count = output.metadata.get("columnsRenamed").unwrap();
        assert_eq!(renamed_count, &serde_json::json!(3));
        let total_columns = output.metadata.get("totalColumns").unwrap();
        assert_eq!(total_columns, &serde_json::json!(3));
    }

    #[test]
    fn test_data_rows_preserved_unchanged() {
        // Verify that data rows are byte-for-byte preserved (no trimming,
        // no quoting changes, no reordering).
        let processor = RenameCsvColumns::new();
        let progress = ProgressReporter::new_noop();

        let csv_input = "name,value,notes\nAlice,\"100,000\",\"has, commas\"\nBob,200,simple\n";
        let input = make_csv_input(csv_input, r#"{"columns": {"name": "person"}}"#);

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        let csv_out = output_to_string(&output);

        // Header renamed.
        assert!(csv_out.starts_with("person,value,notes\n"));
        // Data with commas and quotes preserved.
        assert!(csv_out.contains("Alice,\"100,000\",\"has, commas\""));
        assert!(csv_out.contains("Bob,200,simple"));
    }

    #[test]
    fn test_column_order_preserved() {
        // Columns should stay in the same order — only names change.
        let processor = RenameCsvColumns::new();
        let progress = ProgressReporter::new_noop();

        let input = make_csv_input(
            "z_col,a_col,m_col\n1,2,3\n",
            r#"{"columns": {"m_col": "middle"}}"#,
        );

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        let csv_out = output_to_string(&output);

        // Order: z_col, a_col, middle (only m_col renamed, position preserved).
        assert!(csv_out.starts_with("z_col,a_col,middle\n"));
    }

    #[test]
    fn test_variable_length_rows_handled() {
        // Some CSVs have ragged rows (not all rows have the same number of fields).
        // Our processor should handle this gracefully with flexible(true).
        let processor = RenameCsvColumns::new();
        let progress = ProgressReporter::new_noop();

        let input = make_csv_input(
            "a,b,c\n1,2,3\n4,5\n6,7,8,9\n",
            r#"{"columns": {"a": "first"}}"#,
        );

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        let csv_out = output_to_string(&output);

        // Header renamed.
        assert!(csv_out.starts_with("first,b,c\n"));
        // All rows should be present, including the ragged ones.
        assert!(csv_out.contains("1,2,3"));
        assert!(csv_out.contains("4,5"));
        assert!(csv_out.contains("6,7,8,9"));
    }

    #[test]
    fn test_headers_only_csv() {
        // A CSV with headers but no data rows. The headers should be renamed.
        let processor = RenameCsvColumns::new();
        let progress = ProgressReporter::new_noop();

        let input = make_csv_input("name,age\n", r#"{"columns": {"name": "full_name"}}"#);

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        let csv_out = output_to_string(&output);

        assert!(csv_out.starts_with("full_name,age"));

        // 0 data rows.
        let data_rows = output.metadata.get("dataRows").unwrap();
        assert_eq!(data_rows, &serde_json::json!(0));
    }

    #[test]
    fn test_non_utf8_input_returns_error() {
        // Non-UTF8 input should return a clear error, not a panic.
        let processor = RenameCsvColumns::new();
        let progress = ProgressReporter::new_noop();

        // Create invalid UTF-8 bytes (0xFF 0xFE is not valid UTF-8).
        let bad_bytes: Vec<u8> = vec![0xFF, 0xFE, 0x00, 0x61];
        let input = NodeInput {
            data: bad_bytes,
            filename: "bad.csv".to_string(),
            mime_type: Some("text/csv".to_string()),
            params: serde_json::Map::new(),
        };

        let result = processor.process(input, &progress, &NoopContext);

        // Should be an error, not a panic.
        assert!(result.is_err());

        // The error message should mention UTF-8.
        //
        if let Err(e) = result {
            let error_msg = e.to_string();
            assert!(
                error_msg.contains("UTF-8"),
                "Error should mention UTF-8: got '{error_msg}'"
            );
        }
    }

    #[test]
    fn test_large_csv_only_header_changes() {
        // A large CSV (1000+ rows) should process correctly with only
        // the header row changed. This tests performance and correctness
        // at scale.
        let processor = RenameCsvColumns::new();
        let progress = ProgressReporter::new_noop();

        // Build a CSV with 1000 data rows.
        let mut csv_text = String::from("id,name,value\n");
        for i in 0..1000 {
            csv_text.push_str(&format!("{i},item_{i},{}\n", i * 10));
        }

        let input = make_csv_input(
            &csv_text,
            r#"{"columns": {"id": "identifier", "name": "label"}}"#,
        );

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        let csv_out = output_to_string(&output);

        // Header renamed.
        assert!(csv_out.starts_with("identifier,label,value\n"));

        // Spot-check a few data rows to verify they're unchanged.
        assert!(csv_out.contains("0,item_0,0"));
        assert!(csv_out.contains("500,item_500,5000"));
        assert!(csv_out.contains("999,item_999,9990"));

        // Metadata should show 2 columns renamed and 1000 data rows.
        let renamed_count = output.metadata.get("columnsRenamed").unwrap();
        assert_eq!(renamed_count, &serde_json::json!(2));
        let data_rows = output.metadata.get("dataRows").unwrap();
        assert_eq!(data_rows, &serde_json::json!(1000));
    }

    // --- Output Filename Tests ---

    #[test]
    fn test_output_filename_with_extension() {
        assert_eq!(build_output_filename("data.csv"), "data-renamed.csv");
    }

    #[test]
    fn test_output_filename_without_extension() {
        assert_eq!(build_output_filename("data"), "data-renamed");
    }

    #[test]
    fn test_output_filename_multiple_dots() {
        assert_eq!(
            build_output_filename("my.data.file.csv"),
            "my.data.file-renamed.csv"
        );
    }

    // --- Metadata Tests ---

    #[test]
    fn test_metadata_includes_applied_mapping() {
        // The metadata should include the mapping that was actually applied
        // (only columns that existed in the CSV).
        let processor = RenameCsvColumns::new();
        let progress = ProgressReporter::new_noop();

        let input = make_csv_input(
            "name,age\nAlice,30\n",
            r#"{"columns": {"name": "full_name", "missing": "nope"}}"#,
        );

        let output = processor.process(input, &progress, &NoopContext).unwrap();

        // The "mapping" metadata should only include "name" → "full_name",
        // NOT "missing" → "nope" (because "missing" doesn't exist in the CSV).
        let mapping = output.metadata.get("mapping").unwrap();
        let mapping_obj = mapping.as_object().unwrap();
        assert_eq!(mapping_obj.len(), 1);
        assert_eq!(mapping_obj.get("name").unwrap(), "full_name");
        // "missing" should NOT be in the mapping.
        assert!(mapping_obj.get("missing").is_none());
    }

    // --- Edge Cases ---

    #[test]
    fn test_columns_param_not_object_passthrough() {
        // If "columns" is a string instead of an object, treat it as no mapping.
        let processor = RenameCsvColumns::new();
        let progress = ProgressReporter::new_noop();

        let input = make_csv_input("name,age\nAlice,30\n", r#"{"columns": "not an object"}"#);

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        let csv_out = output_to_string(&output);

        // Should pass through unchanged.
        assert!(csv_out.starts_with("name,age\n"));
    }

    #[test]
    fn test_processor_name() {
        let processor = RenameCsvColumns::new();
        assert_eq!(processor.name(), "rename-csv-columns");
    }

    #[test]
    fn test_default_creates_same_as_new() {
        // Verify that Default and new() produce equivalent processors.
        let p1 = RenameCsvColumns::new();
        let p2 = RenameCsvColumns;
        assert_eq!(p1.name(), p2.name());
    }

    #[test]
    fn test_output_mime_type_is_csv() {
        let processor = RenameCsvColumns::new();
        let progress = ProgressReporter::new_noop();

        let input = make_csv_input("name\nAlice\n", "{}");
        let output = processor.process(input, &progress, &NoopContext).unwrap();

        assert_eq!(output.files[0].mime_type, "text/csv");
    }

    #[test]
    fn test_output_filename_has_renamed_suffix() {
        let processor = RenameCsvColumns::new();
        let progress = ProgressReporter::new_noop();

        let mut input = make_csv_input("name\nAlice\n", "{}");
        input.filename = "my_data.csv".to_string();

        let output = processor.process(input, &progress, &NoopContext).unwrap();

        assert_eq!(output.files[0].filename, "my_data-renamed.csv");
    }
}
