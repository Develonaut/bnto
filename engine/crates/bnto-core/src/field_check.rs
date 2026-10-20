//! Field values checked against their definitions (SEC-13). An enum's
//! `options` are an allowlist — bundled recipes put enum values straight
//! into option slots of the command they run — so a value outside them must
//! never reach a step. Numbers must parse and stay in range; booleans must
//! be `true` or `false`.

use crate::field_def::FieldDef;

impl FieldDef {
    /// Check a value the user typed (e.g. `--param key=value`) before it
    /// becomes the field's value.
    pub fn check_value(&self, raw: &str) -> Result<(), String> {
        match self {
            Self::String { .. } => Ok(()),
            Self::Number { min, max, .. } => {
                let n: f64 = raw
                    .trim()
                    .parse()
                    .map_err(|_| format!("'{raw}' isn't a number"))?;
                check_range(n, *min, *max)
            }
            Self::Boolean { .. } => match raw.trim() {
                "true" | "false" => Ok(()),
                _ => Err(format!("'{raw}' isn't true or false")),
            },
            Self::Enum { options, .. } => check_option(raw, options),
        }
    }

    /// Check the value the field holds now (its default, after overrides),
    /// so a run refuses an out-of-range or unlisted value from any source.
    pub fn check_current(&self) -> Result<(), String> {
        match self {
            Self::Number {
                default: Some(n),
                min,
                max,
                ..
            } => check_range(*n, *min, *max),
            Self::Enum {
                default: Some(v),
                options,
                ..
            } => check_option(v, options),
            _ => Ok(()),
        }
    }
}

fn check_range(n: f64, min: Option<f64>, max: Option<f64>) -> Result<(), String> {
    if let Some(min) = min.filter(|min| n < *min) {
        return Err(format!("{n} is below the minimum of {min}"));
    }
    if let Some(max) = max.filter(|max| n > *max) {
        return Err(format!("{n} is above the maximum of {max}"));
    }
    Ok(())
}

/// An empty `options` list declares no allowlist.
fn check_option(value: &str, options: &[crate::field_def::FieldOption]) -> Result<(), String> {
    if options.is_empty() || options.iter().any(|o| o.value == value) {
        return Ok(());
    }
    let allowed: Vec<String> = options.iter().map(|o| format!("'{}'", o.value)).collect();
    Err(format!(
        "'{value}' isn't one of the options: {}",
        allowed.join(", ")
    ))
}

#[cfg(test)]
mod tests {
    use crate::field_def::{FieldDef, FieldOption};

    fn mode(default: Option<&str>) -> FieldDef {
        FieldDef::Enum {
            label: "Mode".into(),
            options: ["--version", "--help", ""]
                .iter()
                .map(|v| FieldOption {
                    value: v.to_string(),
                    label: v.to_string(),
                })
                .collect(),
            description: None,
            default: default.map(String::from),
            order: None,
        }
    }

    fn quality(default: Option<f64>) -> FieldDef {
        FieldDef::Number {
            label: "Quality".into(),
            description: None,
            default,
            min: Some(1.0),
            max: Some(100.0),
            step: None,
            suffix: None,
            order: None,
        }
    }

    #[test]
    fn enum_accepts_listed_options_including_empty() {
        for v in ["--version", "--help", ""] {
            assert_eq!(mode(None).check_value(v), Ok(()));
        }
    }

    #[test]
    fn enum_refuses_unlisted_values_and_names_the_options() {
        let err = mode(None).check_value("--exec=touch x").unwrap_err();
        assert!(err.contains("'--version', '--help', ''"), "{err}");
    }

    #[test]
    fn enum_with_no_options_has_no_allowlist() {
        let field = FieldDef::Enum {
            label: "F".into(),
            options: Vec::new(),
            description: None,
            default: None,
            order: None,
        };
        assert_eq!(field.check_value("anything"), Ok(()));
    }

    #[test]
    fn number_must_parse_and_stay_in_range() {
        assert_eq!(quality(None).check_value("80"), Ok(()));
        assert!(quality(None).check_value("eighty").is_err());
        assert!(
            quality(None)
                .check_value("0")
                .unwrap_err()
                .contains("minimum")
        );
        assert!(
            quality(None)
                .check_value("101")
                .unwrap_err()
                .contains("maximum")
        );
    }

    #[test]
    fn boolean_must_be_true_or_false() {
        let flag = FieldDef::Boolean {
            label: "F".into(),
            description: None,
            default: None,
            order: None,
        };
        assert_eq!(flag.check_value("true"), Ok(()));
        assert!(flag.check_value("yes").is_err());
    }

    #[test]
    fn current_value_is_checked_too() {
        assert_eq!(mode(Some("--help")).check_current(), Ok(()));
        assert!(mode(Some("--exec")).check_current().is_err());
        assert!(quality(Some(500.0)).check_current().is_err());
        assert_eq!(quality(None).check_current(), Ok(()));
    }
}
