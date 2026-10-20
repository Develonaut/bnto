// Result metadata — records the original/new filename and which transforms ran.

/// Param keys that mark a transform as applied, paired with their display label.
/// Order matters: it is the order labels appear in `transformsApplied`.
const TRANSFORM_LABELS: &[(&str, &str)] = &[
    ("sanitize", "sanitize"),
    ("find", "find/replace"),
    ("case", "case"),
    ("prefix", "prefix"),
    ("suffix", "suffix"),
    ("pattern", "pattern"),
    ("extension", "extension"),
];

/// Build the node-level metadata map for a single rename.
pub(super) fn build_rename_metadata(
    original_filename: &str,
    new_filename: &str,
    params: &serde_json::Map<String, serde_json::Value>,
) -> serde_json::Map<String, serde_json::Value> {
    let mut metadata = serde_json::Map::new();
    metadata.insert(
        "originalFilename".to_string(),
        serde_json::Value::String(original_filename.to_string()),
    );
    metadata.insert(
        "newFilename".to_string(),
        serde_json::Value::String(new_filename.to_string()),
    );
    metadata.insert("transformsApplied".to_string(), transforms_value(params));
    metadata
}

fn transforms_value(params: &serde_json::Map<String, serde_json::Value>) -> serde_json::Value {
    serde_json::Value::Array(
        build_transforms_list(params)
            .into_iter()
            .map(serde_json::Value::String)
            .collect(),
    )
}

/// Build a list of which transformations were applied, for metadata display.
fn build_transforms_list(params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
    let mut transforms: Vec<String> = TRANSFORM_LABELS
        .iter()
        .filter(|(key, _)| params.contains_key(*key))
        .map(|(_, label)| label.to_string())
        .collect();
    if params.contains_key("counterStart") || params.contains_key("counterPad") {
        transforms.push("counter".to_string());
    }
    transforms
}
