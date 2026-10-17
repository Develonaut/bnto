// How file-render escapes the values it puts into a row.

/// The escaping applied to substituted values (never to the template text).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Escape {
    /// Values go in as they are.
    None,
    /// Values can't break a Markdown table cell or start inline markup.
    Markdown,
}

/// Characters that start or end inline Markdown, or end a table cell.
const MARKDOWN_SPECIAL: &[char] = &['\\', '`', '*', '_', '[', ']', '<', '>', '|'];

impl Escape {
    pub(super) const NAMES: &'static str = "none, markdown";

    pub(super) fn parse(name: &str) -> Option<Self> {
        match name {
            "none" => Some(Self::None),
            "markdown" => Some(Self::Markdown),
            _ => None,
        }
    }

    /// `value` escaped for the output.
    pub(super) fn apply(self, value: &str) -> String {
        match self {
            Self::None => value.to_string(),
            Self::Markdown => markdown(value),
        }
    }
}

/// A backslash before each special character; each line break (`\r\n`,
/// `\n` or `\r`) becomes `<br>`, which keeps a table row on one line.
fn markdown(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut chars = value.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\r' if chars.peek() == Some(&'\n') => {}
            '\r' | '\n' => out.push_str("<br>"),
            c if MARKDOWN_SPECIAL.contains(&c) => {
                out.push('\\');
                out.push(c);
            }
            c => out.push(c),
        }
    }
    out
}
