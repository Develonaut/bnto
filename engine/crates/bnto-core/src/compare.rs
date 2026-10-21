// The comparisons a recipe can make, shared by `assert` (fail the run when
// one doesn't hold) and a node's `when` guard (skip the step when it doesn't),
// and the reason each one fails.

use crate::time::parse_rfc3339_utc;

/// How `value` is compared to `expected`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Equals,
    NotEquals,
    /// Both sides are RFC 3339 times; value must be strictly later.
    After,
    /// Both sides are RFC 3339 times; value must be strictly earlier.
    Before,
}

impl Op {
    /// The names `parse` accepts, for error messages.
    pub const NAMES: &'static str = "equals, not-equals, after, before";

    /// The name in messages. `not_equals` until assert's option is renamed.
    pub fn name(self) -> &'static str {
        match self {
            Self::Equals => "equals",
            Self::NotEquals => "not_equals",
            Self::After => "after",
            Self::Before => "before",
        }
    }

    /// The op `name` spells. `not-equals` is the vocabulary spelling (kebab
    /// case, node-vocabulary.md); `not_equals` is assert's older option value.
    pub fn parse(name: &str) -> Option<Self> {
        match name {
            "equals" => Some(Self::Equals),
            "not-equals" | "not_equals" => Some(Self::NotEquals),
            "after" => Some(Self::After),
            "before" => Some(Self::Before),
            _ => None,
        }
    }
}

/// `Ok` when the comparison holds, else the reason it doesn't.
pub fn compare(value: &str, op: Op, expected: &str) -> Result<(), String> {
    match op {
        Op::Equals if value == expected => Ok(()),
        Op::Equals => Err(format!("got \"{value}\", expected \"{expected}\"")),
        Op::NotEquals if value != expected => Ok(()),
        Op::NotEquals => Err(format!(
            "got \"{value}\", expected anything but \"{expected}\""
        )),
        Op::After => compare_times(value, expected, |v, e| v > e, "after"),
        Op::Before => compare_times(value, expected, |v, e| v < e, "before"),
    }
}

fn compare_times(
    value: &str,
    expected: &str,
    holds: fn(u64, u64) -> bool,
    word: &str,
) -> Result<(), String> {
    let v = time(value)?;
    let e = time(expected)?;
    if holds(v, e) {
        Ok(())
    } else {
        Err(format!("{value} is not {word} {expected}"))
    }
}

fn time(text: &str) -> Result<u64, String> {
    parse_rfc3339_utc(text).ok_or_else(|| format!("\"{text}\" is not an RFC 3339 UTC time"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_equals_has_both_spellings() {
        assert_eq!(Op::parse("not-equals"), Some(Op::NotEquals));
        assert_eq!(Op::parse("not_equals"), Some(Op::NotEquals));
        assert_eq!(Op::parse("not equals"), None);
    }

    #[test]
    fn equals_and_not_equals_compare_text() {
        assert!(compare("macos", Op::Equals, "macos").is_ok());
        assert!(compare("linux", Op::Equals, "macos").is_err());
        assert!(compare("linux", Op::NotEquals, "macos").is_ok());
    }
}
