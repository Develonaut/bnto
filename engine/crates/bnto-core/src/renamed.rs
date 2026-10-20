// Node types and params a `bnto migrate` step renamed. When a recipe still
// uses an old name, `bnto check` and a run's pre-flight name the new one
// and point at `bnto migrate`, instead of a bare "unknown". Add a row here
// in the same PR as the migration that renames it.

/// `(old type, new type)` for every renamed node type.
const NODE_TYPES: &[(&str, &str)] = &[
    ("file-metadata", "file-inspect"),
    ("shell-command", "shell-run"),
];

/// `(node type, old param, new param)` for every renamed param.
const PARAMS: &[(&str, &str, &str)] = &[("image-compress", "compression", "quality")];

/// `message` about an unknown `node_type`, with the rename hint appended
/// when a migration renamed that type.
pub fn with_node_type_hint(message: String, node_type: &str) -> String {
    match node_type_hint(node_type) {
        Some(hint) => format!("{message} ({hint})"),
        None => message,
    }
}

fn node_type_hint(node_type: &str) -> Option<String> {
    NODE_TYPES
        .iter()
        .find(|(old, _)| *old == node_type)
        .map(|(_, new)| hint(new))
}

/// What to append to an "unknown param" message for `param` on
/// `node_type`, when a migration renamed it.
pub(crate) fn param_hint(node_type: &str, param: &str) -> Option<String> {
    PARAMS
        .iter()
        .find(|(t, old, _)| *t == node_type && *old == param)
        .map(|(_, _, new)| hint(new))
}

fn hint(new: &str) -> String {
    format!("renamed to \"{new}\"; run `bnto migrate` on the recipe")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_renamed_type_points_at_its_new_name() {
        assert_eq!(
            node_type_hint("shell-command").as_deref(),
            Some("renamed to \"shell-run\"; run `bnto migrate` on the recipe")
        );
        assert_eq!(
            node_type_hint("file-metadata").as_deref(),
            Some("renamed to \"file-inspect\"; run `bnto migrate` on the recipe")
        );
    }

    #[test]
    fn a_type_no_migration_renamed_has_no_hint() {
        assert_eq!(node_type_hint("spreadsheet-pivot"), None);
        assert_eq!(node_type_hint("shell-run"), None);
    }

    #[test]
    fn the_type_hint_is_appended_only_for_renamed_types() {
        assert_eq!(
            with_node_type_hint(
                "unknown node type \"shell-command\"".into(),
                "shell-command"
            ),
            "unknown node type \"shell-command\" \
             (renamed to \"shell-run\"; run `bnto migrate` on the recipe)"
        );
        assert_eq!(
            with_node_type_hint("unknown node type \"nope\"".into(), "nope"),
            "unknown node type \"nope\""
        );
    }

    #[test]
    fn a_renamed_param_points_at_its_new_name_on_its_own_node() {
        assert_eq!(
            param_hint("image-compress", "compression").as_deref(),
            Some("renamed to \"quality\"; run `bnto migrate` on the recipe")
        );
    }

    #[test]
    fn the_same_param_on_another_node_has_no_hint() {
        assert_eq!(param_hint("image-resize", "compression"), None);
        assert_eq!(param_hint("image-compress", "quality"), None);
    }
}
