// Migration: drop params a node no longer has. Keys are dropped only on the
// node types that owned them, so another node's param, or a value nested
// inside a param, that happens to share a name is left alone. The removals
// are `bnto_core::renamed::REMOVED_PARAMS`, the table `bnto check` reads to
// point an old recipe here.

use bnto_core::renamed::REMOVED_PARAMS;
use serde_json::Value;

use super::{mutate_nodes, visit_nodes};

/// Recipes accept a node's params under either key.
const PARAM_KEYS: [&str; 2] = ["parameters", "params"];

/// True if any node still carries a param its node type no longer has.
pub(super) fn has_removed_params(value: &Value) -> bool {
    visit_nodes(value, &|node| {
        let node_type = node.get("type").and_then(Value::as_str);
        PARAM_KEYS.iter().any(|key| {
            node.get(*key)
                .and_then(Value::as_object)
                .is_some_and(|params| removed(node_type).any(|p| params.contains_key(p)))
        })
    })
}

/// Drop each node's removed params.
pub(super) fn drop_removed_params(value: &mut Value) {
    mutate_nodes(value, &|node| {
        let node_type = node.get("type").and_then(Value::as_str).map(str::to_owned);
        for key in PARAM_KEYS {
            if let Some(params) = node.get_mut(key).and_then(Value::as_object_mut) {
                for param in removed(node_type.as_deref()) {
                    params.remove(param);
                }
            }
        }
    });
}

/// The params `node_type` no longer has.
fn removed(node_type: Option<&str>) -> impl Iterator<Item = &'static str> {
    REMOVED_PARAMS
        .iter()
        .filter(move |(t, ..)| Some(*t) == node_type)
        .map(|(_, param, _)| *param)
}

#[cfg(test)]
mod tests {
    use super::super::apply_migrations;
    use serde_json::json;

    const DESCRIPTION: &str = "Remove params their nodes no longer have";

    #[test]
    fn drops_vector_optimizes_precision_and_keeps_its_other_params() {
        let mut value = json!({
            "id": "root",
            "type": "group",
            "nodes": [{ "id": "o", "type": "vector-optimize",
                        "parameters": { "minify": true, "precision": 3 } }]
        });

        assert_eq!(apply_migrations(&mut value), [DESCRIPTION]);
        assert_eq!(value["nodes"][0]["parameters"], json!({ "minify": true }));
    }

    #[test]
    fn leaves_a_same_named_param_on_another_node_type_alone() {
        let mut value = json!({
            "id": "root",
            "type": "group",
            "nodes": [
                { "id": "x", "type": "edit-fields", "parameters": { "values": { "precision": "2" } } },
                { "id": "y", "type": "shell-run", "parameters": { "precision": 3 } }
            ]
        });
        let before = value.clone();

        assert!(apply_migrations(&mut value).is_empty());
        assert_eq!(value, before);
    }

    #[test]
    fn drops_under_the_params_alias_and_in_nested_children() {
        let mut value = json!({
            "id": "root",
            "type": "group",
            "nodes": [{
                "id": "loop",
                "type": "loop",
                "nodes": [{ "id": "o", "type": "vector-optimize", "params": { "precision": 2 } }]
            }],
            "children": [{ "id": "o2", "type": "vector-optimize",
                           "parameters": { "precision": 1, "minify": false } }]
        });

        apply_migrations(&mut value);

        assert_eq!(value["nodes"][0]["nodes"][0]["params"], json!({}));
        assert_eq!(
            value["children"][0]["parameters"],
            json!({ "minify": false })
        );
    }

    #[test]
    fn removed_params_migration_is_idempotent() {
        let mut value = json!({
            "id": "root",
            "type": "group",
            "nodes": [{ "id": "o", "type": "vector-optimize", "parameters": { "precision": 3 } }]
        });

        apply_migrations(&mut value);
        let snapshot = value.clone();

        assert!(apply_migrations(&mut value).is_empty());
        assert_eq!(value, snapshot);
    }
}
