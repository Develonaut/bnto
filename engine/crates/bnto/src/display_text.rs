// Recipe text made safe to print (ISS-125). A recipe's name, description,
// commands and paths are written by its author, and a terminal obeys any
// control sequence in them: ESC can erase the line above, `\r` can overwrite
// what came before it, and a bidi override can show a command reversed.

use std::borrow::Cow;

/// `s` with every C0 and C1 control character shown visibly (ESC as `\e`,
/// `\r`, `\n`, `\t`, `\0`, others as `\x1f` or `\u{9b}`) and each character
/// that reorders or hides text (bidi overrides and isolates, zero-width
/// characters, every other format character, blank fillers) shown as
/// `\u{202e}`. Borrowed
/// when there's nothing to show. A `\` the author wrote stays as is, so
/// Windows paths read naturally; a literal `\e` is therefore ambiguous, but
/// harmless: it prints as the two characters it is.
pub fn display_text(s: &str) -> Cow<'_, str> {
    if !s.chars().any(needs_escape) {
        return Cow::Borrowed(s);
    }
    let mut shown = String::with_capacity(s.len() + 8);
    for c in s.chars() {
        push_shown(&mut shown, c);
    }
    Cow::Owned(shown)
}

/// `text` escaped line by line, keeping its line breaks: for text shown on
/// several lines on purpose (a script in a command line), where a break is
/// layout, not an attack. Any other control character is still shown.
pub fn display_lines(text: &str) -> Cow<'_, str> {
    if !text.chars().any(|c| c != '\n' && needs_escape(c)) {
        return Cow::Borrowed(text);
    }
    let lines: Vec<Cow<str>> = text.split('\n').map(display_text).collect();
    Cow::Owned(lines.join("\n"))
}

fn needs_escape(c: char) -> bool {
    c.is_control() || hides_or_reorders(c)
}

/// Characters that don't render as themselves: every format character
/// (Unicode category Cf: bidi embeddings, overrides, isolates and marks,
/// zero-width characters, the soft hyphen, tag characters), the line and
/// paragraph separators, and the blank-rendering fillers (Hangul fillers,
/// Khmer vowel inherents, the blank Braille pattern) and variation selectors
/// a name could hide behind.
fn hides_or_reorders(c: char) -> bool {
    HIDDEN
        .iter()
        .any(|&(first, last)| (first..=last).contains(&c))
}

/// The ranges [`hides_or_reorders`] matches, first and last included.
const HIDDEN: &[(char, char)] = &[
    ('\u{00ad}', '\u{00ad}'),
    ('\u{034f}', '\u{034f}'),
    ('\u{0600}', '\u{0605}'),
    ('\u{061c}', '\u{061c}'),
    ('\u{06dd}', '\u{06dd}'),
    ('\u{070f}', '\u{070f}'),
    ('\u{0890}', '\u{0891}'),
    ('\u{08e2}', '\u{08e2}'),
    ('\u{115f}', '\u{1160}'),
    ('\u{17b4}', '\u{17b5}'),
    ('\u{180e}', '\u{180e}'),
    ('\u{200b}', '\u{200f}'),
    ('\u{2028}', '\u{202e}'),
    ('\u{2060}', '\u{2064}'),
    ('\u{2066}', '\u{206f}'),
    ('\u{2800}', '\u{2800}'),
    ('\u{3164}', '\u{3164}'),
    ('\u{fe00}', '\u{fe0f}'),
    ('\u{feff}', '\u{feff}'),
    ('\u{ffa0}', '\u{ffa0}'),
    ('\u{fff9}', '\u{fffb}'),
    ('\u{110bd}', '\u{110bd}'),
    ('\u{110cd}', '\u{110cd}'),
    ('\u{13430}', '\u{1343f}'),
    ('\u{1bca0}', '\u{1bca3}'),
    ('\u{1d173}', '\u{1d17a}'),
    ('\u{e0000}', '\u{e007f}'),
    ('\u{e0100}', '\u{e01ef}'),
];

fn push_shown(out: &mut String, c: char) {
    match c {
        '\u{1b}' => out.push_str("\\e"),
        '\r' => out.push_str("\\r"),
        '\n' => out.push_str("\\n"),
        '\t' => out.push_str("\\t"),
        '\0' => out.push_str("\\0"),
        c if c.is_ascii_control() => out.push_str(&format!("\\x{:02x}", u32::from(c))),
        c if needs_escape(c) => out.push_str(&format!("\\u{{{:x}}}", u32::from(c))),
        c => out.push(c),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_is_borrowed_unchanged() {
        let shown = display_text("Compress Images — fast, ünïcode ok");
        assert!(matches!(shown, Cow::Borrowed(_)));
        assert_eq!(shown, "Compress Images — fast, ünïcode ok");
    }

    #[test]
    fn the_iss_125_name_shows_its_escape_and_carriage_return() {
        assert_eq!(display_text("Nice\u{1b}[2K\rEVIL"), "Nice\\e[2K\\rEVIL");
    }

    #[test]
    fn every_c0_control_is_visible() {
        assert_eq!(display_text("a\tb\nc\0d"), "a\\tb\\nc\\0d");
        assert_eq!(display_text("bell\u{7}"), "bell\\x07");
        assert_eq!(display_text("del\u{7f}"), "del\\x7f");
    }

    #[test]
    fn c1_controls_are_visible() {
        // U+009B is a one-character CSI on terminals that take 8-bit C1.
        assert_eq!(display_text("x\u{9b}2Ky"), "x\\u{9b}2Ky");
    }

    #[test]
    fn bidi_overrides_and_isolates_are_flagged() {
        assert_eq!(display_text("rm \u{202e}fdp.exe"), "rm \\u{202e}fdp.exe");
        assert_eq!(display_text("a\u{2066}b\u{2069}"), "a\\u{2066}b\\u{2069}");
        assert_eq!(display_text("a\u{200f}b"), "a\\u{200f}b");
    }

    #[test]
    fn zero_width_and_line_separators_are_flagged() {
        assert_eq!(display_text("a\u{200b}b"), "a\\u{200b}b");
        assert_eq!(display_text("a\u{2028}b"), "a\\u{2028}b");
        assert_eq!(display_text("\u{feff}x"), "\\u{feff}x");
    }

    #[test]
    fn every_invisible_or_format_character_is_flagged() {
        for c in [
            '\u{2800}',
            '\u{17b4}',
            '\u{17b5}',
            '\u{ad}',
            '\u{34f}',
            '\u{115f}',
            '\u{1160}',
            '\u{3164}',
            '\u{ffa0}',
            '\u{180e}',
            '\u{2061}',
            '\u{2064}',
            '\u{206a}',
            '\u{206f}',
            '\u{fe00}',
            '\u{fe0f}',
            '\u{e0000}',
            '\u{e0041}',
            '\u{e007f}',
            '\u{fff9}',
            '\u{fffb}',
            '\u{600}',
        ] {
            let shown = display_text(&format!("a{c}b")).into_owned();
            assert_eq!(shown, format!("a\\u{{{:x}}}b", u32::from(c)));
        }
    }

    #[test]
    fn lines_keep_their_breaks_but_nothing_else() {
        assert_eq!(display_lines("a\nb"), "a\nb");
        assert_eq!(display_lines("a\u{1b}[2K\nb\rc"), "a\\e[2K\nb\\rc");
    }

    #[test]
    fn shown_text_needs_no_second_escape() {
        let once = display_text("Nice\u{1b}[2K\rEVIL").into_owned();
        assert!(matches!(display_text(&once), Cow::Borrowed(_)));
    }

    #[test]
    fn backslashes_stay_as_written() {
        assert_eq!(display_text(r"C:\Tools\x.exe"), r"C:\Tools\x.exe");
    }
}
