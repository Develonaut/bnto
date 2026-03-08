// =============================================================================
// Clean CSV Node — Remove Empty Rows, Trim Whitespace, Deduplicate
// =============================================================================
//
// WHAT IS THIS FILE?
// This is the `clean-csv` node. When a user drops a CSV file on the
// /clean-csv page and clicks "Run", this code does the actual work.
//
// WHAT DOES "CLEANING" A CSV MEAN?
//
//   1. TRIM WHITESPACE: Remove leading/trailing spaces from every cell.
//      "  Alice  " becomes "Alice". This is the most common CSV problem —
//      data exported from spreadsheets often has invisible whitespace.
//
//   2. REMOVE EMPTY ROWS: Skip rows where every cell is blank (after
//      trimming). These are usually accidents from copy-pasting or
//      exporting from spreadsheets with empty rows at the bottom.
//
//   3. REMOVE DUPLICATES: If two rows have the exact same values in
//      every column, keep only the first one. This is order-preserving —
//      the first occurrence wins, later duplicates are dropped.
//
// Each of these operations is controlled by a boolean parameter so users
// can enable/disable them individually.
//
// HOW THE CSV CRATE WORKS:
//
//   The `csv` crate (https://docs.rs/csv) is a high-performance CSV
//   parser for Rust. It handles:
//     - Quoted fields ("field with, comma")
//     - Escaped quotes ("field with ""quote""")
//     - Different line endings (CRLF on Windows, LF on Mac/Linux)
//     - Variable-length rows (some rows have more columns than others)
//
//   We use `ReaderBuilder` to configure the parser and `WriterBuilder`
//   to configure the output writer. The crate operates on byte streams
//   (not strings), which is perfect for WASM since we receive raw bytes.

use bnto_core::errors::BntoError;
use bnto_core::processor::{NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;

// =============================================================================
// CleanCsv — The Node Processor Struct
// =============================================================================

/// The clean-csv node processor.
///
/// RUST CONCEPT: Unit struct
/// `CleanCsv` has no fields — it's a "unit struct", like an empty class.
/// It exists so we can implement the `NodeProcessor` trait on it. The
/// actual configuration comes from `NodeInput.params`, not from the struct.
/// Think of it as a stateless function handler that implements an interface.
pub struct CleanCsv;

// RUST CONCEPT: `impl` block
// An `impl` block adds methods to a struct. It's like defining methods
// on a class in JavaScript. Here we add `new()` which is the Rust
// convention for a constructor (Rust doesn't have a special `new` keyword).
impl CleanCsv {
    /// Create a new CleanCsv processor.
    ///
    /// Since CleanCsv has no fields (it's stateless), this is trivial.
    /// We still provide it for consistency with other node processors
    /// (like CompressImages, ResizeImages) that also have `::new()`.
    pub fn new() -> Self {
        // RUST CONCEPT: `Self`
        // `Self` is a shorthand for the type we're inside (`CleanCsv`).
        // `Self` with no fields = create a CleanCsv unit struct.
        Self
    }
}

// RUST CONCEPT: `Default` trait
// The `Default` trait provides a `default()` method that creates a
// "default" instance. For our unit struct, it's the same as `new()`.
// Implementing Default lets other code write `CleanCsv::default()`
// and satisfies Clippy's `new_without_default` lint.
impl Default for CleanCsv {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// NodeProcessor Implementation — The Core Logic
// =============================================================================
//
// RUST CONCEPT: `impl Trait for Type`
// This implements the `NodeProcessor` trait (interface) for `CleanCsv`.
// The compiler verifies we've implemented all required methods:
//   - `name()` — returns the node type name
//   - `process()` — does the actual CSV cleaning work

impl NodeProcessor for CleanCsv {
    /// Returns the unique name of this node type.
    /// Used for progress reporting and logging.
    fn name(&self) -> &str {
        "clean-csv"
    }

    /// Return self-describing metadata for the clean-csv processor.
    ///
    /// Parameters: trimWhitespace, removeEmptyRows, removeDuplicates — all
    /// booleans, all default to true. Accepts text/csv files only.
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
                    constraints: None,
                },
                ParameterDef {
                    name: "removeEmptyRows".to_string(),
                    label: "Remove Empty Rows".to_string(),
                    description: "Skip rows where every cell is blank".to_string(),
                    param_type: ParameterType::Boolean,
                    default: Some(serde_json::json!(true)),
                    constraints: None,
                },
                ParameterDef {
                    name: "removeDuplicates".to_string(),
                    label: "Remove Duplicates".to_string(),
                    description: "Remove duplicate rows, keeping the first occurrence".to_string(),
                    param_type: ParameterType::Boolean,
                    default: Some(serde_json::json!(true)),
                    constraints: None,
                },
            ],
        }
    }

    /// Clean a CSV file: trim whitespace, remove empty rows, deduplicate.
    ///
    /// This is the main entry point. The Web Worker calls this (via the
    /// WASM bridge) with the raw CSV bytes and configuration parameters.
    ///
    /// PARAMETERS (from `input.params`):
    ///   - `trimWhitespace` (bool, default: true) — trim leading/trailing
    ///     whitespace from every cell
    ///   - `removeEmptyRows` (bool, default: true) — skip rows where all
    ///     cells are empty (after trimming)
    ///   - `removeDuplicates` (bool, default: true) — remove duplicate
    ///     rows, keeping the first occurrence
    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
    ) -> Result<NodeOutput, BntoError> {
        // --- Step 1: Report that we're starting ---
        progress.report(0, "Parsing CSV...");

        // --- Step 2: Read configuration parameters ---
        //
        // Extract boolean params from the JSON config. If a param is
        // missing or not a boolean, we use the default value (true).
        //
        // RUST CONCEPT: `.get("key")` on a Map
        // Returns `Option<&Value>` — either `Some(&value)` if the key
        // exists, or `None` if it doesn't. We chain `.and_then()` to
        // try converting it to a bool, and `.unwrap_or(true)` to fall
        // back to the default if anything fails.
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

        // --- Step 3: Convert bytes to a UTF-8 string ---
        //
        // CSV is a text format, so we need to convert the raw bytes
        // to a Rust string. This can fail if the bytes contain invalid
        // UTF-8 sequences (e.g., a binary file was uploaded by mistake).
        //
        // RUST CONCEPT: `std::str::from_utf8()`
        // Tries to interpret a byte slice as a UTF-8 string. Returns
        // `Ok(&str)` if valid, `Err(Utf8Error)` if not.
        //
        // RUST CONCEPT: `.map_err(|e| ...)`
        // If the Result is `Err`, transform the error into our error type.
        // If it's `Ok`, pass through unchanged. This is how we convert
        // third-party errors into our `BntoError` type.
        let csv_text = std::str::from_utf8(&input.data).map_err(|e| {
            BntoError::InvalidInput(format!(
                "File is not valid UTF-8 text (is this really a CSV?): {e}"
            ))
        })?;

        // --- Step 4: Check for empty input ---
        //
        // If the file is completely empty (no bytes at all), there's
        // nothing to clean. Return an error rather than silently producing
        // an empty file.
        if csv_text.trim().is_empty() {
            return Err(BntoError::InvalidInput(
                "CSV file is empty — no data to clean".to_string(),
            ));
        }

        progress.report(10, "Reading CSV records...");

        // --- Step 5: Parse the CSV using the csv crate ---
        //
        // `ReaderBuilder` configures how the CSV parser works:
        //   - `has_headers(true)` — the first row is treated as column headers
        //   - `flexible(true)` — rows can have different numbers of fields
        //     (normally the csv crate returns an error if a row has more or
        //     fewer fields than the header)
        //
        // `.from_reader()` takes anything that implements `Read` — we give
        // it the byte slice of our CSV text.
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .flexible(true)
            .from_reader(csv_text.as_bytes());

        // --- Step 6: Read the header row ---
        //
        // `reader.headers()` returns the first row as a `StringRecord`.
        // We clone it because we need to own the headers (the reader
        // borrows them internally and we need them after processing).
        //
        // RUST CONCEPT: `.clone()`
        // Creates a deep copy. We need a copy because `reader.headers()`
        // returns a reference (`&StringRecord`) that borrows from `reader`.
        // We can't keep the reference while also iterating `reader.records()`.
        let headers = reader
            .headers()
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to read CSV headers: {e}")))?
            .clone();

        // If we're trimming, trim the header cells too.
        // We build a new header record with trimmed values.
        let cleaned_headers: Vec<String> = if trim_whitespace {
            headers.iter().map(|h| h.trim().to_string()).collect()
        } else {
            headers.iter().map(|h| h.to_string()).collect()
        };

        // Track the number of columns from the header for padding.
        let num_columns = cleaned_headers.len();

        progress.report(20, "Cleaning records...");

        // --- Step 7: Process each data row ---
        //
        // We iterate through all records, apply our cleaning operations,
        // and collect the results into a Vec (dynamic array).
        //
        // RUST CONCEPT: `Vec<Vec<String>>`
        // A vector of vectors of strings. Each inner Vec is one row,
        // where each String is one cell value. Like `string[][]` in TS.
        let mut cleaned_rows: Vec<Vec<String>> = Vec::new();

        // Track the original row count for metadata.
        let mut original_row_count: usize = 0;

        // RUST CONCEPT: `reader.records()`
        // Returns an iterator over the data rows (skipping the header).
        // Each item is `Result<StringRecord, csv::Error>` because parsing
        // can fail for individual rows (e.g., unmatched quotes).
        for result in reader.records() {
            // Count every row we encounter (even ones we'll skip).
            original_row_count += 1;

            // Parse the record. If a row is malformed, we skip it
            // rather than failing the entire operation.
            //
            // RUST CONCEPT: `match` expression
            // Like a switch statement but more powerful. It checks which
            // variant the Result is and runs the matching arm.
            let record = match result {
                Ok(rec) => rec,
                // If a row can't be parsed, skip it silently.
                // This is intentional — a messy CSV might have a few
                // unparseable rows mixed in with good data.
                Err(_) => continue,
            };

            // --- Step 7a: Build the cleaned row ---
            //
            // For each cell in the row, optionally trim whitespace.
            // Also pad the row to have the same number of columns as
            // the header (in case some rows have fewer fields).
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

            // Normalize row width to match the header:
            //   - If the row has FEWER columns, pad with empty strings.
            //   - If the row has MORE columns (e.g., ",," in a 2-col CSV
            //     creates 3 fields), truncate to match the header width.
            //
            // This ensures all rows have exactly `num_columns` fields,
            // which the CSV writer requires for consistent output.
            while row.len() < num_columns {
                row.push(String::new());
            }
            // RUST CONCEPT: `.truncate(n)`
            // Shortens a Vec to `n` elements, dropping any extras.
            // If the Vec already has `n` or fewer elements, it's a no-op.
            row.truncate(num_columns);

            // --- Step 7b: Check if the row is empty ---
            //
            // A row is "empty" if ALL cells are empty strings (after trimming).
            // We skip these rows if removeEmptyRows is enabled.
            if remove_empty_rows {
                // `.iter()` creates an iterator over the cells.
                // `.all()` returns true if the closure returns true for
                // EVERY item. So this checks "are all cells empty?".
                let is_empty = row.iter().all(|cell| cell.is_empty());
                if is_empty {
                    continue; // Skip this row entirely.
                }
            }

            // --- Step 7c: Collect the cleaned row ---
            cleaned_rows.push(row);
        }

        progress.report(60, "Removing duplicates...");

        // --- Step 8: Remove duplicate rows (if enabled) ---
        //
        // We use a HashSet to track which rows we've already seen.
        // For each row, we join all cells into a single string (separated
        // by a null byte, which can't appear in CSV text) and check if
        // we've seen that string before. If yes, skip it.
        //
        // RUST CONCEPT: `std::collections::HashSet`
        // A set of unique values. `.insert()` returns `true` if the
        // value was new, `false` if it was already in the set.
        let mut duplicates_removed: usize = 0;

        if remove_duplicates {
            let mut seen = std::collections::HashSet::new();
            let before_dedup = cleaned_rows.len();

            // `.retain()` keeps only the rows where the closure returns true.
            // It modifies the Vec in-place — no new allocation needed.
            cleaned_rows.retain(|row| {
                // Join all cells with a null byte separator to create a
                // unique key for this row. Null bytes can't appear in
                // normal text, so "a\0b" won't collide with "a" + "\0b".
                let key = row.join("\0");

                // `.insert()` returns true if the key was NEW (not seen before).
                // We keep the row only if it's the first time we've seen it.
                seen.insert(key)
            });

            duplicates_removed = before_dedup - cleaned_rows.len();
        }

        progress.report(80, "Writing cleaned CSV...");

        // --- Step 9: Write the cleaned data back to CSV format ---
        //
        // We use `csv::WriterBuilder` to create a CSV writer that writes
        // to an in-memory buffer (Vec<u8>). Then we write the header
        // followed by each cleaned row.
        let mut writer = csv::WriterBuilder::new().from_writer(Vec::new());

        // Write the header row first.
        //
        // RUST CONCEPT: `?` operator
        // The `?` at the end is shorthand for "if this returns Err,
        // immediately return that error from the current function".
        // It's like an automatic early return on error. Without `?`,
        // you'd need `match result { Ok(v) => v, Err(e) => return Err(e) }`.
        writer.write_record(&cleaned_headers).map_err(|e| {
            BntoError::ProcessingFailed(format!("Failed to write CSV headers: {e}"))
        })?;

        // Write each cleaned data row.
        for row in &cleaned_rows {
            writer.write_record(row).map_err(|e| {
                BntoError::ProcessingFailed(format!("Failed to write CSV row: {e}"))
            })?;
        }

        // Flush the writer and get the bytes out.
        //
        // `.into_inner()` consumes the writer and returns the underlying
        // Vec<u8> buffer. This can fail if there's a flush error.
        let output_bytes = writer.into_inner().map_err(|e| {
            BntoError::ProcessingFailed(format!("Failed to finalize CSV output: {e}"))
        })?;

        progress.report(90, "Building result...");

        // --- Step 10: Calculate the total rows removed ---
        let rows_removed = original_row_count - cleaned_rows.len();

        // --- Step 11: Generate the output filename ---
        //
        // We add "-cleaned" before the file extension, similar to how
        // the image compressor adds "-compressed".
        let output_filename = generate_output_filename(&input.filename);

        // --- Step 12: Build the metadata ---
        //
        // Metadata is a JSON map that the UI displays in the results panel.
        // It tells the user what happened to their file.
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

        // --- Step 13: Return the result ---
        //
        // Package up the cleaned CSV bytes, filename, and metadata
        // into a NodeOutput that the WASM bridge will return to JavaScript.
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
/// Examples:
///   "data.csv"       -> "data-cleaned.csv"
///   "report"         -> "report-cleaned"
///   "my.data.csv"    -> "my.data-cleaned.csv"
///
/// RUST CONCEPT: `&str` vs `String`
/// `&str` is a borrowed string slice (read-only view into string data).
/// `String` is an owned, heap-allocated string (like a JS string).
/// We take `&str` as input (cheap, no copy) and return `String`
/// (owned, because we're creating a new string).
fn generate_output_filename(original: &str) -> String {
    // RUST CONCEPT: `.rfind('.')`
    // Searches for the LAST occurrence of '.' in the string and returns
    // its index as `Option<usize>`. `rfind` = "reverse find".
    // We search from the right so "my.data.csv" splits at the LAST dot.
    if let Some(dot_pos) = original.rfind('.') {
        // Split into stem ("data") and extension (".csv"), then
        // insert "-cleaned" between them.
        let stem = &original[..dot_pos];
        let ext = &original[dot_pos..];
        format!("{stem}-cleaned{ext}")
    } else {
        // No extension found — just append "-cleaned".
        format!("{original}-cleaned")
    }
}

// =============================================================================
// Tests — Every function must be tested!
// =============================================================================
//
// RUST CONCEPT: `#[cfg(test)]`
// This entire module is only compiled when running tests. It doesn't
// end up in the WASM binary. This is like Jest's test files — code
// that only exists for testing purposes.
//
// RUST CONCEPT: `mod tests { ... }`
// A nested module inside clean.rs. It has access to everything in the
// parent module via `use super::*`.

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
        //
        // RUST CONCEPT: Clippy lint `default_constructed_unit_structs`
        // Clippy warns against calling `.default()` on a unit struct
        // because it's redundant — you can just write `CleanCsv` directly.
        // But we want to test that Default IS implemented, so we allow it.
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

        // RUST CONCEPT: `match` to extract the error
        // We can't use `.unwrap_err()` here because `NodeOutput` doesn't
        // implement the `Debug` trait (which `unwrap_err` needs to print
        // the Ok value if it panics). Instead, we use `match` or `if let`.
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
