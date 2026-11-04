// Recipe trust classification and consent management.
//
// Recipes are classified by where they came from (`SourceKind`): bundled
// ones are always trusted, library and by-path ones need consent for
// shell-run nodes, and project ones need it whatever they run. Consent is
// cached per definition hash in `BntoPaths::state/trusted.json`.

use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use bnto_core::PipelineNode;

use crate::catalog::SourceKind;

/// Trust level determines whether consent is needed before execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrustLevel {
    /// Safe to run without prompting (bundled, or no shell commands).
    Trusted,
    /// First time seeing this recipe — prompt for consent.
    FirstRun,
}

/// A cached approval record for a trusted recipe.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Approval {
    /// ISO 8601 timestamp of when consent was given.
    approved_at: String,
}

/// Persistent store for recipe trust approvals.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrustStore {
    /// Map of definition SHA-256 hash → approval record.
    approvals: HashMap<String, Approval>,
}

impl TrustStore {
    /// Load trust store from a JSON file, returning defaults on any error.
    pub fn load(path: &Path) -> Self {
        let Ok(contents) = std::fs::read_to_string(path) else {
            return Self::default();
        };
        serde_json::from_str(&contents).unwrap_or_default()
    }

    /// Save trust store to a JSON file, creating parent directories.
    pub fn save(&self, path: &Path) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create trust directory: {e}"))?;
        }
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize trust store: {e}"))?;
        std::fs::write(path, json).map_err(|e| format!("Failed to write trust store: {e}"))
    }

    /// Check whether a recipe hash has been approved.
    pub fn is_approved(&self, hash: &str) -> bool {
        self.approvals.contains_key(hash)
    }

    /// Record approval for a recipe hash.
    pub fn approve(&mut self, hash: String) {
        self.approvals.insert(
            hash,
            Approval {
                approved_at: now_iso8601(),
            },
        );
    }
}

/// Check whether any node in the tree is a shell-run processor.
pub fn has_shell_commands(nodes: &[PipelineNode]) -> bool {
    for node in nodes {
        if node.node_type == bnto_shell::NODE_TYPE {
            return true;
        }
        if let Some(children) = &node.children
            && has_shell_commands(children)
        {
            return true;
        }
    }
    false
}

/// Determine trust level for a recipe from where it came from (`kind`).
///
/// Bundled recipes are always trusted. A project recipe asks until this
/// exact definition is approved, shell or not: it can move and delete
/// files without one. Library and by-path recipes ask only for shell steps.
pub fn trust_level(
    kind: &SourceKind,
    has_shell: bool,
    store: &TrustStore,
    hash: &str,
) -> TrustLevel {
    let asks = match kind {
        SourceKind::Bundled => false,
        SourceKind::Project(_) => true,
        SourceKind::Library | SourceKind::Path => has_shell,
    };
    if !asks || store.is_approved(hash) {
        TrustLevel::Trusted
    } else {
        TrustLevel::FirstRun
    }
}

/// Compute SHA-256 hash of a recipe definition string.
pub fn definition_hash(json: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(json.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Print consent prompt to stderr and read response from stdin.
///
/// Shows the recipe name, lists shell commands that will execute,
/// and asks for y/n confirmation. Returns true if the user consents.
pub fn prompt_consent(recipe_name: &str, shell_commands: &[String]) -> bool {
    use std::io::{BufRead, Write};

    eprintln!();
    eprintln!("  Warning: This recipe executes external commands:");
    for cmd in shell_commands {
        eprintln!("    - {cmd}");
    }
    eprintln!();
    eprintln!("  Recipe: {recipe_name}");
    eprintln!("  Run `bnto dry-run <recipe>` without input files to inspect the full commands.");
    eprintln!();
    eprint!("  Allow execution? [y/N] ");
    let _ = std::io::stderr().flush();

    let stdin = std::io::stdin();
    let mut line = String::new();
    if stdin.lock().read_line(&mut line).is_err() {
        return false;
    }
    matches!(line.trim().to_lowercase().as_str(), "y" | "yes")
}

/// Format a trust badge for display in the TUI.
pub fn trust_badge(kind: &SourceKind, has_shell: bool) -> &'static str {
    match kind {
        SourceKind::Bundled => "built-in",
        SourceKind::Project(_) => "project",
        SourceKind::Library | SourceKind::Path if has_shell => "local (shell)",
        SourceKind::Library | SourceKind::Path => "local",
    }
}

fn now_iso8601() -> String {
    // Simple timestamp without external chrono dependency.
    let dur = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}", dur.as_secs())
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- has_shell_commands ---

    fn node(node_type: &str, children: Option<Vec<PipelineNode>>) -> PipelineNode {
        PipelineNode {
            id: "n1".into(),
            node_type: node_type.into(),
            params: serde_json::Map::new(),
            children,
            edges: vec![],
            fields: Default::default(),
            always: false,
            when: None,
            requires: vec![],
        }
    }

    #[test]
    fn has_shell_commands_true() {
        let nodes = vec![node("shell-run", None)];
        assert!(has_shell_commands(&nodes));
    }

    #[test]
    fn has_shell_commands_matches_the_registered_processor() {
        use bnto_core::processor::NodeProcessor;
        let name = bnto_shell::ShellRun::new().name().to_string();
        assert!(has_shell_commands(&[node(&name, None)]));
    }

    #[test]
    fn has_shell_commands_false() {
        let nodes = vec![node("image-compress", None)];
        assert!(!has_shell_commands(&nodes));
    }

    #[test]
    fn has_shell_commands_nested() {
        let nodes = vec![node("loop", Some(vec![node("shell-run", None)]))];
        assert!(has_shell_commands(&nodes));
    }

    #[test]
    fn has_shell_commands_deep_nesting() {
        let nodes = vec![node(
            "group",
            Some(vec![node("loop", Some(vec![node("shell-run", None)]))]),
        )];
        assert!(has_shell_commands(&nodes));
    }

    // --- trust_level ---

    fn project() -> SourceKind {
        SourceKind::Project(std::path::PathBuf::from("project"))
    }

    #[test]
    fn bundled_no_shell_is_trusted() {
        let store = TrustStore::default();
        let level = trust_level(&SourceKind::Bundled, false, &store, "hash");
        assert_eq!(level, TrustLevel::Trusted);
    }

    #[test]
    fn bundled_with_shell_is_trusted() {
        let store = TrustStore::default();
        let level = trust_level(&SourceKind::Bundled, true, &store, "hash");
        assert_eq!(level, TrustLevel::Trusted);
    }

    #[test]
    fn library_and_path_no_shell_is_trusted() {
        let store = TrustStore::default();
        for kind in [SourceKind::Library, SourceKind::Path] {
            assert_eq!(
                trust_level(&kind, false, &store, "hash"),
                TrustLevel::Trusted
            );
        }
    }

    #[test]
    fn library_and_path_with_shell_first_run() {
        let store = TrustStore::default();
        for kind in [SourceKind::Library, SourceKind::Path] {
            assert_eq!(
                trust_level(&kind, true, &store, "hash"),
                TrustLevel::FirstRun
            );
        }
    }

    #[test]
    fn library_with_shell_approved() {
        let mut store = TrustStore::default();
        store.approve("abc123".into());
        let level = trust_level(&SourceKind::Library, true, &store, "abc123");
        assert_eq!(level, TrustLevel::Trusted);
    }

    #[test]
    fn project_no_shell_first_run() {
        let store = TrustStore::default();
        assert_eq!(
            trust_level(&project(), false, &store, "hash"),
            TrustLevel::FirstRun
        );
    }

    #[test]
    fn project_with_shell_first_run() {
        let store = TrustStore::default();
        assert_eq!(
            trust_level(&project(), true, &store, "hash"),
            TrustLevel::FirstRun
        );
    }

    #[test]
    fn project_approved_is_trusted() {
        let mut store = TrustStore::default();
        store.approve("abc123".into());
        let level = trust_level(&project(), false, &store, "abc123");
        assert_eq!(level, TrustLevel::Trusted);
    }

    // --- definition_hash ---

    #[test]
    fn hash_is_deterministic() {
        let json = r#"{"name": "test"}"#;
        let h1 = definition_hash(json);
        let h2 = definition_hash(json);
        assert_eq!(h1, h2);
    }

    #[test]
    fn hash_changes_with_content() {
        let h1 = definition_hash(r#"{"name": "test1"}"#);
        let h2 = definition_hash(r#"{"name": "test2"}"#);
        assert_ne!(h1, h2);
    }

    #[test]
    fn hash_is_64_hex_chars() {
        let h = definition_hash("anything");
        assert_eq!(h.len(), 64);
        assert!(h.chars().all(|c| c.is_ascii_hexdigit()));
    }

    // --- TrustStore ---

    #[test]
    fn store_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("trusted.json");

        let mut store = TrustStore::default();
        store.approve("hash1".into());
        store.approve("hash2".into());
        store.save(&path).unwrap();

        let loaded = TrustStore::load(&path);
        assert!(loaded.is_approved("hash1"));
        assert!(loaded.is_approved("hash2"));
        assert!(!loaded.is_approved("hash3"));
    }

    #[test]
    fn store_load_missing_file_returns_default() {
        let tmp = tempfile::tempdir().unwrap();
        let store = TrustStore::load(&tmp.path().join("trusted.json"));
        assert!(store.approvals.is_empty());
    }

    #[test]
    fn store_load_invalid_json_returns_default() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("trusted.json");
        std::fs::write(&path, "not valid json").unwrap();

        let store = TrustStore::load(&path);
        assert!(store.approvals.is_empty());
    }

    // --- trust_badge ---

    #[test]
    fn badge_bundled() {
        assert_eq!(trust_badge(&SourceKind::Bundled, false), "built-in");
        assert_eq!(trust_badge(&SourceKind::Bundled, true), "built-in");
    }

    #[test]
    fn badge_library_no_shell() {
        assert_eq!(trust_badge(&SourceKind::Library, false), "local");
        assert_eq!(trust_badge(&SourceKind::Path, false), "local");
    }

    #[test]
    fn badge_library_with_shell() {
        assert_eq!(trust_badge(&SourceKind::Library, true), "local (shell)");
        assert_eq!(trust_badge(&SourceKind::Path, true), "local (shell)");
    }

    #[test]
    fn badge_project() {
        assert_eq!(trust_badge(&project(), false), "project");
        assert_eq!(trust_badge(&project(), true), "project");
    }
}
