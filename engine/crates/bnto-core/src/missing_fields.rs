// `{{fields.<name>}}` references nothing fills. A field resolves only from
// its own node or a container around it (executor/field_scope.rs), and an
// unset one stays as the placeholder text, so a step would run on the literal
// `{{fields.x}}` and can no-op with exit 0 (ISS-1). A run's pre-flight refuses
// them, and `bnto check` reports the ones no run can fill.

use std::collections::BTreeMap;

use serde_json::Value;

use crate::executor::field_scope::{FieldValues, declare};
use crate::pipeline::PipelineNode;
use crate::placeholders::data_placeholders;

/// A `{{fields.<name>}}` in a node's params or guard that no value fills.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissingField {
    /// The node whose params or guard use the field.
    pub node_id: String,
    pub node_type: String,
    pub field: String,
    /// The nearest node in scope that declares the field with no value, or
    /// `None` when neither the node nor a container around it declares it.
    pub declared_by: Option<String>,
}

/// Every unfilled field reference in `nodes`, depth first in recipe order.
/// Guards are ignored, like the rest of pre-flight: a branch for another OS
/// can't hide one.
pub fn missing_fields(nodes: &[PipelineNode]) -> Vec<MissingField> {
    let mut found = Vec::new();
    walk(nodes, &Scope::default(), &mut found);
    found
}

impl MissingField {
    /// What's wrong, for a run's error or a `bnto check` finding.
    pub fn message(&self) -> String {
        let placeholder = format!("{{{{fields.{}}}}}", self.field);
        match &self.declared_by {
            Some(owner) => format!(
                "{placeholder} has no value: field '{}' on node '{owner}' has no default, \
                 and the run didn't set it",
                self.field
            ),
            None => format!(
                "{placeholder} names a field neither this node nor a container around it \
                 declares"
            ),
        }
    }
}

/// What a node sees: field values, and the nearest node declaring each name.
#[derive(Default)]
struct Scope {
    values: FieldValues,
    declarers: BTreeMap<String, String>,
}

impl Scope {
    /// The scope inside `node`: its own fields declared on top.
    fn enter(&self, node: &PipelineNode) -> Self {
        let mut declarers = self.declarers.clone();
        for name in node.fields.keys() {
            declarers.insert(name.clone(), node.id.clone());
        }
        Self {
            values: declare(&self.values, &node.fields),
            declarers,
        }
    }
}

fn walk(nodes: &[PipelineNode], scope: &Scope, found: &mut Vec<MissingField>) {
    for node in nodes {
        let inner = scope.enter(node);
        found.extend(missing_in(node, &inner));
        if let Some(children) = &node.children {
            walk(children, &inner, found);
        }
    }
}

/// The fields `node`'s params and guard use that `scope` has no value for.
fn missing_in(node: &PipelineNode, scope: &Scope) -> Vec<MissingField> {
    let mut names = Vec::new();
    node.params.values().for_each(|v| field_refs(v, &mut names));
    if let Some(when) = &node.when {
        field_refs(&serde_json::to_value(when).unwrap_or_default(), &mut names);
    }
    names
        .into_iter()
        .filter(|name| !scope.values.contains_key(name))
        .map(|field| MissingField {
            node_id: node.id.clone(),
            node_type: node.node_type.clone(),
            declared_by: scope.declarers.get(&field).cloned(),
            field,
        })
        .collect()
}

/// Add each `{{fields.<name>}}` under `value` to `names`, once, in order.
fn field_refs(value: &Value, names: &mut Vec<String>) {
    match value {
        Value::String(s) => {
            let fields = data_placeholders(s)
                .into_iter()
                .filter_map(|p| p.strip_prefix("{{fields.")?.strip_suffix("}}"));
            for name in fields {
                if !names.iter().any(|n| n == name) {
                    names.push(name.to_string());
                }
            }
        }
        Value::Array(items) => items.iter().for_each(|v| field_refs(v, names)),
        Value::Object(map) => map.values().for_each(|v| field_refs(v, names)),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn nodes(value: serde_json::Value) -> Vec<PipelineNode> {
        serde_json::from_value(value).unwrap()
    }

    /// (node, field, declared by) for each missing field in `ns`.
    fn found(ns: &[PipelineNode]) -> Vec<(String, String, Option<String>)> {
        missing_fields(ns)
            .into_iter()
            .map(|m| (m.node_id, m.field, m.declared_by))
            .collect()
    }

    fn missing(
        node: &str,
        field: &str,
        declared_by: Option<&str>,
    ) -> (String, String, Option<String>) {
        (node.into(), field.into(), declared_by.map(str::to_string))
    }

    #[test]
    fn a_field_only_a_sibling_declares_is_missing_and_undeclared() {
        // download-patreon's shape: `flatten` reads a field `write-config` owns.
        let ns = nodes(json!([
            { "id": "write-config", "type": "shell-run",
              "fields": { "outDir": { "type": "string", "label": "Out", "default": "/tmp" } },
              "parameters": { "command": "echo", "args": ["{{fields.outDir}}"] } },
            { "id": "flatten", "type": "shell-run",
              "parameters": { "command": "ls", "args": ["{{fields.outDir}}"] } }
        ]));
        assert_eq!(found(&ns), vec![missing("flatten", "outDir", None)]);
    }

    #[test]
    fn a_container_field_with_no_value_is_missing_and_names_its_declarer() {
        // release-verify's shape: the group declares `version` with no default.
        let ns = nodes(json!([{
            "id": "checks", "type": "group",
            "fields": { "version": { "type": "string", "label": "Version" } },
            "nodes": [{ "id": "crates", "type": "shell-run",
                        "parameters": { "command": "echo", "args": ["{{fields.version}}"] } }]
        }]));
        assert_eq!(
            found(&ns),
            vec![missing("crates", "version", Some("checks"))]
        );
    }

    #[test]
    fn a_container_field_with_a_value_fills_its_children() {
        // keep-awake's shape.
        let ns = nodes(json!([{
            "id": "session", "type": "group",
            "fields": { "action": { "type": "string", "label": "Action", "default": "start" } },
            "nodes": [{ "id": "caffeinate", "type": "shell-run",
                        "parameters": { "command": "echo", "args": ["{{fields.action}}"] } }]
        }]));
        assert!(found(&ns).is_empty());
    }

    #[test]
    fn an_empty_default_counts_as_a_value() {
        let ns = nodes(json!([{
            "id": "dl", "type": "shell-run",
            "fields": { "browser": { "type": "string", "label": "Browser", "default": "" } },
            "parameters": { "command": "echo", "args": ["{{fields.browser}}"] }
        }]));
        assert!(found(&ns).is_empty());
    }

    #[test]
    fn a_guard_that_uses_a_missing_field_is_reported() {
        let ns = nodes(json!([{
            "id": "stop", "type": "shell-run",
            "when": { "value": "{{fields.action}}", "op": "equals", "expected": "stop" },
            "parameters": { "command": "echo" }
        }]));
        assert_eq!(found(&ns), vec![missing("stop", "action", None)]);
    }

    #[test]
    fn a_branch_for_another_os_is_still_checked() {
        let ns = nodes(json!([{
            "id": "windows", "type": "group",
            "when": { "value": "{{ctx.platform}}", "op": "equals", "expected": "windows" },
            "nodes": [{ "id": "wake", "type": "shell-run",
                        "parameters": { "command": "echo", "args": ["{{fields.duration}}"] } }]
        }]));
        assert_eq!(found(&ns), vec![missing("wake", "duration", None)]);
    }

    #[test]
    fn a_step_declaring_a_field_with_no_value_hides_its_containers_value() {
        let ns = nodes(json!([{
            "id": "outer", "type": "group",
            "fields": { "name": { "type": "string", "label": "Name", "default": "a" } },
            "nodes": [{ "id": "inner", "type": "shell-run",
                        "fields": { "name": { "type": "string", "label": "Name" } },
                        "parameters": { "command": "echo", "args": ["{{fields.name}}"] } }]
        }]));
        assert_eq!(found(&ns), vec![missing("inner", "name", Some("inner"))]);
    }

    #[test]
    fn a_container_param_is_checked_against_its_own_fields() {
        let ns = nodes(json!([{
            "id": "each", "type": "loop",
            "fields": { "dir": { "type": "string", "label": "Dir", "default": "in" } },
            "parameters": { "over": "{{fields.dir}}", "label": "{{fields.nope}}" },
            "nodes": []
        }]));
        assert_eq!(found(&ns), vec![missing("each", "nope", None)]);
    }

    #[test]
    fn each_missing_field_is_reported_once_per_node() {
        let ns = nodes(json!([{
            "id": "dl", "type": "shell-run",
            "parameters": { "command": "echo",
                            "args": ["{{fields.a}}", "{{fields.a}}-{{fields.b}}"] }
        }]));
        assert_eq!(
            found(&ns),
            vec![missing("dl", "a", None), missing("dl", "b", None)]
        );
    }

    #[test]
    fn other_placeholders_are_not_fields() {
        let ns = nodes(json!([{
            "id": "dl", "type": "shell-run",
            "parameters": { "command": "echo",
                            "args": ["{{item.name}}", "{{vars.X}}", "{{url}}", "{{fields.*}}"] }
        }]));
        assert!(found(&ns).is_empty());
    }

    #[test]
    fn the_message_names_the_declaring_node_or_says_none_declares_it() {
        let unset = MissingField {
            node_id: "crates".into(),
            node_type: "shell-run".into(),
            field: "version".into(),
            declared_by: Some("checks".into()),
        };
        assert_eq!(
            unset.message(),
            "{{fields.version}} has no value: field 'version' on node 'checks' has no \
             default, and the run didn't set it"
        );
        let undeclared = MissingField {
            declared_by: None,
            ..unset
        };
        assert_eq!(
            undeclared.message(),
            "{{fields.version}} names a field neither this node nor a container around it \
             declares"
        );
    }
}
