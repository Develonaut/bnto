// The operating systems a recipe runs on, worked out from its platform
// guards (`when {{ctx.platform}} equals macos`), never hand-written. See
// decisions/conditional-steps.md, "OS support".

use crate::compare::{Op, compare};
use crate::metadata::HostPlatform;
use crate::pipeline::{PipelineDefinition, PipelineNode, is_container_node, is_io_node};
use crate::when::When;

/// The value a platform guard compares, exactly as written.
const PLATFORM: &str = "{{ctx.platform}}";

/// Whether `node` runs on `os`: its guard allows `os`, and it is a primitive
/// or holds a step that runs on `os`.
pub fn runs_on(node: &PipelineNode, os: HostPlatform) -> bool {
    guard_allows(node.when.as_ref(), os)
        && (!is_container_node(&node.node_type) || any_runs_on(children(node), os))
}

/// The OSes some root step runs on, in `HostPlatform::ALL` order. A recipe
/// with no steps at all runs everywhere.
pub fn recipe_os(def: &PipelineDefinition) -> Vec<HostPlatform> {
    if !has_step(&def.nodes) {
        return HostPlatform::ALL.to_vec();
    }
    HostPlatform::ALL
        .into_iter()
        .filter(|&os| any_runs_on(&def.nodes, os))
        .collect()
}

/// `os` as people read it: "macOS", "macOS and Linux",
/// "macOS, Linux and Windows".
pub fn describe_os(os: &[HostPlatform]) -> String {
    let labels: Vec<&str> = os.iter().map(|p| p.label()).collect();
    match labels.split_last() {
        None => "no operating system bnto supports".to_string(),
        Some((last, [])) => (*last).to_string(),
        Some((last, rest)) => format!("{} and {last}", rest.join(", ")),
    }
}

/// Whether some step among `nodes` (I/O markers aside) runs on `os`.
fn any_runs_on(nodes: &[PipelineNode], os: HostPlatform) -> bool {
    nodes
        .iter()
        .any(|node| !is_io_node(&node.node_type) && runs_on(node, os))
}

/// Whether `nodes` hold a primitive step at any depth.
fn has_step(nodes: &[PipelineNode]) -> bool {
    nodes.iter().any(|node| {
        !is_io_node(&node.node_type)
            && (!is_container_node(&node.node_type) || has_step(children(node)))
    })
}

fn children(node: &PipelineNode) -> &[PipelineNode] {
    node.children.as_deref().unwrap_or_default()
}

/// Whether a guard lets its step run on `os`. Only a platform guard can say
/// no; any other guard (a field, a loop item, an earlier step's record)
/// may hold on every OS.
fn guard_allows(when: Option<&When>, os: HostPlatform) -> bool {
    match when.map(|when| (when, platform_op(when))) {
        Some((when, Some(op))) => compare(os.as_str(), op, &when.expected).is_ok(),
        _ => true,
    }
}

/// The op of a platform guard: `value` exactly `{{ctx.platform}}`, `op`
/// equals or not-equals, and a literal `expected`. `None` for any other.
fn platform_op(when: &When) -> Option<Op> {
    if when.value != PLATFORM || when.expected.contains("{{") {
        return None;
    }
    when.op()
        .ok()
        .filter(|op| matches!(op, Op::Equals | Op::NotEquals))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};

    use HostPlatform::{Linux, MacOs, Windows};

    fn on(op: &str, os: &str) -> Value {
        json!({ "value": "{{ctx.platform}}", "op": op, "expected": os })
    }

    fn step(id: &str) -> Value {
        json!({ "id": id, "type": "file-rename" })
    }

    fn guarded(mut node: Value, when: Value) -> Value {
        node["when"] = when;
        node
    }

    fn group(id: &str, nodes: Vec<Value>) -> Value {
        json!({ "id": id, "type": "group", "nodes": nodes })
    }

    fn os_of(steps: Vec<Value>) -> Vec<HostPlatform> {
        let mut nodes = vec![json!({ "id": "input", "type": "input" })];
        nodes.extend(steps);
        nodes.push(json!({ "id": "output", "type": "output" }));
        recipe_os(&serde_json::from_value(json!({ "nodes": nodes })).unwrap())
    }

    // --- recipe_os ---

    #[test]
    fn a_recipe_without_guards_runs_everywhere() {
        assert_eq!(os_of(vec![step("a")]), HostPlatform::ALL);
    }

    #[test]
    fn a_recipe_with_no_steps_runs_everywhere() {
        assert_eq!(os_of(vec![]), HostPlatform::ALL);
    }

    #[test]
    fn a_macos_only_group_runs_on_macos() {
        let mac = guarded(group("mac", vec![step("a")]), on("equals", "macos"));
        assert_eq!(os_of(vec![mac]), [MacOs]);
    }

    #[test]
    fn an_unguarded_step_beside_a_macos_group_runs_everywhere() {
        let mac = guarded(group("mac", vec![step("a")]), on("equals", "macos"));
        assert_eq!(os_of(vec![step("b"), mac]), HostPlatform::ALL);
    }

    #[test]
    fn a_guard_on_a_field_may_hold_anywhere() {
        let when = json!({ "value": "{{fields.action}}", "op": "equals", "expected": "start" });
        assert_eq!(os_of(vec![guarded(step("a"), when)]), HostPlatform::ALL);
    }

    #[test]
    fn not_equals_windows_runs_on_the_other_two() {
        for op in ["not-equals", "not_equals"] {
            let step = guarded(step("a"), on(op, "windows"));
            assert_eq!(os_of(vec![step]), [MacOs, Linux], "{op}");
        }
    }

    #[test]
    fn per_os_groups_add_up() {
        let groups =
            ["linux", "windows"].map(|os| guarded(group(os, vec![step(os)]), on("equals", os)));
        assert_eq!(os_of(groups.to_vec()), [Linux, Windows]);
    }

    #[test]
    fn nested_guards_must_all_allow_the_os() {
        let inner = guarded(group("linux", vec![step("a")]), on("equals", "linux"));
        let outer = guarded(group("not-win", vec![inner]), on("not-equals", "windows"));
        assert_eq!(os_of(vec![outer]), [Linux]);
        let inner = guarded(group("win", vec![step("a")]), on("equals", "windows"));
        let outer = guarded(group("mac", vec![inner]), on("equals", "macos"));
        assert_eq!(os_of(vec![outer]), []);
    }

    #[test]
    fn a_group_runs_where_its_steps_do() {
        let mac = guarded(step("a"), on("equals", "macos"));
        assert_eq!(os_of(vec![group("g", vec![mac])]), [MacOs]);
    }

    #[test]
    fn an_empty_group_never_runs() {
        let empty = group("empty", vec![]);
        let mac = guarded(step("a"), on("equals", "macos"));
        assert_eq!(os_of(vec![empty, mac]), [MacOs]);
    }

    #[test]
    fn only_a_literal_platform_comparison_is_a_platform_guard() {
        let not_platform = [
            // A template on the other side, an ordering op, a padded value,
            // an unknown op: none of these rule an OS out.
            json!({ "value": "{{ctx.platform}}", "op": "equals", "expected": "{{fields.os}}" }),
            json!({ "value": "{{ctx.platform}}", "op": "before", "expected": "macos" }),
            json!({ "value": " {{ctx.platform}}", "op": "equals", "expected": "macos" }),
            json!({ "value": "macos", "op": "equals", "expected": "{{ctx.platform}}" }),
            json!({ "value": "{{ctx.platform}}", "op": "is", "expected": "macos" }),
        ];
        for when in not_platform {
            assert_eq!(
                os_of(vec![guarded(step("a"), when.clone())]),
                HostPlatform::ALL,
                "{when}"
            );
        }
    }

    #[test]
    fn a_platform_bnto_does_not_know_runs_nowhere() {
        let step = guarded(step("a"), on("equals", "freebsd"));
        assert_eq!(os_of(vec![step]), []);
    }

    #[test]
    fn keep_awake_runs_on_macos_and_linux() {
        let json = include_str!("../../bnto-engine/recipes/keep-awake.bnto.json");
        assert_eq!(
            recipe_os(&serde_json::from_str(json).unwrap()),
            [MacOs, Linux]
        );
    }

    // --- runs_on ---

    #[test]
    fn runs_on_reads_the_nodes_own_guard() {
        let node: PipelineNode =
            serde_json::from_value(guarded(step("a"), on("equals", "linux"))).unwrap();
        assert!(runs_on(&node, Linux));
        assert!(!runs_on(&node, MacOs));
    }

    // --- describe_os ---

    #[test]
    fn describes_one_two_and_three_oses() {
        assert_eq!(describe_os(&[MacOs]), "macOS");
        assert_eq!(describe_os(&[MacOs, Linux]), "macOS and Linux");
        assert_eq!(describe_os(&HostPlatform::ALL), "macOS, Linux and Windows");
    }
}
