// One item's value for one sort key, and the total order over those values.
//
// Present values order by type first (numbers, then text, then booleans, then
// lists and objects), then within the type: numbers numerically, text by
// Unicode code point, false before true, lists and objects by their JSON text.
// A descending key reverses that order. A missing value (absent, null or "")
// sorts after every present one in either direction, like a spreadsheet's
// blank cells, and ties with other missing values.

use std::cmp::Ordering;

use serde_json::Value;

use super::params::{Case, Order};

/// A field's value, ready to compare.
#[derive(Debug)]
pub(super) enum Key {
    Number(f64),
    Text(String),
    Bool(bool),
    /// A list or object, as its JSON text.
    Other(String),
    Missing,
}

impl Key {
    /// The key for `value`, text lowercased when `case` ignores it.
    pub(super) fn of(value: Option<&Value>, case: Case) -> Self {
        match value {
            None | Some(Value::Null) => Key::Missing,
            Some(Value::String(s)) if s.is_empty() => Key::Missing,
            Some(Value::String(s)) if case == Case::Insensitive => Key::Text(s.to_lowercase()),
            Some(Value::String(s)) => Key::Text(s.clone()),
            Some(Value::Number(n)) => Key::Number(n.as_f64().unwrap_or(0.0)),
            Some(Value::Bool(b)) => Key::Bool(*b),
            Some(other) => Key::Other(other.to_string()),
        }
    }

    /// Order `self` against `other` for a key sorting in `order`.
    pub(super) fn compare(&self, other: &Key, order: Order) -> Ordering {
        match (self, other) {
            (Key::Missing, Key::Missing) => Ordering::Equal,
            (Key::Missing, _) => Ordering::Greater,
            (_, Key::Missing) => Ordering::Less,
            _ if order == Order::Desc => other.ascending(self),
            _ => self.ascending(other),
        }
    }

    /// The ascending order of two present values.
    fn ascending(&self, other: &Key) -> Ordering {
        let by_type = self.rank().cmp(&other.rank());
        by_type.then_with(|| match (self, other) {
            (Key::Number(a), Key::Number(b)) => a.partial_cmp(b).unwrap_or(Ordering::Equal),
            (Key::Text(a), Key::Text(b)) | (Key::Other(a), Key::Other(b)) => a.cmp(b),
            (Key::Bool(a), Key::Bool(b)) => a.cmp(b),
            _ => Ordering::Equal,
        })
    }

    fn rank(&self) -> u8 {
        match self {
            Key::Number(_) => 0,
            Key::Text(_) => 1,
            Key::Bool(_) => 2,
            Key::Other(_) => 3,
            Key::Missing => 4,
        }
    }
}
