use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use super::placeholder::INPUT_KEYS;
use super::*;
use crate::executor::template::build_input_metadata;
use crate::file_data::FileData;

/// A context with the env vars it's given and every path set.
struct Ctx(BTreeMap<String, String>);

impl Ctx {
    fn with(pairs: &[(&str, &str)]) -> Self {
        Self(
            pairs
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        )
    }
}

impl ProcessContext for Ctx {
    fn run_command(&self, _cmd: &str, _args: &[&str]) -> Result<Vec<u8>, crate::BntoError> {
        Err(crate::BntoError::ProcessingFailed("mock".into()))
    }
    fn temp_file(&self, _suffix: &str) -> Result<PathBuf, crate::BntoError> {
        Ok(PathBuf::from("/tmp/mock"))
    }
    fn env_var(&self, key: &str) -> Option<String> {
        self.0.get(key).cloned()
    }
    fn work_dir(&self) -> Result<&Path, crate::BntoError> {
        Ok(Path::new("/work"))
    }
    fn home_dir(&self) -> Option<&Path> {
        Some(Path::new("/home/.bnto"))
    }
    fn output_dir(&self) -> Option<PathBuf> {
        Some(PathBuf::from("/home/.bnto/output"))
    }
}

/// A file recipe declaring secret `ROOT`, whose output node has `output`'s
/// params and fields.
fn recipe(output: Value) -> PipelineDefinition {
    serde_json::from_value(json!({
        "secrets": [{ "key": "ROOT" }],
        "nodes": [
            { "id": "input", "type": "input", "parameters": { "mode": "file-upload" } },
            { "id": "step", "type": "file-rename" },
            output
        ]
    }))
    .unwrap()
}

fn writing_to(directory: &str) -> PipelineDefinition {
    recipe(json!({ "id": "out", "type": "output", "parameters": { "directory": directory } }))
}

fn photo() -> Vec<PipelineFile> {
    vec![PipelineFile {
        name: "summer-trip.jpg".into(),
        data: FileData::Bytes(vec![]),
        mime_type: "image/jpeg".into(),
        metadata: Default::default(),
    }]
}

fn fill(def: &PipelineDefinition, ctx: &Ctx) -> Result<Option<String>, String> {
    fill_output_directory(def, ctx, &photo())
}

fn messages(def: &PipelineDefinition) -> Vec<String> {
    output_directory_findings(def)
        .into_iter()
        .map(|f| f.to_string())
        .collect()
}

// --- Filling ---

#[test]
fn a_declared_secret_fills_the_folder() {
    let ctx = Ctx::with(&[("ROOT", "/srv/index")]);
    let filled = fill(&writing_to("{{env.ROOT}}/out"), &ctx).unwrap();
    assert_eq!(filled.as_deref(), Some("/srv/index/out"));
}

#[test]
fn ctx_and_the_input_file_still_fill() {
    let def = writing_to("{{ctx.paths.output_dir}}/{{node.input.filename_title}}");
    let filled = fill(&def, &Ctx::with(&[])).unwrap();
    assert_eq!(filled.as_deref(), Some("/home/.bnto/output/Summer Trip"));
}

#[test]
fn fields_in_the_output_nodes_scope_fill() {
    let def = recipe(json!({
        "id": "g", "type": "group",
        "fields": { "sub": { "type": "string", "label": "Sub", "default": "reports" } },
        "nodes": [{ "id": "out", "type": "output",
            "fields": { "name": { "type": "string", "label": "Name", "default": "q3" } },
            "parameters": { "directory": "{{fields.sub}}/{{fields.name}}" } }]
    }));
    assert_eq!(
        fill(&def, &Ctx::with(&[])).unwrap().as_deref(),
        Some("reports/q3")
    );
}

#[test]
fn a_filled_value_is_never_rescanned() {
    let ctx = Ctx::with(&[("ROOT", "{{ctx.date}}")]);
    let filled = fill(&writing_to("{{env.ROOT}}"), &ctx).unwrap();
    assert_eq!(filled.as_deref(), Some("{{ctx.date}}"));
}

#[test]
fn no_directory_is_none() {
    let def = recipe(json!({ "id": "out", "type": "output", "parameters": { "mode": "write" } }));
    assert_eq!(fill(&def, &Ctx::with(&[])), Ok(None));
    assert_eq!(fill(&writing_to(""), &Ctx::with(&[])), Ok(None));
    let no_output = recipe(json!({ "id": "more", "type": "file-rename" }));
    assert_eq!(fill(&no_output, &Ctx::with(&[])), Ok(None));
    assert!(output_directory_findings(&no_output).is_empty());
}

// --- Refusing what's left over ---

#[test]
fn an_unset_or_empty_secret_is_refused() {
    let def = writing_to("{{env.ROOT}}/out");
    let unset = fill(&def, &Ctx::with(&[])).unwrap_err();
    assert_eq!(
        unset,
        "node \"out\" (output), param \"directory\": {{env.ROOT}} has no value: ROOT is not set"
    );
    let empty = fill(&def, &Ctx::with(&[("ROOT", "")])).unwrap_err();
    assert!(
        empty.ends_with("{{env.ROOT}} has no value: it is empty"),
        "{empty}"
    );
}

#[test]
fn an_empty_value_from_any_namespace_is_refused() {
    // `{{fields.root}}/out` with an empty field would write to `/out`.
    let def = recipe(json!({ "id": "out", "type": "output",
        "fields": { "root": { "type": "string", "label": "Root", "default": "" } },
        "parameters": { "directory": "{{fields.root}}/out" } }));
    let err = fill(&def, &Ctx::with(&[])).unwrap_err();
    assert!(
        err.ends_with("{{fields.root}} has no value: it is empty"),
        "{err}"
    );
    let no_extension = vec![PipelineFile {
        name: "README".into(),
        ..photo().remove(0)
    }];
    let def = writing_to("{{node.input.filename_ext}}/out");
    let err = fill_output_directory(&def, &Ctx::with(&[]), &no_extension).unwrap_err();
    assert!(
        err.contains("{{node.input.filename_ext}} has no value: it is empty"),
        "{err}"
    );
}

#[test]
fn an_env_value_the_recipe_does_not_declare_is_refused_even_when_set() {
    let ctx = Ctx::with(&[("HOME", "/home/me")]);
    let err = fill(&writing_to("{{env.HOME}}/out"), &ctx).unwrap_err();
    assert!(
        err.contains("{{env.HOME}} names an environment variable"),
        "{err}"
    );
    assert!(err.contains("\"secrets\""), "{err}");
}

#[test]
fn every_placeholder_nothing_fills_gets_its_own_line() {
    let err = fill(&writing_to("out-{{ctx.nope}}-{{item.x}}"), &Ctx::with(&[])).unwrap_err();
    let lines: Vec<&str> = err.lines().collect();
    assert_eq!(lines.len(), 2, "{err}");
    assert!(lines[0].contains("nothing fills {{ctx.nope}}"), "{err}");
    assert!(lines[1].contains("nothing fills {{item.x}}"), "{err}");
}

#[test]
fn the_runs_start_is_not_known_before_it() {
    let err = fill(&writing_to("{{ctx.started}}"), &Ctx::with(&[])).unwrap_err();
    assert!(err.contains("nothing fills {{ctx.started}}"), "{err}");
}

#[test]
fn a_field_with_no_value_and_an_unfilled_var_are_refused() {
    let def = recipe(json!({ "id": "out", "type": "output",
        "fields": { "name": { "type": "string", "label": "Name" } },
        "parameters": { "directory": "{{fields.name}}/{{vars.Drive}}" } }));
    let err = fill(&def, &Ctx::with(&[])).unwrap_err();
    assert!(err.contains("{{fields.name}} has no value"), "{err}");
    assert!(err.contains("{{vars.Drive}} has no value"), "{err}");
}

#[test]
fn the_input_file_is_refused_when_there_is_none() {
    let def = writing_to("{{node.input.filename_stem}}");
    let err = fill_output_directory(&def, &Ctx::with(&[]), &[]).unwrap_err();
    assert!(
        err.contains("{{node.input.filename_stem}} has no value"),
        "{err}"
    );
}

#[test]
fn an_unclosed_placeholder_is_refused() {
    let err = fill(&writing_to("out-{{ctx.date"), &Ctx::with(&[])).unwrap_err();
    assert_eq!(
        err,
        "node \"out\" (output), param \"directory\": unclosed placeholder \"{{ctx.date\""
    );
}

#[test]
fn an_unclosed_placeholder_before_a_closed_one_gets_its_own_line() {
    // A fill keeps the first `{{` as text and fills `{{ctx.date}}` after it.
    let err = fill(&writing_to("a-{{b-{{ctx.date}}"), &Ctx::with(&[])).unwrap_err();
    assert_eq!(err.lines().count(), 1, "{err}");
    assert!(err.ends_with("unclosed placeholder \"{{b-\""), "{err}");
}

// --- bnto check ---

#[test]
fn check_passes_what_a_run_can_fill() {
    // An undeclared `{{fields.*}}` is the field check's finding, not this one's.
    let def = recipe(json!({ "id": "out", "type": "output",
        "fields": { "x": { "type": "string", "label": "X" } },
        "parameters": { "directory":
            "{{env.ROOT}}/{{ctx.date}}/{{node.input.filename_ext}}/{{fields.x}}/{{vars.Drive}}" } }));
    assert!(messages(&def).is_empty(), "{:?}", messages(&def));
}

#[test]
fn check_flags_what_no_run_can_fill() {
    let def = writing_to("{{env.OTHER}}/{{item.x}}/{{node.step.count}}/{{ctx.nope}}/{{url}}");
    let found = messages(&def);
    assert_eq!(found.len(), 5, "{found:?}");
    for (line, placeholder) in found.iter().zip([
        "{{env.OTHER}}",
        "{{item.x}}",
        "{{node.step.count}}",
        "{{ctx.nope}}",
        "{{url}}",
    ]) {
        assert!(
            line.starts_with("node \"out\" (output), param \"directory\": "),
            "{line}"
        );
        assert!(line.contains(placeholder), "{line}");
    }
}

#[test]
fn check_flags_the_input_file_when_the_recipe_takes_none() {
    let def: PipelineDefinition = serde_json::from_value(json!({ "nodes": [
        { "id": "input", "type": "input", "parameters": { "mode": "none" } },
        { "id": "out", "type": "output",
          "parameters": { "directory": "{{node.input.filename}}" } }
    ] }))
    .unwrap();
    assert_eq!(messages(&def).len(), 1, "{:?}", messages(&def));
}

#[test]
fn check_flags_an_unclosed_placeholder() {
    assert_eq!(
        messages(&writing_to("out-{{ctx.date")),
        ["node \"out\" (output), param \"directory\": unclosed placeholder \"{{ctx.date\""]
    );
}

#[test]
fn input_keys_match_the_metadata_a_run_seeds() {
    let seeded = build_input_metadata("a.txt");
    let mut keys: Vec<&str> = seeded
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    let mut listed = INPUT_KEYS.to_vec();
    keys.sort_unstable();
    listed.sort_unstable();
    assert_eq!(keys, listed);
}
