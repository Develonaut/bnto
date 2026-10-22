// Whether a dry run can already tell that a step is skipped on this host:
// a `when` guard on the step or one of its containers that doesn't hold.
// Guards reading `{{node.*}}` or `{{item.*}}` are decided mid-run, so the
// dry run never calls those skipped.

use std::collections::BTreeMap;

use bnto_core::PipelineNode;
use bnto_core::context::NoopContext;
use bnto_core::executor::field_scope;
use bnto_core::executor::template::TemplateContext;
use bnto_core::when::{Guard, When};

/// Why the last node of `path` (a step and the containers above it,
/// outermost first) is skipped on this host, if a guard says so now.
pub fn host_skip(path: &[&PipelineNode]) -> Option<String> {
    (1..=path.len()).find_map(|depth| skip_reason(&path[..depth]))
}

/// The guard of the last node of `path`, decided with the field values it
/// sees (its own and its containers', overrides already applied to their
/// defaults) and `{{ctx.*}}`; `None` when it holds or can't be decided
/// before the run.
fn skip_reason(path: &[&PipelineNode]) -> Option<String> {
    let when = path.last()?.when.as_ref()?;
    if decided_mid_run(when) {
        return None;
    }
    let field_values = field_scope::along(path);
    let tpl = TemplateContext {
        field_values: &field_values,
        process_ctx: &NoopContext,
        node_outputs: &BTreeMap::new(),
        loop_item: &None,
        run_started_ms: None,
    };
    match when.decide_in(&tpl) {
        Ok(Guard::Skip { reason }) => Some(reason),
        Ok(Guard::Run) | Err(_) => None,
    }
}

fn decided_mid_run(when: &When) -> bool {
    [&when.value, &when.expected]
        .iter()
        .any(|side| side.contains("{{node.") || side.contains("{{item."))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(json: serde_json::Value) -> PipelineNode {
        serde_json::from_value(json).unwrap()
    }

    fn host() -> &'static str {
        if cfg!(target_os = "macos") {
            "macos"
        } else if cfg!(target_os = "windows") {
            "windows"
        } else {
            "linux"
        }
    }

    fn platform_group(expected: &str) -> PipelineNode {
        node(serde_json::json!({ "id": "g", "type": "group",
            "when": { "value": "{{ctx.platform}}", "op": "equals", "expected": expected } }))
    }

    fn step() -> PipelineNode {
        node(
            serde_json::json!({ "id": "s", "type": "shell-run", "parameters": { "command": "x" } }),
        )
    }

    #[test]
    fn a_container_for_another_os_skips_its_step() {
        let (group, step) = (platform_group("plan9"), step());
        let reason = host_skip(&[&group, &step]).expect("skipped");
        assert!(reason.contains("{{ctx.platform}} equals plan9"), "{reason}");
    }

    #[test]
    fn the_hosts_own_branch_runs() {
        let (group, step) = (platform_group(host()), step());
        assert_eq!(host_skip(&[&group, &step]), None);
    }

    #[test]
    fn a_guard_on_the_steps_field_uses_its_value() {
        let step = node(serde_json::json!({ "id": "s", "type": "shell-run",
            "fields": { "action": { "type": "enum", "label": "Action",
                "options": [{ "value": "start", "label": "Start" }, { "value": "stop", "label": "Stop" }],
                "default": "stop" } },
            "when": { "value": "{{fields.action}}", "op": "equals", "expected": "start" } }));
        assert!(host_skip(&[&step]).is_some());
    }

    #[test]
    fn a_guard_on_the_step_reads_its_groups_field() {
        let group = node(serde_json::json!({ "id": "g", "type": "group",
            "fields": { "action": { "type": "string", "label": "Action", "default": "stop" } } }));
        let step = node(serde_json::json!({ "id": "s", "type": "shell-run",
            "when": { "value": "{{fields.action}}", "op": "equals", "expected": "start" } }));
        let reason = host_skip(&[&group, &step]).expect("action is stop");
        assert!(reason.contains("\"stop\""), "{reason}");
    }

    #[test]
    fn a_guard_decided_mid_run_is_never_called_skipped() {
        let step = node(serde_json::json!({ "id": "s", "type": "shell-run",
            "when": { "value": "{{node.list.count}}", "op": "equals", "expected": "0" } }));
        assert_eq!(host_skip(&[&step]), None);
    }
}
