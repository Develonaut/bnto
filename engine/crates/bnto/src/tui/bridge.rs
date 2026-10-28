// Pipeline execution bridge — runs the engine on a background thread
// and relays PipelineEvent progress back to the TUI event loop via mpsc.

use crate::run_message::{RunMessage, output_label, run_message};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

use bnto_core::events::{PipelineEvent, PipelineReporter};
use bnto_core::pipeline::PipelineResult;
use bnto_core::{BntoError, InputMode, PipelineDefinition, resolve_input_mode};
use bnto_engine::run_pipeline;

use crate::context::NativeContext;
use crate::dry_run::files::FilePreview;
use crate::run_watchdog::{self, Watched};
use crate::trust_gate::ConsentRequest;
use crate::{host_support, input, io, run_limit};

mod events;
mod output;
mod precheck;
mod reporter;

pub use events::map_pipeline_event;
use output::extract_file_metadata;
pub use output::{FileResultMeta, build_output_files};
pub use precheck::ConsentCheck;
use precheck::{Ready, RunArgs};
use reporter::{channel_reporter, progressive_dir};

/// Message sent from the bridge thread back to the TUI event loop.
#[derive(Debug)]
pub enum BridgeEvent {
    /// A pipeline progress event from the engine. A finished iteration's
    /// progressive files are already written when it arrives.
    Progress(PipelineEvent),
    /// The run passed its checks and needs the user to allow its commands.
    /// Send on `reply` to go ahead; drop it to stop the run.
    ConsentNeeded {
        request: ConsentRequest,
        reply: mpsc::Sender<()>,
    },
    /// Pipeline finished — output files written to this directory.
    Done(RunDone),
    /// Pipeline or setup failed with an error message.
    Error(String),
}

/// A finished run, as the bridge reports it.
#[derive(Debug)]
pub struct RunDone {
    pub output_dir: String,
    /// The run wrote each file in place (overwrite mode), not into
    /// `output_dir`, which is then just the first selected file's folder.
    pub in_place: bool,
    pub file_count: usize,
    pub duration_ms: u64,
    pub file_metadata: Vec<FileResultMeta>,
    pub warnings: Vec<String>,
    /// File transformation previews (populated in preview mode only).
    pub preview_data: Vec<FilePreview>,
    /// What a message-mode recipe reported (keep-awake's status text).
    pub message: Option<RunMessage>,
}

/// Everything one bridge run needs, owned by the worker thread.
struct BridgeJob {
    slug: String,
    definition_json: String,
    selected_files: Vec<PathBuf>,
    param_overrides: HashMap<String, String>,
    preview_mode: bool,
    consent: ConsentCheck,
}

/// Spawn a background thread that runs the pipeline for the given recipe.
///
/// Returns a Receiver that the TUI event loop polls each tick.
/// The thread checks the run, asks for consent if it needs it, runs the
/// engine with the given definition JSON, writes output files, then sends a
/// Done or Error event.
pub fn spawn_pipeline(
    slug: String,
    definition_json: String,
    selected_files: Vec<PathBuf>,
    param_overrides: HashMap<String, String>,
    preview_mode: bool,
    consent: ConsentCheck,
) -> mpsc::Receiver<BridgeEvent> {
    let (tx, rx) = mpsc::channel();
    let job = BridgeJob {
        slug,
        definition_json,
        selected_files,
        param_overrides,
        preview_mode,
        consent,
    };
    thread::spawn(move || run_bridge(tx, &job));
    rx
}

/// Inner bridge logic — runs synchronously on the spawned thread.
fn run_bridge(tx: mpsc::Sender<BridgeEvent>, job: &BridgeJob) {
    match NativeContext::current_dir() {
        Ok(ctx) => run_bridge_in(&tx, job, ctx.with_dry_run(job.preview_mode)),
        Err(e) => {
            let _ = tx.send(BridgeEvent::Error(e.to_string()));
        }
    }
}

/// Run `job` in `ctx`. Any failure is reported to the TUI as a single
/// `Error` event.
fn run_bridge_in(tx: &mpsc::Sender<BridgeEvent>, job: &BridgeJob, ctx: NativeContext) {
    if let Err(e) = try_run_bridge(tx, job, &ctx) {
        let _ = tx.send(BridgeEvent::Error(e));
    }
}

/// Check the run and get consent, run the pipeline, then write (or preview)
/// the results.
fn try_run_bridge(
    tx: &mpsc::Sender<BridgeEvent>,
    job: &BridgeJob,
    ctx: &NativeContext,
) -> Result<(), String> {
    let Some(ready) = checked(tx, job, ctx)? else {
        return Ok(());
    };
    let mode = resolve_mode(&job.definition_json);
    let output_dir = resolve_output_dir(job, &mode, ready.recipe_dir);
    let reporter = channel_reporter(tx, progressive_dir(job.preview_mode, &mode, &output_dir));
    // Run the pipeline (blocking — this is why we're on a background thread).
    let Some(result) = run_watched(tx, ready.prepared, &reporter, ctx)? else {
        return Ok(());
    };
    if job.preview_mode {
        send_preview_done(tx, output_dir, result);
        return Ok(());
    }
    // Normal mode: write output files using mode-aware dispatch.
    let file_metadata = extract_file_metadata(&result, &mode, &output_dir);
    let message = (mode == "message")
        .then(|| run_message(&result, recipe_output_label(&job.definition_json)))
        .flatten();
    io::write_results_with_mode(&result, &mode, &output_dir, false)?;
    send_done(tx, output_dir, &mode, result, file_metadata, message);
    Ok(())
}

/// Everything `bnto run` checks before it asks for consent, then consent.
/// A recipe with no branch for this OS stops first. `None`: not allowed.
fn checked(
    tx: &mpsc::Sender<BridgeEvent>,
    job: &BridgeJob,
    ctx: &NativeContext,
) -> Result<Option<Ready>, String> {
    let host = bnto_core::HostPlatform::current();
    if let Some(why) = host_support::unsupported_host(&job.slug, &job.definition_json, host) {
        return Err(why);
    }
    let (args, overrides) = input_args(job);
    let run = RunArgs {
        slug: &job.slug,
        definition_json: &job.definition_json,
        args: &args,
        overrides: &overrides,
    };
    precheck::check_then_consent(tx, &run, &job.consent, ctx)
}

/// Run the pipeline under the recipe's time limit. `None` when the run was
/// stuck past it and the watchdog already sent the timeout as an `Error`.
fn run_watched(
    tx: &mpsc::Sender<BridgeEvent>,
    prepared: input::PreparedInput,
    reporter: &PipelineReporter,
    ctx: &NativeContext,
) -> Result<Option<PipelineResult>, String> {
    let (json, time_limit) = run_limit::apply(&prepared.definition_json, None)?;
    let stuck_tx = tx.clone();
    let on_stuck = move |error: BntoError| {
        let _ = stuck_tx.send(BridgeEvent::Error(error.to_string()));
    };
    let run = || run_pipeline(&json, prepared.files, reporter, ctx);
    match run_watchdog::watch(ctx.registry(), time_limit, on_stuck, run) {
        Watched::Finished(result) => result.map(Some).map_err(|e| e.to_string()),
        Watched::Ended => Ok(None),
    }
}

/// Positional args and `key=value` overrides for `prepare_inputs`.
///
/// For URL/Text mode, the value moves from the param overrides into the
/// positional args (prepare_inputs expects it there).
fn input_args(job: &BridgeJob) -> (Vec<String>, Vec<String>) {
    let mut args: Vec<String> = job
        .selected_files
        .iter()
        .map(|p| p.to_string_lossy().into_owned())
        .collect();
    let mut remaining_overrides = job.param_overrides.clone();
    let param = match recipe_input_mode(&job.definition_json) {
        InputMode::Url => Some("url"),
        InputMode::Text => Some("text"),
        InputMode::FileUpload | InputMode::None => None,
    };
    if let Some(value) = param.and_then(|p| extract_override_value(&mut remaining_overrides, p)) {
        args.push(value);
    }
    let override_args = remaining_overrides
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect();
    (args, override_args)
}

/// The recipe's input mode (FileUpload when the JSON doesn't parse).
fn recipe_input_mode(definition_json: &str) -> InputMode {
    serde_json::from_str::<PipelineDefinition>(definition_json)
        .map(|def| resolve_input_mode(&def))
        .unwrap_or_default()
}

/// The recipe output node's label, for its message (keep-awake: "Session Log").
fn recipe_output_label(definition_json: &str) -> Option<String> {
    let def = serde_json::from_str::<PipelineDefinition>(definition_json).ok()?;
    output_label(&def)
}

/// Resolve the output mode from the recipe definition.
fn resolve_mode(definition_json: &str) -> String {
    serde_json::from_str::<PipelineDefinition>(definition_json)
        .map(|def| bnto_core::resolve_output_mode(&def))
        .unwrap_or_else(|_| "write".to_string())
}

/// Resolve the output directory, after consent. The recipe's own folder
/// (`recipe_dir`) was filled before it, in every mode, as `bnto run` does,
/// so a placeholder nothing fills stopped the run before any step.
/// Preview mode writes nothing (empty string). Overwrite mode reports the
/// source directory, since files are renamed in place.
fn resolve_output_dir(job: &BridgeJob, mode: &str, recipe_dir: Option<String>) -> String {
    if job.preview_mode {
        String::new()
    } else if mode == "overwrite" {
        source_dir(&job.selected_files)
    } else {
        run_output_dir(&job.slug, recipe_dir)
    }
}

/// Directory of the first selected file (or ".").
fn source_dir(selected_files: &[PathBuf]) -> String {
    selected_files
        .first()
        .and_then(|p| p.parent())
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|| ".".to_string())
}

/// The folder this run writes into, created if missing: the recipe's own
/// folder (as `bnto run` resolves it), else the TUI's temp folder for it.
/// A recipe's folder may be the user's (`~/Documents/Reports`), so nothing
/// in it is deleted; the write replaces only files named like an output.
fn run_output_dir(slug: &str, recipe_dir: Option<String>) -> String {
    let output_dir = match recipe_dir {
        Some(dir) => PathBuf::from(dir),
        None => emptied_temp_dir(slug),
    };
    let _ = std::fs::create_dir_all(&output_dir);
    output_dir.to_string_lossy().into_owned()
}

/// The TUI's own temp folder for `slug`, emptied of the last run's files.
/// The only output folder bnto owns, so the only one it may clear.
fn emptied_temp_dir(slug: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("bnto-tui-{slug}"));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

/// Preview mode: extract file previews, skip writing.
fn send_preview_done(
    tx: &mpsc::Sender<BridgeEvent>,
    output_dir: String,
    result: bnto_core::pipeline::PipelineResult,
) {
    let previews = crate::dry_run::files::extract_previews(&result);
    let _ = tx.send(BridgeEvent::Done(RunDone {
        output_dir,
        in_place: false,
        file_count: result.files.len(),
        duration_ms: result.duration_ms,
        file_metadata: vec![],
        warnings: result.warnings,
        preview_data: previews,
        message: None,
    }));
}

/// Report a finished, written run. Overwrite mode wrote each file in place.
fn send_done(
    tx: &mpsc::Sender<BridgeEvent>,
    output_dir: String,
    mode: &str,
    result: bnto_core::pipeline::PipelineResult,
    file_metadata: Vec<FileResultMeta>,
    message: Option<RunMessage>,
) {
    let _ = tx.send(BridgeEvent::Done(RunDone {
        output_dir,
        in_place: mode == "overwrite",
        file_count: result.files.len(),
        duration_ms: result.duration_ms,
        file_metadata,
        warnings: result.warnings,
        preview_data: vec![],
        message,
    }));
}

/// Find and remove an override value by param name (e.g. "url").
///
/// Override keys use "nodeId:paramName" format from the detail screen.
/// Returns the value and removes the entry so it isn't double-applied.
fn extract_override_value(
    overrides: &mut HashMap<String, String>,
    param_name: &str,
) -> Option<String> {
    let key = overrides
        .keys()
        .find(|k| k.ends_with(&format!(":{param_name}")))
        .cloned();
    key.and_then(|k| overrides.remove(&k))
}

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::FileData;
    use std::fs;
    use std::sync::Arc;
    use std::time::{Duration, Instant};

    use crate::process_registry::ProcessRegistry;
    use crate::tui::screens::results::ResultsModel;

    /// Treated as the shipped recipe, so the run never waits for consent.
    fn no_consent() -> ConsentCheck {
        ConsentCheck {
            bundled: true,
            trusted_file: PathBuf::from("no-trust-file.json"),
        }
    }

    /// Far longer than a stopped run may take, far shorter than the hang.
    const STOP_DEADLINE: Duration = Duration::from_secs(60);

    /// A recipe whose one step runs for minutes, limited to 1s.
    fn hung_recipe_json() -> String {
        let (command, args) = if cfg!(windows) {
            ("ping", serde_json::json!(["-n", "300", "127.0.0.1"]))
        } else {
            ("sleep", serde_json::json!(["300"]))
        };
        serde_json::json!({
            "settings": { "timeout": 1 },
            "nodes": [
                { "id": "input", "type": "input", "parameters": { "mode": "none" } },
                { "id": "hang", "type": "shell-run",
                  "parameters": { "command": command, "args": args } },
                { "id": "output", "type": "output", "parameters": { "mode": "write" } }
            ]
        })
        .to_string()
    }

    #[test]
    fn a_run_past_its_time_limit_is_stopped_and_fails_naming_it() {
        let registry = Arc::new(ProcessRegistry::new());
        let ctx = NativeContext::current_dir()
            .unwrap()
            .with_dry_run(true)
            .with_registry(Arc::clone(&registry));
        let job = BridgeJob {
            slug: "hang".into(),
            definition_json: hung_recipe_json(),
            selected_files: vec![],
            param_overrides: HashMap::new(),
            preview_mode: true,
            consent: no_consent(),
        };
        let (tx, rx) = mpsc::channel();
        let started = Instant::now();
        run_bridge_in(&tx, &job, ctx);
        assert!(started.elapsed() < STOP_DEADLINE, "it was stopped");
        let last = rx.try_iter().last();
        let Some(BridgeEvent::Error(message)) = last else {
            panic!("expected an error last, got {last:?}");
        };
        assert_eq!(message, "Run timed out after 1s");
        assert!(registry.active().is_empty(), "the hung command was killed");
    }

    #[test]
    fn a_recipe_with_no_branch_for_this_os_is_refused_before_any_step() {
        let host = bnto_core::HostPlatform::current().map_or("unknown", |h| h.as_str());
        let other = if host == "macos" { "linux" } else { "macos" };
        let definition_json = serde_json::json!({ "nodes": [
            { "id": "input", "type": "input", "parameters": { "mode": "none" } },
            { "id": "g", "type": "group",
              "when": { "value": "{{ctx.platform}}", "op": "equals", "expected": other },
              "nodes": [{ "id": "s", "type": "edit-fields" }] },
            { "id": "output", "type": "output", "parameters": { "mode": "message" } }
        ] })
        .to_string();
        let job = BridgeJob {
            slug: "elsewhere".into(),
            definition_json,
            selected_files: vec![],
            param_overrides: HashMap::new(),
            preview_mode: true,
            consent: no_consent(),
        };
        let ctx = NativeContext::current_dir().unwrap().with_dry_run(true);
        let (tx, rx) = mpsc::channel();
        run_bridge_in(&tx, &job, ctx);
        let events: Vec<BridgeEvent> = rx.try_iter().collect();
        let [BridgeEvent::Error(message)] = events.as_slice() else {
            panic!("expected only an error, got {events:?}");
        };
        assert!(message.starts_with("elsewhere runs on "), "{message}");
        assert!(message.contains("This machine runs"), "{message}");
    }

    // --- Output folder (ISS-21) ---

    /// A recipe that renames each file to `renamed-<name>` and writes it to
    /// `directory` (no output folder when `None`).
    fn rename_recipe_json(directory: Option<&std::path::Path>) -> String {
        let mut output = serde_json::json!({ "mode": "write" });
        if let Some(dir) = directory {
            output["directory"] = dir.to_string_lossy().into_owned().into();
        }
        serde_json::json!({ "nodes": [
            { "id": "input", "type": "input", "parameters": { "mode": "file-upload" } },
            { "id": "rename", "type": "file-rename", "parameters": { "prefix": "renamed-" } },
            { "id": "output", "type": "output", "parameters": output }
        ] })
        .to_string()
    }

    /// Run `definition_json` on one input `photo.txt` (holding "new") the way
    /// the TUI does, returning the finished run.
    fn run_on_photo(slug: &str, definition_json: String) -> RunDone {
        let inputs = tempfile::tempdir().unwrap();
        let photo = inputs.path().join("photo.txt");
        fs::write(&photo, b"new").unwrap();
        run_job(slug, definition_json, vec![photo])
    }

    /// Run `definition_json` on `selected_files` the way the TUI does.
    fn run_job(slug: &str, definition_json: String, selected_files: Vec<PathBuf>) -> RunDone {
        let job = BridgeJob {
            slug: slug.into(),
            definition_json,
            selected_files,
            param_overrides: HashMap::new(),
            preview_mode: false,
            consent: no_consent(),
        };
        let (tx, rx) = mpsc::channel();
        run_bridge_in(&tx, &job, NativeContext::current_dir().unwrap());
        match rx.try_iter().last() {
            Some(BridgeEvent::Done(done)) => done,
            last => panic!("expected the run to finish, got {last:?}"),
        }
    }

    #[test]
    fn a_run_keeps_files_already_in_the_recipes_folder() {
        // The recipe's folder is the user's (`~/Documents/Reports`), not
        // bnto's: a run writes into it and deletes nothing, as `bnto run` does.
        let folder = tempfile::tempdir().unwrap();
        let canary = folder.path().join("notes.txt");
        fs::write(&canary, b"mine").unwrap();
        run_on_photo("keep", rename_recipe_json(Some(folder.path())));
        assert_eq!(fs::read(&canary).unwrap(), b"mine", "a user file was lost");
        assert_eq!(
            fs::read(folder.path().join("renamed-photo.txt")).unwrap(),
            b"new"
        );
    }

    #[test]
    fn a_run_replaces_only_the_file_named_like_its_output() {
        let folder = tempfile::tempdir().unwrap();
        let earlier = folder.path().join("renamed-photo.txt");
        fs::write(&earlier, b"old").unwrap();
        run_on_photo("replace", rename_recipe_json(Some(folder.path())));
        assert_eq!(fs::read(&earlier).unwrap(), b"new");
    }

    #[test]
    fn a_folder_placeholder_nothing_fills_is_refused_before_the_folder_is_made() {
        // ISS-22: `out-{{ctx.nope}}` used to become `out-` and be created.
        // Preview and overwrite runs refuse it too, as `bnto run` does.
        for (preview_mode, mode) in [(false, "write"), (true, "write"), (false, "overwrite")] {
            let parent = tempfile::tempdir().unwrap();
            let mut def: serde_json::Value = serde_json::from_str(&rename_recipe_json(Some(
                &parent.path().join("out-{{ctx.nope}}"),
            )))
            .unwrap();
            def["nodes"][2]["parameters"]["mode"] = mode.into();
            let inputs = tempfile::tempdir().unwrap();
            let photo = inputs.path().join("photo.txt");
            fs::write(&photo, b"new").unwrap();
            let job = BridgeJob {
                slug: "unfilled".into(),
                definition_json: def.to_string(),
                selected_files: vec![photo.clone()],
                param_overrides: HashMap::new(),
                preview_mode,
                consent: no_consent(),
            };
            let (tx, rx) = mpsc::channel();
            run_bridge_in(&tx, &job, NativeContext::current_dir().unwrap());
            let events: Vec<BridgeEvent> = rx.try_iter().collect();
            let [BridgeEvent::Error(message)] = events.as_slice() else {
                panic!("{mode}, preview {preview_mode}: expected only an error, got {events:?}");
            };
            assert!(message.contains("{{ctx.nope}}"), "{message}");
            let made = fs::read_dir(parent.path()).unwrap().count();
            assert_eq!(made, 0, "{mode}, preview {preview_mode}: made a folder");
            assert!(photo.exists(), "{mode}, preview {preview_mode}: a step ran");
        }
    }

    #[test]
    fn the_forms_value_for_an_output_field_names_the_folder() {
        // ISS-22: the folder is filled from the definition the form's values
        // were applied to, not the recipe as written.
        let parent = tempfile::tempdir().unwrap();
        let mut def: serde_json::Value = serde_json::from_str(&rename_recipe_json(Some(
            &parent.path().join("{{fields.sub}}"),
        )))
        .unwrap();
        def["nodes"][2]["fields"] =
            serde_json::json!({ "sub": { "type": "string", "label": "Sub", "default": "draft" } });
        let inputs = tempfile::tempdir().unwrap();
        let photo = inputs.path().join("photo.txt");
        fs::write(&photo, b"new").unwrap();
        let job = BridgeJob {
            slug: "field-folder".into(),
            definition_json: def.to_string(),
            selected_files: vec![photo],
            param_overrides: HashMap::from([("output:sub".to_string(), "final".to_string())]),
            preview_mode: false,
            consent: no_consent(),
        };
        let (tx, rx) = mpsc::channel();
        run_bridge_in(&tx, &job, NativeContext::current_dir().unwrap());
        let Some(BridgeEvent::Done(done)) = rx.try_iter().last() else {
            panic!("expected the run to finish");
        };
        let folder = parent.path().join("final");
        assert_eq!(done.output_dir, folder.to_string_lossy());
        assert_eq!(fs::read(folder.join("renamed-photo.txt")).unwrap(), b"new");
    }

    // --- Progressive loop output ---

    /// Inputs `a.txt` (`AAAA`) and `b.txt` (`BBBB`) in one folder.
    fn ab_inputs() -> (tempfile::TempDir, Vec<PathBuf>) {
        let inputs = tempfile::tempdir().unwrap();
        let files = [("a.txt", "AAAA"), ("b.txt", "BBBB")].map(|(name, body)| {
            let path = inputs.path().join(name);
            fs::write(&path, body).unwrap();
            path
        });
        (inputs, files.to_vec())
    }

    /// A progressive loop running `inner`, then `after` (if any), then `output`.
    fn progressive_recipe_json(
        inner: serde_json::Value,
        after: Option<serde_json::Value>,
        output: serde_json::Value,
    ) -> String {
        let mut nodes = vec![
            serde_json::json!({ "id": "input", "type": "input",
                "parameters": { "mode": "file-upload" } }),
            serde_json::json!({ "id": "each", "type": "loop", "nodes": [inner], "edges": [],
                "parameters": { "mode": "forEach", "outputPersistence": "progressive" } }),
        ];
        nodes.extend(after);
        nodes.push(serde_json::json!({ "id": "output", "type": "output", "parameters": output }));
        serde_json::json!({ "nodes": nodes }).to_string()
    }

    /// `a` renamed to `b`, so both results are `b.txt`.
    fn a_to_b() -> serde_json::Value {
        serde_json::json!({ "id": "ren", "type": "file-rename",
            "parameters": { "find": "a", "replace": "b" } })
    }

    /// Write mode into `folder`, which already holds the user's `b_1.txt`.
    fn write_beside_users_file(folder: &std::path::Path) -> serde_json::Value {
        fs::write(folder.join("b_1.txt"), "CCCC").unwrap();
        serde_json::json!({ "mode": "write", "directory": folder.to_str().unwrap() })
    }

    /// Each file in `dir` with its content, sorted by name.
    fn contents(dir: &std::path::Path) -> Vec<(String, String)> {
        let mut files: Vec<(String, String)> = fs::read_dir(dir)
            .unwrap()
            .map(|entry| {
                let path = entry.unwrap().path();
                let name = path.file_name().unwrap().to_string_lossy().into_owned();
                (name, fs::read_to_string(&path).unwrap())
            })
            .collect();
        files.sort();
        files
    }

    fn pairs(files: &[(&str, &str)]) -> Vec<(String, String)> {
        files
            .iter()
            .map(|(n, c)| (n.to_string(), c.to_string()))
            .collect()
    }

    #[test]
    fn a_progressive_runs_list_matches_its_files_and_keeps_the_users() {
        // RED: the second `b.txt` was listed and written as `b_1.txt`, over
        // the user's file. Now the loop's copies, the listing and the final
        // write agree on `b_2.txt`, and `b_1.txt` is neither replaced nor
        // listed.
        let (_inputs, files) = ab_inputs();
        let folder = tempfile::tempdir().unwrap();
        let output = write_beside_users_file(folder.path());
        let json = progressive_recipe_json(a_to_b(), None, output);

        let done = run_job("progressive", json, files);

        assert_eq!(listed(&done), ["b.txt", "b_2.txt"]);
        let expected = [("b.txt", "AAAA"), ("b_1.txt", "CCCC"), ("b_2.txt", "BBBB")];
        assert_eq!(contents(folder.path()), pairs(&expected));
    }

    #[test]
    fn a_progressive_loops_copies_are_written_by_the_bridge() {
        // The bridge passes the output folder to its reporter, so the loop's
        // copies land as it runs: `b.txt` and `b_2.txt`, under the loop's
        // names. A step after the loop renames the results to `b-x…`, so
        // only those come from the final write and the listing.
        let (_inputs, files) = ab_inputs();
        let folder = tempfile::tempdir().unwrap();
        let output = write_beside_users_file(folder.path());
        let suffix = serde_json::json!({ "id": "x", "type": "file-rename",
            "parameters": { "suffix": "-x" } });
        let json = progressive_recipe_json(a_to_b(), Some(suffix), output);

        let done = run_job("progressive-after", json, files);

        assert_eq!(listed(&done), ["b-x.txt", "b-x_1.txt"]);
        let expected = [
            ("b-x.txt", "AAAA"),
            ("b-x_1.txt", "BBBB"),
            ("b.txt", "AAAA"),
            ("b_1.txt", "CCCC"),
            ("b_2.txt", "BBBB"),
        ];
        assert_eq!(contents(folder.path()), pairs(&expected));
    }

    #[test]
    fn an_overwrite_run_makes_no_progressive_copies() {
        // As in `bnto run`: overwrite renames each file in its own folder
        // after the run, so the loop copies nothing out as it goes.
        let (inputs, files) = ab_inputs();
        let prefix = serde_json::json!({ "id": "ren", "type": "file-rename",
            "parameters": { "prefix": "renamed-" } });
        let output = serde_json::json!({ "mode": "overwrite" });
        let json = progressive_recipe_json(prefix, None, output);

        run_job("progressive-overwrite", json, files);

        let expected = [("renamed-a.txt", "AAAA"), ("renamed-b.txt", "BBBB")];
        assert_eq!(contents(inputs.path()), pairs(&expected));
    }

    #[test]
    fn a_rename_onto_another_inputs_name_keeps_both_contents() {
        // ISS-48 through the TUI's bridge: `a` renamed to `b.txt`, written
        // beside `b.txt`, must not replace `b` before `b` is read.
        let folder = tempfile::tempdir().unwrap();
        let (a, b) = (folder.path().join("a.txt"), folder.path().join("b.txt"));
        fs::write(&a, b"AAAA").unwrap();
        fs::write(&b, b"BBBB").unwrap();
        let recipe = serde_json::json!({ "nodes": [
            { "id": "input", "type": "input", "parameters": { "mode": "file-upload" } },
            { "id": "ren", "type": "file-rename", "parameters": { "find": "a", "replace": "b" } },
            { "id": "output", "type": "output", "parameters": {
                "mode": "write", "directory": folder.path().to_string_lossy() } }
        ] });
        run_job("swap", recipe.to_string(), vec![a.clone(), b.clone()]);
        assert_eq!(fs::read(&a).unwrap(), b"AAAA");
        assert_eq!(fs::read(&b).unwrap(), b"AAAA");
        assert_eq!(fs::read(folder.path().join("b_1.txt")).unwrap(), b"BBBB");
    }

    #[cfg(unix)]
    #[test]
    fn a_progressive_refusal_names_what_iterations_wrote() {
        // `out/sub` → `.`: iteration 1 writes `sub/b.txt` (= `b.txt`), then
        // the final write sees the conflict. The bridge thread makes the
        // progressive copies before the final write, so the refusal can name
        // them and must not say "nothing was written".
        let inputs = tempfile::tempdir().unwrap();
        let (a, b) = (inputs.path().join("a.txt"), inputs.path().join("b.txt"));
        fs::write(&a, b"AAAA").unwrap();
        fs::write(&b, b"BBBB").unwrap();
        let out = tempfile::tempdir().unwrap();
        std::os::unix::fs::symlink(".", out.path().join("sub")).unwrap();
        let rename = serde_json::json!({ "id": "ren", "type": "file-rename",
            "parameters": { "find": "a", "replace": "sub/b" } });
        let recipe = serde_json::json!({ "nodes": [
            { "id": "input", "type": "input", "parameters": { "mode": "file-upload" } },
            { "id": "each", "type": "loop",
              "parameters": { "mode": "forEach", "outputPersistence": "progressive" },
              "nodes": [rename], "edges": [] },
            { "id": "output", "type": "output", "parameters": {
                "mode": "write", "directory": out.path().to_string_lossy() } }
        ], "edges": [
            { "id": "e1", "source": "input", "target": "each" },
            { "id": "e2", "source": "each", "target": "output" }
        ] });
        let job = BridgeJob {
            slug: "progressive".into(),
            definition_json: recipe.to_string(),
            selected_files: vec![a, b],
            param_overrides: HashMap::new(),
            preview_mode: false,
            consent: no_consent(),
        };
        let (tx, rx) = mpsc::channel();
        run_bridge_in(&tx, &job, NativeContext::current_dir().unwrap());
        let Some(BridgeEvent::Error(error)) = rx.try_iter().last() else {
            panic!("expected the final write to be refused");
        };
        assert!(!error.contains("nothing was written"), "{error}");
        assert!(error.contains("Loop iterations already wrote"), "{error}");
        assert!(error.contains("sub/b.txt"), "{error}");
    }

    #[test]
    fn the_results_list_only_this_runs_outputs() {
        // What the wipe used to protect: files already in the folder must
        // not show up as this run's results.
        let folder = tempfile::tempdir().unwrap();
        fs::write(folder.path().join("notes.txt"), b"mine").unwrap();
        let done = run_on_photo("listing", rename_recipe_json(Some(folder.path())));
        assert_eq!(listed(&done), ["renamed-photo.txt"]);
    }

    /// The names the results screen would list for `done`.
    fn listed(done: &RunDone) -> Vec<String> {
        build_output_files(&done.file_metadata)
            .into_iter()
            .map(|f| f.name)
            .collect()
    }

    /// Rename each file to `renamed-<name>` in place, optionally collecting
    /// a picked folder's files first.
    fn overwrite_recipe_json(collect: bool) -> String {
        let mut nodes = vec![serde_json::json!(
            { "id": "input", "type": "input", "parameters": { "mode": "file-upload" } }
        )];
        if collect {
            nodes.push(serde_json::json!({ "id": "collect", "type": "file-collect",
                "parameters": { "pattern": "*", "recursive": false } }));
        }
        nodes.push(serde_json::json!({ "id": "rename", "type": "file-rename",
            "parameters": { "prefix": "renamed-" } }));
        nodes.push(serde_json::json!({ "id": "output", "type": "output",
            "parameters": { "mode": "overwrite" } }));
        serde_json::json!({ "nodes": nodes }).to_string()
    }

    #[test]
    fn overwrite_results_list_the_files_renamed_in_a_picked_folder() {
        // Overwrite renames in each file's own folder; a picked folder's
        // files sit inside it, not beside it.
        let parent = tempfile::tempdir().unwrap();
        let picked = parent.path().join("photos");
        fs::create_dir(&picked).unwrap();
        fs::write(picked.join("a.txt"), b"a").unwrap();
        fs::write(picked.join("b.txt"), b"b").unwrap();
        let done = run_job(
            "ow-folder",
            overwrite_recipe_json(true),
            vec![picked.clone()],
        );
        assert!(
            picked.join("renamed-a.txt").exists(),
            "the run renamed nothing"
        );
        let mut names = listed(&done);
        names.sort();
        assert_eq!(names, ["renamed-a.txt", "renamed-b.txt"]);
    }

    #[test]
    fn overwrite_results_list_files_renamed_in_two_folders() {
        // Both `a.txt`s become `renamed-a.txt`, each in its own folder. A
        // `renamed-a_1.txt` beside the first is the user's, not a result.
        let (one, two) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
        for dir in [&one, &two] {
            fs::write(dir.path().join("a.txt"), b"a").unwrap();
        }
        fs::write(one.path().join("renamed-a_1.txt"), b"mine").unwrap();
        let files = vec![one.path().join("a.txt"), two.path().join("a.txt")];
        let done = run_job("ow-two", overwrite_recipe_json(false), files);
        assert!(
            two.path().join("renamed-a.txt").exists(),
            "the run renamed nothing"
        );
        assert_eq!(listed(&done), ["renamed-a.txt", "renamed-a.txt"]);
        // Open on each row opens that row's file, not the first folder's.
        assert!(done.in_place);
        let outputs = build_output_files(&done.file_metadata);
        let mut results = ResultsModel::new("ow-two", outputs, 0, None);
        for (row, dir) in [one.path(), two.path()].into_iter().enumerate() {
            results.cursor = row;
            let opened = results.selected_path().unwrap();
            let expected = dir.join("renamed-a.txt");
            assert_eq!(
                fs::canonicalize(opened).unwrap(),
                fs::canonicalize(expected).unwrap()
            );
        }
        let line = results.location_line().unwrap();
        assert_eq!(line, "Written in place, in 2 folders");
    }

    #[test]
    fn a_message_run_lists_no_results_from_its_folder() {
        // Message mode writes nothing, so a file already in the recipe's
        // folder under an output's name is not this run's.
        let folder = tempfile::tempdir().unwrap();
        fs::write(folder.path().join("renamed-photo.txt"), b"mine").unwrap();
        let json = rename_recipe_json(Some(folder.path())).replace("\"write\"", "\"message\"");
        let done = run_on_photo("message", json);
        assert_eq!(listed(&done), Vec::<String>::new());
    }

    #[test]
    fn a_recipe_without_a_folder_gets_an_emptied_bnto_temp_folder() {
        // Only bnto's own temp folder is cleared, so it holds one run.
        let slug = format!("iss21-temp-{}", std::process::id());
        let temp = std::env::temp_dir().join(format!("bnto-tui-{slug}"));
        fs::create_dir_all(&temp).unwrap();
        fs::write(temp.join("stale.txt"), b"last run").unwrap();
        let done = run_on_photo(&slug, rename_recipe_json(None));
        assert_eq!(std::path::Path::new(&done.output_dir), temp);
        assert!(
            !temp.join("stale.txt").exists(),
            "the last run's file stayed"
        );
        assert!(temp.join("renamed-photo.txt").exists());
        let _ = fs::remove_dir_all(&temp);
    }

    #[test]
    fn extract_metadata_reads_original_size() {
        let mut metadata = serde_json::Map::new();
        metadata.insert(
            "originalSize".to_string(),
            serde_json::Value::Number(780_000.into()),
        );
        let result = bnto_core::pipeline::PipelineResult {
            files: vec![bnto_core::pipeline::PipelineFileResult {
                name: "photo.jpg".into(),
                data: FileData::Bytes(vec![]),
                mime_type: "image/jpeg".into(),
                metadata,
            }],
            duration_ms: 100,
            warnings: Vec::new(),
            node_metadata: Default::default(),
        };
        let meta = extract_file_metadata(&result, "write", "out");
        assert_eq!(meta.len(), 1);
        assert_eq!(meta[0].name, "photo.jpg");
        assert_eq!(meta[0].original_size, Some(780_000));
    }

    fn result_with(metadata: serde_json::Value) -> bnto_core::pipeline::PipelineResult {
        bnto_core::pipeline::PipelineResult {
            files: vec![bnto_core::pipeline::PipelineFileResult {
                name: "thumb.webp".into(),
                data: FileData::Bytes(vec![]),
                mime_type: "image/webp".into(),
                metadata: metadata.as_object().unwrap().clone(),
            }],
            duration_ms: 100,
            warnings: Vec::new(),
            node_metadata: Default::default(),
        }
    }

    #[test]
    fn extract_metadata_prefers_the_input_size() {
        // resize → convert: convert's originalSize is the resized image.
        let result = result_with(serde_json::json!({ "originalSize": 300, "inputSize": 900 }));
        assert_eq!(
            extract_file_metadata(&result, "write", "out")[0].original_size,
            Some(900)
        );
    }

    #[test]
    fn extract_metadata_ignores_an_input_size_alone() {
        // No node reported sizes (a rename), so no comparison is shown.
        let result = result_with(serde_json::json!({ "inputSize": 900 }));
        assert_eq!(
            extract_file_metadata(&result, "write", "out")[0].original_size,
            None
        );
    }

    #[test]
    fn extract_metadata_handles_missing() {
        let result = bnto_core::pipeline::PipelineResult {
            files: vec![bnto_core::pipeline::PipelineFileResult {
                name: "file.txt".into(),
                data: FileData::Bytes(vec![]),
                mime_type: "text/plain".into(),
                metadata: serde_json::Map::new(),
            }],
            duration_ms: 50,
            warnings: Vec::new(),
            node_metadata: Default::default(),
        };
        let meta = extract_file_metadata(&result, "write", "out");
        assert_eq!(meta[0].original_size, None);
    }

    #[test]
    fn extract_metadata_handles_non_numeric() {
        let mut metadata = serde_json::Map::new();
        metadata.insert(
            "originalSize".to_string(),
            serde_json::Value::String("not a number".into()),
        );
        let result = bnto_core::pipeline::PipelineResult {
            files: vec![bnto_core::pipeline::PipelineFileResult {
                name: "file.txt".into(),
                data: FileData::Bytes(vec![]),
                mime_type: "text/plain".into(),
                metadata,
            }],
            duration_ms: 50,
            warnings: Vec::new(),
            node_metadata: Default::default(),
        };
        let meta = extract_file_metadata(&result, "write", "out");
        assert_eq!(meta[0].original_size, None);
    }

    #[test]
    fn build_output_files_merges_metadata() {
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().to_path_buf();
        fs::write(dir.join("photo.jpg"), vec![0u8; 290]).unwrap();

        let meta = vec![FileResultMeta {
            name: "photo.jpg".into(),
            path: dir.join("photo.jpg"),
            original_size: Some(780),
        }];
        let outputs = build_output_files(&meta);
        assert_eq!(outputs.len(), 1);
        assert_eq!(outputs[0].original_size, Some(780));
        assert_eq!(outputs[0].size_bytes, 290);
    }

    #[test]
    fn extract_override_value_finds_by_param_name() {
        let mut overrides = HashMap::new();
        overrides.insert("download:url".into(), "https://example.com".into());
        overrides.insert("download:format".into(), "mp4".into());

        let url = extract_override_value(&mut overrides, "url");
        assert_eq!(url, Some("https://example.com".into()));
        // The key should be removed from overrides.
        assert!(!overrides.contains_key("download:url"));
        // Other keys remain.
        assert!(overrides.contains_key("download:format"));
    }

    #[test]
    fn extract_override_value_returns_none_when_missing() {
        let mut overrides = HashMap::new();
        overrides.insert("download:format".into(), "mp4".into());

        let text = extract_override_value(&mut overrides, "text");
        assert_eq!(text, None);
        assert_eq!(overrides.len(), 1);
    }

    #[test]
    fn build_output_files_skips_files_the_run_did_not_write() {
        // `other.txt` was already in the folder; `different.txt` never landed.
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().to_path_buf();
        fs::write(dir.join("other.txt"), vec![0u8; 100]).unwrap();

        let meta = vec![FileResultMeta {
            name: "different.txt".into(),
            path: dir.join("different.txt"),
            original_size: Some(500),
        }];
        let outputs = build_output_files(&meta);
        assert!(outputs.is_empty(), "{outputs:?}");
    }

    // --- Checks before consent, then consent (ISS-77, ISS-78) ---

    /// A shell step (`git`, on every CI runner; `echo` is no program on
    /// Windows) with the output folder `directory`.
    fn shell_recipe_json(directory: &std::path::Path) -> String {
        serde_json::json!({ "nodes": [
            { "id": "input", "type": "input", "parameters": { "mode": "none" } },
            { "id": "say", "type": "shell-run",
              "parameters": { "command": "git", "args": ["--version"] } },
            { "id": "output", "type": "output", "parameters": {
                "mode": "write", "directory": directory.to_string_lossy() } }
        ] })
        .to_string()
    }

    /// A job for a recipe nobody has approved, with an empty trust store.
    fn untrusted_job(definition_json: String, trust: &tempfile::TempDir) -> BridgeJob {
        BridgeJob {
            slug: "probe".into(),
            definition_json,
            selected_files: Vec::new(),
            param_overrides: HashMap::new(),
            preview_mode: false,
            consent: ConsentCheck {
                bundled: false,
                trusted_file: trust.path().join("trusted.json"),
            },
        }
    }

    /// Start `job` on its own thread: it blocks while it waits for consent.
    fn start(job: BridgeJob) -> (mpsc::Receiver<BridgeEvent>, std::thread::JoinHandle<()>) {
        let (tx, rx) = mpsc::channel();
        let handle = std::thread::spawn(move || {
            run_bridge_in(&tx, &job, NativeContext::current_dir().unwrap());
        });
        (rx, handle)
    }

    #[test]
    fn a_refused_run_reports_its_error_without_asking_for_consent() {
        let trust = tempfile::tempdir().unwrap();
        let out = trust.path().join("{{env.BNTO_ISS77_UNDECLARED}}");
        let (rx, handle) = start(untrusted_job(shell_recipe_json(&out), &trust));
        handle.join().unwrap();
        let events: Vec<BridgeEvent> = rx.try_iter().collect();
        assert!(
            matches!(&events[..], [BridgeEvent::Error(e)] if e.contains("BNTO_ISS77_UNDECLARED")),
            "{events:?}"
        );
    }

    #[test]
    fn a_checked_run_waits_for_consent_and_stops_when_it_is_dropped() {
        let trust = tempfile::tempdir().unwrap();
        let out = trust.path().join("out");
        let (rx, handle) = start(untrusted_job(shell_recipe_json(&out), &trust));
        let Ok(BridgeEvent::ConsentNeeded { request, reply }) = rx.recv() else {
            panic!("expected a request for consent");
        };
        assert_eq!(request.commands, vec!["git --version"]);
        assert!(!out.exists(), "the folder was made before consent");
        drop(reply);
        handle.join().unwrap();
        assert!(
            rx.try_iter().next().is_none(),
            "a declined run reported back"
        );
        assert!(!out.exists(), "a declined run made its folder");
    }

    #[test]
    fn an_allowed_run_goes_on_to_finish() {
        let trust = tempfile::tempdir().unwrap();
        let out = trust.path().join("out");
        let (rx, handle) = start(untrusted_job(shell_recipe_json(&out), &trust));
        let Ok(BridgeEvent::ConsentNeeded { reply, .. }) = rx.recv() else {
            panic!("expected a request for consent");
        };
        reply.send(()).unwrap();
        handle.join().unwrap();
        let last = rx.try_iter().last();
        assert!(matches!(last, Some(BridgeEvent::Done(_))), "{last:?}");
    }

    /// A rename whose prefix reads `{{vars.<key>}}`.
    fn vars_prefix_recipe_json(key: &str) -> String {
        serde_json::json!({ "nodes": [
            { "id": "input", "type": "input", "parameters": { "mode": "file-upload" } },
            { "id": "rename", "type": "file-rename",
              "parameters": { "prefix": format!("{{{{vars.{key}}}}}-") } },
            { "id": "output", "type": "output", "parameters": { "mode": "write" } }
        ] })
        .to_string()
    }

    #[test]
    fn a_var_is_filled_from_its_bnto_var_environment_variable() {
        // SAFETY: set_var races any concurrent getenv in this test binary;
        // like the repo's other env tests, it uses a name nothing else reads.
        unsafe { std::env::set_var("BNTO_VAR_ISS78PREFIX", "filled") };
        let done = run_on_photo("vars", vars_prefix_recipe_json("Iss78Prefix"));
        assert_eq!(listed(&done), ["filled-photo.txt"]);
    }

    #[test]
    fn consent_shows_and_pins_the_filled_value_of_a_var() {
        // SEC-29: the prompt shows what runs, and the approval is pinned to
        // it, so a changed value asks again.
        // SAFETY: as above, a name nothing else reads.
        unsafe { std::env::set_var("BNTO_VAR_ISS78ARG", "--version") };
        let trust = tempfile::tempdir().unwrap();
        let json = serde_json::json!({ "nodes": [
            { "id": "input", "type": "input", "parameters": { "mode": "none" } },
            { "id": "say", "type": "shell-run",
              "parameters": { "command": "git", "args": ["{{vars.Iss78Arg}}"] } }
        ] })
        .to_string();
        let (rx, handle) = start(untrusted_job(json.clone(), &trust));
        let Ok(BridgeEvent::ConsentNeeded { request, reply }) = rx.recv() else {
            panic!("expected a request for consent");
        };
        assert_eq!(request.commands, vec!["git --version"]);
        let filled = crate::vars::apply(&json, &[]).unwrap();
        assert_eq!(request.hash, crate::trust::definition_hash(&filled));
        drop(reply);
        handle.join().unwrap();
    }

    /// An untrusted shell step whose arg is `arg`, with a field `title` that
    /// has no default (so the run needs it filled).
    fn shell_reading_json(arg: &str) -> String {
        serde_json::json!({ "nodes": [
            { "id": "input", "type": "input", "parameters": { "mode": "none" } },
            { "id": "say", "type": "shell-run",
              "fields": { "title": { "type": "string", "label": "Title" } },
              "parameters": { "command": "git", "args": [arg] } }
        ] })
        .to_string()
    }

    /// The events a run of `job` sends until its bridge ends. Fails if it
    /// asks for consent; failing drops the request's reply, which stops it.
    fn events_of(job: BridgeJob) -> Vec<BridgeEvent> {
        let (rx, handle) = start(job);
        let events = rx
            .into_iter()
            .inspect(|e| {
                assert!(
                    !matches!(e, BridgeEvent::ConsentNeeded { .. }),
                    "asked first"
                )
            })
            .collect();
        handle.join().unwrap();
        events
    }

    #[test]
    fn an_unset_var_is_refused_before_consent() {
        // A shell step reads it, so a check after consent would ask first.
        let trust = tempfile::tempdir().unwrap();
        let json = shell_reading_json("{{vars.Iss78Unset}}");
        let events = events_of(untrusted_job(json, &trust));
        assert!(
            matches!(&events[..], [BridgeEvent::Error(e)] if e.contains("{{vars.Iss78Unset}}")),
            "{events:?}"
        );
    }

    #[test]
    fn an_empty_field_is_refused_before_consent() {
        let trust = tempfile::tempdir().unwrap();
        let events = events_of(untrusted_job(
            shell_reading_json("{{fields.title}}"),
            &trust,
        ));
        assert!(
            matches!(&events[..], [BridgeEvent::Error(e)]
                if e.contains("Title is empty: fill it in on the recipe's form")
                    && !e.contains("-p")),
            "{events:?}"
        );
    }

    #[test]
    fn an_empty_required_param_is_refused_before_consent() {
        // The shell step needs consent; the overlay step has no image (ISS-74).
        let trust = tempfile::tempdir().unwrap();
        let json = serde_json::json!({ "nodes": [
            { "id": "input", "type": "input", "parameters": { "mode": "none" } },
            { "id": "say", "type": "shell-run",
              "parameters": { "command": "git", "args": ["--version"] } },
            { "id": "stamp", "type": "image-overlay", "parameters": { "overlay": "" } }
        ] })
        .to_string();
        let events = events_of(untrusted_job(json, &trust));
        assert!(
            matches!(&events[..], [BridgeEvent::Error(e)]
                if e.contains("Overlay Image is empty: fill it in on the recipe's form")
                    && !e.contains("-p")),
            "{events:?}"
        );
    }
}
