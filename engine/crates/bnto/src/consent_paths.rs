// The path params the consent prompt lists: every folder or file a recipe's
// steps list, delete under, move or copy into, write, read or find commands
// in, and every name they give their output, so the user sees where it
// reaches before approving it. A file step needs no
// shell to delete a folder, so a step's name isn't enough (§4 of
// decisions/project-recipes.md).

use std::path::Path;

use bnto_core::{PipelineDefinition, PipelineNode};
use serde_json::{Map, Value};

use bnto_core::executor::field_scope::{FieldValues, declare};

use crate::fill_fields::fill_fields;
use crate::path_flag::{PathFlag, fragment_flag, name_flag, path_flag};

/// One path a step uses, as the recipe wrote it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathParam {
    pub node_type: String,
    pub param: &'static str,
    pub value: String,
    /// `value` with its `{{fields.*}}` filled (escaped), when it reads any.
    pub filled: Option<String>,
    /// Why it needs a second look; `None` for a plain folder inside.
    pub flag: Option<PathFlag>,
}

/// Every path param in `def`, in recipe order, nested steps included.
/// `root` is the project root of a project recipe: a relative value that
/// leaves it through a link is flagged too. `shell-run`'s `cwd` is listed
/// but never flagged: the step itself refuses one that leaves the folder.
pub fn path_params(def: &PipelineDefinition, root: Option<&Path>) -> Vec<PathParam> {
    let mut found = Vec::new();
    collect(&def.nodes, root, &FieldValues::new(), &mut found);
    found
}

fn collect(
    nodes: &[PipelineNode],
    root: Option<&Path>,
    values: &FieldValues,
    found: &mut Vec<PathParam>,
) {
    for node in nodes {
        let values = declare(values, &node.fields);
        found.extend(node_paths(node, root, &values));
        if let Some(children) = &node.children {
            collect(children, root, &values, found);
        }
    }
}

/// The path params one step uses, judged by their filled value first (a
/// field's default is the author's to choose), then as written.
fn node_paths(node: &PipelineNode, root: Option<&Path>, values: &FieldValues) -> Vec<PathParam> {
    let row = |param, value: String, flag| PathParam {
        node_type: node.node_type.clone(),
        filled: fill_fields(&value, values),
        param,
        value,
        flag,
    };
    let base = text(&node.params, "base");
    let mut paths = Vec::new();
    for &(param, kind) in path_kinds(&node.node_type) {
        let flag = |v: &str| judge(kind, v, root, base);
        let judged = |v: &str| {
            fill_fields(v, values)
                .and_then(|f| flag(&f))
                .or_else(|| flag(v))
        };
        paths.extend(texts(&node.params, param).map(|v| row(param, v.clone(), judged(&v))));
    }
    paths.extend(effects(node).map(|(param, value, flag)| row(param, value, flag)));
    paths
}

/// The flag `value` of a `kind` param earns; `base` is the step's `base`,
/// which a listed path sits under.
fn judge(kind: Kind, value: &str, root: Option<&Path>, base: Option<&str>) -> Option<PathFlag> {
    match kind {
        Kind::Folder => path_flag(value, root),
        Kind::UnderBase => path_flag(&under(base, value), root),
        Kind::Name => name_flag(value),
        Kind::Fragment => fragment_flag(value, false),
        Kind::Replacement => fragment_flag(value, true),
        Kind::FindInPath => Some(PathFlag::EditsPath),
    }
}

/// `value` under `base` (`base/value`), or `value` with no base.
fn under(base: Option<&str>, value: &str) -> String {
    match base {
        Some(base) if !value.starts_with(['/', '\\']) => {
            format!("{}/{value}", base.trim_end_matches(['/', '\\']))
        }
        _ => value.to_string(),
    }
}

/// How a path param is judged.
#[derive(Clone, Copy)]
enum Kind {
    /// A folder or file a step reaches.
    Folder,
    /// A path `file-list` lists, under the step's `base`.
    UnderBase,
    /// A name a step gives its output, which may hold folders under it.
    Name,
    /// A part of a file name: a separator in it moves the file.
    Fragment,
    /// A rename's replacement, which `$1` fills from the matched path.
    Replacement,
    /// A rename's `find`: with it set, `replace` edits the whole relative
    /// path, folders included, so its parts can build `..` or `.git`.
    FindInPath,
}

/// Every param `file-rename` builds a file's name from.
const RENAME_PARTS: &[(&str, Kind)] = &[
    ("prefix", Kind::Fragment),
    ("suffix", Kind::Fragment),
    ("pattern", Kind::Name),
    ("replace", Kind::Replacement),
    ("extension", Kind::Fragment),
    ("separator", Kind::Fragment),
    ("find", Kind::FindInPath),
];

/// The path params of `node_type`, each a path or a list of them.
fn path_kinds(node_type: &str) -> &'static [(&'static str, Kind)] {
    use Kind::{Folder, Name};
    match node_type {
        "file-list" => &[("base", Folder), ("paths", Kind::UnderBase)],
        "file-delete" => &[("base", Folder)],
        "file-move" | "file-copy" => &[("destination", Folder)],
        "spreadsheet-export" => &[("path", Folder)],
        "spreadsheet-lookup" => &[("table", Folder)],
        "output" => &[("directory", Folder), ("filename", Name)],
        "file-render" | "file-join" | "file-decode" => &[("filename", Name)],
        "file-rename" => RENAME_PARTS,
        bnto_shell::NODE_TYPE => &[("searchPath", Folder)],
        _ => &[],
    }
}

/// One param a step's effect is read from: its name, value and flag.
type Effect = (&'static str, String, Option<PathFlag>);

/// What a step does to files already there, flagged for what it does, not
/// where: overwrite mode, a copy or export that replaces a file, or a mode
/// filled when it runs (it may resolve to overwrite). And `shell-run`'s
/// `cwd`, shown as written (the step refuses one that leaves). `file-move`
/// has none: it refuses a move onto an existing file.
fn effects(node: &PipelineNode) -> Option<Effect> {
    let p = &node.params;
    let key = effect_param(&node.node_type)?;
    let copy_replaces = |v: &str| v == "overwrite" || v == "newer";
    match node.node_type.as_str() {
        "output" => mode_effect(p, key, |v| v == "overwrite", PathFlag::Overwrite),
        "file-copy" => mode_effect(p, key, copy_replaces, PathFlag::ReplacesExisting),
        "spreadsheet-export" => export_overwrite(p, key),
        _ => text(p, key).map(|value| (key, value.to_string(), None)),
    }
}

/// The param `effects` reads a step's effect from, by node type.
fn effect_param(node_type: &str) -> Option<&'static str> {
    match node_type {
        "output" => Some("mode"),
        "file-copy" => Some("conflict"),
        "spreadsheet-export" => Some("overwrite"),
        bnto_shell::NODE_TYPE => Some("cwd"),
        _ => None,
    }
}

/// `key`'s effect: `flag` when `replaces` its value, `MayReplace` when the
/// value holds a placeholder, none otherwise.
fn mode_effect(
    params: &Map<String, Value>,
    key: &'static str,
    replaces: impl Fn(&str) -> bool,
    flag: PathFlag,
) -> Option<Effect> {
    let value = text(params, key)?;
    let flag = if value.contains("{{") {
        PathFlag::MayReplace
    } else if replaces(value) {
        flag
    } else {
        return None;
    };
    Some((key, value.to_string(), Some(flag)))
}

/// `spreadsheet-export`'s `overwrite`, read as the engine reads it: a JSON
/// `true`, or the text "true" in any case (`export/params.rs`).
fn export_overwrite(params: &Map<String, Value>, key: &'static str) -> Option<Effect> {
    let true_text = |v: &str| v.eq_ignore_ascii_case("true");
    match params.get(key)? {
        Value::Bool(true) => Some((key, "true".to_string(), Some(PathFlag::ReplacesExisting))),
        Value::String(_) => mode_effect(params, key, true_text, PathFlag::ReplacesExisting),
        _ => None,
    }
}

/// Every param of `node_type` the prompt lists, for the guard test that no
/// path-like param is missed.
#[cfg(test)]
fn path_param_names(node_type: &str) -> Vec<&'static str> {
    let paths = path_kinds(node_type).iter().map(|(name, _)| *name);
    paths.chain(effect_param(node_type)).collect()
}

fn text<'a>(params: &'a Map<String, Value>, key: &str) -> Option<&'a str> {
    params
        .get(key)
        .and_then(Value::as_str)
        .filter(|s| !s.trim().is_empty())
}

/// A param's text values: one string, or each string in a list.
fn texts<'a>(params: &'a Map<String, Value>, key: &str) -> impl Iterator<Item = String> + 'a {
    let values: Vec<&Value> = match params.get(key) {
        Some(Value::Array(items)) => items.iter().collect(),
        Some(value) => vec![value],
        None => Vec::new(),
    };
    values
        .into_iter()
        .filter_map(Value::as_str)
        .filter(|s| !s.trim().is_empty())
        .map(str::to_string)
}

#[cfg(test)]
mod tests;
