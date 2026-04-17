// Bounded number stepping — parse, step, clamp, format.

use bnto_core::metadata::Constraints;

/// Determine the step size based on the constraint range.
///
/// - Range ≤ 1.0 → step 0.01 (fractional params like opacity)
/// - Range ≤ 100 → step 1 (quality, percentage)
/// - Range > 100 → step 10 (dimensions, large values)
pub fn step_size(constraints: Option<&Constraints>) -> f64 {
    let Some(c) = constraints else { return 1.0 };
    let range = match (c.min, c.max) {
        (Some(min), Some(max)) => max - min,
        _ => return 1.0,
    };
    if range <= 1.0 {
        0.01
    } else if range <= 100.0 {
        1.0
    } else {
        10.0
    }
}

/// Apply a step delta to the current value, clamping to constraints.
///
/// Returns `None` if the value can't be parsed or constraints are missing
/// (unbounded numbers should use text editing instead).
pub fn step(current: &str, delta: f64, constraints: Option<&Constraints>) -> Option<String> {
    let constraints = constraints?;
    let val: f64 = current.parse().ok()?;
    let next = val + delta;

    let clamped = match (constraints.min, constraints.max) {
        (Some(min), Some(max)) => next.clamp(min, max),
        (Some(min), None) => next.max(min),
        (None, Some(max)) => next.min(max),
        (None, None) => return None,
    };

    Some(format_number(clamped))
}

/// Format a number for display — integers show without decimal, floats keep 2 places.
fn format_number(val: f64) -> String {
    if (val - val.round()).abs() < f64::EPSILON {
        format!("{}", val as i64)
    } else {
        format!("{:.2}", val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bounded(min: f64, max: f64) -> Constraints {
        Constraints {
            min: Some(min),
            max: Some(max),
            required: false,
        }
    }

    // --- step_size ---

    #[test]
    fn step_size_small_range() {
        let c = bounded(0.0, 1.0);
        assert!((step_size(Some(&c)) - 0.01).abs() < f64::EPSILON);
    }

    #[test]
    fn step_size_medium_range() {
        let c = bounded(1.0, 100.0);
        assert!((step_size(Some(&c)) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn step_size_large_range() {
        let c = bounded(0.0, 4000.0);
        assert!((step_size(Some(&c)) - 10.0).abs() < f64::EPSILON);
    }

    #[test]
    fn step_size_no_constraints() {
        assert!((step_size(None) - 1.0).abs() < f64::EPSILON);
    }

    // --- step ---

    #[test]
    fn step_increments() {
        let c = bounded(1.0, 100.0);
        assert_eq!(step("80", 1.0, Some(&c)), Some("81".into()));
    }

    #[test]
    fn step_decrements() {
        let c = bounded(1.0, 100.0);
        assert_eq!(step("80", -1.0, Some(&c)), Some("79".into()));
    }

    #[test]
    fn step_clamps_at_max() {
        let c = bounded(1.0, 100.0);
        assert_eq!(step("100", 1.0, Some(&c)), Some("100".into()));
    }

    #[test]
    fn step_clamps_at_min() {
        let c = bounded(1.0, 100.0);
        assert_eq!(step("1", -1.0, Some(&c)), Some("1".into()));
    }

    #[test]
    fn step_fractional_range() {
        let c = bounded(0.0, 1.0);
        let result = step("0.50", 0.01, Some(&c)).unwrap();
        assert_eq!(result, "0.51");
    }

    #[test]
    fn step_returns_none_without_constraints() {
        assert_eq!(step("80", 1.0, None), None);
    }

    #[test]
    fn step_returns_none_for_unbounded() {
        let c = Constraints {
            min: None,
            max: None,
            required: false,
        };
        assert_eq!(step("80", 1.0, Some(&c)), None);
    }

    #[test]
    fn step_returns_none_for_unparseable() {
        let c = bounded(1.0, 100.0);
        assert_eq!(step("abc", 1.0, Some(&c)), None);
    }

    // --- format_number ---

    #[test]
    fn format_integer() {
        assert_eq!(format_number(80.0), "80");
    }

    #[test]
    fn format_decimal() {
        assert_eq!(format_number(0.51), "0.51");
    }
}
