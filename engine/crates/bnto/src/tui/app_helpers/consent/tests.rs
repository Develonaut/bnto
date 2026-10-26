// SEC-19: every TUI path that starts a run asks before an untrusted shell
// recipe's commands run, and remembers the answer like `bnto run` does. The
// run's bridge asks once its checks pass (ISS-77); `would_ask` decides as it
// does, from what each path hands it. The bridge's side is tested in
// `tui/bridge.rs`, the event's in `tui/run_loop/tests.rs`.

use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::super::super::app::{AppMessage, AppModel, DetailOrigin, Screen, update};
use super::super::super::keys::handle_key;
use super::super::super::screens::detail::DetailModel;
use super::super::super::screens::execution::{ExecutionMessage, ExecutionStatus};
use super::super::super::screens::preview::PreviewModel;
use super::super::super::theme::ThemeVariant;
use super::{consent_check, hold};
use crate::storage::paths::BntoPaths;
use crate::trust::{TrustStore, definition_hash};
use crate::trust_gate::{self, ConsentRequest};

const SHELL_RECIPE: &str = r#"{
    "id": "probe", "type": "group", "version": "1.0.0", "name": "Probe",
    "nodes": [
        { "id": "run", "type": "shell-run", "version": "1.0.0",
          "parameters": { "command": "git", "args": ["--version"] } }
    ]
}"#;

fn model() -> (tempfile::TempDir, AppModel) {
    let dir = tempfile::tempdir().unwrap();
    let paths = BntoPaths {
        home: dir.path().to_path_buf(),
    };
    let model = AppModel::with_paths(ThemeVariant::LosAngeles, None, false, paths);
    (dir, model)
}

fn detail(slug: &str, definition_json: &str) -> DetailModel {
    let mut detail = DetailModel::from_test_data(slug, "Probe", "", Vec::new());
    detail.definition_json = definition_json.to_string();
    detail
}

/// Start a run from the detail screen (Run, or Preview with `preview`).
fn run_from_detail(model: AppModel, slug: &str, json: &str, preview: bool) -> AppModel {
    let from = DetailOrigin::Library;
    let model = AppModel {
        screen: Screen::Detail {
            slug: slug.into(),
            from,
        },
        detail: Some(detail(slug, json)),
        ..model
    };
    let msg = if preview {
        AppMessage::PreviewRequested { slug: slug.into() }
    } else {
        AppMessage::ConfigConfirmed { slug: slug.into() }
    };
    update(model, msg)
}

fn status(model: &AppModel) -> ExecutionStatus {
    model.execution.as_ref().expect("a run").status.clone()
}

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

/// The consent the run's bridge would ask for, from what this run hands it.
fn would_ask(model: &AppModel) -> Option<ConsentRequest> {
    let exec = model.execution.as_ref().expect("a run");
    let check = consent_check(model, &exec.slug, &exec.definition_json);
    let store = TrustStore::load(&check.trusted_file);
    trust_gate::consent_needed(&exec.definition_json, &exec.slug, check.bundled, &store)
}

/// The run held for consent, as the bridge's request does.
fn held(model: AppModel) -> AppModel {
    let request = would_ask(&model).expect("the run needs consent");
    hold(model, request)
}

/// The run starts its bridge, which will ask before any command runs.
fn asks_first(model: &AppModel) {
    assert_eq!(status(model), ExecutionStatus::Idle);
    assert!(would_ask(model).is_some(), "the run would start unasked");
}

#[test]
fn running_an_untrusted_shell_recipe_waits_for_consent() {
    let (_dir, model) = model();
    let app = run_from_detail(model, "probe", SHELL_RECIPE, false);
    asks_first(&app);
    let app = held(app);
    assert_eq!(status(&app), ExecutionStatus::AwaitingConsent);
    let request = app.execution.as_ref().unwrap().consent.clone().unwrap();
    assert_eq!(request.commands, vec!["git --version"]);
}

#[test]
fn previewing_an_untrusted_shell_recipe_waits_for_consent() {
    // Preview runs still execute shell steps.
    let (_dir, model) = model();
    let app = run_from_detail(model, "probe", SHELL_RECIPE, true);
    asks_first(&app);
}

#[test]
fn picking_files_for_an_untrusted_shell_recipe_waits_for_consent() {
    let (_dir, model) = model();
    let from = DetailOrigin::Library;
    let app = update(
        AppModel {
            screen: Screen::Picker {
                slug: "probe".into(),
                from,
            },
            detail: Some(detail("probe", SHELL_RECIPE)),
            ..model
        },
        AppMessage::FilesSelected {
            slug: "probe".into(),
        },
    );
    asks_first(&app);
}

#[test]
fn confirming_a_preview_of_an_untrusted_shell_recipe_waits_for_consent() {
    let (_dir, model) = model();
    let from = DetailOrigin::Library;
    let preview = PreviewModel {
        slug: "probe".into(),
        entries: Vec::new(),
        cursor: 0,
        duration_ms: 0,
        warnings: Vec::new(),
        selected_files: Vec::new(),
        param_overrides: HashMap::new(),
        definition_json: SHELL_RECIPE.into(),
    };
    let app = update(
        AppModel {
            screen: Screen::Preview {
                slug: "probe".into(),
                from,
            },
            preview: Some(preview),
            ..model
        },
        AppMessage::PreviewConfirm {
            slug: "probe".into(),
        },
    );
    asks_first(&app);
}

#[test]
fn bundled_recipes_start_without_asking() {
    let (_dir, model) = model();
    let json = model
        .catalog
        .resolve("download-video")
        .unwrap()
        .definition_json
        .clone();
    let app = run_from_detail(model, "download-video", &json, false);
    assert_eq!(status(&app), ExecutionStatus::Idle);
    assert!(would_ask(&app).is_none());
}

#[test]
fn an_edited_copy_of_a_bundled_recipe_still_asks() {
    // The editor runs raw JSON: keeping a bundled slug must not borrow its trust.
    let (_dir, model) = model();
    let json = model
        .catalog
        .resolve("download-video")
        .unwrap()
        .definition_json
        .clone();
    let edited = json.replace("yt-dlp", "evil-tool");
    let app = run_from_detail(model, "download-video", &edited, false);
    asks_first(&app);
}

#[test]
fn allowing_starts_the_run_and_remembers_the_approval() {
    let (_dir, model) = model();
    let app = held(run_from_detail(model, "probe", SHELL_RECIPE, false));
    let app = update(app, AppMessage::Execution(ExecutionMessage::AllowCommands));
    assert_eq!(status(&app), ExecutionStatus::Idle);
    let store = TrustStore::load(&app.paths.trusted_file());
    assert!(store.is_approved(&definition_hash(SHELL_RECIPE)));

    // The same definition runs straight away next time.
    let app = run_from_detail(app, "probe", SHELL_RECIPE, false);
    assert!(would_ask(&app).is_none());
}

#[test]
fn declining_returns_to_the_recipe_without_running() {
    let (_dir, model) = model();
    let app = held(run_from_detail(model, "probe", SHELL_RECIPE, false));
    let app = update(app, AppMessage::Execution(ExecutionMessage::Cancel));
    assert!(matches!(app.screen, Screen::Detail { .. }));
    assert!(app.execution.is_none());
    let store = TrustStore::load(&app.paths.trusted_file());
    assert!(!store.is_approved(&definition_hash(SHELL_RECIPE)));
}

#[test]
fn allow_does_nothing_unless_consent_is_pending() {
    let (_dir, model) = model();
    let json = model
        .catalog
        .resolve("download-video")
        .unwrap()
        .definition_json
        .clone();
    let app = run_from_detail(model, "download-video", &json, false);
    let app = update(app, AppMessage::Execution(ExecutionMessage::AllowCommands));
    assert_eq!(status(&app), ExecutionStatus::Idle);
    let store = TrustStore::load(&app.paths.trusted_file());
    assert!(!store.is_approved(&definition_hash(&json)));
}

#[test]
fn y_allows_and_n_declines_only_while_consent_is_pending() {
    let (_dir, model) = model();
    let app = run_from_detail(model, "probe", SHELL_RECIPE, false);
    // Not yet asked: the bridge is still checking the run.
    assert!(handle_key(&app, key(KeyCode::Char('y'))).is_none());
    assert!(handle_key(&app, key(KeyCode::Char('n'))).is_none());
    let app = held(app);
    assert!(matches!(
        handle_key(&app, key(KeyCode::Char('y'))),
        Some(AppMessage::Execution(ExecutionMessage::AllowCommands))
    ));
    assert!(matches!(
        handle_key(&app, key(KeyCode::Char('n'))),
        Some(AppMessage::Execution(ExecutionMessage::Cancel))
    ));

    let app = update(app, AppMessage::Execution(ExecutionMessage::AllowCommands));
    assert!(handle_key(&app, key(KeyCode::Char('y'))).is_none());
}

/// A model whose library holds `json` as its own `download-video`.
fn model_with_library_copy(json: &str) -> (tempfile::TempDir, AppModel) {
    let dir = tempfile::tempdir().unwrap();
    let paths = BntoPaths {
        home: dir.path().to_path_buf(),
    };
    paths.ensure_dirs().unwrap();
    std::fs::write(paths.recipes_dir().join("download-video.bnto.json"), json).unwrap();
    let model = AppModel::with_paths(ThemeVariant::LosAngeles, None, false, paths);
    (dir, model)
}

fn shipped_download_video() -> String {
    bnto_engine::recipes::builtin_recipes()
        .into_iter()
        .find(|r| r.slug == "download-video")
        .unwrap()
        .definition_json
        .to_string()
}

#[test]
fn the_shipped_recipe_starts_without_asking_when_a_library_copy_shares_its_slug() {
    let shipped = shipped_download_video();
    let (_dir, model) = model_with_library_copy(&shipped.replace("yt-dlp", "my-dlp"));
    let app = run_from_detail(model, "download-video", &shipped, false);
    assert!(would_ask(&app).is_none());
}

#[test]
fn a_library_copy_with_a_built_in_slug_still_asks() {
    let copy = shipped_download_video().replace("yt-dlp", "my-dlp");
    let (_dir, model) = model_with_library_copy(&copy);
    let app = run_from_detail(model, "download-video", &copy, false);
    asks_first(&app);
}
