// A finished run: the preview screen for a preview run, results otherwise.

use crate::tui::app::{AppMessage, AppModel, DetailOrigin, Screen, update};
use crate::tui::bridge::{self, RunDone};
use crate::tui::screens::execution::{ExecutionMessage, ExecutionModel};
use crate::tui::screens::preview::PreviewModel;

/// The model once the bridge reports the run done.
pub(super) fn finish_run(mut model: AppModel, done: RunDone) -> AppModel {
    let (slug, from) = match &model.screen {
        Screen::Execution { slug, from } => (slug.clone(), *from),
        _ => (String::new(), DetailOrigin::Home),
    };
    match model.execution.take() {
        Some(exec) if exec.preview_mode => finish_preview(model, exec, done, slug, from),
        execution => finish_results(AppModel { execution, ..model }, done, slug),
    }
}

/// Open the preview screen with what the dry run would write.
fn finish_preview(
    model: AppModel,
    exec: ExecutionModel,
    done: RunDone,
    slug: String,
    from: DetailOrigin,
) -> AppModel {
    let preview = PreviewModel::new(
        &slug,
        done.preview_data,
        done.duration_ms,
        done.warnings,
        exec.selected_files,
        exec.param_overrides,
        exec.definition_json,
    );
    AppModel {
        screen: Screen::Preview { slug, from },
        execution: None,
        preview: Some(preview),
        ..model
    }
}

/// Hand the outputs to the execution screen, then move on to results.
fn finish_results(model: AppModel, done: RunDone, slug: String) -> AppModel {
    let files = bridge::build_output_files(&done.file_metadata);
    let reported = done
        .message
        .map(|message| ExecutionMessage::Reported { message });
    let steps = reported.into_iter().chain([
        // Files written in place have no one output folder; results find
        // each file's from its path.
        ExecutionMessage::OutputsReady {
            files,
            output_dir: (!done.in_place).then_some(done.output_dir),
            warnings: done.warnings,
        },
        ExecutionMessage::PipelineCompleted {
            duration_ms: done.duration_ms,
            total_files_processed: done.file_count,
        },
    ]);
    let model = steps.fold(model, |model, msg| {
        update(model, AppMessage::Execution(msg))
    });
    update(model, AppMessage::ExecutionComplete { slug })
}
