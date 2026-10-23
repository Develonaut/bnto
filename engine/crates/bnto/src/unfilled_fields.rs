// `bnto run` and `bnto dry-run` stop before consent when a `{{fields.*}}`
// has nothing to fill it, and say which `-p` would. The engine's pre-flight
// refuses the same run (`bnto_core::missing_fields`); checking here first
// means nobody approves a run that would be refused.

use bnto_core::PipelineDefinition;
use bnto_core::first_processing_node_id;
use bnto_core::missing_fields::{MissingField, missing_fields};

use crate::input::apply_param_overrides;

/// Why `definition_json`, with `param_overrides` applied, can't run: one line
/// per unfilled field reference. `Ok` when every reference has a value.
pub fn refuse_unfilled(definition_json: &str, param_overrides: &[String]) -> Result<(), String> {
    let mut def: PipelineDefinition =
        serde_json::from_str(definition_json).map_err(|e| format!("Invalid recipe JSON: {e}"))?;
    let default_node = first_processing_node_id(&def);
    apply_param_overrides(&mut def, param_overrides, default_node.as_deref())?;
    let missing = missing_fields(&def.nodes);
    if missing.is_empty() {
        return Ok(());
    }
    Err(missing.iter().map(line).collect::<Vec<_>>().join("\n"))
}

/// One unfilled reference: the `-p` that fills a declared field, or the
/// engine's reason when nothing in scope declares it.
fn line(missing: &MissingField) -> String {
    match &missing.declared_by {
        Some(owner) => format!(
            "field \"{field}\" (node \"{owner}\") is unset: pass -p {field}=...",
            field = missing.field
        ),
        None => format!("node \"{}\": {}", missing.node_id, missing.message()),
    }
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
        let err = refuse_unfilled(&release_verify_shape(""), &[]).unwrap_err();
        assert_eq!(
            err,
            "field \"version\" (node \"checks\") is unset: pass -p version=..."
        );
    }

    #[test]
    fn a_p_override_fills_the_field() {
        let json = release_verify_shape("");
        assert_eq!(
            refuse_unfilled(&json, &overrides(&["version=0.18.1"])),
            Ok(())
        );
        let by_node = overrides(&["checks:version=0.18.1"]);
        assert_eq!(refuse_unfilled(&json, &by_node), Ok(()));
    }

    #[test]
    fn a_field_nothing_in_scope_declares_gets_no_p_hint() {
        let json = r#"{ "nodes": [{ "id": "flatten", "type": "shell-run",
            "parameters": { "command": "ls", "args": ["{{fields.outDir}}"] } }] }"#;
        let err = refuse_unfilled(json, &[]).unwrap_err();
        assert!(
            err.starts_with("node \"flatten\": {{fields.outDir}} names a field"),
            "{err}"
        );
        assert!(!err.contains("-p"), "no -p can fill it: {err}");
    }

    #[test]
    fn every_unfilled_field_gets_its_own_line() {
        let json = release_verify_shape(r#", "env": { "A": "{{fields.token}}" }"#);
        let err = refuse_unfilled(&json, &[]).unwrap_err();
        assert_eq!(err.lines().count(), 2, "{err}");
    }

    #[test]
    fn a_recipe_with_every_field_filled_passes() {
        let json = r#"{ "nodes": [{ "id": "dl", "type": "shell-run",
            "fields": { "format": { "type": "string", "label": "Format", "default": "mp4" } },
            "parameters": { "command": "echo", "args": ["{{fields.format}}"] } }] }"#;
        assert_eq!(refuse_unfilled(json, &[]), Ok(()));
    }
}
