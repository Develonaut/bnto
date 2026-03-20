// Clean CSV Node — remove empty rows, trim whitespace, deduplicate.
//
// Each cleaning operation (trim, remove empties, deduplicate) is controlled
// by a boolean parameter so users can enable/disable them individually.

use bnto_core::errors::BntoError;
use bnto_core::processor::{NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;

/// The clean-csv node processor. Stateless — config comes from `NodeInput.params`.
pub struct CleanCsv;

impl CleanCsv {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CleanCsv {
    fn default() -> Self {
        Self::new()
    }
}

// --- NodeProcessor Implementation ---

impl NodeProcessor for CleanCsv {
    fn name(&self) -> &str {
        "clean-csv"
    }

    /// Self-describing metadata: trimWhitespace, removeEmptyRows, removeDuplicates
    /// (all booleans, default true). Accepts text/csv only.
    fn metadata(&self) -> bnto_core::NodeMetadata {
        use bnto_core::metadata::*;
        NodeMetadata {
            node_type: "spreadsheet-clean".to_string(),
            operation: "default".to_string(),
            name: "Clean CSV".to_string(),
            description: "Remove empty rows, trim whitespace, and deduplicate CSV data".to_string(),
            category: NodeCategory::Spreadsheet,
            accepts: vec!["text/csv".to_string()],
            platforms: vec!["browser".to_string()],
            parameters: build_clean_parameters(),
        }
    }

    /// Clean a CSV file: trim whitespace, remove empty rows, deduplicate.
    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
    ) -> Result<NodeOutput, BntoError> {
        progress.report(0, "Parsing CSV...");
        let config = CleanConfig::from_params(&input.params);
        let csv_text = parse_csv_input(&input.data)?;

        progress.report(10, "Reading CSV records...");
        let (cleaned_headers, num_columns) = read_and_clean_headers(csv_text, config.trim)?;

        progress.report(20, "Cleaning records...");
        let (mut cleaned_rows, original_row_count) =
            collect_cleaned_rows(csv_text, config.trim, config.remove_empty, num_columns)?;

        progress.report(60, "Removing duplicates...");
        let duplicates_removed = deduplicate_rows(&mut cleaned_rows, config.deduplicate);

        progress.report(80, "Writing cleaned CSV...");
        let output_bytes = write_csv_output(&cleaned_headers, &cleaned_rows)?;

        progress.report(90, "Building result...");
        let metadata =
            build_clean_metadata(original_row_count, cleaned_rows.len(), duplicates_removed);

        progress.report(100, "Done!");
        Ok(build_clean_output(output_bytes, &input.filename, metadata))
    }
}

// --- Configuration ---

struct CleanConfig {
    trim: bool,
    remove_empty: bool,
    deduplicate: bool,
}

impl CleanConfig {
    fn from_params(params: &serde_json::Map<String, serde_json::Value>) -> Self {
        Self {
            trim: bool_param(params, "trimWhitespace", true),
            remove_empty: bool_param(params, "removeEmptyRows", true),
            deduplicate: bool_param(params, "removeDuplicates", true),
        }
    }
}

fn bool_param(
    params: &serde_json::Map<String, serde_json::Value>,
    key: &str,
    default: bool,
) -> bool {
    params.get(key).and_then(|v| v.as_bool()).unwrap_or(default)
}

fn build_clean_output(
    data: Vec<u8>,
    input_filename: &str,
    metadata: serde_json::Map<String, serde_json::Value>,
) -> NodeOutput {
    NodeOutput {
        files: vec![OutputFile {
            data,
            filename: generate_output_filename(input_filename),
            mime_type: "text/csv".to_string(),
        }],
        metadata,
    }
}

// --- Metadata Parameter Definitions ---

/// Boolean cleaning parameter.
fn clean_bool_param(
    name: &str,
    label: &str,
    description: &str,
) -> bnto_core::metadata::ParameterDef {
    use bnto_core::metadata::*;
    ParameterDef {
        name: name.to_string(),
        label: label.to_string(),
        description: description.to_string(),
        param_type: ParameterType::Boolean,
        default: Some(serde_json::json!(true)),
        ..Default::default()
    }
}

fn build_clean_parameters() -> Vec<bnto_core::metadata::ParameterDef> {
    vec![
        clean_bool_param(
            "trimWhitespace",
            "Trim Whitespace",
            "Remove leading and trailing whitespace from every cell",
        ),
        clean_bool_param(
            "removeEmptyRows",
            "Remove Empty Rows",
            "Skip rows where every cell is blank",
        ),
        clean_bool_param(
            "removeDuplicates",
            "Remove Duplicates",
            "Remove duplicate rows, keeping the first occurrence",
        ),
    ]
}

// --- CSV Parsing ---

/// Validate and convert raw bytes to a UTF-8 string, rejecting empty input.
fn parse_csv_input(data: &[u8]) -> Result<&str, BntoError> {
    let csv_text = std::str::from_utf8(data).map_err(|e| {
        BntoError::InvalidInput(format!(
            "File is not valid UTF-8 text (is this really a CSV?): {e}"
        ))
    })?;

    if csv_text.trim().is_empty() {
        return Err(BntoError::InvalidInput(
            "CSV file is empty — no data to clean".to_string(),
        ));
    }

    Ok(csv_text)
}

/// Read and optionally trim the header row. Returns (headers, column_count).
fn read_and_clean_headers(csv_text: &str, trim: bool) -> Result<(Vec<String>, usize), BntoError> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_reader(csv_text.as_bytes());

    let headers = reader
        .headers()
        .map_err(|e| BntoError::ProcessingFailed(format!("Failed to read CSV headers: {e}")))?
        .clone();

    let cleaned: Vec<String> = if trim {
        headers.iter().map(|h| h.trim().to_string()).collect()
    } else {
        headers.iter().map(|h| h.to_string()).collect()
    };

    let num_columns = cleaned.len();
    Ok((cleaned, num_columns))
}

// --- Row Processing ---

/// Iterate data rows, applying trim and empty-row removal.
/// Returns (cleaned_rows, original_row_count).
fn collect_cleaned_rows(
    csv_text: &str,
    trim: bool,
    remove_empty: bool,
    num_columns: usize,
) -> Result<(Vec<Vec<String>>, usize), BntoError> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_reader(csv_text.as_bytes());

    let mut cleaned_rows: Vec<Vec<String>> = Vec::new();
    let mut original_row_count: usize = 0;

    for result in reader.records() {
        original_row_count += 1;
        let record = match result {
            Ok(rec) => rec,
            Err(_) => continue, // skip malformed rows
        };

        let row = normalize_row(&record, trim, num_columns);

        if remove_empty && row.iter().all(|cell| cell.is_empty()) {
            continue;
        }

        cleaned_rows.push(row);
    }

    Ok((cleaned_rows, original_row_count))
}

/// Clean a single record: optionally trim cells, then pad/truncate to match header width.
fn normalize_row(record: &csv::StringRecord, trim: bool, num_columns: usize) -> Vec<String> {
    let mut row: Vec<String> = record
        .iter()
        .map(|cell| {
            if trim {
                cell.trim().to_string()
            } else {
                cell.to_string()
            }
        })
        .collect();

    while row.len() < num_columns {
        row.push(String::new());
    }
    row.truncate(num_columns);
    row
}

/// Remove duplicate rows in-place using null-byte-joined keys. Returns count removed.
fn deduplicate_rows(rows: &mut Vec<Vec<String>>, enabled: bool) -> usize {
    if !enabled {
        return 0;
    }

    let mut seen = std::collections::HashSet::new();
    let before = rows.len();

    // Null byte separator avoids collisions between cell values.
    rows.retain(|row| seen.insert(row.join("\0")));

    before - rows.len()
}

// --- CSV Output ---

/// Write cleaned headers and rows to a CSV byte buffer.
fn write_csv_output(headers: &[String], rows: &[Vec<String>]) -> Result<Vec<u8>, BntoError> {
    let mut writer = csv::WriterBuilder::new().from_writer(Vec::new());

    writer
        .write_record(headers)
        .map_err(|e| BntoError::ProcessingFailed(format!("Failed to write CSV headers: {e}")))?;

    for row in rows {
        writer
            .write_record(row)
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to write CSV row: {e}")))?;
    }

    writer
        .into_inner()
        .map_err(|e| BntoError::ProcessingFailed(format!("Failed to finalize CSV output: {e}")))
}

// --- Result Metadata ---

fn build_clean_metadata(
    original_row_count: usize,
    cleaned_row_count: usize,
    duplicates_removed: usize,
) -> serde_json::Map<String, serde_json::Value> {
    let rows_removed = original_row_count - cleaned_row_count;
    let mut metadata = serde_json::Map::new();
    metadata.insert(
        "originalRows".to_string(),
        serde_json::Value::Number(original_row_count.into()),
    );
    metadata.insert(
        "cleanedRows".to_string(),
        serde_json::Value::Number(cleaned_row_count.into()),
    );
    metadata.insert(
        "rowsRemoved".to_string(),
        serde_json::Value::Number(rows_removed.into()),
    );
    metadata.insert(
        "duplicatesRemoved".to_string(),
        serde_json::Value::Number(duplicates_removed.into()),
    );
    metadata
}

// --- Filename ---

/// Add "-cleaned" before the file extension: "data.csv" -> "data-cleaned.csv"
fn generate_output_filename(original: &str) -> String {
    if let Some(dot_pos) = original.rfind('.') {
        let stem = &original[..dot_pos];
        let ext = &original[dot_pos..];
        format!("{stem}-cleaned{ext}")
    } else {
        format!("{original}-cleaned")
    }
}

// =============================================================================
// Tests — Every function must be tested!
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Test Helpers
    // =========================================================================

    /// Create a NodeInput from raw CSV text and optional params.
    ///
    /// This helper makes tests concise — instead of building a full
    /// NodeInput struct every time, we just pass the CSV text.
    fn make_csv_input(csv_text: &str) -> NodeInput {
        NodeInput {
            data: csv_text.as_bytes().to_vec(),
            filename: "test.csv".to_string(),
            mime_type: Some("text/csv".to_string()),
            params: serde_json::Map::new(),
        }
    }

    /// Create a NodeInput with custom parameters.
    fn make_csv_input_with_params(
        csv_text: &str,
        params: serde_json::Map<String, serde_json::Value>,
    ) -> NodeInput {
        NodeInput {
            data: csv_text.as_bytes().to_vec(),
            filename: "test.csv".to_string(),
            mime_type: Some("text/csv".to_string()),
            params,
        }
    }

    /// Extract the output CSV text from a NodeOutput.
    ///
    /// The output is raw bytes — we convert back to a String for easy
    /// assertion comparisons in tests.
    fn output_csv_text(output: &NodeOutput) -> String {
        String::from_utf8(output.files[0].data.clone()).expect("Output should be valid UTF-8")
    }

    /// Count the number of data rows (excluding the header) in CSV text.
    fn count_data_rows(csv_text: &str) -> usize {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .flexible(true)
            .from_reader(csv_text.as_bytes());
        reader.records().count()
    }

    // =========================================================================
    // Basic Functionality Tests
    // =========================================================================

    #[test]
    fn test_name_returns_clean_csv() {
        // Verify the node processor reports the correct name.
        let processor = CleanCsv::new();
        assert_eq!(processor.name(), "clean-csv");
    }

    #[test]
    fn test_default_creates_same_as_new() {
        // Verify that Default trait works.
        // Clippy warns against `.default()` on a unit struct, but we want
        // to test that Default IS implemented, so we allow it.
        #[allow(clippy::default_constructed_unit_structs)]
        let _processor = CleanCsv::default();
        // If we get here without panic, the test passes.
        // CleanCsv is a unit struct, so there's nothing else to check.
    }

    #[test]
    fn test_basic_csv_passthrough() {
        // A clean CSV with no issues should pass through with all rows intact.
        // Only formatting differences (like trailing newline) should change.
        let csv = "name,age,city\nAlice,30,NYC\nBob,25,LA\n";
        let processor = CleanCsv::new();
        let progress = ProgressReporter::new_noop();
        let input = make_csv_input(csv);

        let output = processor.process(input, &progress).unwrap();

        // Should have one output file.
        assert_eq!(output.files.len(), 1);

        // Should be a CSV file.
        assert_eq!(output.files[0].mime_type, "text/csv");

        // Should preserve both data rows.
        let text = output_csv_text(&output);
        let rows = count_data_rows(&text);
        assert_eq!(rows, 2, "Should preserve both data rows");

        // Should contain both names.
        assert!(text.contains("Alice"), "Should contain Alice");
        assert!(text.contains("Bob"), "Should contain Bob");
    }

    // =========================================================================
    // Remove Empty Rows Tests
    // =========================================================================

    #[test]
    fn test_remove_empty_rows() {
        // CSV with empty rows (all cells blank) — they should be removed.
        let csv = "name,age\nAlice,30\n,,\nBob,25\n,,\n";
        let processor = CleanCsv::new();
        let progress = ProgressReporter::new_noop();
        let input = make_csv_input(csv);

        let output = processor.process(input, &progress).unwrap();
        let text = output_csv_text(&output);
        let rows = count_data_rows(&text);

        // Should keep only Alice and Bob (2 rows), removing 2 empty rows.
        assert_eq!(rows, 2, "Should remove empty rows, keeping 2 data rows");
        assert!(text.contains("Alice"));
        assert!(text.contains("Bob"));

        // Check metadata.
        let removed = output
            .metadata
            .get("rowsRemoved")
            .unwrap()
            .as_u64()
            .unwrap();
        assert_eq!(removed, 2, "Should report 2 rows removed");
    }

    // =========================================================================
    // Trim Whitespace Tests
    // =========================================================================

    #[test]
    fn test_trim_whitespace_from_cells() {
        // CSV with extra whitespace around cell values.
        let csv = "name,age\n  Alice  , 30 \n Bob ,25\n";
        let processor = CleanCsv::new();
        let progress = ProgressReporter::new_noop();
        let input = make_csv_input(csv);

        let output = processor.process(input, &progress).unwrap();
        let text = output_csv_text(&output);

        // After trimming, cells should have no leading/trailing whitespace.
        // The csv crate quotes fields that don't need quoting only when necessary.
        assert!(text.contains("Alice"), "Should contain trimmed 'Alice'");
        assert!(text.contains("Bob"), "Should contain trimmed 'Bob'");
        // Verify there's no "  Alice  " with spaces.
        assert!(
            !text.contains("  Alice"),
            "Should NOT contain '  Alice' with leading spaces"
        );
    }

    // =========================================================================
    // Remove Duplicate Rows Tests
    // =========================================================================

    #[test]
    fn test_remove_duplicate_rows() {
        // CSV with exact duplicate rows — only the first occurrence should remain.
        let csv = "name,age\nAlice,30\nBob,25\nAlice,30\n";
        let processor = CleanCsv::new();
        let progress = ProgressReporter::new_noop();
        let input = make_csv_input(csv);

        let output = processor.process(input, &progress).unwrap();
        let text = output_csv_text(&output);
        let rows = count_data_rows(&text);

        // Should keep Alice (first) and Bob, removing the duplicate Alice.
        assert_eq!(rows, 2, "Should remove 1 duplicate, keeping 2 rows");

        // Check metadata.
        let dupes = output
            .metadata
            .get("duplicatesRemoved")
            .unwrap()
            .as_u64()
            .unwrap();
        assert_eq!(dupes, 1, "Should report 1 duplicate removed");
    }

    // =========================================================================
    // Preserve Header Row Tests
    // =========================================================================

    #[test]
    fn test_preserves_header_row() {
        // The header row should always be present in the output, trimmed.
        let csv = " name , age \nAlice,30\n";
        let processor = CleanCsv::new();
        let progress = ProgressReporter::new_noop();
        let input = make_csv_input(csv);

        let output = processor.process(input, &progress).unwrap();
        let text = output_csv_text(&output);

        // The header should be trimmed.
        assert!(
            text.starts_with("name,age"),
            "Header should be trimmed: got '{}'",
            text.lines().next().unwrap_or("")
        );
    }

    // =========================================================================
    // Variable-Length Rows Tests
    // =========================================================================

    #[test]
    fn test_variable_length_rows_padded() {
        // Some rows have fewer columns than the header — they should be
        // padded with empty strings to match the header width.
        let csv = "name,age,city\nAlice,30\nBob,25,LA\n";
        let processor = CleanCsv::new();
        let progress = ProgressReporter::new_noop();
        let input = make_csv_input(csv);

        let output = processor.process(input, &progress).unwrap();
        let text = output_csv_text(&output);

        // Both rows should be present.
        let rows = count_data_rows(&text);
        assert_eq!(rows, 2, "Should keep both rows");

        // Alice's row should have an empty city field.
        // The csv writer will include the trailing comma for the empty field.
        assert!(text.contains("Alice"), "Should contain Alice's row");
    }

    // =========================================================================
    // Edge Case: Headers Only
    // =========================================================================

    #[test]
    fn test_headers_only_csv() {
        // A CSV with only headers and no data rows.
        let csv = "name,age,city\n";
        let processor = CleanCsv::new();
        let progress = ProgressReporter::new_noop();
        let input = make_csv_input(csv);

        let output = processor.process(input, &progress).unwrap();
        let text = output_csv_text(&output);

        // The output should have the header but zero data rows.
        let rows = count_data_rows(&text);
        assert_eq!(rows, 0, "Should have zero data rows");
        assert!(text.contains("name"), "Should still contain header");

        // Metadata should reflect zero rows.
        let original = output
            .metadata
            .get("originalRows")
            .unwrap()
            .as_u64()
            .unwrap();
        assert_eq!(original, 0, "Original row count should be 0");
    }

    // =========================================================================
    // Edge Case: Empty Input
    // =========================================================================

    #[test]
    fn test_empty_input_returns_error() {
        // Completely empty input (no bytes) should return an error.
        let csv = "";
        let processor = CleanCsv::new();
        let progress = ProgressReporter::new_noop();
        let input = make_csv_input(csv);

        let result = processor.process(input, &progress);

        assert!(result.is_err(), "Empty input should return an error");

        if let Err(err) = result {
            assert!(
                err.to_string().contains("empty"),
                "Error should mention 'empty': got '{}'",
                err
            );
        }
    }

    // =========================================================================
    // Edge Case: Non-UTF8 Input
    // =========================================================================

    #[test]
    fn test_non_utf8_input_returns_error() {
        // Invalid UTF-8 bytes should return a clear error.
        //
        // 0xFF 0xFE is a common byte sequence in files that aren't
        // UTF-8 (it's a UTF-16 BOM). Our parser expects UTF-8 only.
        let bad_bytes = vec![0xFF, 0xFE, 0x00, 0x41]; // Not valid UTF-8
        let processor = CleanCsv::new();
        let progress = ProgressReporter::new_noop();
        let input = NodeInput {
            data: bad_bytes,
            filename: "bad.csv".to_string(),
            mime_type: None,
            params: serde_json::Map::new(),
        };

        let result = processor.process(input, &progress);

        assert!(result.is_err(), "Non-UTF8 input should return an error");

        // Use `if let` instead of `.unwrap_err()` because NodeOutput
        // doesn't implement Debug (required by unwrap_err's panic message).
        if let Err(err) = result {
            assert!(
                err.to_string().contains("UTF-8"),
                "Error should mention UTF-8: got '{}'",
                err
            );
        }
    }

    // =========================================================================
    // Combined Operations Test
    // =========================================================================

    #[test]
    fn test_combined_trim_remove_empty_deduplicate() {
        // A messy CSV that needs all three cleaning operations.
        let csv = "name,age\n  Alice  , 30 \n,,\nBob,25\n  Alice  , 30 \n,,\n";
        let processor = CleanCsv::new();
        let progress = ProgressReporter::new_noop();
        let input = make_csv_input(csv);

        let output = processor.process(input, &progress).unwrap();
        let text = output_csv_text(&output);
        let rows = count_data_rows(&text);

        // After trim + remove empty + deduplicate:
        //   " Alice ", " 30 "  -> "Alice", "30" (kept, first occurrence)
        //   ",,\"               -> removed (empty)
        //   "Bob", "25"         -> kept
        //   " Alice ", " 30 "  -> removed (duplicate after trim)
        //   ",,"                -> removed (empty)
        assert_eq!(
            rows, 2,
            "Should have 2 rows (Alice + Bob), got text:\n{}",
            text
        );

        // Check metadata shows the cleaning results.
        let removed = output
            .metadata
            .get("rowsRemoved")
            .unwrap()
            .as_u64()
            .unwrap();
        assert_eq!(
            removed, 3,
            "Should remove 3 rows total (2 empty + 1 duplicate)"
        );
        let dupes = output
            .metadata
            .get("duplicatesRemoved")
            .unwrap()
            .as_u64()
            .unwrap();
        assert_eq!(dupes, 1, "Should report 1 duplicate removed");
    }

    // =========================================================================
    // Parameter Override Tests
    // =========================================================================

    #[test]
    fn test_remove_duplicates_false_preserves_duplicates() {
        // When removeDuplicates is false, duplicate rows should be kept.
        let csv = "name,age\nAlice,30\nBob,25\nAlice,30\n";
        let processor = CleanCsv::new();
        let progress = ProgressReporter::new_noop();

        let mut params = serde_json::Map::new();
        params.insert(
            "removeDuplicates".to_string(),
            serde_json::Value::Bool(false),
        );
        let input = make_csv_input_with_params(csv, params);

        let output = processor.process(input, &progress).unwrap();
        let text = output_csv_text(&output);
        let rows = count_data_rows(&text);

        // All 3 rows should be kept (including the duplicate).
        assert_eq!(
            rows, 3,
            "Should keep all 3 rows when removeDuplicates is false"
        );

        // Metadata should show 0 duplicates removed.
        let dupes = output
            .metadata
            .get("duplicatesRemoved")
            .unwrap()
            .as_u64()
            .unwrap();
        assert_eq!(dupes, 0);
    }

    #[test]
    fn test_trim_whitespace_false_preserves_whitespace() {
        // When trimWhitespace is false, whitespace should be preserved.
        let csv = "name,age\n  Alice  , 30 \nBob,25\n";
        let processor = CleanCsv::new();
        let progress = ProgressReporter::new_noop();

        let mut params = serde_json::Map::new();
        params.insert("trimWhitespace".to_string(), serde_json::Value::Bool(false));
        let input = make_csv_input_with_params(csv, params);

        let output = processor.process(input, &progress).unwrap();
        let text = output_csv_text(&output);

        // The whitespace should still be there.
        // The csv writer may quote fields with spaces, so check for the
        // actual content rather than exact formatting.
        assert!(
            text.contains("Alice") && text.contains("Bob"),
            "Should contain both names"
        );
        // With trimWhitespace=false, the spaces around Alice should persist.
        // The csv crate will quote the field: "  Alice  "
        assert!(
            text.contains("  Alice  "),
            "Should preserve whitespace around Alice: got:\n{}",
            text
        );
    }

    #[test]
    fn test_remove_empty_rows_false_preserves_empty_rows() {
        // When removeEmptyRows is false, empty rows should be kept.
        let csv = "name,age\nAlice,30\n,,\nBob,25\n";
        let processor = CleanCsv::new();
        let progress = ProgressReporter::new_noop();

        let mut params = serde_json::Map::new();
        params.insert(
            "removeEmptyRows".to_string(),
            serde_json::Value::Bool(false),
        );
        let input = make_csv_input_with_params(csv, params);

        let output = processor.process(input, &progress).unwrap();
        let text = output_csv_text(&output);
        let rows = count_data_rows(&text);

        // All 3 rows (including the empty one) should be kept.
        assert_eq!(
            rows, 3,
            "Should keep all 3 rows when removeEmptyRows is false"
        );
    }

    // =========================================================================
    // Large CSV Test
    // =========================================================================

    #[test]
    fn test_large_csv_1000_rows() {
        // Generate a CSV with 1000+ rows to verify performance.
        let mut csv = String::from("id,name,value\n");
        for i in 0..1200 {
            csv.push_str(&format!("{i},item_{i},{}\n", i * 10));
        }

        let processor = CleanCsv::new();
        let progress = ProgressReporter::new_noop();
        let input = make_csv_input(&csv);

        let output = processor.process(input, &progress).unwrap();
        let text = output_csv_text(&output);
        let rows = count_data_rows(&text);

        // All 1200 rows should be present (no duplicates, no empty rows).
        assert_eq!(rows, 1200, "Should process all 1200 rows");

        // Metadata should show 0 rows removed.
        let removed = output
            .metadata
            .get("rowsRemoved")
            .unwrap()
            .as_u64()
            .unwrap();
        assert_eq!(removed, 0, "No rows should be removed from clean data");
    }

    // =========================================================================
    // Output Filename Tests
    // =========================================================================

    #[test]
    fn test_output_filename_with_extension() {
        let result = generate_output_filename("data.csv");
        assert_eq!(result, "data-cleaned.csv");
    }

    #[test]
    fn test_output_filename_without_extension() {
        let result = generate_output_filename("data");
        assert_eq!(result, "data-cleaned");
    }

    #[test]
    fn test_output_filename_with_multiple_dots() {
        let result = generate_output_filename("my.data.csv");
        assert_eq!(result, "my.data-cleaned.csv");
    }

    #[test]
    fn test_output_filename_in_result() {
        // Verify the output file has the correct cleaned filename.
        let csv = "name\nAlice\n";
        let processor = CleanCsv::new();
        let progress = ProgressReporter::new_noop();
        let input = NodeInput {
            data: csv.as_bytes().to_vec(),
            filename: "employees.csv".to_string(),
            mime_type: None,
            params: serde_json::Map::new(),
        };

        let output = processor.process(input, &progress).unwrap();
        assert_eq!(output.files[0].filename, "employees-cleaned.csv");
    }

    // =========================================================================
    // Metadata Tests
    // =========================================================================

    #[test]
    fn test_metadata_contains_all_fields() {
        let csv = "name,age\nAlice,30\nBob,25\n";
        let processor = CleanCsv::new();
        let progress = ProgressReporter::new_noop();
        let input = make_csv_input(csv);

        let output = processor.process(input, &progress).unwrap();

        // All metadata fields should be present.
        assert!(output.metadata.contains_key("originalRows"));
        assert!(output.metadata.contains_key("cleanedRows"));
        assert!(output.metadata.contains_key("rowsRemoved"));
        assert!(output.metadata.contains_key("duplicatesRemoved"));
    }
}
