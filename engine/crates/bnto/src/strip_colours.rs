// Colour codes out of a command's output before bnto prints it escaped
// (`display_text`): an SGR sequence (ESC [ … m) only sets colour or weight,
// so it's dropped and the text reads clean. Every other control sequence
// stays, to be shown escaped.

use std::borrow::Cow;

/// `line` without its SGR sequences: ESC `[`, digits and `;`, then `m`.
pub fn strip_colours(line: &str) -> Cow<'_, str> {
    if !line.contains('\u{1b}') {
        return Cow::Borrowed(line);
    }
    let mut out = String::with_capacity(line.len());
    let mut rest = line;
    while let Some(at) = rest.find('\u{1b}') {
        out.push_str(&rest[..at]);
        let after = &rest[at..];
        let len = sgr_len(after);
        if len == 0 {
            out.push('\u{1b}');
        }
        rest = &after[len.max(1)..];
    }
    out.push_str(rest);
    Cow::Owned(out)
}

/// The length of the SGR sequence `s` starts with, or 0 when it's another.
fn sgr_len(s: &str) -> usize {
    let Some(params) = s.strip_prefix("\u{1b}[") else {
        return 0;
    };
    let digits = params
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == ';')
        .count();
    match params[digits..].starts_with('m') {
        true => 2 + digits + 1,
        false => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colours_go() {
        assert_eq!(strip_colours("\u{1b}[32mok\u{1b}[0m"), "ok");
        assert_eq!(strip_colours("\u{1b}[1;38;5;208mx\u{1b}[m"), "x");
    }

    #[test]
    fn other_sequences_stay() {
        assert_eq!(strip_colours("a\u{1b}[2Kb"), "a\u{1b}[2Kb");
        assert_eq!(strip_colours("\u{1b}]0;t\u{7}"), "\u{1b}]0;t\u{7}");
        assert_eq!(strip_colours("end\u{1b}"), "end\u{1b}");
    }

    #[test]
    fn plain_text_is_borrowed() {
        assert!(matches!(strip_colours("plain"), Cow::Borrowed(_)));
    }
}
