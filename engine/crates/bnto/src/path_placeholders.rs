// Which `{{…}}` placeholders in a path can't move it: those whose value can
// never hold a path separator or a drive. Everything else (env, fields,
// vars, node outputs, items, loop values, `ctx.paths.*`) can name any folder,
// so the consent prompt flags a path holding one, wherever it sits.

/// `{{ctx.*}}` values made of digits, letters, `-`, `_` and `:` only: the
/// run's date, time, start time and OS (`executor/template.rs`).
const SEPARATOR_FREE: &[&str] = &[
    "ctx.date",
    "ctx.time",
    "ctx.timestamp",
    "ctx.started",
    "ctx.platform",
];

/// The per-file tokens output names fill (`file-rename`'s `naming.rs`,
/// `file-decode`'s `output_name.rs`): the file's own name and extension,
/// its index and counter, and the run's date. A file's own name stays
/// inside its output folder; the others are numbers or a date.
const FILE_TOKENS: &[&str] = &["name", "ext", "index", "counter", "date"];

/// Whether every placeholder in `value` is separator-free. `file_tokens`
/// also allows the per-file tokens of [`FILE_TOKENS`], as an output name
/// param fills them. An unclosed `{{` is never safe: a fill keeps it as
/// text.
pub fn only_safe_placeholders(value: &str, file_tokens: bool) -> bool {
    let mut rest = value;
    while let Some(start) = rest.find("{{") {
        let after = &rest[start + 2..];
        let Some(end) = after.find("}}") else {
            return false;
        };
        let inner = after[..end].trim();
        let safe = SEPARATOR_FREE.contains(&inner) || (file_tokens && FILE_TOKENS.contains(&inner));
        if !safe {
            return false;
        }
        rest = &after[end + 2..];
    }
    true
}

/// The part of `value` before its first placeholder, cut back to the last
/// separator: the folder that is fixed whatever fills it (`assets/` of
/// `assets/{{env.X}}`), or `value` itself when it holds none.
pub fn literal_folder(value: &str) -> &str {
    let Some(start) = value.find("{{") else {
        return value;
    };
    let head = &value[..start];
    head.rfind(['/', '\\']).map_or("", |sep| &head[..sep])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_time_and_platform_are_safe() {
        assert!(only_safe_placeholders(
            "x-{{ctx.date}}/{{ ctx.platform }}",
            false
        ));
        assert!(only_safe_placeholders("plain", false));
    }

    #[test]
    fn anything_else_is_not() {
        for value in ["{{env.X}}", "{{ctx.paths.home_dir}}", "{{name}}", "a{{b"] {
            assert!(!only_safe_placeholders(value, false), "{value}");
        }
    }

    #[test]
    fn an_unknown_dotless_token_is_not_a_files_own() {
        assert!(!only_safe_placeholders("{{path}}", true));
        assert!(!only_safe_placeholders("{{file}}", true));
        assert!(only_safe_placeholders(
            "{{counter}}-{{index}}-{{date}}",
            true
        ));
    }

    #[test]
    fn a_name_param_may_use_the_files_own_tokens() {
        assert!(only_safe_placeholders("{{name}}-small.{{ext}}", true));
        assert!(!only_safe_placeholders("{{item.path}}", true));
    }

    #[test]
    fn the_literal_folder_stops_at_the_first_placeholder() {
        assert_eq!(literal_folder("assets/{{env.X}}"), "assets");
        assert_eq!(literal_folder("out/x-{{ctx.date}}"), "out");
        assert_eq!(literal_folder("{{env.X}}/a"), "");
        assert_eq!(literal_folder("plain/dir"), "plain/dir");
    }
}
