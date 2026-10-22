// A step whose `when` guard didn't hold shows as skipped, at the top level
// and inside a container, where it never started.

use super::*;

fn started(nodes: &[(&str, &str)]) -> ExecutionModel {
    let node_info = nodes
        .iter()
        .map(|(id, t)| (id.to_string(), t.to_string()))
        .collect();
    let message = ExecutionMessage::PipelineStarted {
        total_nodes: nodes.len(),
        total_files: 1,
        node_info,
    };
    update(ExecutionModel::new("s"), message)
}

fn skipped(node_id: &str, parent: Option<&str>) -> ExecutionMessage {
    ExecutionMessage::NodeSkipped {
        node_id: node_id.into(),
        node_type: "file-rename".into(),
        parent_node_id: parent.map(String::from),
    }
}

#[test]
fn a_skipped_top_level_step_shows_as_skipped() {
    let m = started(&[("mac", "group"), ("linux", "group")]);
    let m = update(m, skipped("linux", None));
    assert_eq!(m.nodes[1].status, NodeStatus::Skipped);
    assert_eq!(m.nodes[0].status, NodeStatus::Pending);
}

#[test]
fn a_skipped_child_is_listed_under_its_container() {
    let m = started(&[("g", "group")]);
    let m = update(m, skipped("tag", Some("g")));
    let child = &m.nodes[0].children[0];
    assert_eq!(child.id, "tag");
    assert_eq!(child.node_type, "file-rename");
    assert_eq!(child.status, NodeStatus::Skipped);
}
