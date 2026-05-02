// Loop container configuration — parsed from node params.
// Controls error handling and output persistence behavior for loop nodes.

use serde::Deserialize;

/// How the loop handles child node failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OnErrorStrategy {
    /// Stop the loop on the first child failure (default).
    #[default]
    FailFast,
    /// Record the failure, skip the iteration, continue to the next.
    Continue,
}

/// When loop iteration outputs are written to disk.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputPersistence {
    /// Accumulate all outputs in memory, write after pipeline completes (default).
    #[default]
    Deferred,
    /// Emit output files in events so consumers can write them immediately.
    Progressive,
}

/// Loop-specific configuration parsed from a loop node's `params`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LoopConfig {
    pub on_error: OnErrorStrategy,
    pub output_persistence: OutputPersistence,
}

/// Intermediate struct for serde deserialization of loop params.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawLoopParams {
    #[serde(default)]
    on_error: Option<String>,
    #[serde(default)]
    output_persistence: Option<String>,
}

/// Parse `LoopConfig` from a node's params map.
/// Unknown or missing values fall back to safe defaults.
pub fn parse_loop_config(params: &serde_json::Map<String, serde_json::Value>) -> LoopConfig {
    let raw: RawLoopParams = serde_json::from_value(serde_json::Value::Object(params.clone()))
        .unwrap_or(RawLoopParams {
            on_error: None,
            output_persistence: None,
        });

    let on_error = match raw.on_error.as_deref() {
        Some("continue") => OnErrorStrategy::Continue,
        _ => OnErrorStrategy::FailFast,
    };

    let output_persistence = match raw.output_persistence.as_deref() {
        Some("progressive") => OutputPersistence::Progressive,
        _ => OutputPersistence::Deferred,
    };

    LoopConfig {
        on_error,
        output_persistence,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_params_yields_defaults() {
        let params = serde_json::Map::new();
        let config = parse_loop_config(&params);
        assert_eq!(config.on_error, OnErrorStrategy::FailFast);
        assert_eq!(config.output_persistence, OutputPersistence::Deferred);
    }

    #[test]
    fn on_error_continue_parsed() {
        let mut params = serde_json::Map::new();
        params.insert("onError".into(), "continue".into());
        let config = parse_loop_config(&params);
        assert_eq!(config.on_error, OnErrorStrategy::Continue);
    }

    #[test]
    fn on_error_fail_fast_parsed() {
        let mut params = serde_json::Map::new();
        params.insert("onError".into(), "failFast".into());
        let config = parse_loop_config(&params);
        assert_eq!(config.on_error, OnErrorStrategy::FailFast);
    }

    #[test]
    fn output_persistence_progressive_parsed() {
        let mut params = serde_json::Map::new();
        params.insert("outputPersistence".into(), "progressive".into());
        let config = parse_loop_config(&params);
        assert_eq!(config.output_persistence, OutputPersistence::Progressive);
    }

    #[test]
    fn output_persistence_deferred_parsed() {
        let mut params = serde_json::Map::new();
        params.insert("outputPersistence".into(), "deferred".into());
        let config = parse_loop_config(&params);
        assert_eq!(config.output_persistence, OutputPersistence::Deferred);
    }

    #[test]
    fn unknown_on_error_defaults_to_fail_fast() {
        let mut params = serde_json::Map::new();
        params.insert("onError".into(), "garbage".into());
        let config = parse_loop_config(&params);
        assert_eq!(config.on_error, OnErrorStrategy::FailFast);
    }

    #[test]
    fn unknown_output_persistence_defaults_to_deferred() {
        let mut params = serde_json::Map::new();
        params.insert("outputPersistence".into(), "garbage".into());
        let config = parse_loop_config(&params);
        assert_eq!(config.output_persistence, OutputPersistence::Deferred);
    }

    #[test]
    fn extra_params_ignored() {
        let mut params = serde_json::Map::new();
        params.insert("onError".into(), "continue".into());
        params.insert("mode".into(), "forEach".into());
        params.insert("unrelated".into(), 42.into());
        let config = parse_loop_config(&params);
        assert_eq!(config.on_error, OnErrorStrategy::Continue);
        assert_eq!(config.output_persistence, OutputPersistence::Deferred);
    }

    #[test]
    fn both_fields_parsed_together() {
        let mut params = serde_json::Map::new();
        params.insert("onError".into(), "continue".into());
        params.insert("outputPersistence".into(), "progressive".into());
        let config = parse_loop_config(&params);
        assert_eq!(config.on_error, OnErrorStrategy::Continue);
        assert_eq!(config.output_persistence, OutputPersistence::Progressive);
    }
}
