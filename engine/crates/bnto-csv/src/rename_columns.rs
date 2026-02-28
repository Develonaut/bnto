// =============================================================================
// RenameCsvColumns — Rename Column Headers in a CSV File
// =============================================================================
//
// WHAT IS THIS FILE?
// This implements the "rename-csv-columns" node for Bnto. It takes a CSV
// file and renames column headers based on a user-provided mapping. Data
// rows are left completely unchanged — only the header row is modified.
//
// EXAMPLE:
//   Input CSV:
//     First Name, Last Name, Email
//     Alice, Smith, alice@example.com
//
//   Mapping: {"First Name": "first_name", "Last Name": "last_name"}
//
//   Output CSV:
//     first_name, last_name, Email
//     Alice, Smith, alice@example.com
//
// Notice "Email" was NOT in the mapping, so it stays unchanged.
//
// WHY SEPARATE FROM wasm_bridge.rs?
// Separation of concerns (Bento Box Principle):
//   - rename_columns.rs: Pure Rust CSV logic. Testable natively with `cargo test`.
//     Has no knowledge of JavaScript or WASM boundary concerns.
//   - wasm_bridge.rs: Handles the JS ↔ Rust type conversion. Only that file
//     imports wasm_bindgen and deals with JsValue.

use std::collections::HashMap;

use bnto_core::errors::BntoError;
use bnto_core::processor::{NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;

// =============================================================================
// The RenameCsvColumns Struct
// =============================================================================
//
// RUST CONCEPT: Unit Struct
// `pub struct RenameCsvColumns;` is a "unit struct" — a type with no fields.
// It's like an empty class in JavaScript: `class RenameCsvColumns {}`.
// We use it as a namespace to implement the `NodeProcessor` trait.
// The actual configuration (which columns to rename) comes from the
// `params` field in `NodeInput`, not from the struct itself. This keeps
// the processor stateless and reusable.

/// The rename-csv-columns node processor.
///
/// Renames column headers in a CSV file based on a mapping provided
/// in the `columns` parameter. Data rows are preserved unchanged.
pub struct RenameCsvColumns;

impl RenameCsvColumns {
    /// Create a new RenameCsvColumns processor.
    ///
    /// RUST CONCEPT: `Self`
    /// `Self` is a shorthand for the type we're implementing — in this case,
    /// `RenameCsvColumns`. So `Self` and `RenameCsvColumns` mean the same thing
    /// inside this `impl` block.
    pub fn new() -> Self {
        Self
    }
}

// RUST CONCEPT: `Default` trait
// The `Default` trait provides a way to create a "default" instance of a type.
// It's like having a no-argument constructor. By implementing it, users can
// call `RenameCsvColumns::default()` instead of `RenameCsvColumns::new()`.
// Many Rust libraries and patterns expect types to implement `Default`.
impl Default for RenameCsvColumns {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// NodeProcessor Implementation
// =============================================================================
//
// RUST CONCEPT: `impl Trait for Type`
// This block implements the `NodeProcessor` trait for `RenameCsvColumns`.
// The compiler checks that we provide all required methods (`name` and `process`).
// After this, `RenameCsvColumns` can be used anywhere a `NodeProcessor` is expected.

impl NodeProcessor for RenameCsvColumns {
    /// Returns the unique name of this node type.
    /// Used for logging, progress reporting, and identifying the node in the UI.
    fn name(&self) -> &str {
        "rename-csv-columns"
    }

    /// Process a CSV file: rename column headers based on the provided mapping.
    ///
    /// ARGUMENTS:
    ///   - `input` — the CSV file data, filename, and configuration parameters
    ///   - `progress` — callback to report progress to the UI
    ///
    /// PARAMETERS (inside `input.params`):
    ///   - `columns` (object): Mapping of old column name → new column name
    ///     e.g., `{"First Name": "first_name", "Last Name": "last_name"}`
    ///   - If `columns` is missing or empty, the CSV is returned unchanged.
    ///   - If a mapped column doesn't exist in the CSV, it's silently ignored.
    ///
    /// RETURNS:
    ///   - `Ok(NodeOutput)` with the renamed CSV and metadata
    ///   - `Err(BntoError)` if the input is not valid UTF-8 or CSV parsing fails
    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
    ) -> Result<NodeOutput, BntoError> {
        // --- Step 1: Report that we're starting ---
        progress.report(0, "Starting column rename...");

        // --- Step 2: Convert raw bytes to a UTF-8 string ---
        //
        // CSV files are text, so we need to interpret the raw bytes as a string.
        // `std::str::from_utf8()` checks that the bytes are valid UTF-8.
        //
        // RUST CONCEPT: `map_err()`
        // `.map_err(|e| ...)` transforms the error type if the result is Err.
        // `std::str::from_utf8` returns a `Utf8Error`, but we need a `BntoError`.
        // So we wrap it in `BntoError::InvalidInput` with a human-readable message.
        //
        // RUST CONCEPT: `?` operator
        // The `?` at the end is shorthand for "if this is an error, return it
        // immediately from the function". It's like an early return on error.
        // Without `?`, we'd need a `match` block to handle Ok/Err.
        let csv_text = std::str::from_utf8(&input.data)
            .map_err(|e| BntoError::InvalidInput(format!("CSV is not valid UTF-8: {e}")))?;

        // --- Step 3: Extract the column mapping from parameters ---
        //
        // The `columns` param is a JSON object like:
        //   {"First Name": "first_name", "Last Name": "last_name"}
        //
        // We need to convert this from a serde_json::Value (generic JSON)
        // into a HashMap<String, String> (typed Rust map) for easy lookup.
        let column_mapping = extract_column_mapping(&input.params);

        progress.report(20, "Parsed parameters...");

        // --- Step 4: Parse the CSV and rename headers ---
        //
        // We use the `csv` crate's reader to parse the CSV text.
        // The reader handles quoted fields, escaping, and edge cases.
        //
        // RUST CONCEPT: `csv::ReaderBuilder`
        // The builder pattern lets us configure the CSV reader before creating it.
        // `.flexible(true)` means rows can have different numbers of fields
        // (some CSVs have ragged rows — not all rows have the same column count).
        // `.from_reader()` creates a reader from anything that implements `Read`.
        // `.as_bytes()` converts our string to a byte slice, which implements `Read`.
        let mut reader = csv::ReaderBuilder::new()
            .flexible(true)
            .from_reader(csv_text.as_bytes());

        // --- Step 5: Read the header row ---
        //
        // `.headers()` returns the first row of the CSV as a special `StringRecord`.
        // We clone it because the reader borrows it internally, and we need to
        // modify it (apply our renames).
        //
        // RUST CONCEPT: `.clone()`
        // `.clone()` creates a deep copy of the data. The original stays with the
        // reader, and we get our own copy to modify. In Rust, you have to be
        // explicit about copying data — there's no implicit copy like in JavaScript.
        let headers = reader
            .headers()
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to read CSV headers: {e}")))?
            .clone();

        progress.report(40, "Read headers...");

        // --- Step 6: Apply the column mapping to headers ---
        //
        // For each header, check if it exists in our mapping. If yes, use the
        // new name. If no, keep the original. This preserves column order.
        //
        // RUST CONCEPT: `.iter()` and `.map()`
        // `.iter()` creates an iterator over the headers (like `Array.entries()` in JS).
        // `.map()` transforms each element (like `Array.map()` in JS).
        // `.collect()` gathers the results into a new collection (Vec<String>).
        let mut columns_renamed: u64 = 0;

        let new_headers: Vec<String> = headers
            .iter()
            .map(|header| {
                // Check if this header has a rename mapping.
                // `.get()` returns `Option<&String>` — Some if found, None if not.
                if let Some(new_name) = column_mapping.get(header) {
                    // This header is in the mapping — use the new name.
                    columns_renamed += 1;
                    new_name.clone()
                } else {
                    // This header is NOT in the mapping — keep the original.
                    header.to_string()
                }
            })
            .collect();

        progress.report(60, "Renamed headers...");

        // --- Step 7: Write the output CSV ---
        //
        // We create a CSV writer that writes to an in-memory Vec<u8> (byte buffer).
        // First we write the new headers, then all data rows unchanged.
        //
        // RUST CONCEPT: `csv::WriterBuilder`
        // Similar to the reader builder — configures CSV output settings.
        // `.flexible(true)` allows records with different numbers of fields
        // (matching the reader's behavior — if we read ragged rows, we need
        // to be able to write them back without the writer complaining).
        // `.from_writer(Vec::new())` creates a writer that writes to a fresh
        // byte buffer in memory (like a StringBuffer/ByteArrayOutputStream in Java).
        let mut writer = csv::WriterBuilder::new()
            .flexible(true)
            .from_writer(Vec::new());

        // Write the renamed header row.
        //
        // `.write_record()` writes one row of fields to the CSV.
        // We pass our `new_headers` slice — each String becomes one field.
        writer
            .write_record(&new_headers)
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to write headers: {e}")))?;

        // --- Step 8: Copy all data rows unchanged ---
        //
        // `.records()` returns an iterator over all NON-header rows.
        // Each record is a `StringRecord` — a row of CSV fields.
        // We write each row directly to the output without modification.
        //
        // RUST CONCEPT: `for ... in ...`
        // This is Rust's for loop. It iterates over each item in the iterator.
        // `record` is a `Result<StringRecord, csv::Error>` — each row might
        // fail to parse (e.g., if the CSV is malformed mid-file). We handle
        // each error with `?`.
        let mut row_count: u64 = 0;
        for record in reader.records() {
            let record = record
                .map_err(|e| BntoError::ProcessingFailed(format!("Failed to read CSV row: {e}")))?;
            writer.write_record(record.iter()).map_err(|e| {
                BntoError::ProcessingFailed(format!("Failed to write CSV row: {e}"))
            })?;
            row_count += 1;
        }

        progress.report(90, "Wrote output CSV...");

        // --- Step 9: Finalize the writer and extract bytes ---
        //
        // `.into_inner()` flushes any buffered data and returns the underlying
        // Vec<u8> (our byte buffer). This consumes the writer — we can't write
        // any more after this.
        //
        // RUST CONCEPT: Consuming methods
        // `.into_inner()` takes ownership of the writer (it's gone after this).
        // This is a common Rust pattern: methods prefixed with `into_` consume
        // `self` and return the inner data. It ensures no one accidentally
        // writes to the writer after we've taken the bytes.
        let output_bytes = writer
            .into_inner()
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to finalize CSV: {e}")))?;

        // --- Step 10: Build the output filename ---
        //
        // We add "-renamed" before the file extension.
        // "data.csv" → "data-renamed.csv"
        let output_filename = build_output_filename(&input.filename);

        // --- Step 11: Build metadata for the UI ---
        //
        // The metadata is a JSON object that the UI displays in the results panel.
        // It tells the user what happened: how many columns were renamed, the total
        // column count, the number of data rows, and the actual mapping applied.
        let mut metadata = serde_json::Map::new();
        metadata.insert(
            "columnsRenamed".to_string(),
            serde_json::Value::Number(serde_json::Number::from(columns_renamed)),
        );
        metadata.insert(
            "totalColumns".to_string(),
            serde_json::Value::Number(serde_json::Number::from(new_headers.len() as u64)),
        );
        metadata.insert(
            "dataRows".to_string(),
            serde_json::Value::Number(serde_json::Number::from(row_count)),
        );

        // Include the applied mapping in metadata so the UI can show what changed.
        // We build a JSON object of only the mappings that actually applied
        // (columns that existed in the CSV AND were in the mapping).
        let applied_mapping: serde_json::Map<String, serde_json::Value> = column_mapping
            .iter()
            .filter(|(old_name, _)| headers.iter().any(|h| h == old_name.as_str()))
            .map(|(old_name, new_name)| {
                (
                    old_name.clone(),
                    serde_json::Value::String(new_name.clone()),
                )
            })
            .collect();
        metadata.insert(
            "mapping".to_string(),
            serde_json::Value::Object(applied_mapping),
        );

        progress.report(100, "Done!");

        // --- Step 12: Return the result ---
        //
        // We wrap everything in a `NodeOutput` — the standard return type for
        // all node processors. It contains the output file(s) and metadata.
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

/// Extract the column mapping from the node's parameters.
///
/// Looks for a `columns` key in the params map. The value should be a JSON
/// object mapping old column names to new column names. If the key is missing
/// or the value isn't an object, returns an empty HashMap.
///
/// EXAMPLE:
///   params = {"columns": {"First Name": "first_name", "Last Name": "last_name"}}
///   → HashMap { "First Name" => "first_name", "Last Name" => "last_name" }
///
/// RUST CONCEPT: `HashMap<String, String>`
/// A HashMap is like a JavaScript `Map` or a plain object — it stores
/// key-value pairs. Here, both keys and values are Strings.
fn extract_column_mapping(
    params: &serde_json::Map<String, serde_json::Value>,
) -> HashMap<String, String> {
    // Try to get the "columns" key from params.
    // `.get()` returns `Option<&Value>` — Some if the key exists, None if not.
    let columns_value = match params.get("columns") {
        Some(val) => val,
        // No "columns" param → empty mapping (CSV passes through unchanged).
        None => return HashMap::new(),
    };

    // The value should be a JSON object (map). If it's not (e.g., it's a
    // string or number), we return an empty mapping.
    //
    // RUST CONCEPT: `if let`
    // `if let` is a concise way to match on one pattern. If the value is
    // an Object, we bind the inner map to `obj` and continue. Otherwise,
    // we return an empty HashMap.
    let obj = if let serde_json::Value::Object(obj) = columns_value {
        obj
    } else {
        return HashMap::new();
    };

    // Convert the JSON object into a HashMap<String, String>.
    // We only include entries where the value is a string (ignore numbers,
    // booleans, nulls, etc. — those don't make sense as column names).
    //
    // RUST CONCEPT: `.filter_map()`
    // Like `.map()` but also filters out `None` values. If the closure
    // returns `Some(value)`, the value is included. If it returns `None`,
    // it's skipped. This is like `.map().filter()` combined.
    obj.iter()
        .filter_map(|(key, value)| {
            // Only include entries where the value is a string.
            if let serde_json::Value::String(new_name) = value {
                Some((key.clone(), new_name.clone()))
            } else {
                None
            }
        })
        .collect()
}

/// Build the output filename by inserting "-renamed" before the extension.
///
/// EXAMPLES:
///   "data.csv"           → "data-renamed.csv"
///   "report.tsv"         → "report-renamed.tsv"
///   "noextension"        → "noextension-renamed"
///   "my.data.file.csv"   → "my.data.file-renamed.csv"
fn build_output_filename(input_filename: &str) -> String {
    // Find the last dot to split the filename from the extension.
    //
    // RUST CONCEPT: `.rfind()`
    // `.rfind('.')` searches backwards through the string for a dot.
    // Returns `Some(index)` if found, `None` if not. We search backwards
    // because filenames can have multiple dots (e.g., "my.data.csv").
    match input_filename.rfind('.') {
        Some(dot_pos) => {
            // Split at the last dot: "data" + ".csv"
            let (name, ext) = input_filename.split_at(dot_pos);
            format!("{name}-renamed{ext}")
        }
        None => {
            // No extension — just append "-renamed"
            format!("{input_filename}-renamed")
        }
    }
}

// =============================================================================
// Tests
// =============================================================================
//
// RUST CONCEPT: `#[cfg(test)]`
// This attribute means "only compile this module when running tests".
// It's like having a test-only section that doesn't exist in production code.
// This keeps the production binary smaller and avoids compiling test helpers
// for release builds.

#[cfg(test)]
mod tests {
    use super::*;

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

        let output = processor.process(input, &progress).unwrap();
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

        let output = processor.process(input, &progress).unwrap();
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

        let output = processor.process(input, &progress).unwrap();
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

        let output = processor.process(input, &progress).unwrap();
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

        let output = processor.process(input, &progress).unwrap();
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

        let output = processor.process(input, &progress).unwrap();
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

        let output = processor.process(input, &progress).unwrap();
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

        let output = processor.process(input, &progress).unwrap();
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

        let output = processor.process(input, &progress).unwrap();
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

        let output = processor.process(input, &progress).unwrap();
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

        let result = processor.process(input, &progress);

        // Should be an error, not a panic.
        assert!(result.is_err());

        // The error message should mention UTF-8.
        //
        // RUST CONCEPT: `if let Err(e) = result`
        // We can't use `.unwrap_err()` here because `NodeOutput` doesn't
        // implement the `Debug` trait (which `.unwrap_err()` requires to
        // print the Ok value if it's unexpectedly Ok). So we use pattern
        // matching with `if let` to extract the error.
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

        let output = processor.process(input, &progress).unwrap();
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

        let output = processor.process(input, &progress).unwrap();

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

        let output = processor.process(input, &progress).unwrap();
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
        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].mime_type, "text/csv");
    }

    #[test]
    fn test_output_filename_has_renamed_suffix() {
        let processor = RenameCsvColumns::new();
        let progress = ProgressReporter::new_noop();

        let mut input = make_csv_input("name\nAlice\n", "{}");
        input.filename = "my_data.csv".to_string();

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "my_data-renamed.csv");
    }
}
