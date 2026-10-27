// `bnto run`, `bnto dry-run` and a TUI run stop before consent when a
// `{{fields.*}}` has nothing to fill it, and say how to fill it: the `-p`
// that would on the command line, the form field in the TUI (ISS-97). The engine's pre-flight
// refuses the same run (`bnto_core::missing_fields`); checking here first
// means nobody approves a run that would be refused.

use bnto_core::first_processing_node_id;
use bnto_core::missing_fields::{MissingField, missing_fields};
use bnto_core::{FieldDef, PipelineDefinition, PipelineNode};

use crate::input::apply_param_overrides;

/// How a refusal says to fill a field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FillWith {
    /// The command line: `-p field=...`.
    Flag,
    /// The TUI: the recipe's form, which shows each field by its label.
    Form,
}

/// Why `definition_json`, with `param_overrides` applied, can't run: one line
/// per unfilled field reference. `Ok` when every reference has a value.
pub fn refuse_unfilled(
    definition_json: &str,
    param_overrides: &[String],
    fill: FillWith,
) -> Result<(), String> {
    let mut def: PipelineDefinition =
        serde_json::from_str(definition_json).map_err(|e| format!("Invalid recipe JSON: {e}"))?;
    let default_node = first_processing_node_id(&def);
    apply_param_overrides(&mut def, param_overrides, default_node.as_deref())?;
    let missing = missing_fields(&def.nodes);
    if missing.is_empty() {
        return Ok(());
    }
    let lines: Vec<String> = missing.iter().map(|m| line(m, &def.nodes, fill)).collect();
    Err(lines.join("\n"))
}

/// One unfilled reference: how to fill a declared field, or the engine's
/// reason when nothing in scope declares it.
fn line(missing: &MissingField, nodes: &[PipelineNode], fill: FillWith) -> String {
    let field = &missing.field;
    match (&missing.declared_by, fill) {
        (Some(owner), FillWith::Flag) => {
            format!("field \"{field}\" (node \"{owner}\") is unset: pass -p {field}=...")
        }
        (Some(owner), FillWith::Form) => format!(
            "{} is empty: fill it in on the recipe's form",
            label(nodes, owner, field)
        ),
        (None, _) => format!("node \"{}\": {}", missing.node_id, missing.message()),
    }
}

/// The label `owner` declares `field` with, or the field's name without one.
fn label(nodes: &[PipelineNode], owner: &str, field: &str) -> String {
    find(nodes, owner)
        .and_then(|node| node.fields.get(field))
        .map(field_label)
        .filter(|label| !label.is_empty())
        .map_or_else(|| field.to_string(), str::to_string)
}

/// Every kind of field carries a label.
fn field_label(def: &FieldDef) -> &str {
    match def {
        FieldDef::String { label, .. }
        | FieldDef::Number { label, .. }
        | FieldDef::Boolean { label, .. }
        | FieldDef::Enum { label, .. } => label,
    }
}

/// The node with `id`, at any depth.
fn find<'a>(nodes: &'a [PipelineNode], id: &str) -> Option<&'a PipelineNode> {
    nodes.iter().find_map(|node| {
        if node.id == id {
            return Some(node);
        }
        find(node.children.as_deref().unwrap_or_default(), id)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A group `checks` declaring `version` with no default, and a step
    /// `crates` inside it that uses it. `extra` is added to the step's params.
    fn release_verify_shape(extra: &str) -> String {
        format!(
            r#"{{ "nodes": [{{ "id": "checks", "type": "group",
                "fields": {{ "version": {{ "type": "string", "label": "Version" }} }},
                "nodes": [{{ "id": "crates", "type": "shell-run",
                    "parameters": {{ "command": "echo", "args": ["{{{{fields.version}}}}"] {extra} }} }}] }}] }}"#
        )
    }

    fn overrides(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn an_unset_field_names_the_p_that_fills_it() {
        let err = refuse_unfilled(&release_verify_shape(""), &[], FillWith::Flag).unwrap_err();
        assert_eq!(
            err,
            "field \"version\" (node \"checks\") is unset: pass -p version=..."
        );
    }

    #[test]
    fn a_p_override_fills_the_field() {
        let json = release_verify_shape("");
        assert_eq!(
            refuse_unfilled(&json, &overrides(&["version=0.18.1"]), FillWith::Flag),
            Ok(())
        );
        let by_node = overrides(&["checks:version=0.18.1"]);
        assert_eq!(refuse_unfilled(&json, &by_node, FillWith::Flag), Ok(()));
    }

    #[test]
    fn a_field_nothing_in_scope_declares_gets_no_p_hint() {
        let json = r#"{ "nodes": [{ "id": "flatten", "type": "shell-run",
            "parameters": { "command": "ls", "args": ["{{fields.outDir}}"] } }] }"#;
        let err = refuse_unfilled(json, &[], FillWith::Flag).unwrap_err();
        assert!(
            err.starts_with("node \"flatten\": {{fields.outDir}} names a field"),
            "{err}"
        );
        assert!(!err.contains("-p"), "no -p can fill it: {err}");
    }

    #[test]
    fn every_unfilled_field_gets_its_own_line() {
        let json = release_verify_shape(r#", "env": { "A": "{{fields.token}}" }"#);
        let err = refuse_unfilled(&json, &[], FillWith::Flag).unwrap_err();
        assert_eq!(err.lines().count(), 2, "{err}");
    }

    #[test]
    fn a_recipe_with_every_field_filled_passes() {
        let json = r#"{ "nodes": [{ "id": "dl", "type": "shell-run",
            "fields": { "format": { "type": "string", "label": "Format", "default": "mp4" } },
            "parameters": { "command": "echo", "args": ["{{fields.format}}"] } }] }"#;
        assert_eq!(refuse_unfilled(json, &[], FillWith::Flag), Ok(()));
    }

    // --- The TUI's wording (ISS-97) ---

    #[test]
    fn on_the_form_an_unset_field_names_its_label_and_no_p() {
        let err = refuse_unfilled(&release_verify_shape(""), &[], FillWith::Form).unwrap_err();
        assert_eq!(err, "Version is empty: fill it in on the recipe's form");
    }

    #[test]
    fn on_the_form_a_field_without_a_label_goes_by_its_name() {
        let json = r#"{ "nodes": [{ "id": "g", "type": "group",
            "fields": { "dest": { "type": "string", "label": "" } },
            "nodes": [{ "id": "s", "type": "shell-run",
                "parameters": { "command": "ls", "args": ["{{fields.dest}}"] } }] }] }"#;
        let err = refuse_unfilled(json, &[], FillWith::Form).unwrap_err();
        assert_eq!(err, "dest is empty: fill it in on the recipe's form");
    }
}
