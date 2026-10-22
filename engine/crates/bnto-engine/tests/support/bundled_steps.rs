// The script-mode steps of every bundled recipe, for the linters that hold
// them to a standard: ShellCheck (bundled_scripts.rs) and PSScriptAnalyzer
// (bundled_powershell.rs).
#![allow(dead_code)]

use bnto_engine::recipes::builtin_recipes;
use serde_json::Value;

/// Shells that run a PowerShell script.
pub const POWERSHELLS: &[&str] = &["powershell", "pwsh"];

/// A script-mode step in a bundled recipe.
pub struct ScriptStep {
    pub at: String,
    pub shell: String,
    pub script: String,
}

/// Every script-mode step in the bundled recipes, container children included.
pub fn bundled_script_steps() -> Vec<ScriptStep> {
    let mut steps = Vec::new();
    for recipe in builtin_recipes() {
        let def: Value = serde_json::from_str(recipe.definition_json).unwrap();
        collect_steps(&recipe.slug, &def["nodes"], &mut steps);
    }
    steps
}

fn collect_steps(recipe: &str, nodes: &Value, out: &mut Vec<ScriptStep>) {
    for node in nodes.as_array().into_iter().flatten() {
        let params = &node["parameters"];
        if let Some(script) = params["script"].as_str() {
            out.push(ScriptStep {
                at: format!("{recipe}/{}", node["id"].as_str().unwrap_or("?")),
                shell: params["shell"].as_str().unwrap_or("sh").to_string(),
                script: script.to_string(),
            });
        }
        collect_steps(recipe, &node["nodes"], out);
    }
}

impl ScriptStep {
    /// Whether a PowerShell runs it.
    pub fn is_powershell(&self) -> bool {
        POWERSHELLS.contains(&self.shell.as_str())
    }
}
