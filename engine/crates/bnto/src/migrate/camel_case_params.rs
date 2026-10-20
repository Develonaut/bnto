// Migration: snake_case params → camelCase, the casing the rest of the
// recipe format uses (see `.claude/scopes/rust/node-vocabulary.md`).
// Keys are renamed only on the node types that own them, so another node's
// param, or a value nested inside a param, that happens to share a name is
// left alone. The renames are the snake_case rows of `bnto_core::renamed`,
// the table `bnto check` reads to point an old recipe here.

use bnto_core::renamed::PARAMS;
use serde_json::{Map, Value};

use super::{mutate_nodes, visit_nodes};

/// `(node type, old key, new key)` for each snake_case param rename.
fn renames() -> impl Iterator<Item = &'static (&'static str, &'static str, &'static str)> {
    PARAMS.iter().filter(|(_, old, _)| old.contains('_'))
}

/// Recipes accept a node's params under either key.
const PARAM_KEYS: [&str; 2] = ["parameters", "params"];

/// True if any node still carries one of the old snake_case keys.
pub(super) fn has_snake_case_params(value: &Value) -> bool {
    visit_nodes(value, &|node| {
        let node_type = node.get("type").and_then(Value::as_str);
        PARAM_KEYS.iter().any(|key| {
            node.get(*key)
                .and_then(Value::as_object)
                .is_some_and(|params| old_keys(node_type).any(|old| params.contains_key(old)))
        })
    })
}

/// Rename each node's old keys. When a node has both spellings, the
/// camelCase one wins: it is the one the engine reads.
pub(super) fn rename_snake_case_params(value: &mut Value) {
    mutate_nodes(value, &|node| {
        let node_type = node.get("type").and_then(Value::as_str).map(str::to_owned);
        for key in PARAM_KEYS {
            if let Some(params) = node.get_mut(key).and_then(Value::as_object_mut) {
                rename_keys(params, node_type.as_deref());
            }
        }
    });
}

fn rename_keys(params: &mut Map<String, Value>, node_type: Option<&str>) {
    for (_, old, new) in renames().filter(|(t, ..)| Some(*t) == node_type) {
        if let Some(val) = params.remove(*old) {
            params.entry(new.to_string()).or_insert(val);
        }
    }
}

fn old_keys(node_type: Option<&str>) -> impl Iterator<Item = &'static str> {
    renames()
        .filter(move |(t, ..)| Some(*t) == node_type)
        .map(|(_, old, _)| *old)
}

#[cfg(test)]
mod tests {
    use super::super::apply_migrations;
    use serde_json::json;

    #[test]
    fn renames_every_snake_case_param_on_its_own_node() {
        let mut value = json!({
            "id": "root",
            "type": "group",
            "nodes": [
                { "id": "c", "type": "file-copy",
                  "parameters": { "create_dirs": false, "tolerance_ms": 500, "destination": "out" } },
                { "id": "f", "type": "file-filter",
                  "parameters": { "name_pattern": "*.jpg", "pattern_mode": "glob", "min_size": 1, "max_size": 9 } },
                { "id": "i", "type": "file-inspect", "parameters": { "include_hash": true } },
                { "id": "m", "type": "file-move", "parameters": { "create_dirs": true } },
                { "id": "r", "type": "file-rename",
                  "parameters": { "max_length": 40, "counter_start": 2, "counter_pad": 3 } },
                { "id": "s", "type": "shell-run", "parameters": { "search_path": ["bin"] } },
                { "id": "e", "type": "spreadsheet-export", "parameters": { "name_column": "path" } },
                { "id": "l", "type": "spreadsheet-lookup",
                  "parameters": { "key_column": "name", "match_on": "filename" } }
            ]
        });

        let applied = apply_migrations(&mut value);
        assert_eq!(applied, ["Rename snake_case params to camelCase"]);

        let params = |i: usize| value["nodes"][i]["parameters"].clone();
        assert_eq!(
            params(0),
            json!({ "createDirs": false, "toleranceMs": 500, "destination": "out" })
        );
        assert_eq!(
            params(1),
            json!({ "namePattern": "*.jpg", "patternMode": "glob", "minSize": 1, "maxSize": 9 })
        );
        assert_eq!(params(2), json!({ "includeHash": true }));
        assert_eq!(params(3), json!({ "createDirs": true }));
        assert_eq!(
            params(4),
            json!({ "maxLength": 40, "counterStart": 2, "counterPad": 3 })
        );
        assert_eq!(params(5), json!({ "searchPath": ["bin"] }));
        assert_eq!(params(6), json!({ "nameColumn": "path" }));
        assert_eq!(
            params(7),
            json!({ "keyColumn": "name", "matchOn": "filename" })
        );
    }

    #[test]
    fn renames_under_the_params_alias_and_in_nested_children() {
        let mut value = json!({
            "id": "root",
            "type": "group",
            "nodes": [{
                "id": "loop",
                "type": "loop",
                "nodes": [{ "id": "f", "type": "file-filter", "params": { "min_size": 5 } }]
            }],
            "children": [{ "id": "s", "type": "shell-run", "params": { "search_path": ["x"] } }]
        });

        apply_migrations(&mut value);

        assert_eq!(
            value["nodes"][0]["nodes"][0]["params"],
            json!({ "minSize": 5 })
        );
        assert_eq!(
            value["children"][0]["params"],
            json!({ "searchPath": ["x"] })
        );
    }

    #[test]
    fn leaves_a_same_named_key_on_another_node_type_alone() {
        let mut value = json!({
            "id": "root",
            "type": "group",
            "nodes": [
                { "id": "x", "type": "edit-fields", "parameters": { "values": { "min_size": "1" } } },
                { "id": "y", "type": "file-sort", "parameters": { "create_dirs": true } }
            ]
        });
        let before = value.clone();

        assert!(apply_migrations(&mut value).is_empty());
        assert_eq!(value, before);
    }

    #[test]
    fn camel_case_wins_when_a_node_has_both_spellings() {
        let mut value = json!({
            "id": "root",
            "type": "group",
            "nodes": [{ "id": "c", "type": "file-copy",
                        "parameters": { "create_dirs": false, "createDirs": true } }]
        });

        apply_migrations(&mut value);

        assert_eq!(
            value["nodes"][0]["parameters"],
            json!({ "createDirs": true })
        );
    }

    #[test]
    fn runs_after_the_type_renames_so_old_types_get_new_keys() {
        let mut value = json!({
            "id": "root",
            "type": "group",
            "nodes": [
                { "id": "i", "type": "file-metadata", "parameters": { "include_hash": true } },
                { "id": "s", "type": "shell-command", "parameters": { "search_path": ["bin"] } }
            ]
        });

        apply_migrations(&mut value);

        assert_eq!(value["nodes"][0]["type"], json!("file-inspect"));
        assert_eq!(
            value["nodes"][0]["parameters"],
            json!({ "includeHash": true })
        );
        assert_eq!(value["nodes"][1]["type"], json!("shell-run"));
        assert_eq!(
            value["nodes"][1]["parameters"],
            json!({ "searchPath": ["bin"] })
        );
    }

    #[test]
    fn camel_case_migration_is_idempotent() {
        let mut value = json!({
            "id": "root",
            "type": "group",
            "nodes": [{ "id": "f", "type": "file-filter", "parameters": { "max_size": 9 } }]
        });

        apply_migrations(&mut value);
        let snapshot = value.clone();

        assert!(apply_migrations(&mut value).is_empty());
        assert_eq!(value, snapshot);
    }
}
