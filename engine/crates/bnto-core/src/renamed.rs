// Node types and params a `bnto migrate` step renamed or removed. When a
// recipe still uses an old name, `bnto check` and a run's pre-flight name
// the new one (or why it went) and point at `bnto migrate`, instead of a
// bare "unknown". Add a row here in the same PR as the migration.

/// `(old type, new type)` for every renamed node type. `bnto migrate`'s
/// tests check it applies each one.
pub const NODE_TYPES: &[(&str, &str)] = &[
    ("file-metadata", "file-inspect"),
    ("shell-command", "shell-run"),
];

/// `(node type, old param, new param)` for every renamed param, under the
/// node's current type. `bnto migrate`'s tests check it applies each one.
pub const PARAMS: &[(&str, &str, &str)] = &[
    ("image-compress", "compression", "quality"),
    ("file-copy", "create_dirs", "createDirs"),
    ("file-copy", "tolerance_ms", "toleranceMs"),
    ("file-filter", "name_pattern", "namePattern"),
    ("file-filter", "pattern_mode", "patternMode"),
    ("file-filter", "min_size", "minSize"),
    ("file-filter", "max_size", "maxSize"),
    ("file-inspect", "include_hash", "includeHash"),
    ("file-move", "create_dirs", "createDirs"),
    ("file-rename", "max_length", "maxLength"),
    ("file-rename", "counter_start", "counterStart"),
    ("file-rename", "counter_pad", "counterPad"),
    ("shell-run", "search_path", "searchPath"),
    ("spreadsheet-export", "name_column", "nameColumn"),
    ("spreadsheet-lookup", "key_column", "keyColumn"),
    ("spreadsheet-lookup", "match_on", "matchOn"),
];

/// `(node type, param, why)` for every param a node no longer has, under
/// the node's current type. `bnto migrate`'s tests check it drops each one.
pub const REMOVED_PARAMS: &[(&str, &str, &str)] =
    &[("vector-optimize", "precision", "it never did anything")];

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
/// `node_type`, when a migration renamed or removed it.
pub(crate) fn param_hint(node_type: &str, param: &str) -> Option<String> {
    let renamed = PARAMS
        .iter()
        .find(|(t, old, _)| *t == node_type && *old == param)
        .map(|(_, _, new)| hint(new));
    renamed.or_else(|| removed_hint(node_type, param))
}

fn removed_hint(node_type: &str, param: &str) -> Option<String> {
    REMOVED_PARAMS
        .iter()
        .find(|(t, removed, _)| *t == node_type && *removed == param)
        .map(|(_, _, why)| format!("removed because {why}; {MIGRATE}"))
}

fn hint(new: &str) -> String {
    format!("renamed to \"{new}\"; {MIGRATE}")
}

const MIGRATE: &str = "run `bnto migrate` on the recipe";

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

    #[test]
    fn a_removed_param_says_why_and_points_at_bnto_migrate() {
        assert_eq!(
            param_hint("vector-optimize", "precision").as_deref(),
            Some("removed because it never did anything; run `bnto migrate` on the recipe")
        );
    }

    #[test]
    fn a_removed_params_name_on_another_node_has_no_hint() {
        assert_eq!(param_hint("image-resize", "precision"), None);
    }
}
