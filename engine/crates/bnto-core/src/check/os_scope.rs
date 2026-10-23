// `bnto check` on a node's OS scope: the values `{{ctx.platform}}` can read on
// the way down to a node, narrowed by every platform guard on its path. Finds
// a guard expecting a value it never reads, a step no OS reaches, and
// `requires` on an I/O node, which never runs. A processor reports what its
// own params break on some of those OSes (`NodeProcessor::platform_problems`).

use super::{CheckFinding, finding};
use crate::compare::compare;
use crate::os_support::platform_guard;
use crate::pipeline::PipelineNode;

/// Every value `{{ctx.platform}}` reads: the OSes bnto names, then `unknown`.
pub(super) const PLATFORMS: &[&str] = &["macos", "linux", "windows", "unknown"];

/// `platforms` narrowed by `node`'s own platform guard.
pub(super) fn narrow(platforms: &[&'static str], node: &PipelineNode) -> Vec<&'static str> {
    let Some((op, expected)) = node.when.as_ref().and_then(platform_guard) else {
        return platforms.to_vec();
    };
    let allowed = |p: &&str| compare(p, op, expected).is_ok();
    platforms.iter().copied().filter(allowed).collect()
}

/// A platform guard expecting a value `{{ctx.platform}}` never reads, such
/// as `darwin` or `macOS`: it never holds (`equals`) or always does.
pub(super) fn guard_finding(node: &PipelineNode) -> Option<CheckFinding> {
    let (_, expected) = node.when.as_ref().and_then(platform_guard)?;
    (!PLATFORMS.contains(&expected)).then(|| {
        let message = format!(
            "compares {{{{ctx.platform}}}} with \"{expected}\", which it never reads; \
             it reads macos, linux, windows or unknown"
        );
        finding(node, Some("when"), message)
    })
}

/// A step every OS is ruled out of by the platform guards on its path.
pub(super) fn unreachable_finding(node: &PipelineNode, platforms: &[&str]) -> Option<CheckFinding> {
    let message = "never runs: the platform guards on its path rule out every OS";
    platforms
        .is_empty()
        .then(|| finding(node, None, message.to_string()))
}

/// `requires` on an input or output node: it never runs, so nothing there
/// needs a tool.
pub(super) fn io_requires_finding(node: &PipelineNode) -> Option<CheckFinding> {
    let message = "\"requires\" has no effect on an input or output node; \
                   put it on the step or group that needs the tool";
    (!node.requires.is_empty()).then(|| finding(node, Some("requires"), message.to_string()))
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use crate::check::check_pipeline;
    use crate::context::ProcessContext;
    use crate::errors::BntoError;
    use crate::processor::{NodeInput, NodeOutput, NodeProcessor};
    use crate::progress::ProgressReporter;
    use crate::registry::NodeRegistry;

    /// Works only where `{{ctx.platform}}` reads `windows`.
    struct WindowsOnly;

    impl NodeProcessor for WindowsOnly {
        fn name(&self) -> &str {
            "windows-only"
        }

        fn platform_problems(
            &self,
            _params: &serde_json::Map<String, Value>,
            platforms: &[&str],
        ) -> Vec<String> {
            let elsewhere: Vec<&str> = platforms
                .iter()
                .copied()
                .filter(|p| *p != "windows")
                .collect();
            if elsewhere.is_empty() {
                return Vec::new();
            }
            vec![format!(
                "needs Windows, but can run on {}",
                elsewhere.join(", ")
            )]
        }

        fn process(
            &self,
            _input: NodeInput,
            _progress: &ProgressReporter,
            _ctx: &dyn ProcessContext,
        ) -> Result<NodeOutput, BntoError> {
            unreachable!("check never runs a node")
        }
    }

    fn registry() -> NodeRegistry {
        let mut registry = NodeRegistry::new();
        registry.register("windows-only", Box::new(WindowsOnly));
        registry
    }

    fn platform(op: &str, expected: &str) -> Value {
        json!({ "value": "{{ctx.platform}}", "op": op, "expected": expected })
    }

    fn step(id: &str) -> Value {
        json!({ "id": id, "type": "windows-only" })
    }

    fn group(id: &str, when: Value, nodes: Vec<Value>) -> Value {
        json!({ "id": id, "type": "group", "when": when, "nodes": nodes })
    }

    /// Each finding as `<node>: <message>`.
    fn findings(nodes: Vec<Value>) -> Vec<String> {
        let def = serde_json::from_value(json!({ "nodes": nodes })).unwrap();
        check_pipeline(&def, &registry())
            .findings
            .iter()
            .map(|f| format!("{}: {}", f.node_id, f.message))
            .collect()
    }

    #[test]
    fn a_guard_on_a_value_ctx_platform_never_reads_is_a_finding() {
        for expected in ["darwin", "macOS", "win32", "freebsd"] {
            let found = findings(vec![group(
                "g",
                platform("equals", expected),
                vec![step("s")],
            )]);
            assert!(
                found
                    .iter()
                    .any(|f| f.starts_with("g: compares {{ctx.platform}} with")
                        && f.contains(&format!("\"{expected}\""))),
                "{expected}: {found:?}"
            );
        }
    }

    #[test]
    fn every_value_ctx_platform_reads_is_fine_in_a_guard() {
        for expected in ["macos", "linux", "unknown"] {
            let nodes = vec![group(
                "g",
                platform("not-equals", expected),
                vec![step("s")],
            )];
            let found = findings(nodes);
            assert!(
                !found.iter().any(|f| f.contains("compares")),
                "{expected}: {found:?}"
            );
        }
    }

    #[test]
    fn a_step_contradictory_guards_rule_out_of_every_os_never_runs() {
        let inner = group("lin", platform("equals", "linux"), vec![step("s")]);
        let found = findings(vec![group("mac", platform("equals", "macos"), vec![inner])]);
        assert!(
            found.contains(
                &"s: never runs: the platform guards on its path rule out every OS".to_string()
            ),
            "{found:?}"
        );
    }

    #[test]
    fn a_branch_for_other_oses_still_runs_somewhere() {
        let found = findings(vec![group(
            "other",
            platform("equals", "unknown"),
            vec![step("s")],
        )]);
        assert!(!found.iter().any(|f| f.contains("never runs")), "{found:?}");
    }

    #[test]
    fn a_processor_hears_the_platforms_its_step_can_run_on() {
        let found = findings(vec![step("anywhere")]);
        assert!(
            found.contains(
                &"anywhere: needs Windows, but can run on macos, linux, unknown".to_string()
            ),
            "{found:?}"
        );
        let found = findings(vec![group(
            "win",
            platform("equals", "windows"),
            vec![step("guarded")],
        )]);
        assert!(!found.iter().any(|f| f.starts_with("guarded")), "{found:?}");
    }

    #[test]
    fn requires_on_an_io_node_is_a_finding() {
        let input = json!({ "id": "input", "type": "input", "requires": [{ "binary": "jq" }] });
        let found = findings(vec![
            input,
            json!({ "id": "s", "type": "windows-only",
            "when": platform("equals", "windows") }),
        ]);
        assert!(
            found
                .iter()
                .any(|f| f.starts_with("input: \"requires\" has no effect")),
            "{found:?}"
        );
    }
}
