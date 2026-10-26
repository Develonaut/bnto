// Sessions for the snapshots of the execution and results screens, mid-run
// and after.

use std::collections::HashMap;
use std::path::PathBuf;

use super::Session;
use crate::run_message::RunMessage;
use crate::trust_gate::ConsentRequest;
use crate::tui::app::{DetailOrigin, Screen};
use crate::tui::screens::execution::{self, ExecutionMessage, ExecutionModel};
use crate::tui::screens::results::{OutputFile, ResultsModel};

pub(super) const LOOP_RECIPE: &str =
    include_str!("../../../tests/fixtures/explicit/compress-and-rename.bnto.json");
const RUN_FILES: [&str; 3] = ["beach.jpg", "sunset.jpg", "lake.jpg"];

/// The execution screen after `events`, for a run over [`RUN_FILES`].
pub(super) fn running(definition_json: &str, events: &[ExecutionMessage]) -> Session {
    let files = RUN_FILES
        .iter()
        .map(|f| PathBuf::from("/in").join(f))
        .collect();
    let mut model =
        ExecutionModel::with_inputs("run", files, HashMap::new(), definition_json.to_string());
    for event in events {
        model = execution::update(model, event.clone());
    }
    in_execution(Session::new(), model)
}

fn in_execution(mut session: Session, model: ExecutionModel) -> Session {
    session.model.screen = Screen::Execution {
        slug: "run".into(),
        from: DetailOrigin::Home,
    };
    session.model.execution = Some(model);
    session
}

/// Iteration 2 of 3 of an explicit loop: the first file done, the second
/// compressing, the third waiting, and the loop's later step still pending.
pub(super) fn loop_mid_run() -> Vec<ExecutionMessage> {
    use ExecutionMessage as M;
    let iteration = |i| M::IterationStarted {
        node_id: "process-loop".into(),
        iteration: i,
        total_iterations: 3,
    };
    let child = |id: &str| M::ChildNodeStarted {
        parent_node_id: "process-loop".into(),
        node_id: id.into(),
        node_type: "image-compress".into(),
    };
    vec![
        M::PipelineStarted {
            total_nodes: 1,
            total_files: 3,
            node_info: vec![("process-loop".into(), "loop".into())],
        },
        M::NodeStarted {
            node_id: "process-loop".into(),
            node_type: "loop".into(),
        },
        iteration(0),
        child("compress"),
        M::IterationCompleted {
            node_id: "process-loop".into(),
            iteration: 0,
            total_iterations: 3,
            duration_ms: 40,
            files_produced: 1,
            output_files: vec![],
        },
        iteration(1),
        child("compress"),
        M::FileProgress {
            node_id: "process-loop".into(),
            file_index: 1,
            total_files: 3,
            percent: 50,
            message: "Compressing sunset.jpg...".into(),
        },
        M::CommandOutput {
            node_id: "compress".into(),
            line: "Compressing JPEG (quality: 80)...".into(),
        },
    ]
}

/// File 12 of 20 compressing: more files than fit, with output still showing.
pub(super) fn many_files() -> Session {
    use ExecutionMessage as M;
    let files = (1..=20)
        .map(|i| PathBuf::from(format!("/in/photo{i:02}.jpg")))
        .collect();
    let mut model = ExecutionModel::with_inputs("run", files, HashMap::new(), "{}".into());
    let mut events = vec![
        M::PipelineStarted {
            total_nodes: 1,
            total_files: 20,
            node_info: vec![("c".into(), "image-compress".into())],
        },
        M::NodeStarted {
            node_id: "c".into(),
            node_type: "image-compress".into(),
        },
    ];
    events.extend((0..12).map(|i| M::FileProgress {
        node_id: "c".into(),
        file_index: i,
        total_files: 20,
        percent: 50,
        message: format!("Compressing photo{:02}.jpg...", i + 1),
    }));
    events.push(M::CommandOutput {
        node_id: "c".into(),
        line: "Compressing JPEG (quality: 80)...".into(),
    });
    for event in events {
        model = execution::update(model, event);
    }
    in_execution(Session::new(), model)
}

/// A single step that failed partway through the first file.
pub(super) fn failed_run() -> Vec<ExecutionMessage> {
    use ExecutionMessage as M;
    vec![
        M::PipelineStarted {
            total_nodes: 1,
            total_files: 3,
            node_info: vec![("talk".into(), "shell-run".into())],
        },
        M::NodeStarted {
            node_id: "talk".into(),
            node_type: "shell-run".into(),
        },
        M::CommandOutput {
            node_id: "talk".into(),
            line: "fetching page 1...".into(),
        },
        M::NodeFailed {
            node_id: "talk".into(),
            error: "command exited with status 2".into(),
        },
        M::PipelineFailed {
            node_id: "talk".into(),
            error: "command exited with status 2".into(),
        },
    ]
}

/// A library shell recipe waiting for the user to allow its script.
pub(super) fn consent() -> Session {
    let request = ConsentRequest {
        recipe_name: "Slow Shell".into(),
        commands: vec![
            "sh -c <script> bnto\n    script:\n      for i in 1 2 3; do echo \"line $i\"; done"
                .into(),
        ],
        hash: "h".into(),
    };
    in_execution(
        Session::new(),
        ExecutionModel::new("run").awaiting_consent(request),
    )
}

/// keep-awake's results: its session log in place of an empty file list.
pub(super) fn results_with_message() -> Session {
    let message = RunMessage {
        label: Some("Session Log".into()),
        text: "Session started: staying awake for 1 hour.\nActivity simulation: off".into(),
    };
    let mut session = Session::new();
    session.model.screen = Screen::Results {
        slug: "keep-awake".into(),
    };
    session.model.results =
        Some(ResultsModel::new("keep-awake", vec![], 26, None).with_report(Some(message), vec![]));
    session
}

/// A rename run's results, written in place in two folders: both files are
/// `renamed-a.txt`, so each row names its folder (ISS-43, ISS-46).
pub(super) fn results_written_in_place() -> Session {
    let file = |dir: &str| OutputFile {
        name: "renamed-a.txt".into(),
        path: PathBuf::from("/photos").join(dir).join("renamed-a.txt"),
        size_bytes: 2_048,
        original_size: None,
    };
    let mut session = Session::new();
    session.model.screen = Screen::Results {
        slug: "rename-files".into(),
    };
    let outputs = vec![file("summer"), file("winter")];
    session.model.results = Some(ResultsModel::new("rename-files", outputs, 412, None));
    session
}
