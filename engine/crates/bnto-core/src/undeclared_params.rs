// Params a recipe gives a node that the node doesn't declare. The engine
// would ignore them, so a typo or a param renamed since the recipe was
// written would silently change what a step does. `bnto check` reports
// them and a run's pre-flight refuses them.

use serde_json::{Map, Value};

use crate::input_mode::INJECTED_PARAMS;
use crate::metadata::ParameterDef;
use crate::renamed::param_hint;

/// One param `node_type` doesn't declare, and what to tell the user.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UndeclaredParam {
    pub param: String,
    pub message: String,
}

/// Every key in `params` that isn't one of `declared` or a run's injected
/// input (`INJECTED_PARAMS`), in key order.
pub fn undeclared_params(
    node_type: &str,
    params: &Map<String, Value>,
    declared: &[ParameterDef],
) -> Vec<UndeclaredParam> {
    params
        .keys()
        .filter(|key| !INJECTED_PARAMS.contains(&key.as_str()))
        .filter(|key| !declared.iter().any(|d| d.name == **key))
        .map(|key| UndeclaredParam {
            param: key.clone(),
            message: message(node_type, key),
        })
        .collect()
}

fn message(node_type: &str, param: &str) -> String {
    match param_hint(node_type, param) {
        Some(hint) => format!("{node_type} has no param \"{param}\" ({hint})"),
        None => format!("{node_type} has no param \"{param}\""),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metadata::ParameterType;
    use serde_json::json;

    fn declared(names: &[&str]) -> Vec<ParameterDef> {
        names
            .iter()
            .map(|name| ParameterDef {
                name: name.to_string(),
                param_type: ParameterType::String,
                ..ParameterDef::default()
            })
            .collect()
    }

    fn params(value: Value) -> Map<String, Value> {
        value.as_object().cloned().unwrap()
    }

    #[test]
    fn declared_params_pass() {
        let found = undeclared_params(
            "file-copy",
            &params(json!({ "destination": "out", "conflict": "skip" })),
            &declared(&["destination", "conflict"]),
        );
        assert!(found.is_empty(), "{found:?}");
    }

    #[test]
    fn a_typo_is_named_with_its_node_type() {
        let found = undeclared_params(
            "file-copy",
            &params(json!({ "destinaton": "out" })),
            &declared(&["destination"]),
        );
        assert_eq!(
            found,
            [UndeclaredParam {
                param: "destinaton".into(),
                message: "file-copy has no param \"destinaton\"".into(),
            }]
        );
    }

    #[test]
    fn a_renamed_param_points_at_bnto_migrate() {
        let found = undeclared_params(
            "image-compress",
            &params(json!({ "compression": 60 })),
            &declared(&["quality"]),
        );
        assert_eq!(
            found[0].message,
            "image-compress has no param \"compression\" \
             (renamed to \"quality\"; run `bnto migrate` on the recipe)"
        );
    }

    #[test]
    fn a_removed_param_points_at_bnto_migrate() {
        let found = undeclared_params(
            "vector-optimize",
            &params(json!({ "minify": true, "precision": 3 })),
            &declared(&["minify"]),
        );
        assert_eq!(
            found,
            [UndeclaredParam {
                param: "precision".into(),
                message: "vector-optimize has no param \"precision\" \
                          (removed because it never did anything; run `bnto migrate` on the recipe)"
                    .into(),
            }]
        );
    }

    #[test]
    fn every_undeclared_param_is_listed_in_key_order() {
        let found = undeclared_params(
            "file-filter",
            &params(json!({ "zeta": 1, "alpha": 2, "extensions": "jpg" })),
            &declared(&["extensions"]),
        );
        let names: Vec<_> = found.iter().map(|f| f.param.as_str()).collect();
        assert_eq!(names, ["alpha", "zeta"]);
    }

    #[test]
    fn a_runs_injected_url_or_text_input_is_accepted_on_any_node() {
        let found = undeclared_params(
            "shell-run",
            &params(json!({ "command": "yt-dlp", "url": "https://x", "text": "hi" })),
            &declared(&["command"]),
        );
        assert!(found.is_empty(), "{found:?}");
    }

    #[test]
    fn no_params_means_nothing_undeclared() {
        assert!(undeclared_params("file-copy", &Map::new(), &declared(&[])).is_empty());
    }
}
