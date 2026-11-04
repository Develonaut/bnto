// A recipe's `{{fields.*}}` placeholders filled with the values a run would
// use (a field's default, or a `-p` value the run was given), so the
// consent prompt shows what runs, not only the template: an author's
// default can be anything.

use bnto_core::executor::field_scope::FieldValues;
use serde_json::Value;

use crate::display_text::display_text;

/// `text` with each `{{fields.NAME}}` it holds filled from `values`, the
/// value escaped (`display_text`), or `None` when it holds none that fill.
pub fn fill_fields(text: &str, values: &FieldValues) -> Option<String> {
    let mut filled = String::with_capacity(text.len());
    let mut rest = text;
    let mut any = false;
    while let Some(start) = rest.find("{{") {
        let Some(len) = rest[start..].find("}}") else {
            break;
        };
        let token = &rest[start..start + len + 2];
        let value = field_name(token).and_then(|name| values.get(name));
        filled.push_str(&rest[..start]);
        match value {
            Some(value) => filled.push_str(&display_text(&shown(value))),
            None => filled.push_str(token),
        }
        any |= value.is_some();
        rest = &rest[start + len + 2..];
    }
    filled.push_str(rest);
    any.then_some(filled)
}

/// `NAME` of a `{{fields.NAME}}` token, spaces inside the braces allowed.
fn field_name(token: &str) -> Option<&str> {
    token[2..token.len() - 2].trim().strip_prefix("fields.")
}

/// A value as a run puts it into text: a string as is, anything else as JSON.
fn shown(value: &Value) -> String {
    value
        .as_str()
        .map_or_else(|| value.to_string(), str::to_string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn values() -> FieldValues {
        [
            ("fmt".to_string(), json!("--exec=touch X\u{1b}[2K")),
            ("n".to_string(), json!(3)),
        ]
        .into_iter()
        .collect()
    }

    #[test]
    fn a_field_fills_with_its_value_escaped() {
        assert_eq!(
            fill_fields("echo {{fields.fmt}} -n {{ fields.n }}", &values()).as_deref(),
            Some("echo --exec=touch X\\e[2K -n 3")
        );
    }

    #[test]
    fn text_without_a_known_field_has_nothing_to_fill() {
        assert_eq!(fill_fields("echo hi", &values()), None);
        assert_eq!(fill_fields("{{fields.other}} {{env.X}}", &values()), None);
    }

    #[test]
    fn other_placeholders_stay_as_written() {
        assert_eq!(
            fill_fields("{{env.X}}/{{fields.n}}", &values()).as_deref(),
            Some("{{env.X}}/3")
        );
    }
}
