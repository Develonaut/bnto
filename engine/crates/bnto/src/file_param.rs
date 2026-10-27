// `-p` gives a file param (`ParameterType::File`, like Watermark Images'
// overlay) as a path: the CLI reads the file and hands the node its base64
// text, the form the web's upload control sends (ISS-75).

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use bnto_core::metadata::{ParameterDef, ParameterType};
use bnto_core::{NodeRegistry, PipelineNode};

/// The value `-p node_id:key=value` gives a param `node_id`'s type declares
/// as a file: `value` read as a path and base64-encoded. `Ok(None)` when the
/// param isn't a file param (or the node isn't found), so `value` stands,
/// and for an empty value, which leaves the param unfilled for the run's
/// required-param check to refuse.
pub fn read_file_param(
    nodes: &[PipelineNode],
    registry: &NodeRegistry,
    node_id: &str,
    key: &str,
    value: &str,
) -> Result<Option<String>, String> {
    if value.is_empty() {
        return Ok(None);
    }
    let Some(node) = find_node(nodes, node_id) else {
        return Ok(None);
    };
    let Some(def) = file_param_def(registry, node, key) else {
        return Ok(None);
    };
    // Worded like the engine's own param refusals, with no CLI flag in it:
    // the TUI's form fills params through the same overrides.
    let bytes = std::fs::read(value).map_err(|e| {
        format!(
            "Node '{node_id}': param \"{key}\" ({}): can't read {value}: {e}",
            def.label
        )
    })?;
    Ok(Some(BASE64.encode(bytes)))
}

/// `key`'s definition when `node`'s type declares it as a file param.
fn file_param_def(registry: &NodeRegistry, node: &PipelineNode, key: &str) -> Option<ParameterDef> {
    let processor = registry.resolve(&node.node_type, &node.params)?;
    processor
        .metadata()
        .parameters
        .into_iter()
        .find(|def| def.name == key && matches!(def.param_type, ParameterType::File { .. }))
}

/// The node `-p` sets for `id`: the first in order, each node's children
/// searched before the next node (as `input::inject_param` walks).
fn find_node<'a>(nodes: &'a [PipelineNode], id: &str) -> Option<&'a PipelineNode> {
    nodes.iter().find_map(|node| {
        if node.id == id {
            return Some(node);
        }
        find_node(node.children.as_deref().unwrap_or_default(), id)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nodes(json: &str) -> Vec<PipelineNode> {
        serde_json::from_str(json).unwrap()
    }

    #[test]
    fn test_a_file_param_reads_its_path_as_base64() {
        let dir = tempfile::tempdir().unwrap();
        let logo = dir.path().join("logo.png");
        std::fs::write(&logo, b"png bytes").unwrap();
        let nodes = nodes(r#"[{ "id": "wm", "type": "image-overlay", "params": {} }]"#);
        let registry = bnto_engine::create_registry();

        let value = read_file_param(&nodes, &registry, "wm", "overlay", logo.to_str().unwrap());

        assert_eq!(value, Ok(Some(BASE64.encode(b"png bytes"))));
    }

    #[test]
    fn test_a_file_param_in_a_container_is_found() {
        let dir = tempfile::tempdir().unwrap();
        let logo = dir.path().join("logo.png");
        std::fs::write(&logo, b"x").unwrap();
        let nodes = nodes(
            r#"[{ "id": "g", "type": "group", "params": {}, "children": [
                { "id": "wm", "type": "image-overlay", "params": {} }
            ] }]"#,
        );
        let registry = bnto_engine::create_registry();

        let value = read_file_param(&nodes, &registry, "wm", "overlay", logo.to_str().unwrap());

        assert_eq!(value, Ok(Some(BASE64.encode(b"x"))));
    }

    #[test]
    fn test_another_param_keeps_its_text() {
        let nodes = nodes(r#"[{ "id": "wm", "type": "image-overlay", "params": {} }]"#);
        let registry = bnto_engine::create_registry();

        let value = read_file_param(&nodes, &registry, "wm", "position", "center");

        assert_eq!(value, Ok(None));
    }

    #[test]
    fn test_an_empty_file_param_stays_unfilled() {
        let nodes = nodes(r#"[{ "id": "wm", "type": "image-overlay", "params": {} }]"#);
        let registry = bnto_engine::create_registry();

        let value = read_file_param(&nodes, &registry, "wm", "overlay", "");

        assert_eq!(value, Ok(None));
    }

    #[test]
    fn test_a_missing_file_names_the_param_and_path() {
        let dir = tempfile::tempdir().unwrap();
        let missing = dir.path().join("missing.png");
        let nodes = nodes(r#"[{ "id": "wm", "type": "image-overlay", "params": {} }]"#);
        let registry = bnto_engine::create_registry();

        let err = read_file_param(
            &nodes,
            &registry,
            "wm",
            "overlay",
            missing.to_str().unwrap(),
        )
        .unwrap_err();

        assert!(
            err.starts_with("Node 'wm': param \"overlay\" (Overlay Image): can't read "),
            "{err}"
        );
        assert!(err.contains("missing.png"), "{err}");
    }
}
