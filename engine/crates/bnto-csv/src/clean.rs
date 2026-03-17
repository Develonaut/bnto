// =============================================================================
// Clean CSV Node — Remove Empty Rows, Trim Whitespace, Deduplicate
// =============================================================================
//
// Cleaning operations (each controlled by a boolean parameter):
//   1. Trim whitespace from every cell
//   2. Remove rows where every cell is blank (after trimming)
//   3. Remove duplicate rows (order-preserving, first occurrence wins)

use bnto_core::errors::BntoError;
use bnto_core::processor::{NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;

// =============================================================================
// CleanCsv — The Node Processor Struct
// =============================================================================

/// Stateless clean-csv node processor. Configuration comes from `NodeInput.params`.
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

// =============================================================================
// NodeProcessor Implementation
// =============================================================================

impl NodeProcessor for CleanCsv {
    fn name(&self) -> &str {
        "clean-csv"
    }

    fn metadata(&self) -> bnto_core::NodeMetadata {
        use bnto_core::metadata::*;
        NodeMetadata {
            node_type: "spreadsheet".to_string(),
            operation: "clean".to_string(),
            name: "Clean CSV".to_string(),
            description: "Remove empty rows, trim whitespace, and deduplicate CSV data".to_string(),
            category: NodeCategory::Spreadsheet,
            accepts: vec!["text/csv".to_string()],
            platforms: vec!["browser".to_string()],
            parameters: vec![
                ParameterDef {
                    name: "trimWhitespace".to_string(),
                    label: "Trim Whitespace".to_string(),
                    description: "Remove leading and trailing whitespace from every cell"
                        .to_string(),
                    param_type: ParameterType::Boolean,
                    default: Some(serde_json::json!(true)),
                    visible_when: Some(ParamCondition::Single(ParamConditionEntry {
                        param: "operation".to_string(),
                        equals: "clean".to_string(),
                    })),
                    ..Default::default()
                },
                ParameterDef {
                    name: "removeEmptyRows".to_string(),
                    label: "Remove Empty Rows".to_string(),
                    description: "Skip rows where every cell is blank".to_string(),
                    param_type: ParameterType::Boolean,
                    default: Some(serde_json::json!(true)),
                    visible_when: Some(ParamCondition::Single(ParamConditionEntry {
                        param: "operation".to_string(),
                        equals: "clean".to_string(),
                    })),
                    ..Default::default()
                },
                ParameterDef {
                    name: "removeDuplicates".to_string(),
                    label: "Remove Duplicates".to_string(),
                    description: "Remove duplicate rows, keeping the first occurrence".to_string(),
                    param_type: ParameterType::Boolean,
                    default: Some(serde_json::json!(true)),
                    visible_when: Some(ParamCondition::Single(ParamConditionEntry {
                        param: "operation".to_string(),
                        equals: "clean".to_string(),
                    })),
                    ..Default::default()
                },
            ],
        }
    }

    /// Clean a CSV file: trim whitespace, remove empty rows, deduplicate.
    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
    ) -> Result<NodeOutput, BntoError> {
        progress.report(0, "Parsing CSV...");

        // --- Read configuration parameters (default: all enabled) ---
        let trim_whitespace = input
            .params
            .get("trimWhitespace")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let remove_empty_rows = input
            .params
            .get("removeEmptyRows")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let remove_duplicates = input
            .params
            .get("removeDuplicates")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        // --- Convert bytes to UTF-8 ---
        let csv_text = std::str::from_utf8(&input.data).map_err(|e| {
            BntoError::InvalidInput(format!(
                "File is not valid UTF-8 text (is this really a CSV?): {e}"
            ))
        })?;

        if csv_text.trim().is_empty() {
            return Err(BntoError::InvalidInput(
                "CSV file is empty — no data to clean".to_string(),
            ));
        }

        progress.report(10, "Reading CSV records...");

        // --- Parse CSV ---
        // flexible(true) allows rows with different field counts.
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .flexible(true)
            .from_reader(csv_text.as_bytes());

        // --- Read and optionally trim header row ---
        let headers = reader
            .headers()
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to read CSV headers: {e}")))?
            .clone();

        let cleaned_headers: Vec<String> = if trim_whitespace {
            headers.iter().map(|h| h.trim().to_string()).collect()
        } else {
            headers.iter().map(|h| h.to_string()).collect()
        };

        let num_columns = cleaned_headers.len();

        progress.report(20, "Cleaning records...");

        // --- Process each data row ---
        let mut cleaned_rows: Vec<Vec<String>> = Vec::new();
        let mut original_row_count: usize = 0;

        for result in reader.records() {
            original_row_count += 1;

            // Skip malformed rows rather than failing the entire operation.
            let record = match result {
                Ok(rec) => rec,
                Err(_) => continue,
            };

            // Build cleaned row, optionally trimming whitespace.
            let mut row: Vec<String> = record
                .iter()
                .map(|cell| {
                    if trim_whitespace {
                        cell.trim().to_string()
                    } else {
                        cell.to_string()
                    }
                })
                .collect();

            // Normalize row width to match header (pad short rows, truncate long ones).
            while row.len() < num_columns {
                row.push(String::new());
            }
            row.truncate(num_columns);

            // Skip empty rows (all cells blank after trimming).
            if remove_empty_rows {
                let is_empty = row.iter().all(|cell| cell.is_empty());
                if is_empty {
                    continue;
                }
            }

            cleaned_rows.push(row);
        }

        progress.report(60, "Removing duplicates...");

        // --- Deduplicate rows using a HashSet ---
        // Rows are joined with null bytes as keys (null bytes can't appear in CSV text,
        // so "a\0b" won't collide with "a" + "\0b").
        let mut duplicates_removed: usize = 0;

        if remove_duplicates {
            let mut seen = std::collections::HashSet::new();
            let before_dedup = cleaned_rows.len();

            cleaned_rows.retain(|row| {
                let key = row.join("\0");
                seen.insert(key)
            });

            duplicates_removed = before_dedup - cleaned_rows.len();
        }

        progress.report(80, "Writing cleaned CSV...");

        // --- Write output CSV ---
        let mut writer = csv::WriterBuilder::new().from_writer(Vec::new());

        writer.write_record(&cleaned_headers).map_err(|e| {
            BntoError::ProcessingFailed(format!("Failed to write CSV headers: {e}"))
        })?;

        for row in &cleaned_rows {
            writer.write_record(row).map_err(|e| {
                BntoError::ProcessingFailed(format!("Failed to write CSV row: {e}"))
            })?;
        }

        let output_bytes = writer.into_inner().map_err(|e| {
            BntoError::ProcessingFailed(format!("Failed to finalize CSV output: {e}"))
        })?;

        progress.report(90, "Building result...");

        // --- Build metadata for the UI results panel ---
        let rows_removed = original_row_count - cleaned_rows.len();
        let output_filename = generate_output_filename(&input.filename);

        let mut metadata = serde_json::Map::new();
        metadata.insert(
            "originalRows".to_string(),
            serde_json::Value::Number(serde_json::Number::from(original_row_count)),
        );
        metadata.insert(
            "cleanedRows".to_string(),
            serde_json::Value::Number(serde_json::Number::from(cleaned_rows.len())),
        );
        metadata.insert(
            "rowsRemoved".to_string(),
            serde_json::Value::Number(serde_json::Number::from(rows_removed)),
        );
        metadata.insert(
            "duplicatesRemoved".to_string(),
            serde_json::Value::Number(serde_json::Number::from(duplicates_removed)),
        );

        progress.report(100, "Done!");

        Ok(NodeOutput {
            files: vec![OutputFile {
                data: output_bytes,
                filename: output_filename,
                mime_type: "text/csv".to_string(),
            }],
            metadata,
        })
    }
}

// =============================================================================
// Helper Functions
// =============================================================================

/// Generate an output filename by adding "-cleaned" before the extension.
///
/// "data.csv" -> "data-cleaned.csv", "report" -> "report-cleaned"
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
// Tests
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
        #[allow(clippy::default_constructed_unit_structs)]
        let _processor = CleanCsv::default();
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

        // NodeOutput doesn't impl Debug, so use `if let` instead of `unwrap_err()`.
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

        // NodeOutput doesn't impl Debug, so use `if let` instead of `unwrap_err()`.
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
