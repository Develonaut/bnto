//! The output node's `directory`, filled in before a run starts. `bnto run`
//! and the TUI make the folder before any step runs, so each placeholder in
//! it needs a value then; one left as text would name a folder `{{env.KEY}}`
//! (ISS-22). `bnto check` flags the ones no run can fill.

mod placeholder;

use std::cell::RefCell;
use std::collections::BTreeMap;

use crate::check::CheckFinding;
use crate::context::ProcessContext;
use crate::executor::field_scope::along;
use crate::executor::interpolate::interpolate;
use crate::executor::template::build_node_outputs_for_input;
use crate::pipeline::{PipelineDefinition, PipelineFile, PipelineNode, output_node_path};
use placeholder::{Recipe, Sources, never_filled};

/// The recipe's output folder with every placeholder filled, or `None` when
/// its output node names none. An `Err` names each placeholder nothing
/// fills, one line each, so the run stops before any folder is made.
///
/// Fills the output node's fields in scope, `{{env.*}}` from the recipe's
/// declared `secrets`, the `{{ctx.*}}` known before a run, and the input
/// node's file metadata from `files`. A filled value is never rescanned.
pub fn fill_output_directory(
    def: &PipelineDefinition,
    ctx: &dyn ProcessContext,
    files: &[PipelineFile],
) -> Result<Option<String>, String> {
    let Some(out) = OutputDir::of(def) else {
        return Ok(None);
    };
    let sources = Sources {
        fields: along(&out.scope),
        inputs: build_node_outputs_for_input(&def.nodes, files),
        ctx,
    };
    let values = fill_each(&out, &Recipe::of(def), &sources)
        .map_err(|problems| lines(out.node, problems))?;
    Ok(Some(interpolate(&out.dir, |inner| {
        values.get(inner).cloned()
    })))
}

/// A finding for each placeholder in the output folder that no run can fill.
pub(crate) fn output_directory_findings(def: &PipelineDefinition) -> Vec<CheckFinding> {
    let Some(out) = OutputDir::of(def) else {
        return Vec::new();
    };
    let recipe = Recipe::of(def);
    placeholders(&out.dir)
        .iter()
        .filter_map(|inner| never_filled(inner, &recipe))
        .map(|why| finding(out.node, why))
        .collect()
}

/// The output node, the containers around it and its non-empty `directory`.
struct OutputDir<'a> {
    node: &'a PipelineNode,
    /// The containers around `node`, outermost first, then `node` itself.
    scope: Vec<&'a PipelineNode>,
    dir: String,
}

impl<'a> OutputDir<'a> {
    fn of(def: &'a PipelineDefinition) -> Option<Self> {
        let scope = output_node_path(&def.nodes)?;
        let node = *scope.last()?;
        let dir = node.params.get("directory")?.as_str()?.to_string();
        (!dir.is_empty()).then_some(Self { node, scope, dir })
    }
}

/// Each placeholder's value, or why each one that has none has none.
fn fill_each(
    out: &OutputDir,
    recipe: &Recipe,
    sources: &Sources,
) -> Result<BTreeMap<String, String>, Vec<String>> {
    let mut values = BTreeMap::new();
    let mut problems = Vec::new();
    for inner in placeholders(&out.dir) {
        match never_filled(&inner, recipe).map_or_else(|| sources.value(&inner), Err) {
            Ok(value) => {
                values.insert(inner, value);
            }
            Err(why) => problems.push(why),
        }
    }
    if problems.is_empty() {
        Ok(values)
    } else {
        Err(problems)
    }
}

/// Each `{{…}}` in `dir`, inside the braces, once, in order. Read the way a
/// fill reads them (`interpolate`), so the two never disagree.
fn placeholders(dir: &str) -> Vec<String> {
    let found = RefCell::new(Vec::<String>::new());
    interpolate(dir, |inner| {
        let mut found = found.borrow_mut();
        if !found.iter().any(|f| f == inner) {
            found.push(inner.to_string());
        }
        None
    });
    found.into_inner()
}

fn finding(output: &PipelineNode, message: String) -> CheckFinding {
    CheckFinding {
        node_id: output.id.clone(),
        node_type: output.node_type.clone(),
        param: Some("directory".to_string()),
        message,
    }
}

/// One line per problem, as `bnto check` prints its findings.
fn lines(output: &PipelineNode, problems: Vec<String>) -> String {
    let lines: Vec<String> = problems
        .into_iter()
        .map(|why| finding(output, why).to_string())
        .collect();
    lines.join("\n")
}

#[cfg(test)]
mod tests;
