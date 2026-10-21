// A node's `when` guard: a comparison that decides whether the step runs,
// like a step pipeline's `if:` / `when:`. When it doesn't hold, the step is
// skipped and its files pass through untouched (decisions/conditional-steps.md).

use serde::{Deserialize, Serialize};
#[cfg(feature = "ts")]
use ts_rs::TS;

use crate::compare::{Op, compare};

/// `when: { value, op, expected }` on a node. `value` and `expected` take
/// templates (`{{ctx.platform}}`, `{{fields.action}}`, `{{item.kind}}`, …);
/// `op` is one of `equals`, `not-equals`, `after`, `before`.
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(
    feature = "ts",
    ts(
        export,
        export_to = "../../../../packages/@bnto/nodes/src/generated/definitionTypes/"
    )
)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct When {
    pub value: String,
    pub op: String,
    pub expected: String,
}

/// What a `when` guard decided.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Guard {
    Run,
    /// Skip the step; `reason` says why, for progress and previews.
    Skip {
        reason: String,
    },
}

impl When {
    /// The comparison's op, or an error naming the ones that exist.
    pub fn op(&self) -> Result<Op, String> {
        Op::parse(&self.op)
            .ok_or_else(|| format!("when: 'op' \"{}\" is not one of {}", self.op, Op::NAMES))
    }

    /// Run or skip, given `value` and `expected` with their templates filled.
    pub fn decide(&self, value: &str, expected: &str) -> Result<Guard, String> {
        Ok(match compare(value, self.op()?, expected) {
            Ok(()) => Guard::Run,
            Err(why) => Guard::Skip {
                reason: format!("when {} {} {}: {why}", self.value, self.op, self.expected),
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn when(value: &str, op: &str, expected: &str) -> When {
        When {
            value: value.into(),
            op: op.into(),
            expected: expected.into(),
        }
    }

    #[test]
    fn a_holding_comparison_runs_the_step() {
        let guard = when("{{ctx.platform}}", "equals", "macos").decide("macos", "macos");
        assert_eq!(guard, Ok(Guard::Run));
    }

    #[test]
    fn a_failing_comparison_skips_it_and_says_why() {
        let guard = when("{{ctx.platform}}", "equals", "macos").decide("linux", "macos");
        let Ok(Guard::Skip { reason }) = guard else {
            panic!("{guard:?}")
        };
        assert!(reason.contains("{{ctx.platform}}"), "{reason}");
        assert!(reason.contains("linux"), "{reason}");
        assert!(reason.contains("macos"), "{reason}");
    }

    #[test]
    fn both_spellings_of_not_equals_work() {
        for op in ["not-equals", "not_equals"] {
            let guard = when("{{ctx.platform}}", op, "windows").decide("macos", "windows");
            assert_eq!(guard, Ok(Guard::Run), "{op}");
        }
    }

    #[test]
    fn an_unknown_op_is_an_error() {
        let guard = when("a", "is", "a").decide("a", "a");
        let Err(message) = guard else {
            panic!("{guard:?}")
        };
        assert!(message.contains("\"is\""), "{message}");
    }

    #[test]
    fn a_guard_without_a_value_is_refused() {
        let error = serde_json::from_str::<When>(r#"{"op":"equals","expected":"macos"}"#)
            .expect_err("value is required");
        assert!(error.to_string().contains("`value`"), "{error}");
    }

    #[test]
    fn reads_and_writes_the_recipe_shape() {
        let json = r#"{"value":"{{ctx.platform}}","op":"equals","expected":"macos"}"#;
        let parsed: When = serde_json::from_str(json).unwrap();
        assert_eq!(parsed, when("{{ctx.platform}}", "equals", "macos"));
        assert_eq!(serde_json::to_string(&parsed).unwrap(), json);
    }
}
