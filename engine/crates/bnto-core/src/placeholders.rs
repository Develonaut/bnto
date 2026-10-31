//! Spotting placeholders that bring runtime data into a param. Values from
//! fields, files, earlier steps and the environment can be hostile (a file
//! name, a CSV cell), so params that must stay literal — a command name,
//! interpreter code — refuse them. `{{vars.*}}` is run configuration set by
//! the person running bnto, filled in before the engine starts, so it isn't
//! data here.

/// Namespaced placeholders the executor fills: `{{ns.key}}`.
const DATA_NAMESPACES: &[&str] = &["fields", "env", "ctx", "node", "item", "loop"];

/// Bare tokens shell-run fills: `{{url}}`, `{{path}}`, ...
const DATA_TOKENS: &[&str] = &["url", "input", "path", "file", "output_dir"];

/// The data placeholders in `s`, as written (`{{item.name}}`). Text that
/// only looks like one (`{{fields.*}}`, `{{ not a key }}`) isn't matched.
pub fn data_placeholders(s: &str) -> Vec<&str> {
    let mut found = Vec::new();
    let mut rest = s;
    while let Some(start) = rest.find("{{") {
        let after = &rest[start + 2..];
        let Some(end) = after.find("}}") else { break };
        let inner = &after[..end];
        if is_data_placeholder(inner) {
            found.push(&rest[start..start + end + 4]);
            rest = &after[end + 2..];
        } else {
            rest = after;
        }
    }
    found
}

/// True when `s` brings in any runtime data through a placeholder.
pub fn mentions_data(s: &str) -> bool {
    !data_placeholders(s).is_empty()
}

fn is_data_placeholder(inner: &str) -> bool {
    match inner.split_once('.') {
        Some((namespace, key)) => DATA_NAMESPACES.contains(&namespace) && is_key(key),
        None => DATA_TOKENS.contains(&inner),
    }
}

/// A placeholder key: letters, digits, `_`, `-` and `.` (node refs).
fn is_key(key: &str) -> bool {
    !key.is_empty()
        && key
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_executor_and_shell_data_placeholders() {
        let s = "a {{fields.v}} {{env.HOME}} {{node.dl.path}} {{item.row-1}} {{url}} {{path}}";
        assert_eq!(
            data_placeholders(s),
            [
                "{{fields.v}}",
                "{{env.HOME}}",
                "{{node.dl.path}}",
                "{{item.row-1}}",
                "{{url}}",
                "{{path}}"
            ]
        );
    }

    #[test]
    fn the_loops_item_is_data() {
        // A loop over CSV rows reads a cell through `{{loop.*}}` (ISS-109).
        assert_eq!(data_placeholders("x {{loop.cmd}}"), ["{{loop.cmd}}"]);
    }

    #[test]
    fn vars_are_run_configuration_not_data() {
        assert!(!mentions_data("{{vars.Tools}}/bin"));
    }

    #[test]
    fn text_that_only_looks_like_a_placeholder_is_ignored() {
        for s in [
            "{{fields.*}}",
            "{{ fields.v }}",
            "{{other.x}}",
            "{{fields.}}",
            "{{",
            "}} {{",
        ] {
            assert!(!mentions_data(s), "{s}");
        }
    }

    #[test]
    fn a_stray_open_brace_does_not_hide_a_placeholder() {
        assert_eq!(data_placeholders("{{ x {{item.k}}"), ["{{item.k}}"]);
    }
}
