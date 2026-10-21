// `bnto check` on `when` guards: an op that doesn't exist, a malformed
// placeholder in the value or expected, and a guard on an I/O node (which
// never runs, so its guard would be silently ignored).

use super::{CheckFinding, finding, placeholder};
use crate::pipeline::PipelineNode;

/// Problems with a processing or container node's guard.
pub(super) fn when_findings(node: &PipelineNode) -> Vec<CheckFinding> {
    let Some(when) = &node.when else {
        return Vec::new();
    };
    let op = when.op().err();
    let placeholders = [&when.value, &when.expected]
        .into_iter()
        .flat_map(|text| placeholder::value_problems(&text.as_str().into()));
    op.into_iter()
        .chain(placeholders)
        .map(|message| finding(node, Some("when"), message))
        .collect()
}

/// `when` on an input or output node: the executor never runs it.
pub(super) fn io_when_finding(node: &PipelineNode) -> Option<CheckFinding> {
    let message = "\"when\" has no effect on an input or output node; \
                   guard the processing steps instead";
    node.when
        .as_ref()
        .map(|_| finding(node, None, message.to_string()))
}
