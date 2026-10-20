// Whether a recipe may run as-is or needs the user's consent first — the
// one decision `bnto run`, `bnto dry-run` and the TUI share, so no surface
// can run a library recipe's commands without asking.

use std::path::Path;

use crate::trust::{self, TrustLevel, TrustStore};

/// What the user is asked to approve before an untrusted recipe runs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsentRequest {
    pub recipe_name: String,
    /// Each shell command the recipe runs, as consent shows it.
    pub commands: Vec<String>,
    /// The definition hash an approval is pinned to.
    pub hash: String,
}

/// The consent the recipe in `raw_json` still needs, or `None` when it may
/// run: bundled, no shell steps, or this exact definition already approved.
/// Unparseable JSON needs none here; the run itself refuses it.
pub fn consent_needed(
    raw_json: &str,
    fallback_name: &str,
    is_bundled: bool,
    store: &TrustStore,
) -> Option<ConsentRequest> {
    let def: bnto_core::PipelineDefinition = serde_json::from_str(raw_json).ok()?;
    let hash = trust::definition_hash(raw_json);
    let has_shell = trust::has_shell_commands(&def.nodes);
    if trust::trust_level(is_bundled, has_shell, store, &hash) == TrustLevel::Trusted {
        return None;
    }
    Some(ConsentRequest {
        recipe_name: recipe_name(raw_json, fallback_name),
        commands: crate::consent_lines::shell_command_lines(&def.nodes),
        hash,
    })
}

/// The recipe's `name`, falling back to how the user referred to it.
fn recipe_name(raw_json: &str, fallback: &str) -> String {
    serde_json::from_str::<serde_json::Value>(raw_json)
        .ok()
        .and_then(|v| v["name"].as_str().map(String::from))
        .unwrap_or_else(|| fallback.to_string())
}

/// Remember the approval of `hash` in the trust store at `path`.
pub fn record_approval(path: &Path, hash: String) {
    let mut store = TrustStore::load(path);
    store.approve(hash);
    let _ = store.save(path);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SHELL_RECIPE: &str = r#"{
        "id": "probe", "type": "group", "version": "1.0.0", "name": "Probe",
        "nodes": [
            { "id": "run", "type": "shell-command", "version": "1.0.0",
              "parameters": { "command": "git", "args": ["--version"] } }
        ]
    }"#;

    const PLAIN_RECIPE: &str = r#"{
        "id": "plain", "type": "group", "version": "1.0.0", "name": "Plain",
        "nodes": [
            { "id": "r", "type": "file-rename", "version": "1.0.0", "parameters": {} }
        ]
    }"#;

    #[test]
    fn untrusted_shell_recipe_needs_consent() {
        let request = consent_needed(SHELL_RECIPE, "probe", false, &TrustStore::default())
            .expect("a library shell recipe must ask");
        assert_eq!(request.recipe_name, "Probe");
        assert_eq!(request.commands, vec!["git --version"]);
        assert_eq!(request.hash, trust::definition_hash(SHELL_RECIPE));
    }

    #[test]
    fn bundled_recipes_run_without_asking() {
        assert_eq!(
            consent_needed(SHELL_RECIPE, "probe", true, &TrustStore::default()),
            None
        );
    }

    #[test]
    fn recipes_without_shell_steps_run_without_asking() {
        assert_eq!(
            consent_needed(PLAIN_RECIPE, "plain", false, &TrustStore::default()),
            None
        );
    }

    #[test]
    fn an_approved_definition_runs_without_asking() {
        let mut store = TrustStore::default();
        store.approve(trust::definition_hash(SHELL_RECIPE));
        assert_eq!(consent_needed(SHELL_RECIPE, "probe", false, &store), None);
    }

    #[test]
    fn approval_is_pinned_to_the_exact_definition() {
        let mut store = TrustStore::default();
        store.approve(trust::definition_hash(SHELL_RECIPE));
        let edited = SHELL_RECIPE.replace("--version", "--help");
        assert!(consent_needed(&edited, "probe", false, &store).is_some());
    }

    #[test]
    fn recorded_approval_is_read_back() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("state").join("trusted.json");
        let hash = trust::definition_hash(SHELL_RECIPE);
        record_approval(&path, hash.clone());
        assert!(TrustStore::load(&path).is_approved(&hash));
    }
}
