// Execution, Results and Preview screen keys.

use crossterm::event::{KeyCode, KeyEvent};

use super::super::app::{AppMessage, AppModel, Screen};
use super::super::screens::execution::{ExecutionMessage, ExecutionStatus};
use super::super::screens::preview::PreviewMessage;
use super::super::screens::results::ResultsMessage;

/// Handle key events on the Execution screen.
///
/// Enter transitions to Results when the pipeline has finished (Completed or Failed).
/// Esc cancels a running pipeline (handled above in handle_key as a special case).
#[expect(clippy::too_many_lines)]
pub(super) fn handle_execution_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    let awaiting = model
        .execution
        .as_ref()
        .is_some_and(|e| e.status == ExecutionStatus::AwaitingConsent);
    if awaiting {
        return consent_key(key);
    }
    let finished = model.execution.as_ref().is_some_and(|e| {
        matches!(
            e.status,
            ExecutionStatus::Completed | ExecutionStatus::Failed
        )
    });

    match key.code {
        KeyCode::Enter if finished => {
            if let Screen::Execution { slug, .. } = &model.screen {
                Some(AppMessage::ExecutionComplete { slug: slug.clone() })
            } else {
                None
            }
        }
        KeyCode::Esc => Some(AppMessage::Execution(ExecutionMessage::Cancel)),
        _ => None,
    }
}

/// While a run awaits consent: `y` allows its commands, `n` or Esc declines.
fn consent_key(key: KeyEvent) -> Option<AppMessage> {
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            Some(AppMessage::Execution(ExecutionMessage::AllowCommands))
        }
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
            Some(AppMessage::Execution(ExecutionMessage::Cancel))
        }
        _ => None,
    }
}

/// Handle key events on the Results screen.
pub(super) fn handle_results_key(_model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => Some(AppMessage::Results(ResultsMessage::CursorDown)),
        KeyCode::Char('k') | KeyCode::Up => Some(AppMessage::Results(ResultsMessage::CursorUp)),
        KeyCode::Char('o') => Some(AppMessage::OpenFile),
        KeyCode::Char('O') => Some(AppMessage::OpenFolder),
        KeyCode::Char('r') => Some(AppMessage::RunAnother),
        _ => None,
    }
}

/// Handle key events on the Preview screen.
pub(super) fn handle_preview_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    let preview = model.preview.as_ref()?;

    match key.code {
        KeyCode::Char('j') | KeyCode::Down => Some(AppMessage::Preview(PreviewMessage::CursorDown)),
        KeyCode::Char('k') | KeyCode::Up => Some(AppMessage::Preview(PreviewMessage::CursorUp)),
        // Enter → confirm: re-run pipeline with writes.
        KeyCode::Enter => Some(AppMessage::PreviewConfirm {
            slug: preview.slug.clone(),
        }),
        _ => None,
    }
}
