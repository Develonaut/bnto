// `bnto check <path>...` — check recipe files on disk without running them.
//
// Prints one line per finding, naming the file, then a verdict line; exits 1
// when anything was found. A file that can't be read or parsed is a finding
// for that file, so one bad path never hides the rest.

use std::process;

use bnto_core::{NodeRegistry, PipelineDefinition, check_pipeline};

/// Check every file in `paths`, print the findings and the verdict, and exit
/// non-zero on any finding.
pub fn run_check(paths: &[String]) {
    let registry = bnto_engine::create_registry();
    let findings: Vec<String> = paths
        .iter()
        .flat_map(|path| check_file(path, &registry))
        .collect();
    for line in &findings {
        println!("{line}");
    }
    println!("{}", verdict(paths.len(), findings.len()));
    if !findings.is_empty() {
        process::exit(1);
    }
}

/// Finding lines for the recipe file at `path`.
fn check_file(path: &str, registry: &NodeRegistry) -> Vec<String> {
    match std::fs::read_to_string(path) {
        Ok(json) => check_source(path, &json, registry),
        Err(e) => vec![format!("{path}: cannot read file: {e}")],
    }
}

/// Finding lines for recipe JSON `json`, each prefixed with `path`.
fn check_source(path: &str, json: &str, registry: &NodeRegistry) -> Vec<String> {
    let definition = match parse_definition(json) {
        Ok(definition) => definition,
        Err(message) => return vec![format!("{path}: {message}")],
    };
    check_pipeline(&definition, registry)
        .iter()
        .map(|finding| format!("{path}: {finding}"))
        .collect()
}

/// Parse JSON text into a pipeline, telling bad JSON apart from JSON that
/// isn't a recipe.
fn parse_definition(json: &str) -> Result<PipelineDefinition, String> {
    let value: serde_json::Value =
        serde_json::from_str(json).map_err(|e| format!("invalid JSON: {e}"))?;
    serde_json::from_value(value).map_err(|e| format!("not a recipe: {e}"))
}

/// The last line: `Check: PASS (<n> recipes)` or `Check: FAIL (<n> findings)`.
fn verdict(recipes: usize, findings: usize) -> String {
    if findings == 0 {
        format!("Check: PASS ({})", count(recipes, "recipe"))
    } else {
        format!("Check: FAIL ({})", count(findings, "finding"))
    }
}

fn count(n: usize, noun: &str) -> String {
    if n == 1 {
        format!("1 {noun}")
    } else {
        format!("{n} {noun}s")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(json: &str) -> Vec<String> {
        check_source("r.bnto.json", json, &bnto_engine::create_registry())
    }

    #[test]
    fn test_verdict_passes_with_the_recipe_count() {
        assert_eq!(verdict(1, 0), "Check: PASS (1 recipe)");
        assert_eq!(verdict(3, 0), "Check: PASS (3 recipes)");
    }

    #[test]
    fn test_verdict_fails_with_the_finding_count() {
        assert_eq!(verdict(2, 1), "Check: FAIL (1 finding)");
        assert_eq!(verdict(1, 4), "Check: FAIL (4 findings)");
    }

    #[test]
    fn test_sound_recipe_has_no_findings() {
        let json = r#"{ "nodes": [{ "id": "f", "type": "file-filter" }] }"#;
        assert!(check(json).is_empty());
    }

    #[test]
    fn test_each_finding_names_the_file() {
        let json = r#"{ "nodes": [{ "id": "z", "type": "nope" }] }"#;
        assert_eq!(
            check(json),
            vec![r#"r.bnto.json: node "z" (nope): unknown node type "nope""#]
        );
    }

    #[test]
    fn test_invalid_json_is_one_finding() {
        let lines = check("{ nope");
        assert_eq!(lines.len(), 1);
        assert!(
            lines[0].starts_with("r.bnto.json: invalid JSON:"),
            "{lines:?}"
        );
    }

    #[test]
    fn test_json_that_is_not_a_recipe_is_one_finding() {
        let lines = check(r#"{ "name": "no nodes" }"#);
        assert_eq!(lines.len(), 1);
        assert!(
            lines[0].starts_with("r.bnto.json: not a recipe:"),
            "{lines:?}"
        );
    }

    #[test]
    fn test_unreadable_file_is_one_finding() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("missing.bnto.json");
        let path = path.to_str().unwrap();
        let lines = check_file(path, &bnto_engine::create_registry());
        assert_eq!(lines.len(), 1);
        assert!(lines[0].starts_with(&format!("{path}: cannot read file:")));
    }
}
