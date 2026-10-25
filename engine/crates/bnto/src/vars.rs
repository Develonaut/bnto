// `bnto run --vars` and `bnto dry-run --vars` — load run vars and resolve `{{vars.KEY}}`.
//
// Each file is one JSON object. Later files win over earlier ones, and a
// `BNTO_VAR_<KEY>` environment variable (key upper-cased) wins over every file.

use std::collections::BTreeMap;

use serde_json::Value;

/// Merged vars from every `--vars` file, in order.
pub type VarFiles = BTreeMap<String, String>;

/// Load and merge `--vars` files. A missing file is skipped; a file that
/// isn't a JSON object is an error naming it.
pub fn load_var_files(paths: &[String]) -> Result<VarFiles, String> {
    let mut vars = VarFiles::new();
    for path in paths {
        let Ok(text) = std::fs::read_to_string(path) else {
            continue;
        };
        vars.extend(parse_var_file(path, &text)?);
    }
    Ok(vars)
}

/// One file's vars: its string, number and boolean values, as text.
fn parse_var_file(path: &str, text: &str) -> Result<VarFiles, String> {
    let value: Value = serde_json::from_str(text)
        .map_err(|e| format!("--vars file {path} is not valid JSON: {e}"))?;
    let Value::Object(map) = value else {
        return Err(format!("--vars file {path} is not a JSON object"));
    };
    Ok(map
        .into_iter()
        .filter_map(|(k, v)| scalar_text(&v).map(|t| (k, t)))
        .collect())
}

/// A string, number or boolean as text; other values aren't vars.
fn scalar_text(value: &Value) -> Option<String> {
    match value {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

/// A var's value: `BNTO_VAR_<KEY>` from `env` first, then the files.
pub fn lookup(key: &str, files: &VarFiles, env: &dyn Fn(&str) -> Option<String>) -> Option<String> {
    env(&env_name(key)).or_else(|| files.get(key).cloned())
}

/// Resolve the recipe's `{{vars.*}}` from `--vars` files and the process
/// environment. Errors with the run's failure line when a key is unset.
pub fn apply(definition_json: &str, var_paths: &[String]) -> Result<String, String> {
    // Only reparse recipes that use vars, so others pass through byte for byte.
    if !definition_json.contains("{{vars.") {
        return Ok(definition_json.to_string());
    }
    let files = load_var_files(var_paths)?;
    let mut def: Value =
        serde_json::from_str(definition_json).map_err(|e| format!("Invalid recipe JSON: {e}"))?;
    let env = |name: &str| std::env::var(name).ok();
    bnto_core::vars::apply_vars(&mut def, &|key| lookup(key, &files, &env))
        .map_err(|keys| unresolved_message(&keys))?;
    Ok(def.to_string())
}

/// `BNTO_VAR_<KEY>`, the key upper-cased.
fn env_name(key: &str) -> String {
    format!("BNTO_VAR_{}", key.to_uppercase())
}

/// The failure line for unset keys, one clause per key.
fn unresolved_message(keys: &[String]) -> String {
    keys.iter()
        .map(|k| {
            format!(
                "unresolved {{{{vars.{k}}}}} (no --vars file or {} sets it)",
                env_name(k)
            )
        })
        .collect::<Vec<_>>()
        .join("; ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write(dir: &tempfile::TempDir, name: &str, body: &str) -> String {
        let path = dir.path().join(name);
        std::fs::write(&path, body).unwrap();
        path.to_string_lossy().into_owned()
    }

    fn no_env(_: &str) -> Option<String> {
        None
    }

    #[test]
    fn a_later_file_wins() {
        let dir = tempfile::tempdir().unwrap();
        let a = write(&dir, "a.json", r#"{"Drive": "C:", "Only": "a"}"#);
        let b = write(&dir, "b.json", r#"{"Drive": "D:"}"#);
        let vars = load_var_files(&[a, b]).unwrap();
        assert_eq!(vars["Drive"], "D:");
        assert_eq!(vars["Only"], "a");
    }

    #[test]
    fn a_missing_file_is_skipped() {
        let dir = tempfile::tempdir().unwrap();
        let a = write(&dir, "a.json", r#"{"Drive": "C:"}"#);
        let gone = dir.path().join("gone.json").to_string_lossy().into_owned();
        let vars = load_var_files(&[gone, a]).unwrap();
        assert_eq!(vars["Drive"], "C:");
    }

    #[test]
    fn numbers_and_booleans_are_text_and_other_values_are_skipped() {
        let dir = tempfile::tempdir().unwrap();
        let a = write(
            &dir,
            "a.json",
            r#"{"Slots": 3, "On": true, "List": [1], "Obj": {}, "Nil": null}"#,
        );
        let vars = load_var_files(&[a]).unwrap();
        assert_eq!(vars["Slots"], "3");
        assert_eq!(vars["On"], "true");
        assert_eq!(vars.len(), 2, "{vars:?}");
    }

    #[test]
    fn a_file_that_is_not_an_object_is_an_error_naming_it() {
        let dir = tempfile::tempdir().unwrap();
        let a = write(&dir, "list.json", "[1, 2]");
        let err = load_var_files(&[a]).unwrap_err();
        assert!(err.contains("list.json"), "{err}");
    }

    #[test]
    fn a_file_that_is_not_json_is_an_error_naming_it() {
        let dir = tempfile::tempdir().unwrap();
        let a = write(&dir, "broken.json", "{ nope");
        let err = load_var_files(&[a]).unwrap_err();
        assert!(err.contains("broken.json"), "{err}");
    }

    #[test]
    fn the_environment_wins_over_the_files() {
        let files: VarFiles = [("AssetDrive".to_string(), "C:".to_string())].into();
        let env = |k: &str| (k == "BNTO_VAR_ASSETDRIVE").then(|| "E:".to_string());
        assert_eq!(lookup("AssetDrive", &files, &env).as_deref(), Some("E:"));
        assert_eq!(lookup("AssetDrive", &files, &no_env).as_deref(), Some("C:"));
        assert_eq!(lookup("Other", &files, &no_env), None);
    }

    #[test]
    fn apply_resolves_a_destination() {
        let dir = tempfile::tempdir().unwrap();
        let a = write(&dir, "a.json", r#"{"Out": "D:/Assets"}"#);
        let def = r#"{"nodes":[{"parameters":{"destination":"{{vars.Out}}/Art"}}]}"#;
        let resolved = apply(def, &[a]).unwrap();
        assert!(resolved.contains("D:/Assets/Art"), "{resolved}");
    }

    #[test]
    fn apply_names_an_unset_key() {
        let def = r#"{"p":"{{vars.BntoTestUnsetVarXyz}}"}"#;
        let err = apply(def, &[]).unwrap_err();
        assert_eq!(
            err,
            "unresolved {{vars.BntoTestUnsetVarXyz}} (no --vars file or BNTO_VAR_BNTOTESTUNSETVARXYZ sets it)"
        );
    }

    #[test]
    fn apply_leaves_a_recipe_without_vars_untouched() {
        let def = r#"{"z":1,"a":"{{env.X}}"}"#;
        assert_eq!(apply(def, &[]).unwrap(), def);
    }
}
