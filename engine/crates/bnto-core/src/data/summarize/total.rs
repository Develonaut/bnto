// One group's running total of one field.
//
// Numbers add as they are, and so does text that reads as a finite number
// (every CSV value is text). A blank value (absent, null or "") is skipped,
// like a spreadsheet's SUM over empty cells. Anything else is an error, so a
// typo or a unit ("12 KB") fails the run instead of being quietly dropped.
// The total stays an integer while every value is one and it fits in i64.

use serde_json::{Number, Value};

/// A running total, integer until a value or an overflow makes it fractional.
#[derive(Debug, Clone, Copy)]
pub(super) enum Total {
    Int(i64),
    Float(f64),
}

impl Default for Total {
    fn default() -> Self {
        Total::Int(0)
    }
}

impl Total {
    /// Add `value` to the total, or say why it isn't a number.
    pub(super) fn add(&mut self, value: Option<&Value>) -> Result<(), String> {
        match number_of(value)? {
            None => {}
            Some(Total::Int(n)) => self.add_int(n),
            Some(Total::Float(f)) => *self = Total::Float(self.as_f64() + f),
        }
        Ok(())
    }

    fn add_int(&mut self, n: i64) {
        *self = match *self {
            Total::Int(t) => t
                .checked_add(n)
                .map_or(Total::Float(t as f64 + n as f64), Total::Int),
            Total::Float(t) => Total::Float(t + n as f64),
        };
    }

    fn as_f64(self) -> f64 {
        match self {
            Total::Int(n) => n as f64,
            Total::Float(f) => f,
        }
    }

    /// The total as JSON, or an error when it grew past what f64 holds.
    pub(super) fn into_value(self) -> Result<Value, String> {
        match self {
            Total::Int(n) => Ok(n.into()),
            Total::Float(f) => Number::from_f64(f)
                .map(Value::Number)
                .ok_or_else(|| "the total is too large to hold".to_string()),
        }
    }
}

/// `value` as a number to add, `None` when blank, or why it isn't a number.
fn number_of(value: Option<&Value>) -> Result<Option<Total>, String> {
    match value {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Number(n)) => Ok(Some(match n.as_i64() {
            Some(i) => Total::Int(i),
            None => Total::Float(n.as_f64().unwrap_or(0.0)),
        })),
        Some(Value::String(s)) => number_in_text(s),
        Some(other) => Err(format!("{other} is not a number")),
    }
}

fn number_in_text(text: &str) -> Result<Option<Total>, String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    if let Ok(i) = trimmed.parse::<i64>() {
        return Ok(Some(Total::Int(i)));
    }
    match trimmed.parse::<f64>() {
        Ok(f) if f.is_finite() => Ok(Some(Total::Float(f))),
        _ => Err(format!("\"{text}\" is not a number")),
    }
}
