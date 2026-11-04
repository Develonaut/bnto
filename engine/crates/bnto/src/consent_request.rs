// What a recipe that needs consent is shown as before the user approves it:
// where it came from, its steps by node type (an author's name for a step
// can say anything), the paths it reaches, its shell commands and the files
// from the working folder they run (§4 of decisions/project-recipes.md).

use bnto_core::{PipelineDefinition, PipelineNode};

use crate::catalog::SourceKind;
use crate::consent_files::{RunFile, files_it_runs};
use crate::consent_paths::{PathParam, path_params};

/// What the user is asked to approve before an untrusted recipe runs. Text
/// is as the recipe wrote it; `consent_body` escapes it for display.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsentRequest {
    pub recipe_name: String,
    /// Where it came from: "Project recipe `tidy` from `<file>`".
    pub origin: String,
    /// Each step's node type in recipe order, indented two spaces per level.
    pub steps: Vec<String>,
    /// Every path param, flagged where it reaches outside.
    pub paths: Vec<PathParam>,
    /// Each shell command the recipe runs, as consent shows it.
    pub commands: Vec<String>,
    /// Files from the working folder its commands run or read.
    pub files: Vec<RunFile>,
    /// A project recipe: those files are the project's.
    pub project: bool,
    /// The definition hash an approval is pinned to.
    pub hash: String,
}

impl ConsentRequest {
    /// The request for `def` (named `recipe_name`, resolved as `slug`), read
    /// as `kind`, pinned to `hash`. A project recipe's paths are judged
    /// against its root.
    pub fn new(
        def: &PipelineDefinition,
        recipe_name: String,
        slug: &str,
        kind: &SourceKind,
        hash: String,
    ) -> Self {
        let root = match kind {
            SourceKind::Project { root, .. } => Some(root.as_path()),
            _ => None,
        };
        Self {
            recipe_name,
            origin: origin(kind, slug),
            steps: step_types(&def.nodes),
            paths: path_params(def, root),
            commands: crate::consent_lines::shell_command_lines(&def.nodes),
            files: files_it_runs(def, root),
            project: root.is_some(),
            hash,
        }
    }
}

#[cfg(test)]
impl ConsentRequest {
    /// A library recipe's request that runs `commands`, for tests of the
    /// screens that hold and show it.
    pub fn for_commands(recipe_name: &str, commands: Vec<String>) -> Self {
        Self {
            recipe_name: recipe_name.into(),
            origin: format!(
                "Local recipe `{}`",
                recipe_name.to_lowercase().replace(' ', "-")
            ),
            steps: vec![bnto_shell::NODE_TYPE.into()],
            paths: Vec::new(),
            commands,
            files: Vec::new(),
            project: false,
            hash: "h".into(),
        }
    }
}

/// Where the recipe came from, naming its file when there is one.
pub fn origin(kind: &SourceKind, slug: &str) -> String {
    match kind {
        SourceKind::Bundled => format!("Built-in recipe `{slug}`"),
        SourceKind::Library(Some(file)) => {
            format!("Library recipe `{slug}` from `{}`", file.display())
        }
        SourceKind::Library(None) => format!("Local recipe `{slug}`"),
        SourceKind::Project {
            file: Some(file), ..
        } => {
            format!("Project recipe `{slug}` from `{}`", file.display())
        }
        SourceKind::Project { file: None, .. } => format!("Project recipe `{slug}`, edited here"),
        SourceKind::Path(path) => format!("Recipe file `{}`", path.display()),
    }
}

/// Every node's type, containers' children indented under them.
fn step_types(nodes: &[PipelineNode]) -> Vec<String> {
    let mut steps = Vec::new();
    collect_steps(nodes, 0, &mut steps);
    steps
}

fn collect_steps(nodes: &[PipelineNode], depth: usize, steps: &mut Vec<String>) {
    for node in nodes {
        steps.push(format!("{}{}", "  ".repeat(depth), node.node_type));
        if let Some(children) = &node.children {
            collect_steps(children, depth + 1, steps);
        }
    }
}

#[cfg(test)]
mod tests;
