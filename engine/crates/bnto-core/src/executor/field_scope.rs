// Fields in scope: a container's declared fields reach every step inside it,
// and the nearest declaration of a name wins (decisions/conditional-steps.md).
// The executor, `bnto check` and the CLI's dry run all read fields this way.

use std::collections::BTreeMap;

use serde_json::Value;

use super::PipelineContext;
use super::resolve::collect_field_values;
use crate::field_def::FieldDefs;
use crate::pipeline::PipelineNode;

/// `{{fields.*}}` values by field name.
pub type FieldValues = BTreeMap<String, Value>;

/// `inherited` with `fields` declared on top. A declared name replaces the
/// inherited value even when it has none of its own, so a step never reads
/// a container's value for a field it declares itself.
pub fn declare(inherited: &FieldValues, fields: &FieldDefs) -> FieldValues {
    let mut values = inherited.clone();
    values.retain(|name, _| !fields.contains_key(name));
    values.extend(collect_field_values(fields, &FieldValues::new()));
    values
}

/// The field values the last node of `path` sees: its own and its
/// containers' (`path` lists them outermost first).
pub fn along(path: &[&PipelineNode]) -> FieldValues {
    path.iter().fold(FieldValues::new(), |values, node| {
        declare(&values, &node.fields)
    })
}

impl<F: Fn() -> u64 + Copy> PipelineContext<'_, F> {
    /// The field values `node` sees: its containers', then its own.
    pub(super) fn fields_for(&self, node: &PipelineNode) -> FieldValues {
        declare(&self.fields, &node.fields)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn node(json: Value) -> PipelineNode {
        serde_json::from_value(json).unwrap()
    }

    fn with_name(id: &str, default: Option<&str>) -> PipelineNode {
        let mut field = json!({ "type": "string", "label": "Name" });
        if let Some(default) = default {
            field["default"] = json!(default);
        }
        node(json!({ "id": id, "type": "group", "fields": { "name": field } }))
    }

    #[test]
    fn a_node_without_fields_sees_its_containers() {
        let (group, step) = (
            with_name("g", Some("outer")),
            node(json!({ "id": "s", "type": "x" })),
        );
        assert_eq!(along(&[&group, &step])["name"], "outer");
    }

    #[test]
    fn the_nearest_declaration_wins() {
        let (outer, inner) = (with_name("o", Some("outer")), with_name("i", Some("inner")));
        assert_eq!(along(&[&outer, &inner])["name"], "inner");
    }

    #[test]
    fn a_declaration_with_no_value_still_shadows() {
        let (outer, inner) = (with_name("o", Some("outer")), with_name("i", None));
        assert!(!along(&[&outer, &inner]).contains_key("name"));
    }

    #[test]
    fn an_empty_path_has_no_fields() {
        assert!(along(&[]).is_empty());
    }
}
