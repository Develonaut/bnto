// Whether a recipe may run as-is or needs the user's consent first — the
// one decision `bnto run`, `bnto dry-run` and the TUI share, so no surface
// can run a library recipe's commands, or any project recipe, without asking.

use std::path::Path;

use crate::catalog::SourceKind;
pub use crate::consent_request::ConsentRequest;
use crate::trust::{self, TrustLevel, TrustStore};

/// The consent the recipe in `raw_json` still needs, or `None` when it may
/// run: bundled, a library or by-path recipe with no shell steps, or this
/// exact definition already approved (`trust::trust_level`). Unparseable
/// JSON needs none here; the run itself refuses it. `fallback_name` is how
/// the user named the recipe (its slug, or a path), which the prompt shows.
pub fn consent_needed(
    raw_json: &str,
    fallback_name: &str,
    kind: &SourceKind,
    store: &TrustStore,
) -> Option<ConsentRequest> {
    let def: bnto_core::PipelineDefinition = serde_json::from_str(raw_json).ok()?;
    let hash = trust::definition_hash(raw_json);
    let has_shell = trust::has_shell_commands(&def.nodes);
    if trust::trust_level(kind, has_shell, store, &hash) == TrustLevel::Trusted {
        return None;
    }
    let name = recipe_name(raw_json, fallback_name);
    Some(ConsentRequest::new(&def, name, fallback_name, kind, hash))
}

/// `consent_needed` for `raw_json`, shown as `shown_json` would run: the
/// same recipe with what the run fills before it starts (`-p` values).
/// Approval stays pinned to `raw_json`.
pub fn consent_needed_shown(
    raw_json: &str,
    shown_json: &str,
    fallback_name: &str,
    kind: &SourceKind,
    store: &TrustStore,
) -> Option<ConsentRequest> {
    let request = consent_needed(raw_json, fallback_name, kind, store)?;
    let Ok(shown) = serde_json::from_str::<bnto_core::PipelineDefinition>(shown_json) else {
        return Some(request);
    };
    let ConsentRequest {
        recipe_name, hash, ..
    } = request;
    Some(ConsentRequest::new(
        &shown,
        recipe_name,
        fallback_name,
        kind,
        hash,
    ))
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
            { "id": "run", "type": "shell-run", "version": "1.0.0",
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
        let request = consent_needed(
            SHELL_RECIPE,
            "probe",
            &SourceKind::Library(None),
            &TrustStore::default(),
        )
        .expect("a library shell recipe must ask");
        assert_eq!(request.recipe_name, "Probe");
        assert_eq!(request.commands, vec!["git --version"]);
        assert_eq!(request.hash, trust::definition_hash(SHELL_RECIPE));
    }

    #[test]
    fn bundled_recipes_run_without_asking() {
        assert_eq!(
            consent_needed(
                SHELL_RECIPE,
                "probe",
                &SourceKind::Bundled,
                &TrustStore::default()
            ),
            None
        );
    }

    #[test]
    fn recipes_without_shell_steps_run_without_asking() {
        assert_eq!(
            consent_needed(
                PLAIN_RECIPE,
                "plain",
                &SourceKind::Library(None),
                &TrustStore::default()
            ),
            None
        );
    }

    #[test]
    fn an_approved_definition_runs_without_asking() {
        let mut store = TrustStore::default();
        store.approve(trust::definition_hash(SHELL_RECIPE));
        assert_eq!(
            consent_needed(SHELL_RECIPE, "probe", &SourceKind::Library(None), &store),
            None
        );
    }

    #[test]
    fn approval_is_pinned_to_the_exact_definition() {
        let mut store = TrustStore::default();
        store.approve(trust::definition_hash(SHELL_RECIPE));
        let edited = SHELL_RECIPE.replace("--version", "--help");
        assert!(consent_needed(&edited, "probe", &SourceKind::Library(None), &store).is_some());
    }

    fn project() -> SourceKind {
        SourceKind::Project {
            root: std::path::PathBuf::from("project"),
            file: None,
        }
    }

    #[test]
    fn a_project_recipe_without_shell_steps_needs_consent() {
        let request = consent_needed(PLAIN_RECIPE, "plain", &project(), &TrustStore::default())
            .expect("a project recipe must ask, shell or not");
        assert_eq!(request.recipe_name, "Plain");
        assert!(request.commands.is_empty());
        assert_eq!(request.hash, trust::definition_hash(PLAIN_RECIPE));
    }

    #[test]
    fn an_approved_project_recipe_runs_without_asking() {
        let mut store = TrustStore::default();
        store.approve(trust::definition_hash(PLAIN_RECIPE));
        assert_eq!(
            consent_needed(PLAIN_RECIPE, "plain", &project(), &store),
            None
        );
    }

    #[test]
    fn an_edited_project_recipe_asks_again() {
        let mut store = TrustStore::default();
        store.approve(trust::definition_hash(PLAIN_RECIPE));
        let edited = PLAIN_RECIPE.replace("Plain", "Plainer");
        assert!(consent_needed(&edited, "plain", &project(), &store).is_some());
    }

    #[test]
    fn by_path_recipes_without_shell_steps_run_without_asking() {
        let store = TrustStore::default();
        assert_eq!(
            consent_needed(
                PLAIN_RECIPE,
                "plain",
                &SourceKind::Path("x.bnto.json".into()),
                &store
            ),
            None
        );
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
