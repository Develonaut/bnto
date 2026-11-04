use super::*;
use crate::catalog::SourceKind;
use crate::trust::TrustStore;
use crate::trust_gate::consent_needed;
use std::path::PathBuf;

fn lines(json: &str, kind: &SourceKind) -> Vec<String> {
    let request = consent_needed(json, "probe", kind, &TrustStore::default()).expect("asks");
    prompt_lines(&request)
}

#[test]
fn a_shell_recipe_names_its_risks_source_commands_and_steps() {
    let json = r#"{ "name": "Probe", "nodes": [
        { "id": "s", "type": "shell-run", "parameters": { "command": "git", "args": ["--version"] } }
    ] }"#;
    let file = PathBuf::from("/home/me/.bnto/recipes/probe.bnto.json");
    assert_eq!(
        lines(json, &SourceKind::Library(Some(file))),
        [
            "",
            "  Warning: This recipe runs commands on your computer.",
            "  runs 1 command",
            "",
            "  Recipe: Probe",
            "  Library recipe `probe` from `/home/me/.bnto/recipes/probe.bnto.json`",
            "",
            "  Commands:",
            "    - git --version",
            "  Steps:",
            "    - shell-run",
            "",
            "  Run `bnto dry-run <recipe>` without input files to inspect the full commands.",
            "",
            "  runs 1 command",
        ]
    );
}

#[test]
fn a_flagged_path_is_marked() {
    let json = r#"{ "name": "Tidy", "nodes": [
        { "id": "d", "type": "file-delete", "parameters": { "base": "~/Documents" } }
    ] }"#;
    let kind = SourceKind::Project {
        root: PathBuf::from("/repo"),
        file: None,
    };
    let shown = lines(json, &kind);
    assert!(
        shown.contains(&"  Warning: This project recipe hasn't been approved yet.".to_string())
    );
    assert!(shown.contains(&"    ! file-delete base: ~/Documents  (your home folder)".to_string()));
    assert!(!shown.iter().any(|l| l.contains("Commands")));
}
