// Parameter definitions for the 7 engine-defined IO / container / data node
// types: input, output, loop, group, parallel, transform, edit-fields.
//
// These types don't have `NodeProcessor` implementations (they're structural /
// declarative). Their parameter schemas live here so the engine stays the
// single source of truth for the UI contract — `@bnto/nodes` consumes the
// generated catalog instead of hand-writing overlays.

use std::collections::HashMap;

use super::ParameterDef;

mod concurrency;
mod data;
mod helpers;
mod input;
mod loop_node;
mod output;

use concurrency::{group_params, parallel_params};
use data::{edit_fields_params, transform_params};
use input::input_params;
use loop_node::loop_params;
use output::output_params;

/// Return a map of node type → parameter definitions for the 7 engine-defined
/// non-processor node types. Callers look up by type name (`"input"`, etc.).
pub fn io_container_param_defs() -> HashMap<&'static str, Vec<ParameterDef>> {
    HashMap::from([
        ("input", input_params()),
        ("output", output_params()),
        ("loop", loop_params()),
        ("group", group_params()),
        ("parallel", parallel_params()),
        ("transform", transform_params()),
        ("edit-fields", edit_fields_params()),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metadata::{ParamCondition, ParamConditionEntry, ParameterType};

    fn get_param<'a>(params: &'a [ParameterDef], name: &str) -> &'a ParameterDef {
        params
            .iter()
            .find(|p| p.name == name)
            .unwrap_or_else(|| panic!("missing param `{}`", name))
    }

    // --- Top-level map ---

    #[test]
    fn io_container_param_defs_has_seven_entries() {
        let defs = io_container_param_defs();
        assert_eq!(
            defs.len(),
            7,
            "should cover exactly the 7 engine-defined non-processor node types"
        );
    }

    #[test]
    fn io_container_param_defs_has_expected_keys() {
        let defs = io_container_param_defs();
        for key in [
            "input",
            "output",
            "loop",
            "group",
            "parallel",
            "transform",
            "edit-fields",
        ] {
            assert!(defs.contains_key(key), "missing key `{}`", key);
        }
    }

    // --- input ---

    #[test]
    fn input_has_eight_params_in_schema_order() {
        let params = input_params();
        let names: Vec<&str> = params.iter().map(|p| p.name.as_str()).collect();
        assert_eq!(
            names,
            vec![
                "mode",
                "accept",
                "extensions",
                "label",
                "multiple",
                "maxFileSize",
                "maxFiles",
                "placeholder",
            ]
        );
    }

    #[test]
    fn input_mode_has_expected_option_labels() {
        let params = input_params();
        let mode = get_param(&params, "mode");
        let ParameterType::Enum { options } = &mode.param_type else {
            panic!("expected Enum param_type for input.mode");
        };
        let pairs: Vec<(&str, &str)> = options
            .iter()
            .map(|o| (o.value.as_str(), o.label.as_str()))
            .collect();
        assert_eq!(
            pairs,
            vec![
                ("file-upload", "File Upload"),
                ("text", "Text"),
                ("url", "URL"),
                ("none", "No Input"),
            ]
        );
    }

    #[test]
    fn input_mode_defaults_to_file_upload_with_select_control() {
        let params = input_params();
        let mode = get_param(&params, "mode");
        assert_eq!(mode.default, Some(serde_json::json!("file-upload")));
        assert_eq!(mode.control.as_deref(), Some("select"));
    }

    #[test]
    fn input_extensions_has_tag_picker_and_fifteen_options() {
        let params = input_params();
        let ext = get_param(&params, "extensions");
        assert_eq!(ext.control.as_deref(), Some("tagPicker"));
        let ParameterType::Enum { options } = &ext.param_type else {
            panic!("expected Enum param_type for input.extensions");
        };
        assert_eq!(options.len(), 15, "15 canonical extensions");
        assert_eq!(options[0].value, ".jpg");
        assert_eq!(options[14].value, ".wav");
    }

    #[test]
    fn input_accept_has_tag_picker_and_visible_when_file_upload() {
        let params = input_params();
        let accept = get_param(&params, "accept");
        assert_eq!(accept.control.as_deref(), Some("tagPicker"));
        assert_eq!(
            accept.visible_when,
            Some(ParamCondition::Single(ParamConditionEntry {
                param: "mode".to_string(),
                equals: "file-upload".to_string(),
            }))
        );
    }

    #[test]
    fn input_placeholder_visible_when_text_or_url() {
        let params = input_params();
        let placeholder = get_param(&params, "placeholder");
        let Some(ParamCondition::Any(entries)) = &placeholder.visible_when else {
            panic!("expected Any visible_when on input.placeholder");
        };
        let pairs: Vec<(&str, &str)> = entries
            .iter()
            .map(|e| (e.param.as_str(), e.equals.as_str()))
            .collect();
        assert_eq!(pairs, vec![("mode", "text"), ("mode", "url")]);
    }

    #[test]
    fn input_multiple_defaults_to_true_as_switch() {
        let params = input_params();
        let multiple = get_param(&params, "multiple");
        assert_eq!(multiple.default, Some(serde_json::json!(true)));
        assert_eq!(multiple.control.as_deref(), Some("switch"));
    }

    // --- output ---

    #[test]
    fn output_has_six_params_in_schema_order() {
        let params = output_params();
        let names: Vec<&str> = params.iter().map(|p| p.name.as_str()).collect();
        assert_eq!(
            names,
            vec![
                "mode",
                "directory",
                "filename",
                "zip",
                "label",
                "autoDownload"
            ]
        );
    }

    #[test]
    fn output_directory_is_string_with_template_hint() {
        let params = output_params();
        let dir = get_param(&params, "directory");
        assert_eq!(dir.param_type, ParameterType::String);
        assert!(
            dir.description.contains("{{ctx.date}}"),
            "description should mention template variables"
        );
    }

    #[test]
    fn output_mode_has_four_options() {
        let params = output_params();
        let mode = get_param(&params, "mode");
        let ParameterType::Enum { options } = &mode.param_type else {
            panic!("expected Enum param_type for output.mode");
        };
        assert_eq!(options.len(), 4);
        assert_eq!(options[0].value, "write");
        assert_eq!(options[1].value, "overwrite");
        assert_eq!(options[2].value, "message");
        assert_eq!(options[3].value, "none");
    }

    #[test]
    fn output_mode_defaults_to_write() {
        let params = output_params();
        let mode = get_param(&params, "mode");
        assert_eq!(mode.default, Some(serde_json::json!("write")));
    }

    #[test]
    fn output_write_fields_visible_when_mode_write() {
        let params = output_params();
        let write_cond = Some(ParamCondition::Single(ParamConditionEntry {
            param: "mode".to_string(),
            equals: "write".to_string(),
        }));
        for name in ["directory", "filename", "zip", "autoDownload"] {
            let p = get_param(&params, name);
            assert_eq!(
                p.visible_when, write_cond,
                "`{}` should be visible only when mode=write",
                name
            );
        }
    }

    #[test]
    fn output_label_is_always_visible() {
        let params = output_params();
        let label = get_param(&params, "label");
        assert!(
            label.visible_when.is_none(),
            "output.label should have no visible_when"
        );
    }

    // --- loop ---

    #[test]
    fn loop_has_five_params() {
        let params = loop_params();
        let names: Vec<&str> = params.iter().map(|p| p.name.as_str()).collect();
        assert_eq!(
            names,
            vec!["mode", "items", "count", "condition", "breakCondition"]
        );
    }

    #[test]
    fn loop_mode_is_required_with_three_options_and_no_default() {
        let params = loop_params();
        let mode = get_param(&params, "mode");
        assert!(mode.default.is_none(), "loop.mode has no default in schema");
        assert_eq!(
            mode.constraints.as_ref().map(|c| c.required),
            Some(true),
            "loop.mode must be required"
        );
        let ParameterType::Enum { options } = &mode.param_type else {
            panic!("expected Enum param_type for loop.mode");
        };
        let pairs: Vec<(&str, &str)> = options
            .iter()
            .map(|o| (o.value.as_str(), o.label.as_str()))
            .collect();
        assert_eq!(
            pairs,
            vec![
                ("forEach", "For Each"),
                ("times", "N Times"),
                ("while", "While"),
            ]
        );
    }

    #[test]
    fn loop_count_visible_and_required_when_times() {
        let params = loop_params();
        let count = get_param(&params, "count");
        let times_cond = ParamCondition::Single(ParamConditionEntry {
            param: "mode".to_string(),
            equals: "times".to_string(),
        });
        assert_eq!(count.visible_when, Some(times_cond.clone()));
        assert_eq!(count.required_when, Some(times_cond));
    }

    #[test]
    fn loop_condition_visible_and_required_when_while() {
        let params = loop_params();
        let condition = get_param(&params, "condition");
        let while_cond = ParamCondition::Single(ParamConditionEntry {
            param: "mode".to_string(),
            equals: "while".to_string(),
        });
        assert_eq!(condition.visible_when, Some(while_cond.clone()));
        assert_eq!(condition.required_when, Some(while_cond));
    }

    #[test]
    fn loop_break_condition_is_always_visible() {
        let params = loop_params();
        let brk = get_param(&params, "breakCondition");
        assert!(brk.visible_when.is_none());
        assert!(brk.required_when.is_none());
    }

    // --- group ---

    #[test]
    fn group_has_single_mode_param_with_sequential_default() {
        let params = group_params();
        assert_eq!(params.len(), 1);
        let mode = get_param(&params, "mode");
        assert_eq!(mode.default, Some(serde_json::json!("sequential")));
        assert_eq!(mode.control.as_deref(), Some("select"));
        let ParameterType::Enum { options } = &mode.param_type else {
            panic!("expected Enum param_type for group.mode");
        };
        let pairs: Vec<(&str, &str)> = options
            .iter()
            .map(|o| (o.value.as_str(), o.label.as_str()))
            .collect();
        assert_eq!(
            pairs,
            vec![("sequential", "Sequential"), ("parallel", "Parallel")]
        );
    }

    // --- parallel ---

    #[test]
    fn parallel_has_three_params_with_tasks_not_surfaceable() {
        let params = parallel_params();
        let names: Vec<&str> = params.iter().map(|p| p.name.as_str()).collect();
        assert_eq!(names, vec!["tasks", "maxWorkers", "errorStrategy"]);

        let tasks = get_param(&params, "tasks");
        assert!(
            !tasks.surfaceable,
            "parallel.tasks is internal wiring — must not surface"
        );
        assert_eq!(tasks.constraints.as_ref().map(|c| c.required), Some(true));
    }

    #[test]
    fn parallel_error_strategy_defaults_to_fail_fast() {
        let params = parallel_params();
        let strat = get_param(&params, "errorStrategy");
        assert_eq!(strat.default, Some(serde_json::json!("failFast")));
        let ParameterType::Enum { options } = &strat.param_type else {
            panic!("expected Enum param_type for parallel.errorStrategy");
        };
        let pairs: Vec<(&str, &str)> = options
            .iter()
            .map(|o| (o.value.as_str(), o.label.as_str()))
            .collect();
        assert_eq!(
            pairs,
            vec![("failFast", "Fail Fast"), ("collectAll", "Collect All")]
        );
    }

    #[test]
    fn parallel_max_workers_enforces_min_one() {
        let params = parallel_params();
        let mw = get_param(&params, "maxWorkers");
        let constraints = mw
            .constraints
            .as_ref()
            .expect("maxWorkers must have constraints");
        assert_eq!(constraints.min, Some(1.0));
    }

    // --- transform ---

    #[test]
    fn transform_expression_uses_textarea_control() {
        let params = transform_params();
        let expr = get_param(&params, "expression");
        assert_eq!(expr.control.as_deref(), Some("textarea"));
        assert_eq!(expr.param_type, ParameterType::String);
        assert_eq!(
            expr.placeholder.as_deref(),
            Some("firstName + \" \" + lastName")
        );
    }

    #[test]
    fn transform_mappings_is_object_with_key_value_control() {
        let params = transform_params();
        let mappings = get_param(&params, "mappings");
        assert_eq!(mappings.param_type, ParameterType::Object);
        assert_eq!(mappings.control.as_deref(), Some("keyValue"));
    }

    // --- edit-fields ---

    #[test]
    fn edit_fields_values_is_required_object_with_key_value_control() {
        let params = edit_fields_params();
        let values = get_param(&params, "values");
        assert_eq!(values.param_type, ParameterType::Object);
        assert_eq!(values.control.as_deref(), Some("keyValue"));
        assert_eq!(values.constraints.as_ref().map(|c| c.required), Some(true));
    }

    #[test]
    fn edit_fields_keep_only_set_defaults_to_false_switch() {
        let params = edit_fields_params();
        let k = get_param(&params, "keepOnlySet");
        assert_eq!(k.default, Some(serde_json::json!(false)));
        assert_eq!(k.control.as_deref(), Some("switch"));
    }

    // --- Round-trip ---

    #[test]
    fn all_params_serialize_to_valid_json() {
        let defs = io_container_param_defs();
        for (key, params) in defs {
            let json = serde_json::to_string(&params)
                .unwrap_or_else(|e| panic!("{} failed to serialize: {}", key, e));
            let parsed: serde_json::Value = serde_json::from_str(&json)
                .unwrap_or_else(|e| panic!("{} failed to parse back: {}", key, e));
            assert!(parsed.is_array(), "{} should serialize as array", key);
        }
    }
}
