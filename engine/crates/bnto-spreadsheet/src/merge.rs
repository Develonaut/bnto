// Merge CSV Node — combine multiple CSV files into one.
//
// Batch processor: receives all files at once, merges rows into a single
// output CSV. Supports header reconciliation (first-file vs union) and
// optional deduplication of rows across files.

use bnto_core::context::ProcessContext;
use bnto_core::errors::BntoError;
use bnto_core::processor::{BatchInput, NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;

/// The spreadsheet-merge node processor. Stateless — config comes from params.
pub struct MergeSpreadsheets;

impl MergeSpreadsheets {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MergeSpreadsheets {
    fn default() -> Self {
        Self::new()
    }
}

// --- NodeProcessor Implementation ---

impl NodeProcessor for MergeSpreadsheets {
    fn name(&self) -> &str {
        "spreadsheet-merge"
    }

    fn metadata(&self) -> bnto_core::NodeMetadata {
        use bnto_core::metadata::*;
        NodeMetadata {
            node_type: "spreadsheet-merge".to_string(),
            name: "Merge CSV".to_string(),
            description:
                "Combine multiple CSV files into one, with header reconciliation and deduplication."
                    .to_string(),
            category: NodeCategory::Spreadsheet,
            accepts: vec!["text/csv".to_string()],
            platforms: vec!["browser".to_string()],
            parameters: build_merge_parameters(),
            input_cardinality: InputCardinality::Batch,
            requires: vec![],
        }
    }

    /// Single-file process — not the primary path for a batch processor, but
    /// required by the trait. Returns the file unchanged.
    fn process(
        &self,
        input: NodeInput,
        _progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        Ok(NodeOutput {
            files: vec![OutputFile {
                data: input.data,
                filename: input.filename,
                mime_type: input.mime_type.unwrap_or_else(|| "text/csv".to_string()),
            }],
            metadata: serde_json::Map::new(),
        })
    }

    /// Merge all input CSV files into a single output.
    fn process_batch(
        &self,
        input: BatchInput,
        progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        if input.files.is_empty() {
            return Err(BntoError::InvalidInput(
                "No files to merge — need at least one CSV file".to_string(),
            ));
        }

        let config = MergeConfig::from_params(&input.params);

        progress.report(0, "Parsing CSV files...");
        let parsed = parse_all_files(&input)?;

        progress.report(30, "Reconciling headers...");
        let merged_headers = reconcile_headers(&parsed, &config);

        progress.report(50, "Merging rows...");
        let mut all_rows = collect_all_rows(&parsed, &merged_headers)?;
        let total_input_rows = all_rows.len();

        progress.report(70, "Deduplicating...");
        let duplicates_removed = deduplicate_rows(&mut all_rows, config.deduplicate);

        progress.report(90, "Writing merged CSV...");
        let output_bytes = write_csv_output(&merged_headers, &all_rows)?;

        let metadata = build_merge_metadata(
            input.files.len(),
            total_input_rows,
            all_rows.len(),
            duplicates_removed,
        );

        progress.report(100, "Done!");
        Ok(NodeOutput {
            files: vec![OutputFile {
                data: output_bytes,
                filename: "merged.csv".to_string(),
                mime_type: "text/csv".to_string(),
            }],
            metadata,
        })
    }
}

// --- Configuration ---

struct MergeConfig {
    header_handling: HeaderHandling,
    deduplicate: bool,
}

enum HeaderHandling {
    /// Use headers from the first file. Other files' rows are mapped by position.
    FirstFile,
    /// Union of all headers across files. Missing columns get empty values.
    Union,
}

impl MergeConfig {
    fn from_params(params: &serde_json::Map<String, serde_json::Value>) -> Self {
        let header_handling = match params.get("headerHandling").and_then(|v| v.as_str()) {
            Some("union") => HeaderHandling::Union,
            _ => HeaderHandling::FirstFile,
        };

        let deduplicate = params
            .get("deduplicate")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        Self {
            header_handling,
            deduplicate,
        }
    }
}

// --- Metadata Parameter Definitions ---

fn build_merge_parameters() -> Vec<bnto_core::metadata::ParameterDef> {
    use bnto_core::metadata::*;
    vec![
        ParameterDef {
            name: "headerHandling".to_string(),
            label: "Header Handling".to_string(),
            description: "How to reconcile headers across files".to_string(),
            param_type: ParameterType::Enum {
                options: vec!["first-file".to_string(), "union".to_string()],
            },
            default: Some(serde_json::json!("first-file")),
            ..Default::default()
        },
        ParameterDef {
            name: "deduplicate".to_string(),
            label: "Remove Duplicates".to_string(),
            description: "Remove duplicate rows across all files".to_string(),
            param_type: ParameterType::Boolean,
            default: Some(serde_json::json!(false)),
            ..Default::default()
        },
    ]
}

// --- Parsed File ---

struct ParsedCsv {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

/// Parse all batch files into header+rows structures.
fn parse_all_files(input: &BatchInput) -> Result<Vec<ParsedCsv>, BntoError> {
    let mut parsed = Vec::with_capacity(input.files.len());

    for file in &input.files {
        let csv_text = std::str::from_utf8(&file.data).map_err(|e| {
            BntoError::InvalidInput(format!("File '{}' is not valid UTF-8: {e}", file.filename))
        })?;

        if csv_text.trim().is_empty() {
            return Err(BntoError::InvalidInput(format!(
                "File '{}' is empty",
                file.filename
            )));
        }

        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .flexible(true)
            .from_reader(csv_text.as_bytes());

        let headers: Vec<String> = reader
            .headers()
            .map_err(|e| {
                BntoError::ProcessingFailed(format!(
                    "Failed to read headers from '{}': {e}",
                    file.filename
                ))
            })?
            .iter()
            .map(|h| h.trim().to_string())
            .collect();

        let mut rows = Vec::new();
        for result in reader.records() {
            let record = match result {
                Ok(rec) => rec,
                Err(_) => continue,
            };
            let row: Vec<String> = record.iter().map(|c| c.trim().to_string()).collect();
            rows.push(row);
        }

        parsed.push(ParsedCsv { headers, rows });
    }

    Ok(parsed)
}

// --- Header Reconciliation ---

/// Determine the merged header set based on the config strategy.
fn reconcile_headers(parsed: &[ParsedCsv], config: &MergeConfig) -> Vec<String> {
    match config.header_handling {
        HeaderHandling::FirstFile => parsed
            .first()
            .map(|p| p.headers.clone())
            .unwrap_or_default(),
        HeaderHandling::Union => {
            let mut seen = std::collections::HashSet::new();
            let mut headers = Vec::new();
            for file in parsed {
                for header in &file.headers {
                    if seen.insert(header.clone()) {
                        headers.push(header.clone());
                    }
                }
            }
            headers
        }
    }
}

// --- Row Collection ---

/// Map each file's rows to the merged header order.
///
/// For union mode: maps by header name (missing columns → empty).
/// For first-file mode: if headers match by name, map by name. Otherwise
/// fall back to positional mapping (column 0 → column 0, etc.).
fn collect_all_rows(
    parsed: &[ParsedCsv],
    merged_headers: &[String],
) -> Result<Vec<Vec<String>>, BntoError> {
    let mut all_rows = Vec::new();

    for file in parsed {
        // Check if this file's headers overlap with merged headers by name.
        let has_name_match = file.headers.iter().any(|h| merged_headers.contains(h));

        for row in &file.rows {
            let mapped: Vec<String> = if has_name_match {
                // Map by header name
                let col_map: Vec<Option<usize>> = merged_headers
                    .iter()
                    .map(|h| file.headers.iter().position(|fh| fh == h))
                    .collect();
                col_map
                    .iter()
                    .map(|idx| idx.and_then(|i| row.get(i)).cloned().unwrap_or_default())
                    .collect()
            } else {
                // No header overlap — map by position, pad/truncate to match
                let mut positional: Vec<String> = merged_headers
                    .iter()
                    .enumerate()
                    .map(|(i, _)| row.get(i).cloned().unwrap_or_default())
                    .collect();
                positional.truncate(merged_headers.len());
                positional
            };
            all_rows.push(mapped);
        }
    }

    Ok(all_rows)
}

// --- Deduplication ---

/// Remove duplicate rows in-place. Returns count removed.
fn deduplicate_rows(rows: &mut Vec<Vec<String>>, enabled: bool) -> usize {
    if !enabled {
        return 0;
    }

    let mut seen = std::collections::HashSet::new();
    let before = rows.len();
    rows.retain(|row| seen.insert(row.join("\0")));
    before - rows.len()
}

// --- CSV Output ---

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

fn build_merge_metadata(
    files_merged: usize,
    total_input_rows: usize,
    output_rows: usize,
    duplicates_removed: usize,
) -> serde_json::Map<String, serde_json::Value> {
    let mut metadata = serde_json::Map::new();
    metadata.insert(
        "filesMerged".to_string(),
        serde_json::Value::Number(files_merged.into()),
    );
    metadata.insert(
        "totalInputRows".to_string(),
        serde_json::Value::Number(total_input_rows.into()),
    );
    metadata.insert(
        "outputRows".to_string(),
        serde_json::Value::Number(output_rows.into()),
    );
    metadata.insert(
        "duplicatesRemoved".to_string(),
        serde_json::Value::Number(duplicates_removed.into()),
    );
    metadata
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::NoopContext;
    use bnto_core::processor::BatchFile;

    // --- Test Helpers ---

    fn make_batch_input(files: Vec<(&str, &str)>) -> BatchInput {
        BatchInput {
            files: files
                .into_iter()
                .map(|(name, content)| BatchFile {
                    data: content.as_bytes().to_vec(),
                    filename: name.to_string(),
                    mime_type: Some("text/csv".to_string()),
                })
                .collect(),
            params: serde_json::Map::new(),
        }
    }

    fn make_batch_with_params(
        files: Vec<(&str, &str)>,
        params: serde_json::Map<String, serde_json::Value>,
    ) -> BatchInput {
        BatchInput {
            files: files
                .into_iter()
                .map(|(name, content)| BatchFile {
                    data: content.as_bytes().to_vec(),
                    filename: name.to_string(),
                    mime_type: Some("text/csv".to_string()),
                })
                .collect(),
            params,
        }
    }

    fn output_csv_text(output: &NodeOutput) -> String {
        String::from_utf8(output.files[0].data.clone()).expect("Output should be valid UTF-8")
    }

    fn count_data_rows(csv_text: &str) -> usize {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .flexible(true)
            .from_reader(csv_text.as_bytes());
        reader.records().count()
    }

    // --- Trait Tests ---

    #[test]
    fn test_name_returns_merge_csv() {
        let processor = MergeSpreadsheets::new();
        assert_eq!(processor.name(), "spreadsheet-merge");
    }

    #[test]
    fn test_metadata_has_batch_cardinality() {
        let processor = MergeSpreadsheets::new();
        let meta = processor.metadata();
        assert_eq!(
            meta.input_cardinality,
            bnto_core::metadata::InputCardinality::Batch
        );
        assert_eq!(meta.node_type, "spreadsheet-merge");
    }

    #[test]
    fn test_metadata_has_two_parameters() {
        let processor = MergeSpreadsheets::new();
        let meta = processor.metadata();
        assert_eq!(meta.parameters.len(), 2);
        let names: Vec<&str> = meta.parameters.iter().map(|p| p.name.as_str()).collect();
        assert!(names.contains(&"headerHandling"));
        assert!(names.contains(&"deduplicate"));
    }

    #[test]
    fn test_default_creates_same_as_new() {
        #[allow(clippy::default_constructed_unit_structs)]
        let _processor = MergeSpreadsheets::default();
    }

    #[test]
    fn test_validate_passes_with_no_params() {
        let processor = MergeSpreadsheets::new();
        let errors = processor.validate(&serde_json::Map::new());
        assert!(errors.is_empty());
    }

    // --- Basic Merge Tests ---

    #[test]
    fn test_merge_two_files_same_headers() {
        let processor = MergeSpreadsheets::new();
        let progress = ProgressReporter::new_noop();
        let input = make_batch_input(vec![
            ("a.csv", "name,age\nAlice,30\nBob,25\n"),
            ("b.csv", "name,age\nCharlie,35\nDiana,28\n"),
        ]);

        let output = processor
            .process_batch(input, &progress, &NoopContext)
            .unwrap();
        let text = output_csv_text(&output);
        let rows = count_data_rows(&text);

        assert_eq!(rows, 4, "Should merge all 4 rows");
        assert!(text.contains("Alice"));
        assert!(text.contains("Bob"));
        assert!(text.contains("Charlie"));
        assert!(text.contains("Diana"));
        assert_eq!(output.files[0].filename, "merged.csv");
        assert_eq!(output.files[0].mime_type, "text/csv");
    }

    #[test]
    fn test_merge_single_file() {
        let processor = MergeSpreadsheets::new();
        let progress = ProgressReporter::new_noop();
        let input = make_batch_input(vec![("a.csv", "name,age\nAlice,30\n")]);

        let output = processor
            .process_batch(input, &progress, &NoopContext)
            .unwrap();
        let text = output_csv_text(&output);
        let rows = count_data_rows(&text);

        assert_eq!(rows, 1);
        assert!(text.contains("Alice"));
    }

    #[test]
    fn test_merge_three_files() {
        let processor = MergeSpreadsheets::new();
        let progress = ProgressReporter::new_noop();
        let input = make_batch_input(vec![
            ("a.csv", "id\n1\n2\n"),
            ("b.csv", "id\n3\n4\n"),
            ("c.csv", "id\n5\n"),
        ]);

        let output = processor
            .process_batch(input, &progress, &NoopContext)
            .unwrap();
        let text = output_csv_text(&output);
        let rows = count_data_rows(&text);

        assert_eq!(rows, 5, "Should merge rows from all 3 files");
    }

    // --- Empty Input ---

    #[test]
    fn test_merge_empty_batch_returns_error() {
        let processor = MergeSpreadsheets::new();
        let progress = ProgressReporter::new_noop();
        let input = BatchInput {
            files: vec![],
            params: serde_json::Map::new(),
        };

        let result = processor.process_batch(input, &progress, &NoopContext);
        assert!(result.is_err());
    }

    #[test]
    fn test_merge_empty_file_returns_error() {
        let processor = MergeSpreadsheets::new();
        let progress = ProgressReporter::new_noop();
        let input = make_batch_input(vec![("a.csv", "")]);

        let result = processor.process_batch(input, &progress, &NoopContext);
        assert!(result.is_err());
    }

    // --- Header Handling: first-file (default) ---

    #[test]
    fn test_first_file_headers_columns_matched_by_position() {
        let processor = MergeSpreadsheets::new();
        let progress = ProgressReporter::new_noop();
        // Second file has different header names — with first-file mode,
        // rows map by column position, headers come from first file.
        let input = make_batch_input(vec![
            ("a.csv", "name,age\nAlice,30\n"),
            ("b.csv", "nombre,edad\nCarlos,40\n"),
        ]);

        let output = processor
            .process_batch(input, &progress, &NoopContext)
            .unwrap();
        let text = output_csv_text(&output);

        // Headers should be from first file
        assert!(text.starts_with("name,age"), "Headers from first file");
        assert_eq!(count_data_rows(&text), 2);
        assert!(text.contains("Carlos"));
    }

    // --- Header Handling: union ---

    #[test]
    fn test_union_headers_includes_all_columns() {
        let processor = MergeSpreadsheets::new();
        let progress = ProgressReporter::new_noop();
        let mut params = serde_json::Map::new();
        params.insert("headerHandling".to_string(), serde_json::json!("union"));
        let input = make_batch_with_params(
            vec![
                ("a.csv", "name,age\nAlice,30\n"),
                ("b.csv", "name,city\nBob,NYC\n"),
            ],
            params,
        );

        let output = processor
            .process_batch(input, &progress, &NoopContext)
            .unwrap();
        let text = output_csv_text(&output);

        // Union headers: name, age, city
        let first_line = text.lines().next().unwrap();
        assert!(first_line.contains("name"));
        assert!(first_line.contains("age"));
        assert!(first_line.contains("city"));

        assert_eq!(count_data_rows(&text), 2);
        // Alice should have empty city, Bob should have empty age
        assert!(text.contains("Alice"));
        assert!(text.contains("Bob"));
    }

    // --- Deduplication ---

    #[test]
    fn test_deduplicate_removes_cross_file_duplicates() {
        let processor = MergeSpreadsheets::new();
        let progress = ProgressReporter::new_noop();
        let mut params = serde_json::Map::new();
        params.insert("deduplicate".to_string(), serde_json::json!(true));
        let input = make_batch_with_params(
            vec![
                ("a.csv", "name,age\nAlice,30\nBob,25\n"),
                ("b.csv", "name,age\nAlice,30\nCharlie,35\n"),
            ],
            params,
        );

        let output = processor
            .process_batch(input, &progress, &NoopContext)
            .unwrap();
        let text = output_csv_text(&output);
        let rows = count_data_rows(&text);

        assert_eq!(rows, 3, "Should have 3 unique rows (Alice, Bob, Charlie)");
        let dupes = output
            .metadata
            .get("duplicatesRemoved")
            .unwrap()
            .as_u64()
            .unwrap();
        assert_eq!(dupes, 1);
    }

    #[test]
    fn test_deduplicate_false_keeps_duplicates() {
        let processor = MergeSpreadsheets::new();
        let progress = ProgressReporter::new_noop();
        let input = make_batch_input(vec![("a.csv", "name\nAlice\n"), ("b.csv", "name\nAlice\n")]);

        let output = processor
            .process_batch(input, &progress, &NoopContext)
            .unwrap();
        let text = output_csv_text(&output);
        let rows = count_data_rows(&text);

        assert_eq!(rows, 2, "Dedup off by default — keep both Alices");
    }

    // --- Parameterized: different params produce different output ---

    #[test]
    fn test_different_header_handling_produces_different_output() {
        let processor = MergeSpreadsheets::new();
        let progress = ProgressReporter::new_noop();

        let files = vec![
            ("a.csv", "name,age\nAlice,30\n"),
            ("b.csv", "name,city\nBob,NYC\n"),
        ];

        // First-file mode
        let input1 = make_batch_input(files.clone());
        let out1 = processor
            .process_batch(input1, &progress, &NoopContext)
            .unwrap();

        // Union mode
        let mut params = serde_json::Map::new();
        params.insert("headerHandling".to_string(), serde_json::json!("union"));
        let input2 = make_batch_with_params(files, params);
        let out2 = processor
            .process_batch(input2, &progress, &NoopContext)
            .unwrap();

        // Outputs should differ — union has more columns
        assert_ne!(out1.files[0].data, out2.files[0].data);
    }

    #[test]
    fn test_different_deduplicate_produces_different_output() {
        let processor = MergeSpreadsheets::new();
        let progress = ProgressReporter::new_noop();

        let files = vec![("a.csv", "name\nAlice\n"), ("b.csv", "name\nAlice\n")];

        // Without dedup
        let input1 = make_batch_input(files.clone());
        let out1 = processor
            .process_batch(input1, &progress, &NoopContext)
            .unwrap();

        // With dedup
        let mut params = serde_json::Map::new();
        params.insert("deduplicate".to_string(), serde_json::json!(true));
        let input2 = make_batch_with_params(files, params);
        let out2 = processor
            .process_batch(input2, &progress, &NoopContext)
            .unwrap();

        assert_ne!(
            count_data_rows(&output_csv_text(&out1)),
            count_data_rows(&output_csv_text(&out2)),
        );
    }

    // --- Metadata ---

    #[test]
    fn test_output_metadata_fields() {
        let processor = MergeSpreadsheets::new();
        let progress = ProgressReporter::new_noop();
        let input = make_batch_input(vec![
            ("a.csv", "name\nAlice\nBob\n"),
            ("b.csv", "name\nCharlie\n"),
        ]);

        let output = processor
            .process_batch(input, &progress, &NoopContext)
            .unwrap();

        assert_eq!(
            output
                .metadata
                .get("filesMerged")
                .unwrap()
                .as_u64()
                .unwrap(),
            2
        );
        assert_eq!(
            output
                .metadata
                .get("totalInputRows")
                .unwrap()
                .as_u64()
                .unwrap(),
            3
        );
        assert_eq!(
            output.metadata.get("outputRows").unwrap().as_u64().unwrap(),
            3
        );
        assert_eq!(
            output
                .metadata
                .get("duplicatesRemoved")
                .unwrap()
                .as_u64()
                .unwrap(),
            0
        );
    }

    // --- Error Handling ---

    #[test]
    fn test_non_utf8_file_returns_error() {
        let processor = MergeSpreadsheets::new();
        let progress = ProgressReporter::new_noop();
        let input = BatchInput {
            files: vec![BatchFile {
                data: vec![0xFF, 0xFE, 0x00],
                filename: "bad.csv".to_string(),
                mime_type: Some("text/csv".to_string()),
            }],
            params: serde_json::Map::new(),
        };

        let result = processor.process_batch(input, &progress, &NoopContext);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("UTF-8"));
        }
    }

    // --- Edge Cases ---

    #[test]
    fn test_merge_files_with_varying_column_counts() {
        let processor = MergeSpreadsheets::new();
        let progress = ProgressReporter::new_noop();
        // First file has 2 cols, second has 3 — first-file mode uses 2 cols
        let input = make_batch_input(vec![
            ("a.csv", "name,age\nAlice,30\n"),
            ("b.csv", "name,age,city\nBob,25,NYC\n"),
        ]);

        let output = processor
            .process_batch(input, &progress, &NoopContext)
            .unwrap();
        let text = output_csv_text(&output);
        let rows = count_data_rows(&text);

        assert_eq!(rows, 2);
        // With first-file headers (name, age), Bob's city gets dropped
        let header_line = text.lines().next().unwrap();
        assert_eq!(header_line, "name,age");
    }

    #[test]
    fn test_merge_preserves_row_order() {
        let processor = MergeSpreadsheets::new();
        let progress = ProgressReporter::new_noop();
        let input = make_batch_input(vec![("a.csv", "id\n1\n2\n"), ("b.csv", "id\n3\n4\n")]);

        let output = processor
            .process_batch(input, &progress, &NoopContext)
            .unwrap();
        let text = output_csv_text(&output);

        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(text.as_bytes());
        let ids: Vec<String> = reader
            .records()
            .filter_map(|r| r.ok())
            .map(|r| r.get(0).unwrap_or("").to_string())
            .collect();

        assert_eq!(ids, vec!["1", "2", "3", "4"]);
    }

    #[test]
    fn test_merge_with_test_fixtures() {
        let processor = MergeSpreadsheets::new();
        let progress = ProgressReporter::new_noop();
        let csv1 = include_bytes!("../../../../test-fixtures/csv/simple.csv");
        let csv2 = include_bytes!("../../../../test-fixtures/csv/simple.csv");

        let input = BatchInput {
            files: vec![
                BatchFile {
                    data: csv1.to_vec(),
                    filename: "simple1.csv".to_string(),
                    mime_type: Some("text/csv".to_string()),
                },
                BatchFile {
                    data: csv2.to_vec(),
                    filename: "simple2.csv".to_string(),
                    mime_type: Some("text/csv".to_string()),
                },
            ],
            params: serde_json::Map::new(),
        };

        let output = processor
            .process_batch(input, &progress, &NoopContext)
            .unwrap();
        let text = output_csv_text(&output);
        let rows = count_data_rows(&text);

        // simple.csv has 5 data rows, merging two copies = 10 rows
        assert_eq!(
            rows, 10,
            "Should have 10 rows from two copies of simple.csv"
        );
    }

    #[test]
    fn test_merge_with_dedup_and_fixtures() {
        let processor = MergeSpreadsheets::new();
        let progress = ProgressReporter::new_noop();
        let csv_data = include_bytes!("../../../../test-fixtures/csv/simple.csv");

        let mut params = serde_json::Map::new();
        params.insert("deduplicate".to_string(), serde_json::json!(true));

        let input = BatchInput {
            files: vec![
                BatchFile {
                    data: csv_data.to_vec(),
                    filename: "simple1.csv".to_string(),
                    mime_type: Some("text/csv".to_string()),
                },
                BatchFile {
                    data: csv_data.to_vec(),
                    filename: "simple2.csv".to_string(),
                    mime_type: Some("text/csv".to_string()),
                },
            ],
            params,
        };

        let output = processor
            .process_batch(input, &progress, &NoopContext)
            .unwrap();
        let text = output_csv_text(&output);
        let rows = count_data_rows(&text);

        // Deduplicated: should have 5 unique rows (all duplicates removed)
        assert_eq!(rows, 5, "Should have 5 unique rows after dedup");
        let dupes = output
            .metadata
            .get("duplicatesRemoved")
            .unwrap()
            .as_u64()
            .unwrap();
        assert_eq!(dupes, 5, "Should remove 5 duplicates");
    }
}
