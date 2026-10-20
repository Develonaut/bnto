// `bnto check <path>...` — check recipe files on disk without running them.
//
// Prints one line per finding, naming the file, then a note per param left
// for `bnto run -p` to give, then a verdict line; exits 1 when anything was
// found. Notes never fail the check. A file that can't be read or parsed is
// a finding for that file, so one bad path never hides the rest.

use std::process;

use bnto_core::{NodeRegistry, PipelineDefinition, RunTimeParam, check_pipeline};

/// One file's output: finding lines, then note lines.
#[derive(Debug, Default)]
struct FileLines {
    findings: Vec<String>,
    notes: Vec<String>,
}

/// Check every file in `paths`, print the findings, notes and verdict, and
/// exit non-zero on any finding.
pub fn run_check(paths: &[String]) {
    let registry = bnto_engine::create_registry();
    let (mut findings, mut notes) = (0, 0);
    for path in paths {
        let lines = check_file(path, &registry);
        for line in lines.findings.iter().chain(&lines.notes) {
            println!("{line}");
        }
        findings += lines.findings.len();
        notes += lines.notes.len();
    }
    println!("{}", verdict(paths.len(), findings, notes));
    if findings > 0 {
        process::exit(1);
    }
}

/// Output lines for the recipe file at `path`.
fn check_file(path: &str, registry: &NodeRegistry) -> FileLines {
    match std::fs::read_to_string(path) {
        Ok(json) => check_source(path, &json, registry),
        Err(e) => finding_only(format!("{path}: cannot read file: {e}")),
    }
}

/// Output lines for recipe JSON `json`, each prefixed with `path`.
fn check_source(path: &str, json: &str, registry: &NodeRegistry) -> FileLines {
    let definition = match parse_definition(json) {
        Ok(definition) => definition,
        Err(message) => return finding_only(format!("{path}: {message}")),
    };
    let report = check_pipeline(&definition, registry);
    FileLines {
        findings: report
            .findings
            .iter()
            .map(|f| format!("{path}: {f}"))
            .collect(),
        notes: report.run_time.iter().map(|p| note_line(path, p)).collect(),
    }
}

fn finding_only(line: String) -> FileLines {
    FileLines {
        findings: vec![line],
        notes: Vec::new(),
    }
}

/// `<path>: note: <param>; give it with bnto run -p <node>:<param>=<value>`.
fn note_line(path: &str, param: &RunTimeParam) -> String {
    format!(
        "{path}: note: {param}; give it with bnto run -p {}:{}=<value>",
        param.node_id, param.param
    )
}

/// Parse JSON text into a pipeline, telling bad JSON apart from JSON that
/// isn't a recipe.
fn parse_definition(json: &str) -> Result<PipelineDefinition, String> {
    let value: serde_json::Value =
        serde_json::from_str(json).map_err(|e| format!("invalid JSON: {e}"))?;
    serde_json::from_value(value).map_err(|e| format!("not a recipe: {e}"))
}

/// The last line: `Check: PASS (<n> recipes)` or `Check: FAIL (<n> findings)`,
/// with `, <n> notes` added when there are any.
fn verdict(recipes: usize, findings: usize, notes: usize) -> String {
    let (word, mut counts) = if findings == 0 {
        ("PASS", count(recipes, "recipe"))
    } else {
        ("FAIL", count(findings, "finding"))
    };
    if notes > 0 {
        counts = format!("{counts}, {}", count(notes, "note"));
    }
    format!("Check: {word} ({counts})")
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
        check_source("r.bnto.json", json, &bnto_engine::create_registry()).findings
    }

    #[test]
    fn test_verdict_passes_with_the_recipe_count() {
        assert_eq!(verdict(1, 0, 0), "Check: PASS (1 recipe)");
        assert_eq!(verdict(3, 0, 0), "Check: PASS (3 recipes)");
    }

    #[test]
    fn test_verdict_fails_with_the_finding_count() {
        assert_eq!(verdict(2, 1, 0), "Check: FAIL (1 finding)");
        assert_eq!(verdict(1, 4, 0), "Check: FAIL (4 findings)");
    }

    #[test]
    fn test_verdict_counts_notes_without_failing() {
        assert_eq!(verdict(1, 0, 1), "Check: PASS (1 recipe, 1 note)");
        assert_eq!(verdict(2, 1, 3), "Check: FAIL (1 finding, 3 notes)");
    }

    #[test]
    fn test_a_param_left_for_run_time_is_a_note_naming_the_flag() {
        let json = r#"{ "nodes": [{ "id": "clean", "type": "file-delete" }] }"#;
        let lines = check_source("r.bnto.json", json, &bnto_engine::create_registry());
        assert!(lines.findings.is_empty(), "{lines:?}");
        assert_eq!(
            lines.notes,
            vec![
                "r.bnto.json: note: node \"clean\" (file-delete), param \"base\": left for run time; \
                 give it with bnto run -p clean:base=<value>"
            ]
        );
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
    fn test_every_edge_finding_names_the_file() {
        let json = r#"{
            "nodes": [{ "id": "a", "type": "file-filter" }, { "id": "b", "type": "file-filter" }],
            "edges": [{ "id": "e1", "source": "a", "target": "b" },
                      { "id": "e2", "source": "b", "target": "a" },
                      { "id": "e3", "source": "a", "target": "nope" }]
        }"#;
        assert_eq!(
            check(json),
            vec![
                r#"r.bnto.json: node "a" (file-filter): edge "e3" connects to "nope", which is not a node in this recipe"#,
                r#"r.bnto.json: node "a" (file-filter): edges form a cycle: a → b → a"#,
            ]
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
        let lines = check_file(path, &bnto_engine::create_registry()).findings;
        assert_eq!(lines.len(), 1);
        assert!(lines[0].starts_with(&format!("{path}: cannot read file:")));
    }
}
