// Recipe trust classification and consent management.
//
// Recipes are classified by source (built-in vs local file) and whether
// they contain shell-command nodes. Built-in recipes are always trusted.
// Local recipes with shell-command nodes require first-run consent, cached
// per definition hash in `BntoPaths::state/trusted.json`.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use bnto_core::PipelineNode;

/// Where the recipe came from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipeSource {
    /// Shipped with the bnto binary — audited, always trusted.
    BuiltIn,
    /// Loaded from a local `.bnto.json` file.
    LocalFile(PathBuf),
}

/// Trust level determines whether consent is needed before execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrustLevel {
    /// Safe to run without prompting (built-in, or no shell commands).
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

/// Classify a recipe argument as built-in or local file.
pub fn classify_source(recipe_arg: &str) -> RecipeSource {
    if bnto_engine::recipes::builtin_recipe_by_slug(recipe_arg).is_some() {
        RecipeSource::BuiltIn
    } else {
        RecipeSource::LocalFile(PathBuf::from(recipe_arg))
    }
}

/// Check whether any node in the tree is a shell-command processor.
pub fn has_shell_commands(nodes: &[PipelineNode]) -> bool {
    for node in nodes {
        if node.node_type == "shell-command" {
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

/// Determine trust level for a recipe.
pub fn trust_level(
    source: &RecipeSource,
    has_shell: bool,
    store: &TrustStore,
    hash: &str,
) -> TrustLevel {
    match source {
        RecipeSource::BuiltIn => TrustLevel::Trusted,
        RecipeSource::LocalFile(_) => {
            if !has_shell || store.is_approved(hash) {
                TrustLevel::Trusted
            } else {
                TrustLevel::FirstRun
            }
        }
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
    eprintln!("  Run `bnto dry-run <recipe>` to inspect the full command details.");
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

/// Extract unique shell command binary names from a node tree.
pub fn collect_shell_command_names(nodes: &[PipelineNode]) -> Vec<String> {
    let mut names = Vec::new();
    collect_names_recursive(nodes, &mut names);
    names.sort();
    names.dedup();
    names
}

fn collect_names_recursive(nodes: &[PipelineNode], names: &mut Vec<String>) {
    for node in nodes {
        if node.node_type == "shell-command"
            && let Some(cmd) = node.params.get("command").and_then(|v| v.as_str())
        {
            names.push(cmd.to_string());
        }
        if let Some(children) = &node.children {
            collect_names_recursive(children, names);
        }
    }
}

/// Format a trust level as a display label for the TUI.
pub fn trust_badge(source: &RecipeSource, has_shell: bool) -> &'static str {
    match source {
        RecipeSource::BuiltIn => "built-in",
        RecipeSource::LocalFile(_) => {
            if has_shell {
                "local (shell)"
            } else {
                "local"
            }
        }
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
    use serde_json::json;

    // --- classify_source ---

    #[test]
    fn classify_builtin_slug() {
        assert_eq!(classify_source("compress-images"), RecipeSource::BuiltIn,);
    }

    #[test]
    fn classify_local_path() {
        let source = classify_source("./my-recipe.bnto.json");
        assert!(matches!(source, RecipeSource::LocalFile(_)));
        if let RecipeSource::LocalFile(p) = source {
            assert_eq!(p, PathBuf::from("./my-recipe.bnto.json"));
        }
    }

    #[test]
    fn classify_unknown_slug_is_local() {
        let source = classify_source("nonexistent-recipe");
        assert!(matches!(source, RecipeSource::LocalFile(_)));
    }

    // --- has_shell_commands ---

    fn node(node_type: &str, children: Option<Vec<PipelineNode>>) -> PipelineNode {
        PipelineNode {
            id: "n1".into(),
            node_type: node_type.into(),
            params: serde_json::Map::new(),
            children,
            fields: Default::default(),
        }
    }

    #[test]
    fn has_shell_commands_true() {
        let nodes = vec![node("shell-command", None)];
        assert!(has_shell_commands(&nodes));
    }

    #[test]
    fn has_shell_commands_false() {
        let nodes = vec![node("image-compress", None)];
        assert!(!has_shell_commands(&nodes));
    }

    #[test]
    fn has_shell_commands_nested() {
        let nodes = vec![node("loop", Some(vec![node("shell-command", None)]))];
        assert!(has_shell_commands(&nodes));
    }

    #[test]
    fn has_shell_commands_deep_nesting() {
        let nodes = vec![node(
            "group",
            Some(vec![node("loop", Some(vec![node("shell-command", None)]))]),
        )];
        assert!(has_shell_commands(&nodes));
    }

    // --- trust_level ---

    #[test]
    fn builtin_no_shell_is_trusted() {
        let store = TrustStore::default();
        assert_eq!(
            trust_level(&RecipeSource::BuiltIn, false, &store, "hash"),
            TrustLevel::Trusted,
        );
    }

    #[test]
    fn builtin_with_shell_is_trusted() {
        let store = TrustStore::default();
        assert_eq!(
            trust_level(&RecipeSource::BuiltIn, true, &store, "hash"),
            TrustLevel::Trusted,
        );
    }

    #[test]
    fn local_no_shell_is_trusted() {
        let store = TrustStore::default();
        let source = RecipeSource::LocalFile("recipe.bnto.json".into());
        assert_eq!(
            trust_level(&source, false, &store, "hash"),
            TrustLevel::Trusted,
        );
    }

    #[test]
    fn local_with_shell_first_run() {
        let store = TrustStore::default();
        let source = RecipeSource::LocalFile("recipe.bnto.json".into());
        assert_eq!(
            trust_level(&source, true, &store, "hash"),
            TrustLevel::FirstRun,
        );
    }

    #[test]
    fn local_with_shell_approved() {
        let mut store = TrustStore::default();
        store.approve("abc123".into());
        let source = RecipeSource::LocalFile("recipe.bnto.json".into());
        assert_eq!(
            trust_level(&source, true, &store, "abc123"),
            TrustLevel::Trusted,
        );
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
        let store = TrustStore::load(Path::new("/nonexistent/path/trusted.json"));
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

    // --- collect_shell_command_names ---

    #[test]
    fn collect_names_from_shell_nodes() {
        let mut params = serde_json::Map::new();
        params.insert("command".into(), json!("yt-dlp"));

        let nodes = vec![PipelineNode {
            id: "dl".into(),
            node_type: "shell-command".into(),
            params,
            children: None,
            fields: Default::default(),
        }];
        assert_eq!(collect_shell_command_names(&nodes), vec!["yt-dlp"]);
    }

    #[test]
    fn collect_names_deduplicates() {
        let make_node = |cmd: &str| {
            let mut params = serde_json::Map::new();
            params.insert("command".into(), json!(cmd));
            PipelineNode {
                id: "n".into(),
                node_type: "shell-command".into(),
                params,
                children: None,
                fields: Default::default(),
            }
        };
        let nodes = vec![
            make_node("ffmpeg"),
            make_node("yt-dlp"),
            make_node("ffmpeg"),
        ];
        assert_eq!(
            collect_shell_command_names(&nodes),
            vec!["ffmpeg", "yt-dlp"],
        );
    }

    #[test]
    fn collect_names_empty_for_non_shell() {
        let nodes = vec![node("image-compress", None)];
        assert!(collect_shell_command_names(&nodes).is_empty());
    }

    // --- trust_badge ---

    #[test]
    fn badge_builtin() {
        assert_eq!(trust_badge(&RecipeSource::BuiltIn, false), "built-in");
        assert_eq!(trust_badge(&RecipeSource::BuiltIn, true), "built-in");
    }

    #[test]
    fn badge_local_no_shell() {
        let source = RecipeSource::LocalFile("r.bnto.json".into());
        assert_eq!(trust_badge(&source, false), "local");
    }

    #[test]
    fn badge_local_with_shell() {
        let source = RecipeSource::LocalFile("r.bnto.json".into());
        assert_eq!(trust_badge(&source, true), "local (shell)");
    }
}
