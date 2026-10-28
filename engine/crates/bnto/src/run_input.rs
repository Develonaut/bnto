// What a suggested `bnto run` line ends with, by the recipe's input mode:
// `bnto info`'s hint and a dry run's `Run with:` footer share it (ISS-96).

use bnto_core::InputMode;

/// The placeholder for the run's input, or `None` for a recipe that takes
/// none: `bnto run` refuses any argument then.
pub fn run_input(mode: InputMode) -> Option<&'static str> {
    match mode {
        InputMode::None => None,
        InputMode::Url => Some("<url>"),
        InputMode::Text => Some("<text>"),
        InputMode::FileUpload => Some("<files...>"),
    }
}

/// `line`, followed by the placeholder for `mode`'s input if it takes one.
pub fn with_run_input(line: String, mode: InputMode) -> String {
    match run_input(mode) {
        Some(input) => format!("{line} {input}"),
        None => line,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_recipe_with_no_input_gets_nothing_after_it() {
        assert_eq!(
            with_run_input("bnto run a".into(), InputMode::None),
            "bnto run a"
        );
    }

    #[test]
    fn each_input_mode_gets_its_placeholder() {
        let line = |mode| with_run_input("bnto run a".into(), mode);
        assert_eq!(line(InputMode::Url), "bnto run a <url>");
        assert_eq!(line(InputMode::Text), "bnto run a <text>");
        assert_eq!(line(InputMode::FileUpload), "bnto run a <files...>");
    }
}
