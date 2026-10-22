// What the running screen shows before and during a run: real file names,
// every step of a container listed up front, and files marked done once the
// run has moved past them (the run output audit's R5–R7).

use std::collections::HashMap;
use std::path::PathBuf;

use super::*;

const LOOP_RECIPE: &str =
    include_str!("../../../../tests/fixtures/explicit/compress-and-rename.bnto.json");

fn started(
    files: &[&str],
    definition_json: &str,
    node_info: Vec<(String, String)>,
) -> ExecutionModel {
    let selected: Vec<PathBuf> = files.iter().map(|f| PathBuf::from("/in").join(f)).collect();
    let model =
        ExecutionModel::with_inputs("s", selected, HashMap::new(), definition_json.to_string());
    update(
        model,
        ExecutionMessage::PipelineStarted {
            total_nodes: node_info.len().max(1),
            total_files: files.len(),
            node_info,
        },
    )
}

fn loop_node_info() -> Vec<(String, String)> {
    let def: serde_json::Value = serde_json::from_str(LOOP_RECIPE).unwrap();
    def["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|n| n["type"] == "loop")
        .map(|n| (n["id"].as_str().unwrap().to_string(), "loop".to_string()))
        .collect()
}

#[test]
fn waiting_files_show_their_names() {
    let m = started(&["beach.jpg", "sunset.png"], "{}", vec![]);
    let names: Vec<&str> = m.files.iter().map(|f| f.name.as_str()).collect();
    assert_eq!(names, ["beach.jpg", "sunset.png"]);
}

#[test]
fn a_containers_steps_are_listed_before_they_start() {
    let m = started(&["a.jpg"], LOOP_RECIPE, loop_node_info());
    let children: Vec<&str> = m.nodes[0]
        .children
        .iter()
        .map(|c| c.node_type.as_str())
        .collect();
    assert_eq!(children, ["image-compress", "file-rename"]);
    assert!(
        m.nodes[0]
            .children
            .iter()
            .all(|c| c.status == NodeStatus::Pending)
    );
}

#[test]
fn a_file_is_done_once_the_run_moves_past_it() {
    let m = started(
        &["a.jpg", "b.jpg"],
        "{}",
        vec![("n".into(), "image-compress".into())],
    );
    let progress = |m, index: usize| {
        update(
            m,
            ExecutionMessage::FileProgress {
                node_id: "n".into(),
                file_index: index,
                total_files: 2,
                percent: 50,
                message: String::new(),
            },
        )
    };
    let m = progress(progress(m, 0), 1);
    assert_eq!(m.files[0].status, FileStatus::Done);
    assert_eq!(m.files[1].status, FileStatus::Processing);
}

#[test]
fn a_finished_loop_iteration_marks_its_file_done() {
    let m = started(&["a.jpg", "b.jpg"], LOOP_RECIPE, loop_node_info());
    let loop_id = m.nodes[0].id.clone();
    let m = update(
        m,
        ExecutionMessage::IterationCompleted {
            node_id: loop_id.clone(),
            iteration: 0,
            total_iterations: 2,
            duration_ms: 5,
            files_produced: 1,
            output_files: vec![],
        },
    );
    assert_eq!(m.files[0].status, FileStatus::Done);
    let m = update(
        m,
        ExecutionMessage::IterationFailed {
            node_id: loop_id,
            iteration: 1,
            total_iterations: 2,
            error: "boom".into(),
        },
    );
    assert!(matches!(m.files[1].status, FileStatus::Failed(_)));
}

#[test]
fn a_done_file_shows_its_name_not_its_last_progress_message() {
    let m = started(
        &["a.jpg", "b.jpg"],
        "{}",
        vec![("n".into(), "image-compress".into())],
    );
    let progress = |m, index: usize, message: &str| {
        update(
            m,
            ExecutionMessage::FileProgress {
                node_id: "n".into(),
                file_index: index,
                total_files: 2,
                percent: 50,
                message: message.into(),
            },
        )
    };
    let m = progress(
        progress(m, 0, "Compressing a.jpg..."),
        1,
        "Compressing b.jpg...",
    );
    assert_eq!(m.files[0].name, "a.jpg");
    assert_eq!(m.files[1].name, "Compressing b.jpg...");
}
