// Mode-aware input preparation for the CLI.
//
// Reads the recipe's input mode (file-upload, url, text) and prepares
// the right inputs for the engine. Handles --param overrides via
// type-safe mutation on PipelineDefinition.

use bnto_core::{
    InputMode, PipelineDefinition, PipelineFile, PipelineNode, first_processing_node_id,
    resolve_input_mode,
};

use crate::io;

/// Ready-to-execute inputs: potentially mutated definition JSON + files.
#[derive(Debug)]
pub struct PreparedInput {
    pub definition_json: String,
    pub files: Vec<PipelineFile>,
}

/// Prepare inputs based on the recipe's declared input mode.
///
/// - **file-upload**: loads files from disk paths
/// - **url**: validates URL, injects into processing node's params, empty files
/// - **text**: injects text into processing node's params, empty files
///
/// Applies `--param` overrides after mode-specific preparation.
pub fn prepare_inputs(
    definition_json: &str,
    args: &[String],
    param_overrides: &[String],
) -> Result<PreparedInput, String> {
    let mut def: PipelineDefinition =
        serde_json::from_str(definition_json).map_err(|e| format!("Invalid recipe JSON: {e}"))?;

    let mode = resolve_input_mode(&def);
    let processing_node_id = first_processing_node_id(&def);

    let files = match mode {
        InputMode::FileUpload => {
            if args.is_empty() {
                return Err("This recipe requires input files. Usage: bnto run <recipe> <file1> [file2 ...]".to_string());
            }
            load_files(args)?
        }
        InputMode::Url => {
            if args.is_empty() {
                return Err(
                    "This recipe requires a URL. Usage: bnto run <recipe> <url>".to_string()
                );
            }
            let url = &args[0];
            if !is_url(url) {
                return Err(format!(
                    "Expected a URL (http:// or https://) but got: {url}"
                ));
            }
            let node_id = processing_node_id
                .as_deref()
                .ok_or("Recipe has no processing node to inject URL into".to_string())?;
            inject_param(
                &mut def,
                node_id,
                "url",
                &serde_json::Value::String(url.clone()),
            )?;
            vec![]
        }
        InputMode::Text => {
            if args.is_empty() {
                return Err(
                    "This recipe requires text input. Usage: bnto run <recipe> <text>".to_string(),
                );
            }
            let text = args.join(" ");
            let node_id = processing_node_id
                .as_deref()
                .ok_or("Recipe has no processing node to inject text into".to_string())?;
            inject_param(&mut def, node_id, "text", &serde_json::Value::String(text))?;
            vec![]
        }
    };

    apply_param_overrides(&mut def, param_overrides, processing_node_id.as_deref())?;

    let mutated_json =
        serde_json::to_string(&def).map_err(|e| format!("Failed to serialize definition: {e}"))?;

    Ok(PreparedInput {
        definition_json: mutated_json,
        files,
    })
}

/// Returns the input mode label for status messages.
pub fn mode_label(definition_json: &str) -> &'static str {
    let def: Result<PipelineDefinition, _> = serde_json::from_str(definition_json);
    match def.map(|d| resolve_input_mode(&d)) {
        Ok(InputMode::Url) => "URL input",
        Ok(InputMode::Text) => "text input",
        _ => "files",
    }
}

/// Check if a string looks like an HTTP(S) URL.
pub fn is_url(s: &str) -> bool {
    s.starts_with("http://") || s.starts_with("https://")
}

/// Load files from disk paths, skipping failures. Returns error if none load.
fn load_files(paths: &[String]) -> Result<Vec<PipelineFile>, String> {
    let files: Vec<PipelineFile> = paths
        .iter()
        .filter_map(|path| match io::read_pipeline_file(path) {
            Ok(f) => Some(f),
            Err(e) => {
                eprintln!("Warning: skipping {path}: {e}");
                None
            }
        })
        .collect();

    if files.is_empty() {
        return Err("no valid input files".to_string());
    }
    Ok(files)
}

/// Inject a parameter value into a specific node by ID.
/// Walks the definition tree including container children.
fn inject_param(
    def: &mut PipelineDefinition,
    node_id: &str,
    key: &str,
    value: &serde_json::Value,
) -> Result<(), String> {
    if inject_param_in_nodes(&mut def.nodes, node_id, key, value) {
        Ok(())
    } else {
        Err(format!("Node '{node_id}' not found in definition"))
    }
}

fn inject_param_in_nodes(
    nodes: &mut [PipelineNode],
    node_id: &str,
    key: &str,
    value: &serde_json::Value,
) -> bool {
    for node in nodes.iter_mut() {
        if node.id == node_id {
            node.params.insert(key.to_string(), value.clone());
            return true;
        }
        if let Some(children) = &mut node.children
            && inject_param_in_nodes(children, node_id, key, value)
        {
            return true;
        }
    }
    false
}

/// Apply `--param` overrides to the definition.
///
/// Format: `key=value` targets the default processing node.
/// Format: `nodeId:key=value` targets a specific node.
fn apply_param_overrides(
    def: &mut PipelineDefinition,
    overrides: &[String],
    default_node_id: Option<&str>,
) -> Result<(), String> {
    for entry in overrides {
        let (node_id, key, value) = parse_override(entry, default_node_id)?;
        let json_value = parse_param_value(&value);
        inject_param(def, &node_id, &key, &json_value)?;
    }
    Ok(())
}

/// Parse a `--param` value into (node_id, key, value).
fn parse_override(
    entry: &str,
    default_node_id: Option<&str>,
) -> Result<(String, String, String), String> {
    // Check for nodeId:key=value format
    if let Some((prefix, kv)) = entry.split_once(':') {
        if let Some((key, value)) = kv.split_once('=') {
            return Ok((prefix.to_string(), key.to_string(), value.to_string()));
        }
        return Err(format!(
            "Invalid --param format: {entry}. Expected key=value or nodeId:key=value"
        ));
    }

    // Simple key=value — use default processing node
    if let Some((key, value)) = entry.split_once('=') {
        let node_id = default_node_id
            .ok_or("No processing node found to apply --param override")?
            .to_string();
        return Ok((node_id, key.to_string(), value.to_string()));
    }

    Err(format!(
        "Invalid --param format: {entry}. Expected key=value or nodeId:key=value"
    ))
}

/// Parse a param value string into a serde_json::Value.
/// Tries number, bool, then falls back to string.
fn parse_param_value(s: &str) -> serde_json::Value {
    if let Ok(n) = s.parse::<i64>() {
        return serde_json::Value::Number(n.into());
    }
    if let Ok(f) = s.parse::<f64>()
        && let Some(n) = serde_json::Number::from_f64(f)
    {
        return serde_json::Value::Number(n);
    }
    match s {
        "true" => serde_json::Value::Bool(true),
        "false" => serde_json::Value::Bool(false),
        _ => serde_json::Value::String(s.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- is_url tests ---

    #[test]
    fn test_is_url_valid() {
        assert!(is_url("https://www.youtube.com/watch?v=jNQXAC9IVRw"));
        assert!(is_url("http://example.com"));
        assert!(is_url("https://example.com/path?q=1"));
    }

    #[test]
    fn test_is_url_invalid() {
        assert!(!is_url("ftp://example.com"));
        assert!(!is_url("/path/to/file"));
        assert!(!is_url("file.jpg"));
        assert!(!is_url("example.com"));
    }

    // --- inject_param tests ---

    fn test_def(json: &str) -> PipelineDefinition {
        serde_json::from_str(json).unwrap()
    }

    #[test]
    fn test_inject_param_top_level() {
        let mut def = test_def(
            r#"{
                "formatVersion": "1.0.0",
                "nodes": [
                    { "id": "compress", "type": "image-compress", "params": { "quality": 80 } }
                ]
            }"#,
        );
        inject_param(
            &mut def,
            "compress",
            "quality",
            &serde_json::Value::Number(50.into()),
        )
        .unwrap();
        assert_eq!(def.nodes[0].params["quality"], 50);
    }

    #[test]
    fn test_inject_param_nested() {
        let mut def = test_def(
            r#"{
                "formatVersion": "1.0.0",
                "nodes": [
                    {
                        "id": "group-1",
                        "type": "group",
                        "params": {},
                        "children": [
                            { "id": "download", "type": "video-download", "params": {} }
                        ]
                    }
                ]
            }"#,
        );
        inject_param(
            &mut def,
            "download",
            "url",
            &serde_json::Value::String("https://example.com".to_string()),
        )
        .unwrap();
        let children = def.nodes[0].children.as_ref().unwrap();
        assert_eq!(children[0].params["url"], "https://example.com");
    }

    #[test]
    fn test_inject_param_missing_node() {
        let mut def = test_def(
            r#"{
                "formatVersion": "1.0.0",
                "nodes": [
                    { "id": "compress", "type": "image-compress", "params": {} }
                ]
            }"#,
        );
        let result = inject_param(
            &mut def,
            "nonexistent",
            "key",
            &serde_json::Value::String("val".to_string()),
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    // --- prepare_inputs tests ---

    #[test]
    fn test_prepare_inputs_url() {
        let json = r#"{
            "formatVersion": "1.0.0",
            "nodes": [
                { "id": "in", "type": "input", "params": { "mode": "url" } },
                { "id": "download", "type": "video-download", "params": {} },
                { "id": "out", "type": "output", "params": {} }
            ]
        }"#;
        let args = vec!["https://www.youtube.com/watch?v=jNQXAC9IVRw".to_string()];
        let result = prepare_inputs(json, &args, &[]).unwrap();
        assert!(result.files.is_empty());
        // Verify URL was injected into the definition
        let def: PipelineDefinition = serde_json::from_str(&result.definition_json).unwrap();
        assert_eq!(
            def.nodes[1].params["url"],
            "https://www.youtube.com/watch?v=jNQXAC9IVRw"
        );
    }

    #[test]
    fn test_prepare_inputs_url_validates() {
        let json = r#"{
            "formatVersion": "1.0.0",
            "nodes": [
                { "id": "in", "type": "input", "params": { "mode": "url" } },
                { "id": "download", "type": "video-download", "params": {} },
                { "id": "out", "type": "output", "params": {} }
            ]
        }"#;
        let args = vec!["not-a-url".to_string()];
        let result = prepare_inputs(json, &args, &[]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Expected a URL"));
    }

    #[test]
    fn test_prepare_inputs_url_injects_into_node() {
        let json = r#"{
            "formatVersion": "1.0.0",
            "nodes": [
                { "id": "in", "type": "input", "params": { "mode": "url" } },
                { "id": "download", "type": "video-download", "params": { "format": "mp4" } },
                { "id": "out", "type": "output", "params": {} }
            ]
        }"#;
        let url = "https://www.youtube.com/watch?v=jNQXAC9IVRw";
        let args = vec![url.to_string()];
        let result = prepare_inputs(json, &args, &[]).unwrap();
        let def: PipelineDefinition = serde_json::from_str(&result.definition_json).unwrap();
        // URL injected alongside existing params
        assert_eq!(def.nodes[1].params["url"], url);
        assert_eq!(def.nodes[1].params["format"], "mp4");
    }

    #[test]
    fn test_prepare_inputs_no_args_file_error() {
        let json = r#"{
            "formatVersion": "1.0.0",
            "nodes": [
                { "id": "in", "type": "input", "params": { "mode": "file-upload" } },
                { "id": "compress", "type": "image-compress", "params": {} }
            ]
        }"#;
        let result = prepare_inputs(json, &[], &[]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("input files"));
    }

    #[test]
    fn test_prepare_inputs_no_args_url_error() {
        let json = r#"{
            "formatVersion": "1.0.0",
            "nodes": [
                { "id": "in", "type": "input", "params": { "mode": "url" } },
                { "id": "download", "type": "video-download", "params": {} }
            ]
        }"#;
        let result = prepare_inputs(json, &[], &[]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("URL"));
    }

    // --- apply_overrides tests ---

    #[test]
    fn test_apply_overrides_simple() {
        let json = r#"{
            "formatVersion": "1.0.0",
            "nodes": [
                { "id": "in", "type": "input", "params": {} },
                { "id": "compress", "type": "image-compress", "params": { "quality": 80 } },
                { "id": "out", "type": "output", "params": {} }
            ]
        }"#;
        let mut def: PipelineDefinition = serde_json::from_str(json).unwrap();
        let node_id = first_processing_node_id(&def);
        apply_param_overrides(&mut def, &["quality=50".to_string()], node_id.as_deref()).unwrap();
        assert_eq!(def.nodes[1].params["quality"], 50);
    }

    #[test]
    fn test_apply_overrides_qualified() {
        let json = r#"{
            "formatVersion": "1.0.0",
            "nodes": [
                { "id": "in", "type": "input", "params": {} },
                { "id": "download", "type": "video-download", "params": {} },
                { "id": "out", "type": "output", "params": {} }
            ]
        }"#;
        let mut def: PipelineDefinition = serde_json::from_str(json).unwrap();
        apply_param_overrides(
            &mut def,
            &["download:url=https://example.com".to_string()],
            None,
        )
        .unwrap();
        assert_eq!(def.nodes[1].params["url"], "https://example.com");
    }

    // --- parse_param_value tests ---

    #[test]
    fn test_parse_param_value_integer() {
        assert_eq!(parse_param_value("50"), serde_json::json!(50));
    }

    #[test]
    fn test_parse_param_value_float() {
        assert_eq!(parse_param_value("0.5"), serde_json::json!(0.5));
    }

    #[test]
    fn test_parse_param_value_bool() {
        assert_eq!(parse_param_value("true"), serde_json::json!(true));
        assert_eq!(parse_param_value("false"), serde_json::json!(false));
    }

    #[test]
    fn test_parse_param_value_string() {
        assert_eq!(parse_param_value("hello"), serde_json::json!("hello"));
    }
}
