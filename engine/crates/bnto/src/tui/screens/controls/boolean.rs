// Boolean toggle — flips "true"↔"false".

/// Flip a boolean value string. Returns "true" if current is "false" and vice versa.
pub fn toggle(current: &str) -> String {
    if current == "true" {
        "false".to_string()
    } else {
        "true".to_string()
    }
}

/// Display label for a boolean value: `[x]` for true, `[ ]` for false.
pub fn display_label(current: &str) -> &'static str {
    if current == "true" { "[x]" } else { "[ ]" }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggle_true_to_false() {
        assert_eq!(toggle("true"), "false");
    }

    #[test]
    fn toggle_false_to_true() {
        assert_eq!(toggle("false"), "true");
    }

    #[test]
    fn toggle_nonsense_becomes_true() {
        assert_eq!(toggle("banana"), "true");
    }

    #[test]
    fn display_label_true_shows_checked() {
        assert_eq!(display_label("true"), "[x]");
    }

    #[test]
    fn display_label_false_shows_unchecked() {
        assert_eq!(display_label("false"), "[ ]");
    }
}
