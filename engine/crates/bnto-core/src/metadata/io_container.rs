// Parameter definitions for the 7 engine-defined IO / container / data node
// types: input, output, loop, group, parallel, transform, edit-fields.
//
// These types don't have `NodeProcessor` implementations (they're structural /
// declarative). Their parameter schemas live here so the engine stays the
// single source of truth for the UI contract — `@bnto/nodes` consumes the
// generated catalog instead of hand-writing overlays.

use std::collections::HashMap;

use super::{
    Constraints, OptionEntry, ParamCondition, ParamConditionEntry, ParameterDef, ParameterType,
};

/// Return a map of node type → parameter definitions for the 7 engine-defined
/// non-processor node types. Callers look up by type name (`"input"`, etc.).
pub fn io_container_param_defs() -> HashMap<&'static str, Vec<ParameterDef>> {
    let mut map = HashMap::new();
    map.insert("input", input_params());
    map.insert("output", output_params());
    map.insert("loop", loop_params());
    map.insert("group", group_params());
    map.insert("parallel", parallel_params());
    map.insert("transform", transform_params());
    map.insert("edit-fields", edit_fields_params());
    map
}

// --- Helpers ---

/// `visibleWhen: { param, equals }` — single-condition shorthand.
fn visible_when(param: &str, equals: &str) -> ParamCondition {
    ParamCondition::Single(ParamConditionEntry {
        param: param.to_string(),
        equals: equals.to_string(),
    })
}

/// `visibleWhen: [{param, equals}, ...]` — OR logic shorthand.
fn visible_when_any(conditions: &[(&str, &str)]) -> ParamCondition {
    ParamCondition::Any(
        conditions
            .iter()
            .map(|(param, equals)| ParamConditionEntry {
                param: (*param).to_string(),
                equals: (*equals).to_string(),
            })
            .collect(),
    )
}

fn option(value: &str, label: &str) -> OptionEntry {
    OptionEntry {
        value: value.to_string(),
        label: label.to_string(),
    }
}

fn required(required: bool) -> Constraints {
    Constraints {
        min: None,
        max: None,
        required,
    }
}

fn min(min: f64, required: bool) -> Constraints {
    Constraints {
        min: Some(min),
        max: None,
        required,
    }
}

// --- input ---

fn input_params() -> Vec<ParameterDef> {
    let file_upload = "file-upload";
    vec![
        ParameterDef {
            name: "mode".to_string(),
            label: "Mode".to_string(),
            description: "How data is provided to the recipe.".to_string(),
            param_type: ParameterType::Enum {
                options: vec![
                    option("file-upload", "File Upload"),
                    option("text", "Text"),
                    option("url", "URL"),
                ],
            },
            default: Some(serde_json::json!("file-upload")),
            control: Some("select".to_string()),
            ..Default::default()
        },
        ParameterDef {
            name: "accept".to_string(),
            label: "Accepted MIME Types".to_string(),
            description:
                "MIME types accepted (e.g., \"image/jpeg\", \"image/png\"). Derived from extensions."
                    .to_string(),
            param_type: ParameterType::String,
            control: Some("tagPicker".to_string()),
            visible_when: Some(visible_when("mode", file_upload)),
            ..Default::default()
        },
        ParameterDef {
            name: "extensions".to_string(),
            label: "File Extensions".to_string(),
            description: "File extensions accepted (e.g., \".jpg\", \".png\").".to_string(),
            param_type: ParameterType::Enum {
                options: vec![
                    option(".jpg", ".jpg"),
                    option(".jpeg", ".jpeg"),
                    option(".png", ".png"),
                    option(".webp", ".webp"),
                    option(".gif", ".gif"),
                    option(".svg", ".svg"),
                    option(".csv", ".csv"),
                    option(".json", ".json"),
                    option(".txt", ".txt"),
                    option(".pdf", ".pdf"),
                    option(".xml", ".xml"),
                    option(".zip", ".zip"),
                    option(".mp3", ".mp3"),
                    option(".mp4", ".mp4"),
                    option(".wav", ".wav"),
                ],
            },
            control: Some("tagPicker".to_string()),
            visible_when: Some(visible_when("mode", file_upload)),
            ..Default::default()
        },
        ParameterDef {
            name: "label".to_string(),
            label: "Label".to_string(),
            description: "Human-readable label for the input control.".to_string(),
            param_type: ParameterType::String,
            placeholder: Some("JPEG, PNG, or WebP images".to_string()),
            visible_when: Some(visible_when("mode", file_upload)),
            ..Default::default()
        },
        ParameterDef {
            name: "multiple".to_string(),
            label: "Multiple".to_string(),
            description: "Whether multiple files or items are accepted.".to_string(),
            param_type: ParameterType::Boolean,
            default: Some(serde_json::json!(true)),
            control: Some("switch".to_string()),
            visible_when: Some(visible_when("mode", file_upload)),
            ..Default::default()
        },
        ParameterDef {
            name: "maxFileSize".to_string(),
            label: "Max File Size".to_string(),
            description: "Maximum file size in bytes. 0 = no limit.".to_string(),
            param_type: ParameterType::Number,
            default: Some(serde_json::json!(0)),
            constraints: Some(min(0.0, false)),
            visible_when: Some(visible_when("mode", file_upload)),
            ..Default::default()
        },
        ParameterDef {
            name: "maxFiles".to_string(),
            label: "Max Files".to_string(),
            description: "Maximum number of files. 0 = no limit.".to_string(),
            param_type: ParameterType::Number,
            default: Some(serde_json::json!(0)),
            constraints: Some(min(0.0, false)),
            visible_when: Some(visible_when("mode", file_upload)),
            ..Default::default()
        },
        ParameterDef {
            name: "placeholder".to_string(),
            label: "Placeholder".to_string(),
            description: "Placeholder text for text or URL input.".to_string(),
            param_type: ParameterType::String,
            visible_when: Some(visible_when_any(&[("mode", "text"), ("mode", "url")])),
            ..Default::default()
        },
    ]
}

// --- output ---

fn output_params() -> Vec<ParameterDef> {
    let download = "download";
    vec![
        ParameterDef {
            name: "mode".to_string(),
            label: "Mode".to_string(),
            description: "How results are delivered to the user.".to_string(),
            param_type: ParameterType::Enum {
                options: vec![
                    option("download", "Download"),
                    option("display", "Display"),
                    option("preview", "Preview"),
                ],
            },
            default: Some(serde_json::json!("download")),
            control: Some("select".to_string()),
            ..Default::default()
        },
        ParameterDef {
            name: "directory".to_string(),
            label: "Output Directory".to_string(),
            description:
                "Output directory path. Supports {{ctx.date}}, {{ctx.timestamp}} templates."
                    .to_string(),
            param_type: ParameterType::String,
            default: Some(serde_json::json!("")),
            placeholder: Some("{{ctx.date}}-bulk-download".to_string()),
            visible_when: Some(visible_when("mode", download)),
            ..Default::default()
        },
        ParameterDef {
            name: "filename".to_string(),
            label: "Filename Template".to_string(),
            description: "Filename template for output files.".to_string(),
            param_type: ParameterType::String,
            placeholder: Some("compressed-{{name}}".to_string()),
            visible_when: Some(visible_when("mode", download)),
            ..Default::default()
        },
        ParameterDef {
            name: "zip".to_string(),
            label: "ZIP Multiple".to_string(),
            description: "Auto-zip when there are multiple output files.".to_string(),
            param_type: ParameterType::Boolean,
            default: Some(serde_json::json!(true)),
            control: Some("switch".to_string()),
            visible_when: Some(visible_when("mode", download)),
            ..Default::default()
        },
        ParameterDef {
            name: "label".to_string(),
            label: "Label".to_string(),
            description: "Label for the download button or display section.".to_string(),
            param_type: ParameterType::String,
            placeholder: Some("Compressed Images".to_string()),
            ..Default::default()
        },
        ParameterDef {
            name: "autoDownload".to_string(),
            label: "Auto-Download".to_string(),
            description: "Automatically download results on completion.".to_string(),
            param_type: ParameterType::Boolean,
            default: Some(serde_json::json!(true)),
            control: Some("switch".to_string()),
            visible_when: Some(visible_when("mode", download)),
            ..Default::default()
        },
    ]
}

// --- loop ---

fn loop_params() -> Vec<ParameterDef> {
    vec![
        ParameterDef {
            name: "mode".to_string(),
            label: "Mode".to_string(),
            description:
                "How the loop iterates: over items, N times, or while a condition holds."
                    .to_string(),
            param_type: ParameterType::Enum {
                options: vec![
                    option("forEach", "For Each"),
                    option("times", "N Times"),
                    option("while", "While"),
                ],
            },
            constraints: Some(required(true)),
            control: Some("select".to_string()),
            ..Default::default()
        },
        ParameterDef {
            name: "items".to_string(),
            label: "Items".to_string(),
            description: "Optional template expression for the item source. The Rust engine iterates over the incoming file batch by default — this is only needed for custom data sources.".to_string(),
            param_type: ParameterType::String,
            placeholder: Some("{{index . \"list-files\" \"files\"}}".to_string()),
            visible_when: Some(visible_when("mode", "forEach")),
            ..Default::default()
        },
        ParameterDef {
            name: "count".to_string(),
            label: "Count".to_string(),
            description: "Number of times to repeat.".to_string(),
            param_type: ParameterType::Number,
            constraints: Some(min(0.0, false)),
            visible_when: Some(visible_when("mode", "times")),
            required_when: Some(visible_when("mode", "times")),
            ..Default::default()
        },
        ParameterDef {
            name: "condition".to_string(),
            label: "Condition".to_string(),
            description: "Expr expression that must evaluate to true to continue looping."
                .to_string(),
            param_type: ParameterType::String,
            placeholder: Some("counter < 10".to_string()),
            visible_when: Some(visible_when("mode", "while")),
            required_when: Some(visible_when("mode", "while")),
            ..Default::default()
        },
        ParameterDef {
            name: "breakCondition".to_string(),
            label: "Break Condition".to_string(),
            description: "Optional expr expression — breaks out of the loop early when true."
                .to_string(),
            param_type: ParameterType::String,
            placeholder: Some("item.status == 'done'".to_string()),
            ..Default::default()
        },
    ]
}

// --- group ---

fn group_params() -> Vec<ParameterDef> {
    vec![ParameterDef {
        name: "mode".to_string(),
        label: "Mode".to_string(),
        description:
            "How child nodes execute — sequentially (one after another) or in parallel (concurrently)."
                .to_string(),
        param_type: ParameterType::Enum {
            options: vec![
                option("sequential", "Sequential"),
                option("parallel", "Parallel"),
            ],
        },
        default: Some(serde_json::json!("sequential")),
        control: Some("select".to_string()),
        ..Default::default()
    }]
}

// --- parallel ---

fn parallel_params() -> Vec<ParameterDef> {
    vec![
        ParameterDef {
            name: "tasks".to_string(),
            label: "Tasks".to_string(),
            description: "Array of task definitions to execute concurrently.".to_string(),
            param_type: ParameterType::Object,
            constraints: Some(required(true)),
            surfaceable: false,
            ..Default::default()
        },
        ParameterDef {
            name: "maxWorkers".to_string(),
            label: "Max Workers".to_string(),
            description: "Maximum number of concurrent workers. Defaults to the number of tasks."
                .to_string(),
            param_type: ParameterType::Number,
            constraints: Some(min(1.0, false)),
            ..Default::default()
        },
        ParameterDef {
            name: "errorStrategy".to_string(),
            label: "Error Strategy".to_string(),
            description: "How to handle task errors — fail immediately or collect all results."
                .to_string(),
            param_type: ParameterType::Enum {
                options: vec![
                    option("failFast", "Fail Fast"),
                    option("collectAll", "Collect All"),
                ],
            },
            default: Some(serde_json::json!("failFast")),
            control: Some("select".to_string()),
            ..Default::default()
        },
    ]
}

// --- transform ---

fn transform_params() -> Vec<ParameterDef> {
    vec![
        ParameterDef {
            name: "expression".to_string(),
            label: "Expression".to_string(),
            description:
                "Expr expression for a single transformation. Mutually exclusive with mappings."
                    .to_string(),
            param_type: ParameterType::String,
            placeholder: Some("firstName + \" \" + lastName".to_string()),
            control: Some("textarea".to_string()),
            ..Default::default()
        },
        ParameterDef {
            name: "mappings".to_string(),
            label: "Mappings".to_string(),
            description:
                "Map of field names to expr expressions for multi-field transformations. Mutually exclusive with expression."
                    .to_string(),
            param_type: ParameterType::Object,
            control: Some("keyValue".to_string()),
            ..Default::default()
        },
    ]
}

// --- edit-fields ---

fn edit_fields_params() -> Vec<ParameterDef> {
    vec![
        ParameterDef {
            name: "values".to_string(),
            label: "Values".to_string(),
            description:
                "Map of field names to values. Values can be static or template expressions (e.g., {{.record.name}})."
                    .to_string(),
            param_type: ParameterType::Object,
            constraints: Some(required(true)),
            control: Some("keyValue".to_string()),
            ..Default::default()
        },
        ParameterDef {
            name: "keepOnlySet".to_string(),
            label: "Keep Only Set Fields".to_string(),
            description:
                "When true, only output fields that are explicitly set in values.".to_string(),
            param_type: ParameterType::Boolean,
            default: Some(serde_json::json!(false)),
            control: Some("switch".to_string()),
            ..Default::default()
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn input_mode_has_three_option_labels() {
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
    fn output_mode_has_three_option_labels() {
        let params = output_params();
        let mode = get_param(&params, "mode");
        let ParameterType::Enum { options } = &mode.param_type else {
            panic!("expected Enum param_type for output.mode");
        };
        let pairs: Vec<(&str, &str)> = options
            .iter()
            .map(|o| (o.value.as_str(), o.label.as_str()))
            .collect();
        assert_eq!(
            pairs,
            vec![
                ("download", "Download"),
                ("display", "Display"),
                ("preview", "Preview"),
            ]
        );
    }

    #[test]
    fn output_download_fields_visible_when_mode_download() {
        let params = output_params();
        let download_cond = Some(ParamCondition::Single(ParamConditionEntry {
            param: "mode".to_string(),
            equals: "download".to_string(),
        }));
        for name in ["directory", "filename", "zip", "autoDownload"] {
            let p = get_param(&params, name);
            assert_eq!(
                p.visible_when, download_cond,
                "`{}` should be visible only when mode=download",
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
