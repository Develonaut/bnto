// A run's time limit: the recipe's `settings.timeout`, or `bnto run
// --timeout`, which overrides it. The override is written into the recipe,
// so the engine's between-step checks and the CLI's watchdog use one value.

use bnto_core::PipelineSettings;
use serde_json::Value;

/// `definition_json` with `override_secs` (when given) as its time limit,
/// and the limit the run ends up with. Refuses a recipe whose `settings`
/// don't parse, before anything runs.
pub fn apply(
    definition_json: &str,
    override_secs: Option<u64>,
) -> Result<(String, Option<u64>), String> {
    let mut definition: Value =
        serde_json::from_str(definition_json).map_err(|e| format!("invalid recipe JSON: {e}"))?;
    let json = match override_secs {
        Some(secs) => {
            set_timeout(&mut definition, secs);
            definition.to_string()
        }
        None => definition_json.to_string(),
    };
    Ok((json, time_limit(&definition)?))
}

/// Put `secs` in the recipe's `settings.timeout`, adding `settings` if absent.
fn set_timeout(definition: &mut Value, secs: u64) {
    let Some(root) = definition.as_object_mut() else {
        return;
    };
    let settings = root
        .entry("settings")
        .or_insert_with(|| Value::Object(Default::default()));
    if settings.is_null() {
        *settings = Value::Object(Default::default());
    }
    if let Some(settings) = settings.as_object_mut() {
        settings.insert("timeout".to_string(), secs.into());
    }
}

/// The recipe's `settings.timeout`, read the way the engine reads it.
fn time_limit(definition: &Value) -> Result<Option<u64>, String> {
    match definition.get("settings") {
        None | Some(Value::Null) => Ok(None),
        Some(settings) => serde_json::from_value::<PipelineSettings>(settings.clone())
            .map(|s| s.timeout)
            .map_err(|e| e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn limit(json: &str, override_secs: Option<u64>) -> Option<u64> {
        apply(json, override_secs).unwrap().1
    }

    #[test]
    fn test_the_recipes_limit_applies_without_an_override() {
        let json = r#"{ "settings": { "timeout": 90 }, "nodes": [] }"#;
        assert_eq!(apply(json, None).unwrap(), (json.to_string(), Some(90)));
    }

    #[test]
    fn test_no_limit_anywhere_means_none() {
        assert_eq!(limit(r#"{ "nodes": [] }"#, None), None);
        assert_eq!(limit(r#"{ "settings": null, "nodes": [] }"#, None), None);
    }

    #[test]
    fn test_the_override_replaces_the_recipes_limit() {
        let json = r#"{ "settings": { "iteration": "auto", "timeout": 600 }, "nodes": [] }"#;
        let (json, secs) = apply(json, Some(5)).unwrap();
        assert_eq!(secs, Some(5));
        let written: Value = serde_json::from_str(&json).unwrap();
        assert_eq!(written["settings"]["timeout"], 5);
        assert_eq!(written["settings"]["iteration"], "auto");
    }

    #[test]
    fn test_the_override_limits_a_recipe_without_settings() {
        let (json, secs) = apply(r#"{ "nodes": [] }"#, Some(3)).unwrap();
        assert_eq!(secs, Some(3));
        let written: Value = serde_json::from_str(&json).unwrap();
        assert_eq!(written["settings"]["timeout"], 3);
    }

    #[test]
    fn test_a_bad_recipe_limit_is_refused_before_the_run() {
        let err = apply(r#"{ "settings": { "timeout": 0 }, "nodes": [] }"#, None).unwrap_err();
        assert!(err.contains("settings.timeout must be"), "{err}");
    }

    #[test]
    fn test_the_override_wins_over_a_bad_recipe_limit() {
        let json = r#"{ "settings": { "timeout": 0 }, "nodes": [] }"#;
        assert_eq!(limit(json, Some(4)), Some(4));
    }
}
