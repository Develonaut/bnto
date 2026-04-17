// Enum cycling — navigate through a fixed set of {value, label} options.

use bnto_core::metadata::OptionEntry;

/// Cycle to the next option's value, wrapping at the end.
pub fn cycle_next(current: &str, options: &[OptionEntry]) -> String {
    let idx = options.iter().position(|o| o.value == current);
    match idx {
        Some(i) => options[(i + 1) % options.len()].value.clone(),
        None if !options.is_empty() => options[0].value.clone(),
        None => current.to_string(),
    }
}

/// Cycle to the previous option's value, wrapping at the start.
pub fn cycle_prev(current: &str, options: &[OptionEntry]) -> String {
    let idx = options.iter().position(|o| o.value == current);
    match idx {
        Some(0) => options[options.len() - 1].value.clone(),
        Some(i) => options[i - 1].value.clone(),
        None if !options.is_empty() => options[0].value.clone(),
        None => current.to_string(),
    }
}

/// Map a stored value to its display label. Falls back to the raw value.
pub fn display_label<'a>(current: &'a str, options: &'a [OptionEntry]) -> &'a str {
    options
        .iter()
        .find(|o| o.value == current)
        .map(|o| o.label.as_str())
        .unwrap_or(current)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_options() -> Vec<OptionEntry> {
        vec![
            OptionEntry {
                value: "jpeg".into(),
                label: "JPEG".into(),
            },
            OptionEntry {
                value: "png".into(),
                label: "PNG".into(),
            },
            OptionEntry {
                value: "webp".into(),
                label: "WebP".into(),
            },
        ]
    }

    #[test]
    fn cycle_next_advances() {
        assert_eq!(cycle_next("jpeg", &sample_options()), "png");
    }

    #[test]
    fn cycle_next_wraps_at_end() {
        assert_eq!(cycle_next("webp", &sample_options()), "jpeg");
    }

    #[test]
    fn cycle_prev_goes_back() {
        assert_eq!(cycle_prev("png", &sample_options()), "jpeg");
    }

    #[test]
    fn cycle_prev_wraps_at_start() {
        assert_eq!(cycle_prev("jpeg", &sample_options()), "webp");
    }

    #[test]
    fn cycle_next_unknown_value_resets_to_first() {
        assert_eq!(cycle_next("bmp", &sample_options()), "jpeg");
    }

    #[test]
    fn cycle_prev_unknown_value_resets_to_first() {
        assert_eq!(cycle_prev("bmp", &sample_options()), "jpeg");
    }

    #[test]
    fn cycle_next_empty_options_returns_current() {
        assert_eq!(cycle_next("jpeg", &[]), "jpeg");
    }

    #[test]
    fn display_label_maps_value_to_label() {
        assert_eq!(display_label("jpeg", &sample_options()), "JPEG");
    }

    #[test]
    fn display_label_unknown_falls_back_to_value() {
        assert_eq!(display_label("bmp", &sample_options()), "bmp");
    }
}
