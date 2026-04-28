// Secret declarations for recipes — pre-flight validation of required env vars.
//
// Recipes declare what secrets they need via a `secrets` array in the
// PipelineDefinition. This module defines the type and validation logic.
// See strategy/recipe-secrets.md for the full design.

use serde::{Deserialize, Serialize};

use crate::context::ProcessContext;

/// A secret required by a recipe, declared in the definition's `secrets` array.
///
/// The `key` matches the `{{env.KEY}}` placeholder used in node params.
/// At execution time, the key is resolved via `ProcessContext::env_var()`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SecretDef {
    /// Environment variable name (e.g., `"OPENAI_API_KEY"`).
    pub key: String,

    /// Human-readable description shown in error messages and `bnto doctor`.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,

    /// Whether the pipeline should refuse to start without this secret.
    /// Optional secrets resolve to empty string if absent.
    #[serde(default = "default_required")]
    pub required: bool,
}

fn default_required() -> bool {
    true
}

/// Result of validating a single secret.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecretStatus {
    pub key: String,
    pub description: String,
    pub required: bool,
    pub present: bool,
}

/// Validate that all required secrets are available via the ProcessContext.
///
/// Returns a list of statuses for each declared secret. Use `missing_required()`
/// to extract only the ones that would block execution.
pub fn check_secrets(secrets: &[SecretDef], ctx: &dyn ProcessContext) -> Vec<SecretStatus> {
    secrets
        .iter()
        .map(|s| SecretStatus {
            key: s.key.clone(),
            description: s.description.clone(),
            required: s.required,
            present: ctx.env_var(&s.key).is_some(),
        })
        .collect()
}

/// Filter statuses to only required secrets that are missing.
pub fn missing_required(statuses: &[SecretStatus]) -> Vec<&SecretStatus> {
    statuses
        .iter()
        .filter(|s| s.required && !s.present)
        .collect()
}

/// Format a user-friendly error message for missing required secrets.
pub fn format_missing_error(missing: &[&SecretStatus]) -> String {
    let mut lines = vec!["Missing required secrets:".to_string()];
    for s in missing {
        if s.description.is_empty() {
            lines.push(format!("  - {}", s.key));
        } else {
            lines.push(format!("  - {} ({})", s.key, s.description));
        }
    }
    lines.push(String::new());
    lines.push("Set them via environment variables or add to ~/.config/bnto/.env".to_string());
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::NoopContext;
    use std::collections::BTreeMap;
    use std::path::{Path, PathBuf};

    /// Mock context that returns controlled env var values.
    struct MockCtx {
        env: BTreeMap<String, String>,
    }

    impl MockCtx {
        fn new(pairs: &[(&str, &str)]) -> Self {
            Self {
                env: pairs
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect(),
            }
        }
    }

    impl ProcessContext for MockCtx {
        fn run_command(&self, _cmd: &str, _args: &[&str]) -> Result<Vec<u8>, crate::BntoError> {
            Err(crate::BntoError::ProcessingFailed("mock".into()))
        }
        fn temp_file(&self, _suffix: &str) -> Result<PathBuf, crate::BntoError> {
            Ok(PathBuf::from("/tmp/mock"))
        }
        fn env_var(&self, key: &str) -> Option<String> {
            self.env.get(key).cloned()
        }
        fn work_dir(&self) -> Result<&Path, crate::BntoError> {
            Err(crate::BntoError::ProcessingFailed("mock".into()))
        }
    }

    fn secret(key: &str, required: bool) -> SecretDef {
        SecretDef {
            key: key.to_string(),
            description: String::new(),
            required,
        }
    }

    fn secret_with_desc(key: &str, desc: &str, required: bool) -> SecretDef {
        SecretDef {
            key: key.to_string(),
            description: desc.to_string(),
            required,
        }
    }

    #[test]
    fn all_required_present() {
        let secrets = vec![secret("API_KEY", true), secret("DB_URL", true)];
        let ctx = MockCtx::new(&[("API_KEY", "sk-123"), ("DB_URL", "postgres://")]);
        let statuses = check_secrets(&secrets, &ctx);
        let missing = missing_required(&statuses);
        assert!(missing.is_empty());
    }

    #[test]
    fn required_secret_missing() {
        let secrets = vec![secret("API_KEY", true)];
        let ctx = MockCtx::new(&[]);
        let statuses = check_secrets(&secrets, &ctx);
        let missing = missing_required(&statuses);
        assert_eq!(missing.len(), 1);
        assert_eq!(missing[0].key, "API_KEY");
    }

    #[test]
    fn optional_secret_missing_is_ok() {
        let secrets = vec![secret("OPTIONAL_KEY", false)];
        let ctx = MockCtx::new(&[]);
        let statuses = check_secrets(&secrets, &ctx);
        let missing = missing_required(&statuses);
        assert!(missing.is_empty());
    }

    #[test]
    fn mixed_required_and_optional() {
        let secrets = vec![
            secret("REQUIRED", true),
            secret("OPTIONAL", false),
            secret("ALSO_REQUIRED", true),
        ];
        let ctx = MockCtx::new(&[("REQUIRED", "yes")]);
        let statuses = check_secrets(&secrets, &ctx);
        let missing = missing_required(&statuses);
        assert_eq!(missing.len(), 1);
        assert_eq!(missing[0].key, "ALSO_REQUIRED");
    }

    #[test]
    fn empty_secrets_list() {
        let ctx = MockCtx::new(&[]);
        let statuses = check_secrets(&[], &ctx);
        assert!(statuses.is_empty());
    }

    #[test]
    fn noop_context_returns_all_missing() {
        let secrets = vec![secret("KEY", true)];
        let statuses = check_secrets(&secrets, &NoopContext);
        assert!(!statuses[0].present);
    }

    #[test]
    fn format_error_with_descriptions() {
        let statuses = [
            SecretStatus {
                key: "API_KEY".into(),
                description: "OpenAI API key".into(),
                required: true,
                present: false,
            },
            SecretStatus {
                key: "DB_URL".into(),
                description: String::new(),
                required: true,
                present: false,
            },
        ];
        let missing: Vec<&SecretStatus> = statuses.iter().collect();
        let msg = format_missing_error(&missing);
        assert!(msg.contains("API_KEY (OpenAI API key)"));
        assert!(msg.contains("  - DB_URL"));
        assert!(!msg.contains("DB_URL ("));
        assert!(msg.contains("~/.config/bnto/.env"));
    }

    #[test]
    fn secret_def_deserializes_with_defaults() {
        let json = r#"{ "key": "API_KEY" }"#;
        let s: SecretDef = serde_json::from_str(json).unwrap();
        assert_eq!(s.key, "API_KEY");
        assert!(s.required);
        assert!(s.description.is_empty());
    }

    #[test]
    fn secret_def_deserializes_full() {
        let json = r#"{
            "key": "OPENAI_API_KEY",
            "description": "OpenAI API key for GPT-4",
            "required": false
        }"#;
        let s: SecretDef = serde_json::from_str(json).unwrap();
        assert_eq!(s.key, "OPENAI_API_KEY");
        assert_eq!(s.description, "OpenAI API key for GPT-4");
        assert!(!s.required);
    }

    #[test]
    fn secret_def_round_trip() {
        let original = secret_with_desc("KEY", "desc", true);
        let json = serde_json::to_string(&original).unwrap();
        let parsed: SecretDef = serde_json::from_str(&json).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn empty_description_omitted_in_serialization() {
        let s = secret("KEY", true);
        let json = serde_json::to_string(&s).unwrap();
        assert!(!json.contains("description"));
    }
}
