// Number helpers — idle stepping, clamping and display formatting.

use crate::field::{Field, FieldKind};
use crate::form::FormMessage;

/// Step the value by the field's increment (default 1), clamped to bounds.
pub(super) fn step_number(field: Field, msg: &FormMessage) -> Field {
    let FieldKind::Number { min, max, step, .. } = &field.kind else {
        return field;
    };
    let current: f64 = field.value.parse().unwrap_or(0.0);
    let increment = step.unwrap_or(1.0);
    let new_val = match msg {
        FormMessage::CycleNext => current + increment,
        FormMessage::CyclePrev => current - increment,
        _ => current,
    };
    let clamped = clamp_number(new_val, *min, *max);
    Field {
        value: format_number(clamped),
        ..field
    }
}

pub(super) fn clamp_number(val: f64, min: Option<f64>, max: Option<f64>) -> f64 {
    let mut v = val;
    if let Some(lo) = min {
        v = v.max(lo);
    }
    if let Some(hi) = max {
        v = v.min(hi);
    }
    v
}

/// Format a number, stripping trailing ".0" for clean display.
pub(super) fn format_number(val: f64) -> String {
    if val.fract() == 0.0 {
        format!("{}", val as i64)
    } else {
        format!("{val}")
    }
}
