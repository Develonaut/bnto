// The comparisons assert can make, and the reason each one fails.

use crate::time::parse_rfc3339_utc;

/// How `value` is compared to `expected`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Op {
    Equals,
    NotEquals,
    /// Both sides are RFC 3339 times; value must be strictly later.
    After,
    /// Both sides are RFC 3339 times; value must be strictly earlier.
    Before,
}

impl Op {
    pub(super) const NAMES: &'static str = "equals, not_equals, after, before";

    pub(super) fn name(self) -> &'static str {
        match self {
            Self::Equals => "equals",
            Self::NotEquals => "not_equals",
            Self::After => "after",
            Self::Before => "before",
        }
    }

    pub(super) fn parse(name: &str) -> Option<Self> {
        match name {
            "equals" => Some(Self::Equals),
            "not_equals" => Some(Self::NotEquals),
            "after" => Some(Self::After),
            "before" => Some(Self::Before),
            _ => None,
        }
    }
}

/// `Ok` when the comparison holds, else the reason it doesn't.
pub(super) fn compare(value: &str, op: Op, expected: &str) -> Result<(), String> {
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
