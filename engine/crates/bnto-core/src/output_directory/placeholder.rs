// One placeholder in an output folder: whether any run could fill it (from
// the recipe alone, for `bnto check`), and what fills it in this run.

use std::collections::BTreeMap;

use serde_json::Value;

use crate::context::ProcessContext;
use crate::executor::field_scope::FieldValues;
use crate::executor::template::{CTX_KEYS_BEFORE_RUN, resolve_ctx, value_to_string};
use crate::pipeline::{InputMode, PipelineDefinition, resolve_input_mode};

/// The input file metadata `{{node.<input id>.*}}` holds before a run
/// (`build_input_metadata`).
pub(super) const INPUT_KEYS: &[&str] = &[
    "filename",
    "filename_stem",
    "filename_title",
    "filename_ext",
];

/// What the recipe fixes about its output folder's placeholders.
pub(super) struct Recipe<'a> {
    /// The keys the recipe declares in `secrets`.
    secrets: Vec<&'a str>,
    /// The input node's id, when the recipe takes files.
    file_input: Option<&'a str>,
}

impl<'a> Recipe<'a> {
    pub(super) fn of(def: &'a PipelineDefinition) -> Self {
        let takes_files = resolve_input_mode(def) == InputMode::FileUpload;
        Self {
            secrets: def.secrets.iter().map(|s| s.key.as_str()).collect(),
            file_input: def
                .nodes
                .iter()
                .find(|n| n.node_type == "input" && takes_files)
                .map(|n| n.id.as_str()),
        }
    }

    /// True when `{{node.<key>}}` is the input file's metadata.
    fn reads_input_file(&self, key: &str) -> bool {
        key.rsplit_once('.')
            .is_some_and(|(id, prop)| self.file_input == Some(id) && INPUT_KEYS.contains(&prop))
    }
}

/// Why no run can fill `{{inner}}` in the output folder, or `None` when one
/// can. Fields and vars are left to the run: `-p`, the TUI form, `--vars`.
pub(super) fn never_filled(inner: &str, recipe: &Recipe) -> Option<String> {
    let p = format!("{{{{{inner}}}}}");
    let Some((namespace, key)) = inner.split_once('.') else {
        return Some(not_before_run(&p));
    };
    match namespace {
        "fields" | "vars" => None,
        "env" if recipe.secrets.contains(&key) => None,
        "env" => Some(format!(
            "{p} names an environment variable the recipe doesn't declare in \"secrets\""
        )),
        "ctx" if CTX_KEYS_BEFORE_RUN.contains(&key) => None,
        "node" if recipe.reads_input_file(key) => None,
        "node" => Some(format!(
            "{} (of {{{{node.*}}}}, only the input file's {} are)",
            not_before_run(&p),
            INPUT_KEYS.join(", ")
        )),
        _ => Some(not_before_run(&p)),
    }
}

fn not_before_run(p: &str) -> String {
    format!("nothing fills {p} before the run starts, when the output folder is made")
}

/// What this run fills a placeholder with.
pub(super) struct Sources<'a> {
    /// The output node's fields in scope.
    pub fields: FieldValues,
    /// The input node's file metadata, by node id.
    pub inputs: BTreeMap<String, Value>,
    pub ctx: &'a dyn ProcessContext,
}

impl Sources<'_> {
    /// `{{inner}}`'s value in this run, or why it has none. An empty value
    /// counts as none, whatever fills it: `{{env.ROOT}}/out` or
    /// `{{fields.root}}/out` would otherwise write to `/out`.
    pub(super) fn value(&self, inner: &str) -> Result<String, String> {
        let p = format!("{{{{{inner}}}}}");
        let value = self.lookup(inner, &p)?;
        if value.is_empty() {
            return Err(format!("{p} has no value: it is empty"));
        }
        Ok(value)
    }

    /// `{{inner}}`'s value in this run, empty or not; `p` is how it's written.
    fn lookup(&self, inner: &str, p: &str) -> Result<String, String> {
        let no_value = || format!("{p} has no value");
        let (namespace, key) = inner.split_once('.').ok_or_else(no_value)?;
        match namespace {
            "env" => self
                .ctx
                .env_var(key)
                .ok_or_else(|| format!("{p} has no value: {key} is not set")),
            "fields" => self
                .fields
                .get(key)
                .map(value_to_string)
                .ok_or_else(no_value),
            "ctx" => resolve_ctx(key, self.ctx, None)
                .map(|v| value_to_string(&v))
                .ok_or_else(no_value),
            "node" => self.input_value(key).ok_or_else(no_value),
            _ => Err(no_value()),
        }
    }

    fn input_value(&self, key: &str) -> Option<String> {
        let (id, prop) = key.rsplit_once('.')?;
        self.inputs.get(id)?.get(prop).map(value_to_string)
    }
}
