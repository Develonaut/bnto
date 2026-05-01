// CSV-to-JSON Node — convert CSV rows to a JSON array of objects.
//
// Each row becomes a JSON object keyed by column headers. All values are
// strings (no type coercion). Supports configurable delimiter and optional
// pretty-printing.

use bnto_core::context::ProcessContext;
use bnto_core::errors::BntoError;
use bnto_core::processor::{NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;

/// The spreadsheet-convert node processor. Stateless — config comes from `NodeInput.params`.
pub struct ConvertFormat;

impl ConvertFormat {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ConvertFormat {
    fn default() -> Self {
        Self::new()
    }
}

// --- NodeProcessor Implementation ---

impl NodeProcessor for ConvertFormat {
    fn name(&self) -> &str {
        "spreadsheet-convert"
    }

    fn metadata(&self) -> bnto_core::NodeMetadata {
        use bnto_core::metadata::*;
        NodeMetadata {
            node_type: "spreadsheet-convert".to_string(),
            name: "CSV to JSON".to_string(),
            description: "Convert CSV rows to a JSON array of objects".to_string(),
            category: NodeCategory::Spreadsheet,
            accepts: vec!["text/csv".to_string()],
            platforms: vec!["browser".to_string()],
            parameters: build_convert_parameters(),
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
        let config = ConvertConfig::from_params(&input.params);
        let csv_text = parse_and_strip_bom(&input.data)?;

        progress.report(10, "Reading headers...");
        let delimiter = resolve_delimiter(&config.delimiter);
        let headers = read_headers(csv_text, delimiter)?;

        progress.report(20, "Converting rows to JSON...");
        let rows = read_rows(csv_text, delimiter, &headers)?;

        progress.report(70, "Serializing JSON...");
        let json_bytes = serialize_json(&rows, config.pretty)?;

        progress.report(90, "Building result...");
        let metadata = build_convert_metadata(headers.len(), rows.len(), json_bytes.len());
        let output_filename = generate_output_filename(&input.filename);

        progress.report(100, "Done!");
        Ok(NodeOutput {
            files: vec![OutputFile {
                data: json_bytes,
                filename: output_filename,
                mime_type: "application/json".to_string(),
                metadata: serde_json::Map::new(),
            }],
            metadata,
        })
    }
}

// --- Configuration ---

struct ConvertConfig {
    delimiter: String,
    pretty: bool,
}

impl ConvertConfig {
    fn from_params(params: &serde_json::Map<String, serde_json::Value>) -> Self {
        Self {
            delimiter: params
                .get("delimiter")
                .and_then(|v| v.as_str())
                .unwrap_or(",")
                .to_string(),
            pretty: params
                .get("pretty")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
        }
    }
}

/// Map delimiter name to byte. Supports "comma", "semicolon", "tab", or literal single char.
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

// --- Metadata Parameter Definitions ---

fn build_convert_parameters() -> Vec<bnto_core::metadata::ParameterDef> {
    use bnto_core::metadata::*;
    vec![
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
            name: "pretty".to_string(),
            label: "Pretty Print".to_string(),
            description: "Format output JSON with indentation".to_string(),
            param_type: ParameterType::Boolean,
            default: Some(serde_json::json!(false)),
            ..Default::default()
        },
    ]
}

// --- CSV Parsing ---

/// Validate UTF-8 and strip BOM if present.
fn parse_and_strip_bom(data: &[u8]) -> Result<&str, BntoError> {
    // UTF-8 BOM is 3 bytes: EF BB BF
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
            "CSV file is empty — no data to convert".to_string(),
        ));
    }

    Ok(text)
}

/// Read the header row from the CSV, returning column names.
fn read_headers(csv_text: &str, delimiter: u8) -> Result<Vec<String>, BntoError> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(delimiter)
        .flexible(true)
        .from_reader(csv_text.as_bytes());

    let headers = reader
        .headers()
        .map_err(|e| BntoError::ProcessingFailed(format!("Failed to read CSV headers: {e}")))?
        .clone();

    let names: Vec<String> = headers.iter().map(|h| h.trim().to_string()).collect();

    if names.is_empty() {
        return Err(BntoError::InvalidInput(
            "CSV has no columns — cannot convert to JSON".to_string(),
        ));
    }

    Ok(names)
}

/// Read all data rows into a vec of JSON objects keyed by headers.
fn read_rows(
    csv_text: &str,
    delimiter: u8,
    headers: &[String],
) -> Result<Vec<serde_json::Map<String, serde_json::Value>>, BntoError> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(delimiter)
        .flexible(true)
        .from_reader(csv_text.as_bytes());

    let mut rows = Vec::new();

    for result in reader.records() {
        let record = result
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to read CSV row: {e}")))?;

        let mut obj = serde_json::Map::new();
        for (i, header) in headers.iter().enumerate() {
            let value = record.get(i).unwrap_or("").to_string();
            obj.insert(header.clone(), serde_json::Value::String(value));
        }
        rows.push(obj);
    }

    Ok(rows)
}

// --- JSON Serialization ---

fn serialize_json(
    rows: &[serde_json::Map<String, serde_json::Value>],
    pretty: bool,
) -> Result<Vec<u8>, BntoError> {
    let json_value = serde_json::Value::Array(
        rows.iter()
            .map(|obj| serde_json::Value::Object(obj.clone()))
            .collect(),
    );

    let bytes = if pretty {
        serde_json::to_vec_pretty(&json_value)
    } else {
        serde_json::to_vec(&json_value)
    }
    .map_err(|e| BntoError::ProcessingFailed(format!("Failed to serialize JSON: {e}")))?;

    Ok(bytes)
}

// --- Result Metadata ---

fn build_convert_metadata(
    column_count: usize,
    row_count: usize,
    output_size: usize,
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
    metadata.insert(
        "outputSize".to_string(),
        serde_json::Value::Number(output_size.into()),
    );
    metadata
}

// --- Filename ---

/// Replace .csv extension with .json: "data.csv" -> "data.json"
fn generate_output_filename(original: &str) -> String {
    if let Some(dot_pos) = original.rfind('.') {
        let stem = &original[..dot_pos];
        format!("{stem}.json")
    } else {
        format!("{original}.json")
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

    fn make_input(csv_text: &str) -> NodeInput {
        NodeInput {
            data: csv_text.as_bytes().to_vec(),
            filename: "test.csv".to_string(),
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
            filename: "test.csv".to_string(),
            mime_type: Some("text/csv".to_string()),
            params,
        }
    }

    fn make_input_bytes(data: Vec<u8>) -> NodeInput {
        NodeInput {
            data,
            filename: "test.csv".to_string(),
            mime_type: Some("text/csv".to_string()),
            params: serde_json::Map::new(),
        }
    }

    fn output_json(output: &NodeOutput) -> serde_json::Value {
        serde_json::from_slice(&output.files[0].data).expect("Output should be valid JSON")
    }

    fn process_ok(input: NodeInput) -> NodeOutput {
        let processor = ConvertFormat::new();
        let progress = ProgressReporter::new_noop();
        processor
            .process(input, &progress, &NoopContext)
            .expect("should succeed")
    }

    fn process_err(input: NodeInput) -> BntoError {
        let processor = ConvertFormat::new();
        let progress = ProgressReporter::new_noop();
        let result = processor.process(input, &progress, &NoopContext);
        assert!(result.is_err(), "expected an error");
        match result {
            Err(e) => e,
            Ok(_) => unreachable!(),
        }
    }

    // --- Trait Basics ---

    #[test]
    fn test_name() {
        assert_eq!(ConvertFormat::new().name(), "spreadsheet-convert");
    }

    #[test]
    fn test_default() {
        #[allow(clippy::default_constructed_unit_structs)]
        let _p = ConvertFormat::default();
    }

    #[test]
    fn test_metadata_node_type() {
        let meta = ConvertFormat::new().metadata();
        assert_eq!(meta.node_type, "spreadsheet-convert");
    }

    #[test]
    fn test_metadata_has_delimiter_and_pretty_params() {
        let meta = ConvertFormat::new().metadata();
        let names: Vec<&str> = meta.parameters.iter().map(|p| p.name.as_str()).collect();
        assert!(names.contains(&"delimiter"));
        assert!(names.contains(&"pretty"));
    }

    // --- Happy Path ---

    #[test]
    fn test_simple_csv_to_json() {
        let csv = "name,age,city\nAlice,30,NYC\nBob,25,LA\n";
        let output = process_ok(make_input(csv));
        let json = output_json(&output);

        let arr = json.as_array().expect("should be array");
        assert_eq!(arr.len(), 2);
        assert_eq!(arr[0]["name"], "Alice");
        assert_eq!(arr[0]["age"], "30");
        assert_eq!(arr[0]["city"], "NYC");
        assert_eq!(arr[1]["name"], "Bob");
    }

    #[test]
    fn test_values_are_strings() {
        let csv = "id,price,active\n1,19.99,true\n";
        let output = process_ok(make_input(csv));
        let json = output_json(&output);

        let row = &json[0];
        assert!(row["id"].is_string(), "id should be a string");
        assert!(row["price"].is_string(), "price should be a string");
        assert!(row["active"].is_string(), "active should be a string");
        assert_eq!(row["id"], "1");
        assert_eq!(row["price"], "19.99");
        assert_eq!(row["active"], "true");
    }

    #[test]
    fn test_headers_only_produces_empty_array() {
        let csv = "name,age,city\n";
        let output = process_ok(make_input(csv));
        let json = output_json(&output);

        let arr = json.as_array().expect("should be array");
        assert_eq!(arr.len(), 0);
    }

    #[test]
    fn test_single_row() {
        let csv = "x\n42\n";
        let output = process_ok(make_input(csv));
        let json = output_json(&output);

        assert_eq!(json.as_array().unwrap().len(), 1);
        assert_eq!(json[0]["x"], "42");
    }

    #[test]
    fn test_single_column() {
        let csv = "email\nalice@example.com\nbob@example.com\n";
        let output = process_ok(make_input(csv));
        let json = output_json(&output);

        assert_eq!(json[0]["email"], "alice@example.com");
        assert_eq!(json[1]["email"], "bob@example.com");
    }

    // --- Empty Cells ---

    #[test]
    fn test_empty_cells_become_empty_strings() {
        let csv = "name,age,city\nAlice,,NYC\n,25,\n";
        let output = process_ok(make_input(csv));
        let json = output_json(&output);

        assert_eq!(json[0]["age"], "");
        assert_eq!(json[1]["name"], "");
        assert_eq!(json[1]["city"], "");
    }

    #[test]
    fn test_short_row_gets_empty_string_for_missing_columns() {
        let csv = "name,age,city\nAlice\n";
        let output = process_ok(make_input(csv));
        let json = output_json(&output);

        assert_eq!(json[0]["name"], "Alice");
        assert_eq!(json[0]["age"], "");
        assert_eq!(json[0]["city"], "");
    }

    // --- Quoted Fields ---

    #[test]
    fn test_quoted_fields_with_commas() {
        let csv = "name,bio\n\"Alice\",\"Loves coding, hiking\"\n";
        let output = process_ok(make_input(csv));
        let json = output_json(&output);

        assert_eq!(json[0]["bio"], "Loves coding, hiking");
    }

    #[test]
    fn test_quoted_fields_with_newlines() {
        let csv = "name,bio\n\"Alice\",\"Line one\nLine two\"\n";
        let output = process_ok(make_input(csv));
        let json = output_json(&output);

        assert_eq!(json[0]["bio"], "Line one\nLine two");
    }

    #[test]
    fn test_escaped_quotes() {
        let csv = "name,bio\n\"Alice\",\"Says \"\"hello\"\"\"\n";
        let output = process_ok(make_input(csv));
        let json = output_json(&output);

        assert_eq!(json[0]["bio"], "Says \"hello\"");
    }

    // --- Unicode ---

    #[test]
    fn test_unicode_characters() {
        let csv = "name,city\nMüller,München\n田中太郎,東京\n";
        let output = process_ok(make_input(csv));
        let json = output_json(&output);

        assert_eq!(json[0]["name"], "Müller");
        assert_eq!(json[0]["city"], "München");
        assert_eq!(json[1]["name"], "田中太郎");
    }

    #[test]
    fn test_emoji() {
        let csv = "name,note\nEmma 🎉,rocket 🚀\n";
        let output = process_ok(make_input(csv));
        let json = output_json(&output);

        assert_eq!(json[0]["name"], "Emma 🎉");
        assert_eq!(json[0]["note"], "rocket 🚀");
    }

    // --- BOM Handling ---

    #[test]
    fn test_utf8_bom_stripped() {
        let csv_with_bom = b"\xEF\xBB\xBFname,age\nAlice,30\n";
        let output = process_ok(make_input_bytes(csv_with_bom.to_vec()));
        let json = output_json(&output);

        // BOM should be stripped — header should be "name", not "\u{FEFF}name"
        assert_eq!(json[0]["name"], "Alice");
        assert!(json[0].get("\u{FEFF}name").is_none());
    }

    // --- CRLF Handling ---

    #[test]
    fn test_crlf_line_endings() {
        let csv = "name,age\r\nAlice,30\r\nBob,25\r\n";
        let output = process_ok(make_input(csv));
        let json = output_json(&output);

        assert_eq!(json.as_array().unwrap().len(), 2);
        assert_eq!(json[0]["name"], "Alice");
        assert_eq!(json[1]["name"], "Bob");
    }

    // --- Delimiter Configuration ---

    #[test]
    fn test_semicolon_delimiter() {
        let csv = "name;age;city\nAlice;30;NYC\n";
        let mut params = serde_json::Map::new();
        params.insert("delimiter".into(), serde_json::json!("semicolon"));
        let output = process_ok(make_input_with_params(csv, params));
        let json = output_json(&output);

        assert_eq!(json[0]["name"], "Alice");
        assert_eq!(json[0]["age"], "30");
        assert_eq!(json[0]["city"], "NYC");
    }

    #[test]
    fn test_tab_delimiter() {
        let csv = "name\tage\nAlice\t30\n";
        let mut params = serde_json::Map::new();
        params.insert("delimiter".into(), serde_json::json!("tab"));
        let output = process_ok(make_input_with_params(csv, params));
        let json = output_json(&output);

        assert_eq!(json[0]["name"], "Alice");
        assert_eq!(json[0]["age"], "30");
    }

    #[test]
    fn test_pipe_delimiter() {
        let csv = "name|age\nAlice|30\n";
        let mut params = serde_json::Map::new();
        params.insert("delimiter".into(), serde_json::json!("pipe"));
        let output = process_ok(make_input_with_params(csv, params));
        let json = output_json(&output);

        assert_eq!(json[0]["name"], "Alice");
    }

    #[test]
    fn test_default_delimiter_is_comma() {
        let csv = "name,age\nAlice,30\n";
        let output = process_ok(make_input(csv));
        let json = output_json(&output);

        assert_eq!(json[0]["name"], "Alice");
    }

    // --- Pretty Print ---

    #[test]
    fn test_compact_output_by_default() {
        let csv = "name\nAlice\n";
        let output = process_ok(make_input(csv));
        let text = String::from_utf8(output.files[0].data.clone()).unwrap();

        // Compact JSON has no newlines within the array
        assert!(!text.contains('\n'), "Compact output should be single line");
    }

    #[test]
    fn test_pretty_output() {
        let csv = "name\nAlice\n";
        let mut params = serde_json::Map::new();
        params.insert("pretty".into(), serde_json::json!(true));
        let output = process_ok(make_input_with_params(csv, params));
        let text = String::from_utf8(output.files[0].data.clone()).unwrap();

        assert!(text.contains('\n'), "Pretty output should have newlines");
        assert!(text.contains("  "), "Pretty output should have indentation");
    }

    #[test]
    fn test_pretty_changes_output() {
        let csv = "name,age\nAlice,30\nBob,25\n";

        let compact = process_ok(make_input(csv));
        let compact_size = compact.files[0].data.len();

        let mut params = serde_json::Map::new();
        params.insert("pretty".into(), serde_json::json!(true));
        let pretty = process_ok(make_input_with_params(csv, params));
        let pretty_size = pretty.files[0].data.len();

        assert!(
            pretty_size > compact_size,
            "Pretty should be larger: {pretty_size} vs {compact_size}"
        );
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

    // --- Output Filename ---

    #[test]
    fn test_output_filename_csv_to_json() {
        assert_eq!(generate_output_filename("data.csv"), "data.json");
    }

    #[test]
    fn test_output_filename_no_extension() {
        assert_eq!(generate_output_filename("data"), "data.json");
    }

    #[test]
    fn test_output_filename_multiple_dots() {
        assert_eq!(generate_output_filename("my.data.csv"), "my.data.json");
    }

    #[test]
    fn test_output_file_has_json_extension() {
        let csv = "name\nAlice\n";
        let output = process_ok(make_input(csv));
        assert_eq!(output.files[0].filename, "test.json");
    }

    #[test]
    fn test_output_mime_type() {
        let csv = "name\nAlice\n";
        let output = process_ok(make_input(csv));
        assert_eq!(output.files[0].mime_type, "application/json");
    }

    // --- Metadata ---

    #[test]
    fn test_metadata_fields() {
        let csv = "name,age,city\nAlice,30,NYC\nBob,25,LA\n";
        let output = process_ok(make_input(csv));

        assert_eq!(output.metadata["columnCount"], 3);
        assert_eq!(output.metadata["rowCount"], 2);
        assert!(output.metadata["outputSize"].as_u64().unwrap() > 0);
    }

    // --- Fixture Files ---

    #[test]
    fn test_fixture_simple() {
        let data = include_bytes!("../../../../test-fixtures/csv/simple.csv");
        let output = process_ok(make_input_bytes(data.to_vec()));
        let json = output_json(&output);
        let arr = json.as_array().unwrap();

        assert_eq!(arr.len(), 5);
        assert_eq!(arr[0]["name"], "Alice");
    }

    #[test]
    fn test_fixture_crlf() {
        let data = include_bytes!("../../../../test-fixtures/csv/crlf.csv");
        let output = process_ok(make_input_bytes(data.to_vec()));
        let json = output_json(&output);

        assert_eq!(json.as_array().unwrap().len(), 3);
        assert_eq!(json[0]["name"], "Alice");
    }

    #[test]
    fn test_fixture_bom() {
        let data = include_bytes!("../../../../test-fixtures/csv/bom.csv");
        let output = process_ok(make_input_bytes(data.to_vec()));
        let json = output_json(&output);

        // BOM stripped — "name" header should not have BOM prefix
        assert_eq!(json[0]["name"], "Alice");
    }

    #[test]
    fn test_fixture_unicode() {
        let data = include_bytes!("../../../../test-fixtures/csv/unicode.csv");
        let output = process_ok(make_input_bytes(data.to_vec()));
        let json = output_json(&output);

        assert_eq!(json[0]["name"], "Müller");
        assert_eq!(json[1]["name"], "田中太郎");
        assert!(json[3]["name"].as_str().unwrap().contains('🎉'));
    }

    #[test]
    fn test_fixture_quoted_fields() {
        let data = include_bytes!("../../../../test-fixtures/csv/quoted-fields.csv");
        let output = process_ok(make_input_bytes(data.to_vec()));
        let json = output_json(&output);

        assert_eq!(json[0]["bio"], "Loves coding, hiking");
        assert!(json[2]["bio"].as_str().unwrap().contains('\n'));
    }

    #[test]
    fn test_fixture_semicolon() {
        let data = include_bytes!("../../../../test-fixtures/csv/semicolon.csv");
        let mut params = serde_json::Map::new();
        params.insert("delimiter".into(), serde_json::json!("semicolon"));
        let input = make_input_with_params(std::str::from_utf8(data).unwrap(), params);
        let output = process_ok(input);
        let json = output_json(&output);

        assert_eq!(json[0]["name"], "Alice");
        assert_eq!(json[0]["age"], "30");
    }

    #[test]
    fn test_fixture_empty_cells() {
        let data = include_bytes!("../../../../test-fixtures/csv/empty-cells.csv");
        let output = process_ok(make_input_bytes(data.to_vec()));
        let json = output_json(&output);

        // Row 1: Alice has empty city
        assert_eq!(json[0]["city"], "");
        // Row 2: Bob has empty age
        assert_eq!(json[1]["age"], "");
    }

    #[test]
    fn test_fixture_numeric_values_stay_strings() {
        let data = include_bytes!("../../../../test-fixtures/csv/numeric-values.csv");
        let output = process_ok(make_input_bytes(data.to_vec()));
        let json = output_json(&output);

        // All values should be strings, not JSON numbers/bools
        assert!(json[0]["price"].is_string());
        assert!(json[0]["active"].is_string());
        assert_eq!(json[0]["price"], "19.99");
        assert_eq!(json[0]["active"], "true");
    }

    // --- Large CSV ---

    #[test]
    fn test_large_csv_1000_rows() {
        let mut csv = String::from("id,name,value\n");
        for i in 0..1000 {
            csv.push_str(&format!("{i},item_{i},{}\n", i * 10));
        }
        let output = process_ok(make_input(&csv));
        let json = output_json(&output);

        assert_eq!(json.as_array().unwrap().len(), 1000);
        assert_eq!(output.metadata["rowCount"], 1000);
        assert_eq!(output.metadata["columnCount"], 3);
    }

    // --- Delimiter Resolution ---

    #[test]
    fn test_resolve_delimiter_names() {
        assert_eq!(resolve_delimiter("comma"), b',');
        assert_eq!(resolve_delimiter("semicolon"), b';');
        assert_eq!(resolve_delimiter("tab"), b'\t');
        assert_eq!(resolve_delimiter("pipe"), b'|');
    }

    #[test]
    fn test_resolve_delimiter_literals() {
        assert_eq!(resolve_delimiter(","), b',');
        assert_eq!(resolve_delimiter(";"), b';');
        assert_eq!(resolve_delimiter("|"), b'|');
    }

    #[test]
    fn test_resolve_delimiter_unknown_defaults_to_comma() {
        assert_eq!(resolve_delimiter("unknown"), b',');
        assert_eq!(resolve_delimiter(""), b',');
    }
}
