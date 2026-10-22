// Which node `--param key=value` sets: the one that declares field `key`.
// A container's fields reach every step inside it, so a field may live on a
// group rather than on the first step. Shared by `bnto run` and `bnto dry-run`.

use bnto_core::PipelineNode;

/// The id of the node that declares field `key`, anywhere in `nodes`.
/// `Ok(None)` when none does; an error naming each one when several do,
/// since `key=value` can't say which it means.
pub fn field_owner(nodes: &[PipelineNode], key: &str) -> Result<Option<String>, String> {
    let mut owners = Vec::new();
    collect_owners(nodes, key, &mut owners);
    match owners.as_slice() {
        [] => Ok(None),
        [owner] => Ok(Some(owner.clone())),
        _ => Err(ambiguous(key, &owners)),
    }
}

fn collect_owners(nodes: &[PipelineNode], key: &str, owners: &mut Vec<String>) {
    for node in nodes {
        if node.fields.contains_key(key) {
            owners.push(node.id.clone());
        }
        if let Some(children) = &node.children {
            collect_owners(children, key, owners);
        }
    }
}

fn ambiguous(key: &str, owners: &[String]) -> String {
    let named: Vec<String> = owners.iter().map(|id| format!("\"{id}\"")).collect();
    let forms: Vec<String> = owners.iter().map(|id| format!("{id}:{key}=…")).collect();
    format!(
        "--param {key}: nodes {} each declare a field \"{key}\". Name one: {}",
        named.join(", "),
        forms.join(" or ")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn nodes(json: serde_json::Value) -> Vec<PipelineNode> {
        serde_json::from_value(json).unwrap()
    }

    fn declaring(id: &str, key: &str) -> serde_json::Value {
        json!({ "id": id, "type": "shell-run",
            "fields": { key: { "type": "string", "label": "X" } } })
    }

    #[test]
    fn no_declaration_is_none() {
        let ns = nodes(json!([declaring("a", "mode")]));
        assert_eq!(field_owner(&ns, "size"), Ok(None));
    }

    #[test]
    fn finds_a_declaration_inside_a_container() {
        let ns =
            nodes(json!([{ "id": "g", "type": "group", "nodes": [declaring("deep", "mode")] }]));
        assert_eq!(field_owner(&ns, "mode"), Ok(Some("deep".into())));
    }

    #[test]
    fn a_container_can_be_the_owner() {
        let mut group = declaring("g", "mode");
        group["type"] = json!("group");
        group["nodes"] = json!([{ "id": "s", "type": "shell-run" }]);
        assert_eq!(
            field_owner(&nodes(json!([group])), "mode"),
            Ok(Some("g".into()))
        );
    }

    #[test]
    fn two_declarations_name_both_and_how_to_pick() {
        let ns = nodes(json!([declaring("a", "mode"), declaring("b", "mode")]));
        let err = field_owner(&ns, "mode").unwrap_err();
        assert!(err.contains("\"a\", \"b\""), "{err}");
        assert!(err.contains("a:mode=… or b:mode=…"), "{err}");
    }
}
