// Pre-flight (SEC-13): every node's field values must fit their definitions
// before any step runs, whatever set them — `--param`, the TUI form, vars or
// the recipe itself. Checked up front so a bad value never runs half a recipe.

use crate::errors::BntoError;
use crate::pipeline::PipelineNode;

/// Refuse the run when any field value, in any node, breaks its definition.
pub(super) fn check_field_values(nodes: &[PipelineNode]) -> Result<(), BntoError> {
    for node in nodes {
        for (name, field) in &node.fields {
            field.check_current().map_err(|e| {
                BntoError::InvalidInput(format!("Node '{}', field '{name}': {e}", node.id))
            })?;
        }
        if let Some(children) = &node.children {
            check_field_values(children)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nodes(json: serde_json::Value) -> Vec<PipelineNode> {
        serde_json::from_value(json).unwrap()
    }

    fn mode_field(default: &str) -> serde_json::Value {
        serde_json::json!({
            "mode": {
                "type": "enum", "label": "Mode", "default": default,
                "options": [{ "value": "a", "label": "A" }, { "value": "b", "label": "B" }]
            }
        })
    }

    #[test]
    fn listed_values_pass() {
        let ns = nodes(serde_json::json!([{ "id": "n", "type": "t", "fields": mode_field("b") }]));
        assert!(check_field_values(&ns).is_ok());
    }

    #[test]
    fn an_unlisted_value_is_refused_naming_node_and_field() {
        let ns = nodes(serde_json::json!([{ "id": "n", "type": "t", "fields": mode_field("-x") }]));
        let err = check_field_values(&ns).unwrap_err().to_string();
        assert!(err.contains("Node 'n', field 'mode'"), "{err}");
    }

    #[test]
    fn nested_nodes_are_checked() {
        let ns = nodes(serde_json::json!([{
            "id": "loop", "type": "loop",
            "children": [{ "id": "inner", "type": "t", "fields": mode_field("-x") }]
        }]));
        assert!(check_field_values(&ns).is_err());
    }
}
