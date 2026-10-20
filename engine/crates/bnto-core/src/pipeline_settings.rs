// Recipe-level settings — the root definition's `settings`: how the executor
// iterates over input files, and the whole run's time limit.

use serde::{Deserialize, Serialize};
#[cfg(feature = "ts")]
use ts_rs::TS;

/// How the executor handles iteration over multiple input files.
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(
    feature = "ts",
    ts(
        export,
        export_to = "../../../../packages/@bnto/nodes/src/generated/definitionTypes/"
    )
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum IterationMode {
    /// Execute exactly what's defined — containers control iteration.
    /// This is the existing behavior and the default for backward compatibility.
    #[default]
    Explicit,
    /// Wrap contiguous per-file processor sequences in implicit per-file loops.
    /// Flat recipes produce identical output to explicit-loop recipes.
    Auto,
}

/// Recipe-level settings on the root Definition. Extensible — new fields
/// can be added without changing the schema shape.
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(
    feature = "ts",
    ts(
        export,
        export_to = "../../../../packages/@bnto/nodes/src/generated/definitionTypes/"
    )
)]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PipelineSettings {
    /// How the executor iterates over multiple input files.
    #[serde(default)]
    pub iteration: IterationMode,
    /// The whole run's time limit, in seconds. Once it has passed, no step,
    /// file or loop iteration starts and the run fails. Unset means no limit.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "positive_seconds"
    )]
    #[cfg_attr(feature = "ts", ts(optional, type = "number"))]
    pub timeout: Option<u64>,
}

/// A time limit is a whole number of seconds above 0: a 0s limit would fail
/// every run before its first step. (A `with` module, not `deserialize_with`,
/// because ts-rs reads `with` and warns on the other.)
mod positive_seconds {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(secs: &Option<u64>, serializer: S) -> Result<S::Ok, S::Error> {
        secs.serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<u64>, D::Error> {
        match u64::deserialize(deserializer) {
            Ok(secs) if secs > 0 => Ok(Some(secs)),
            _ => Err(serde::de::Error::custom(
                "settings.timeout must be a whole number of seconds above 0",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(json: &str) -> Result<PipelineSettings, serde_json::Error> {
        serde_json::from_str(json)
    }

    #[test]
    fn test_timeout_is_the_whole_runs_limit_in_seconds() {
        let settings = parse(r#"{ "timeout": 90 }"#).unwrap();
        assert_eq!(settings.timeout, Some(90));
        assert_eq!(settings.iteration, IterationMode::Explicit);
    }

    #[test]
    fn test_no_timeout_means_no_limit() {
        assert_eq!(parse("{}").unwrap().timeout, None);
    }

    #[test]
    fn test_a_timeout_that_is_not_seconds_above_zero_is_refused() {
        for bad in ["0", "-5", "1.5", r#""30s""#, "null"] {
            let err = parse(&format!(r#"{{ "timeout": {bad} }}"#)).unwrap_err();
            assert!(
                err.to_string()
                    .contains("settings.timeout must be a whole number of seconds above 0"),
                "{bad}: {err}"
            );
        }
    }

    #[test]
    fn test_no_timeout_is_left_out_when_written() {
        let json = serde_json::to_value(PipelineSettings::default()).unwrap();
        assert_eq!(json, serde_json::json!({ "iteration": "explicit" }));
    }
}
