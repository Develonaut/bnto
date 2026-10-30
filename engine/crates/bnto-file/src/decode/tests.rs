use super::*;
use bnto_core::context::NoopContext;
use bnto_core::metadata::ParameterType;
use serde_json::json;

fn input(filename: &str, text: &[u8], params: serde_json::Value) -> NodeInput {
    NodeInput {
        data: FileData::Bytes(text.to_vec()),
        filename: filename.to_string(),
        mime_type: Some("text/plain".to_string()),
        params: params.as_object().cloned().unwrap_or_default(),
    }
}

fn run(filename: &str, text: &[u8], params: serde_json::Value) -> Result<NodeOutput, BntoError> {
    let input = input(filename, text, params);
    FileDecode::new().process(input, &ProgressReporter::new_noop(), &NoopContext)
}

fn decoded(text: &[u8]) -> Vec<u8> {
    let out = run("data.b64", text, json!({})).unwrap();
    bytes_of(&out.files[0])
}

fn bytes_of(file: &OutputFile) -> Vec<u8> {
    match &file.data {
        FileData::Bytes(bytes) => bytes.clone(),
        other => panic!("expected decoded bytes, got {other:?}"),
    }
}

fn error_text(filename: &str, text: &[u8], params: serde_json::Value) -> String {
    match run(filename, text, params) {
        Ok(_) => panic!("expected {filename} to fail"),
        Err(e) => e.to_string(),
    }
}

/// Bytes that are not UTF-8, so a text round trip would change them.
const BINARY: [u8; 8] = [0x00, 0xff, 0xfe, 0x80, b'h', b'i', 0x0a, 0xc3];
/// `BINARY` in standard base64.
const BINARY_B64: &str = "AP/+gGhpCsM=";

// --- Trait basics ---

#[test]
fn test_name_is_file_decode() {
    assert_eq!(FileDecode::new().name(), "file-decode");
    assert_eq!(FileDecode::new().metadata().node_type, "file-decode");
}

#[test]
fn test_metadata_is_a_per_file_browser_node() {
    let meta = FileDecode::new().metadata();
    assert_eq!(meta.input_cardinality, InputCardinality::PerFile);
    assert!(meta.platforms.contains(&"browser".to_string()));
    assert!(meta.platforms.contains(&"cli".to_string()));
}

#[test]
fn test_encoding_param_is_an_enum_defaulting_to_base64() {
    let meta = FileDecode::new().metadata();
    let encoding = meta
        .parameters
        .iter()
        .find(|p| p.name == "encoding")
        .unwrap();
    assert_eq!(encoding.default, Some(json!("base64")));
    let ParameterType::Enum { options } = &encoding.param_type else {
        panic!("encoding should be an enum");
    };
    let values: Vec<&str> = options.iter().map(|o| o.value.as_str()).collect();
    assert_eq!(values, ["base64"]);
}

#[test]
fn test_filename_param_is_an_optional_string() {
    let meta = FileDecode::new().metadata();
    let filename = meta
        .parameters
        .iter()
        .find(|p| p.name == "filename")
        .unwrap();
    assert_eq!(filename.param_type, ParameterType::String);
    assert_eq!(filename.default, None);
}

#[test]
fn test_validate_passes_with_no_params() {
    assert!(
        FileDecode::new()
            .validate(&serde_json::Map::new())
            .is_empty()
    );
}

#[test]
fn test_validate_refuses_an_unknown_encoding() {
    let params = json!({ "encoding": "rot13" }).as_object().cloned().unwrap();
    let problems = FileDecode::new().validate(&params);
    assert_eq!(problems.len(), 1, "{problems:?}");
    assert!(problems[0].contains("rot13"), "{problems:?}");
}

#[test]
fn test_validate_leaves_a_templated_encoding_to_the_run() {
    let params = json!({ "encoding": "{{env.ENCODING}}" })
        .as_object()
        .cloned()
        .unwrap();
    assert!(FileDecode::new().validate(&params).is_empty());
}

// --- Decoding ---

#[test]
fn test_base64_decodes_to_its_exact_bytes() {
    assert_eq!(decoded(BINARY_B64.as_bytes()), BINARY);
}

#[test]
fn test_explicit_base64_encoding_decodes_the_same() {
    let out = run(
        "data.b64",
        BINARY_B64.as_bytes(),
        json!({ "encoding": "base64" }),
    )
    .unwrap();
    assert_eq!(bytes_of(&out.files[0]), BINARY);
}

#[test]
fn test_base64_without_padding_decodes_the_same() {
    assert_eq!(decoded(b"AP/+gGhpCsM"), BINARY);
    assert_eq!(decoded(b"aGk"), b"hi");
    assert_eq!(decoded(b"aGk="), b"hi");
}

#[test]
fn test_line_wrapped_base64_with_whitespace_decodes_the_same() {
    assert_eq!(decoded(b"  AP/+\r\ngGhp\n\tCsM=\n"), BINARY);
}

#[test]
fn test_a_longer_wrapped_text_decodes_to_its_json() {
    // 76-column lines, as MIME and `base64` without `-w 0` wrap them.
    let json_text =
        br#"[{"name":"alpha","size":1},{"name":"beta","size":2},{"name":"gamma","size":3}]"#;
    let encoded = "W3sibmFtZSI6ImFscGhhIiwic2l6ZSI6MX0seyJuYW1lIjoiYmV0YSIsInNpemUiOjJ9LHsibmFt\n\
                   ZSI6ImdhbW1hIiwic2l6ZSI6M31d\n";
    assert_eq!(decoded(encoded.as_bytes()), json_text);
}

#[test]
fn test_empty_file_decodes_to_empty() {
    assert_eq!(decoded(b""), b"");
}

#[test]
fn test_whitespace_only_file_decodes_to_empty() {
    assert_eq!(decoded(b" \r\n\t\n"), b"");
}

// --- Errors ---

#[test]
fn test_text_that_isnt_base64_fails_naming_the_file() {
    let err = error_text("notes.b64", b"this is not base64!", json!({}));
    assert!(err.contains("notes.b64"), "{err}");
    assert!(err.contains("not valid base64"), "{err}");
    assert!(err.contains("'!'"), "{err}");
}

#[test]
fn test_a_truncated_text_fails_with_a_reason() {
    let err = error_text("cut.b64", b"aGVsbG8gd", json!({}));
    assert!(
        err.contains("cut.b64") && err.contains("not valid base64"),
        "{err}"
    );
}

#[test]
fn test_misplaced_padding_fails() {
    let err = error_text("pad.b64", b"aG=k", json!({}));
    assert!(err.contains("pad.b64"), "{err}");
}

#[test]
fn test_url_safe_alphabet_is_refused_with_a_hint() {
    let err = error_text("token.b64", b"-_8", json!({}));
    assert!(err.contains("token.b64") && err.contains("'-'"), "{err}");
    assert!(err.contains("URL-safe"), "{err}");
}

#[test]
fn test_unknown_encoding_fails_the_run() {
    let err = error_text("data.b64", b"aGk=", json!({ "encoding": "rot13" }));
    assert!(err.contains("rot13"), "{err}");
}

// --- Output name and MIME type ---

#[test]
fn test_output_keeps_the_input_name_by_default() {
    let out = run("sub/data.b64", b"aGk=", json!({})).unwrap();
    assert_eq!(out.files.len(), 1);
    assert_eq!(out.files[0].filename, "sub/data.b64");
    assert_eq!(out.files[0].mime_type, "application/octet-stream");
}

#[test]
fn test_filename_param_renames_the_output() {
    let out = run("data.b64", b"W10=", json!({ "filename": "{{name}}.json" })).unwrap();
    assert_eq!(out.files[0].filename, "data.json");
    assert_eq!(out.files[0].mime_type, "application/json");
    assert_eq!(bytes_of(&out.files[0]), b"[]");
}

#[test]
fn test_filename_name_drops_only_the_last_extension() {
    let out = run("report.csv.b64", b"aGk=", json!({ "filename": "{{name}}" })).unwrap();
    assert_eq!(out.files[0].filename, "report.csv");
    assert_eq!(out.files[0].mime_type, "text/csv");
}

#[test]
fn test_filename_name_keeps_the_input_folders() {
    let out = run(
        "in.d/sub/data.b64",
        b"aGk=",
        json!({ "filename": "{{name}}.txt" }),
    )
    .unwrap();
    assert_eq!(out.files[0].filename, "in.d/sub/data.txt");
}

#[test]
fn test_filename_ext_reads_the_input_extension() {
    let out = run(
        "photo.png",
        b"aGk=",
        json!({ "filename": "{{name}}-decoded.{{ext}}" }),
    )
    .unwrap();
    assert_eq!(out.files[0].filename, "photo-decoded.png");
    assert_eq!(out.files[0].mime_type, "image/png");
}

#[test]
fn test_a_name_that_reads_like_a_token_is_kept_as_written() {
    // RED (ISS-35): `{{ext}}` inside the filled name was filled next.
    let out = run(
        "a{{ext}}.png",
        b"aGk=",
        json!({ "filename": "{{name}}.{{ext}}" }),
    )
    .unwrap();
    assert_eq!(out.files[0].filename, "a{{ext}}.png");
}

#[test]
fn test_a_name_without_an_extension_fills_name_whole() {
    let out = run("README", b"aGk=", json!({ "filename": "{{name}}.md" })).unwrap();
    assert_eq!(out.files[0].filename, "README.md");
    let dotfile = run(".secret", b"aGk=", json!({ "filename": "{{name}}.txt" })).unwrap();
    assert_eq!(dotfile.files[0].filename, ".secret.txt");
}

#[test]
fn test_a_literal_filename_is_used_as_is() {
    let out = run("blob.b64", b"aGk=", json!({ "filename": "items.json" })).unwrap();
    assert_eq!(out.files[0].filename, "items.json");
}

#[test]
fn test_an_empty_filename_keeps_the_input_name() {
    let out = run("data.b64", b"aGk=", json!({ "filename": "" })).unwrap();
    assert_eq!(out.files[0].filename, "data.b64");
}

#[test]
fn test_different_filenames_give_different_outputs() {
    let csv = run("data.b64", b"aGk=", json!({ "filename": "{{name}}.csv" })).unwrap();
    let json_out = run("data.b64", b"aGk=", json!({ "filename": "{{name}}.json" })).unwrap();
    assert_ne!(csv.files[0].filename, json_out.files[0].filename);
    assert_ne!(csv.files[0].mime_type, json_out.files[0].mime_type);
}

// --- Metadata ---

#[test]
fn test_output_metadata_carries_the_decoded_size() {
    let out = run("data.b64", BINARY_B64.as_bytes(), json!({})).unwrap();
    assert_eq!(out.metadata.get("decoded_size"), Some(&json!(BINARY.len())));
    assert!(
        out.files[0].metadata.is_empty(),
        "file metadata comes from the node's metadata on top of the input's"
    );
}

#[test]
fn test_decoded_size_adds_up_across_files() {
    assert_eq!(FileDecode::new().count_keys(), ["decoded_size"]);
}
