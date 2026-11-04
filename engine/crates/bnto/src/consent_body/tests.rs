use super::*;
use crate::catalog::SourceKind;
use crate::consent_summary::{NOT_APPROVED, RUNS_COMMANDS};
use crate::trust::TrustStore;
use crate::trust_gate::consent_needed;
use std::path::PathBuf;

fn request(json: &str, kind: &SourceKind) -> ConsentRequest {
    consent_needed(json, "tidy", kind, &TrustStore::default()).expect("asks")
}

fn project() -> SourceKind {
    SourceKind::Project {
        root: PathBuf::from("/repo"),
        file: Some(PathBuf::from("/repo/.bnto/recipes/tidy.bnto.json")),
    }
}

const NO_SHELL: &str = r#"{ "name": "Tidy", "nodes": [
    { "id": "d", "type": "file-delete", "parameters": { "base": "/Users/me" } },
    { "id": "m", "type": "file-move", "parameters": { "destination": "archive" } }
] }"#;

#[test]
fn a_project_recipe_without_shell_steps_lists_no_commands() {
    let body = consent_body(&request(NO_SHELL, &project()));
    assert_eq!(body[0], ConsentLine::Heading(NOT_APPROVED.into()));
    assert!(!body.contains(&ConsentLine::Section("Commands")));
    assert!(!body.contains(&ConsentLine::Heading(RUNS_COMMANDS.into())));
}

#[test]
fn the_body_names_the_source_steps_and_flagged_paths() {
    let body = consent_body(&request(NO_SHELL, &project()));
    assert_eq!(
        body,
        [
            ConsentLine::Heading(NOT_APPROVED.into()),
            ConsentLine::Summary("1 flagged path · deletes files · moves files".into()),
            ConsentLine::Blank,
            ConsentLine::Detail("Recipe: Tidy".into()),
            ConsentLine::Detail(
                "Project recipe `tidy` from `/repo/.bnto/recipes/tidy.bnto.json`".into()
            ),
            ConsentLine::Blank,
            ConsentLine::Section("Paths"),
            ConsentLine::Flagged("file-delete base: /Users/me  (absolute)".into()),
            ConsentLine::Item("file-move destination: archive".into()),
            ConsentLine::Section("Steps"),
            ConsentLine::Item("file-delete".into()),
            ConsentLine::Item("file-move".into()),
        ]
    );
}

#[test]
fn files_its_commands_run_say_changes_wont_ask_again() {
    let json = r#"{ "name": "Lint", "nodes": [
        { "id": "s", "type": "shell-run", "parameters": { "command": "lint", "searchPath": ["bin"] } }
    ] }"#;
    let body = consent_body(&request(json, &project()));
    let at = body
        .iter()
        .position(|l| *l == ConsentLine::Section("Project files its commands run or read"))
        .expect("the files section");
    assert_eq!(body[at + 1], ConsentLine::Item("bin  (searchPath)".into()));
    assert_eq!(body[at + 2], ConsentLine::Note(FILES_NOTE));
    assert_eq!(body[0], ConsentLine::Heading(RUNS_COMMANDS.into()));
}

#[test]
fn escape_and_carriage_return_show_escaped_in_name_and_script() {
    // ISS-125: the name and a script line can't rewrite the prompt.
    let json = r#"{ "name": "Nice\u001b[2K\rEVIL", "nodes": [
        { "id": "s", "type": "shell-run",
          "parameters": { "shell": "sh", "script": "echo ok\u001b[1A\rrm -rf x" } }
    ] }"#;
    let body = consent_body(&request(json, &SourceKind::Library(None)));
    assert!(body.contains(&ConsentLine::Detail("Recipe: Nice\\e[2K\\rEVIL".into())));
    assert!(body.contains(&ConsentLine::More(
        "        echo ok\\e[1A\\rrm -rf x".into()
    )));
    let text = format!("{body:?}");
    assert!(!text.contains('\u{1b}') && !text.contains('\r'), "{text}");
}

#[test]
fn a_library_shell_step_without_a_command_is_not_called_a_project_recipe() {
    let json = r#"{ "name": "Odd", "nodes": [
        { "id": "s", "type": "shell-run", "parameters": {} }
    ] }"#;
    let body = consent_body(&request(json, &SourceKind::Library(None)));
    assert_eq!(body[0], ConsentLine::Heading(RUNS_COMMANDS.into()));
}

#[test]
fn the_summary_names_the_risks_above_the_fold() {
    let json = r#"{ "name": "Tidy", "nodes": [
        { "id": "d", "type": "file-delete", "parameters": { "base": "/Users/me" } },
        { "id": "m", "type": "file-move", "parameters": { "destination": "~/x" } },
        { "id": "c", "type": "file-copy", "parameters": { "destination": "b", "conflict": "overwrite" } },
        { "id": "s", "type": "shell-run", "parameters": { "command": "git" } }
    ] }"#;
    let body = consent_body(&request(json, &project()));
    assert_eq!(
        body[1],
        ConsentLine::Summary(
            "2 flagged paths · deletes files · moves files · overwrites files · runs 1 command"
                .into()
        )
    );
}

#[test]
fn a_no_shell_project_recipe_says_what_it_does() {
    let body = consent_body(&request(NO_SHELL, &project()));
    assert_eq!(
        body[1],
        ConsentLine::Summary("1 flagged path · deletes files · moves files".into())
    );
}

const FIELD_DEFAULT: &str = r#"{ "name": "Fdef", "nodes": [
    { "id": "s", "type": "shell-run",
      "fields": { "fmt": { "type": "string", "label": "Format",
                           "default": "--exec=touch HIDDEN\u001b[2K" } },
      "parameters": { "command": "echo", "args": ["{{fields.fmt}}"] } },
    { "id": "d", "type": "file-delete",
      "fields": { "dir": { "type": "string", "label": "Dir", "default": "/Users/me" } },
      "parameters": { "base": "{{fields.dir}}" } }
] }"#;

#[test]
fn a_fields_default_shows_beside_the_command_that_reads_it() {
    let body = consent_body(&request(FIELD_DEFAULT, &SourceKind::Library(None)));
    let at = body
        .iter()
        .position(|l| *l == ConsentLine::Item("echo {{fields.fmt}}".into()))
        .expect("the command as written");
    assert_eq!(
        body[at + 1],
        ConsentLine::More("      runs: echo --exec=touch HIDDEN\\e[2K".into())
    );
}

#[test]
fn a_path_filled_from_a_field_shows_its_value_and_is_judged_by_it() {
    let body = consent_body(&request(FIELD_DEFAULT, &SourceKind::Library(None)));
    assert!(
        body.contains(&ConsentLine::Flagged(
            "file-delete base: {{fields.dir}} = /Users/me  (absolute)".into()
        )),
        "{body:#?}"
    );
}

#[test]
fn a_value_given_for_the_run_shows_in_place_of_the_default() {
    let shown = FIELD_DEFAULT.replace("--exec=touch HIDDEN\\u001b[2K", "--given");
    let request = crate::trust_gate::consent_needed_shown(
        FIELD_DEFAULT,
        &shown,
        "fdef",
        &SourceKind::Library(None),
        &TrustStore::default(),
    )
    .expect("asks");
    assert_eq!(request.hash, crate::trust::definition_hash(FIELD_DEFAULT));
    let body = consent_body(&request);
    assert!(
        body.contains(&ConsentLine::More("      runs: echo --given".into())),
        "{body:#?}"
    );
}
