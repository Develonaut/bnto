// The options shell-command runs its command with: the recipe's `timeout`
// (seconds, default 300) and its `env`, with dangerous variables stripped.

use std::time::Duration;

use bnto_core::CommandOptions;
use serde_json::{Map, Value};

use crate::validate::{self, DEFAULT_TIMEOUT_SECS};

/// The `CommandOptions` a shell-command node's `params` ask for.
pub(crate) fn command_options(params: &Map<String, Value>) -> CommandOptions {
    let secs = params
        .get("timeout")
        .and_then(Value::as_u64)
        .unwrap_or(DEFAULT_TIMEOUT_SECS);
    CommandOptions {
        env: env_vars(params),
        timeout: Some(Duration::from_secs(secs)),
    }
}

/// `env` as name/value pairs, sorted by name, dangerous names removed. A
/// value that isn't text (a number, a bool) is passed as its JSON text.
fn env_vars(params: &Map<String, Value>) -> Vec<(String, String)> {
    let Some(env) = params.get("env").and_then(Value::as_object) else {
        return Vec::new();
    };
    validate::sanitize_env(env)
        .into_iter()
        .map(|(name, value)| match value {
            Value::String(text) => (name, text),
            other => (name, other.to_string()),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn params(value: Value) -> Map<String, Value> {
        value.as_object().unwrap().clone()
    }

    #[test]
    fn test_timeout_is_the_param_in_seconds() {
        let options = command_options(&params(serde_json::json!({ "timeout": 7 })));
        assert_eq!(options.timeout, Some(Duration::from_secs(7)));
    }

    #[test]
    fn test_timeout_defaults_when_unset() {
        let options = command_options(&params(serde_json::json!({})));
        assert_eq!(
            options.timeout,
            Some(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
        );
    }

    #[test]
    fn test_env_vars_are_passed_sorted_by_name() {
        let options = command_options(&params(serde_json::json!({
            "env": { "B_VAR": "two", "A_VAR": "one" }
        })));
        assert_eq!(
            options.env,
            vec![
                ("A_VAR".to_string(), "one".to_string()),
                ("B_VAR".to_string(), "two".to_string())
            ]
        );
    }

    #[test]
    fn test_a_number_or_bool_env_value_is_passed_as_text() {
        let options = command_options(&params(serde_json::json!({
            "env": { "COUNT": 3, "DEBUG": true }
        })));
        assert_eq!(
            options.env,
            vec![
                ("COUNT".to_string(), "3".to_string()),
                ("DEBUG".to_string(), "true".to_string())
            ]
        );
    }

    #[test]
    fn test_dangerous_env_vars_are_stripped() {
        let options = command_options(&params(serde_json::json!({
            "env": { "LD_PRELOAD": "/evil.so", "SAFE": "yes" }
        })));
        assert_eq!(options.env, vec![("SAFE".to_string(), "yes".to_string())]);
    }
}
