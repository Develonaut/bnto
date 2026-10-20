// `bnto run` — resolve a recipe, gate it on consent, execute it through the
// engine with progress on stderr, then write and summarize the results.

use std::sync::Arc;
use std::time::Instant;

use bnto_core::logging::{LogEntry, LogLevel, Logger};
use colored::Colorize;

use crate::catalog::RecipeCatalog;
use crate::consent::check_trust;
use crate::run_output::write_output;
use crate::run_verdict::RunVerdict;
use crate::run_watchdog::RunWatchdog;
use crate::{context, dry_run, input, progress, run_limit, telemetry, vars};

/// The `bnto run` switches that change what a run does.
#[derive(Clone, Copy)]
pub struct RunFlags<'a> {
    /// `--yes`: skip the trust consent prompt.
    pub yes: bool,
    /// `--dry-run`: run every node, write nothing.
    pub dry_run: bool,
    /// `--vars`: files of run vars for `{{vars.KEY}}`.
    pub vars: &'a [String],
    /// `--json`: also print the run summary as one JSON line on stdout.
    pub json: bool,
    /// `--timeout`: the run's time limit in seconds, over the recipe's.
    pub timeout: Option<u64>,
}

/// Input facts recorded for logging and telemetry.
struct RunStats<'a> {
    recipe_path: &'a str,
    file_count: usize,
    total_bytes: u64,
    param_names: Vec<String>,
}

/// Where and how results are written.
struct RunTarget {
    mode: String,
    output_dir: String,
    dry_run: bool,
    /// The run's time limit in seconds, if it has one.
    time_limit: Option<u64>,
}

/// Run a recipe (slug or path) against `inputs`.
pub fn run_recipe(
    recipe_path: &str,
    inputs: &[String],
    output_override: Option<&str>,
    param_overrides: &[String],
    flags: RunFlags,
    logger: &Arc<dyn Logger>,
    catalog: &RecipeCatalog,
) {
    let verdict = RunVerdict::new(flags.json);
    let raw_json = read_recipe(recipe_path, catalog, &verdict);
    check_trust(recipe_path, &raw_json, flags.yes, catalog);
    let prepared = input::prepare_inputs(&raw_json, inputs, param_overrides);
    let mut prepared = verdict.or_fail(prepared, "Error:");
    prepared.definition_json = resolve_vars(&prepared.definition_json, flags.vars, &verdict);
    let limited = run_limit::apply(&prepared.definition_json, flags.timeout);
    let (definition_json, time_limit) = verdict.or_fail(limited, "Error:");
    prepared.definition_json = definition_json;
    print_run_banner(recipe_path, &raw_json, &prepared);
    let stats = run_stats(recipe_path, &prepared, param_overrides);
    log_run_start(logger, &stats);

    let start = Instant::now();
    let ctx = context::NativeContext::current_dir().map(|c| c.with_dry_run(flags.dry_run));
    let ctx = verdict.or_fail(ctx, "Error:");
    let target = run_target(output_override, flags.dry_run, time_limit, &prepared, &ctx);
    // `ctx` lives to the end of this function: its scratch dir holds the
    // temp files the output step copies from, and dropping it deletes them.
    match execute(prepared, &ctx, &target, logger, &verdict) {
        Ok(result) => finish_run(logger, &stats, start, &result, &target, &verdict),
        Err(e) => fail_run(logger, &e, &verdict),
    }
}

/// Run the pipeline with progress on stderr, tallying nodes for the verdict.
fn execute(
    prepared: input::PreparedInput,
    ctx: &context::NativeContext,
    target: &RunTarget,
    logger: &Arc<dyn Logger>,
    verdict: &RunVerdict,
) -> Result<bnto_core::PipelineResult, bnto_core::BntoError> {
    let stderr = progress::stderr_reporter(Arc::clone(logger), progressive_dir(target));
    let reporter = verdict.tallying(stderr);
    if target.dry_run {
        dry_run::print_shell_note(&prepared.definition_json);
    }
    watched(target.time_limit, verdict, || {
        bnto_engine::run_pipeline(&prepared.definition_json, prepared.files, &reporter, ctx)
    })
}

/// `run`, watched when the run has a time limit: once the limit passes, the
/// run timed out, whatever `run` returned.
fn watched<T>(
    time_limit: Option<u64>,
    verdict: &RunVerdict,
    run: impl FnOnce() -> Result<T, bnto_core::BntoError>,
) -> Result<T, bnto_core::BntoError> {
    let Some(secs) = time_limit else {
        return run();
    };
    let watchdog = RunWatchdog::for_run(secs, verdict.clone());
    let result = run();
    if watchdog.disarm() {
        return Err(bnto_core::BntoError::TimedOut { secs });
    }
    result
}

/// Read recipe JSON — try catalog slug first, then disk path.
fn read_recipe(path: &str, catalog: &RecipeCatalog, verdict: &RunVerdict) -> String {
    if let Some(entry) = catalog.resolve(path) {
        return entry.definition_json.clone();
    }
    verdict.or_fail(std::fs::read_to_string(path), "Error reading recipe:")
}

/// Collect file count, byte total and overridden param names.
fn run_stats<'a>(
    recipe_path: &'a str,
    prepared: &input::PreparedInput,
    param_overrides: &[String],
) -> RunStats<'a> {
    RunStats {
        recipe_path,
        file_count: prepared.files.len(),
        total_bytes: prepared
            .files
            .iter()
            .map(|f| f.data.len().unwrap_or(0))
            .sum(),
        param_names: param_overrides
            .iter()
            .filter_map(|p| p.split('=').next().map(|k| k.to_string()))
            .collect(),
    }
}

/// Log the start of a run.
fn log_run_start(logger: &Arc<dyn Logger>, stats: &RunStats) {
    logger.log(LogEntry {
        level: LogLevel::Info,
        target: "cli",
        message: format!(
            "run: recipe={} files={} bytes={}",
            stats.recipe_path, stats.file_count, stats.total_bytes
        ),
        elapsed_us: None,
    });
}

/// Resolve the output mode and directory for this run.
fn run_target(
    output_override: Option<&str>,
    dry_run: bool,
    time_limit: Option<u64>,
    prepared: &input::PreparedInput,
    ctx: &context::NativeContext,
) -> RunTarget {
    let def_json = &prepared.definition_json;
    RunTarget {
        mode: resolve_mode(def_json),
        // Priority chain: explicit --output > recipe directory param > default "."
        output_dir: resolve_effective_output_dir(output_override, def_json, ctx, &prepared.files),
        dry_run,
        time_limit,
    }
}

/// Resolve the output mode from the recipe definition.
fn resolve_mode(definition_json: &str) -> String {
    serde_json::from_str::<bnto_core::PipelineDefinition>(definition_json)
        .map(|def| bnto_core::resolve_output_mode(&def))
        .unwrap_or_else(|_| "write".to_string())
}

/// Directory the progress reporter writes iteration output to, creating it
/// first. Overwrite mode keeps files in their source directories, and a dry
/// run writes nothing, so both get `None` (no progressive writes).
fn progressive_dir(target: &RunTarget) -> Option<String> {
    if target.mode == "overwrite" || target.dry_run {
        return None;
    }
    let _ = std::fs::create_dir_all(&target.output_dir);
    Some(target.output_dir.clone())
}

/// Log, report telemetry, and write results for a successful run.
fn finish_run(
    logger: &Arc<dyn Logger>,
    stats: &RunStats,
    start: Instant,
    result: &bnto_core::PipelineResult,
    target: &RunTarget,
    verdict: &RunVerdict,
) {
    logger.log(LogEntry {
        level: LogLevel::Info,
        target: "cli",
        message: format!("run complete: {} output files", result.files.len()),
        elapsed_us: Some(start.elapsed().as_micros() as u64),
    });
    telemetry::capture(telemetry::events::cli_recipe_run_with_params(
        stats.recipe_path,
        result.duration_ms,
        stats.file_count,
        stats.total_bytes,
        result.files.len(),
        true,
        &stats.param_names,
    ));
    write_output(
        result,
        &target.mode,
        &target.output_dir,
        target.dry_run,
        verdict,
    );
}

/// Resolve `{{vars.*}}` before any node runs; an unset key ends the run.
fn resolve_vars(definition_json: &str, var_paths: &[String], verdict: &RunVerdict) -> String {
    verdict.or_fail(vars::apply(definition_json, var_paths), "Pipeline failed:")
}

/// Log and report a failed run, then exit.
fn fail_run(logger: &Arc<dyn Logger>, e: &bnto_core::BntoError, verdict: &RunVerdict) -> ! {
    logger.log(LogEntry {
        level: LogLevel::Error,
        target: "cli",
        message: format!("run failed: {e}"),
        elapsed_us: None,
    });
    telemetry::capture(telemetry::events::cli_error("run", &e.to_string()));
    verdict.fail("Pipeline failed:", e)
}

/// Resolve the effective output directory using the priority chain:
/// 1. Explicit `--output` flag (highest priority)
/// 2. Recipe's output node `directory` parameter (with {{ctx.*}} + {{node.*}} templates resolved)
/// 3. Default: current directory (".")
fn resolve_effective_output_dir(
    output_override: Option<&str>,
    definition_json: &str,
    process_ctx: &dyn bnto_core::ProcessContext,
    files: &[bnto_core::PipelineFile],
) -> String {
    // Explicit --output always wins.
    if let Some(dir) = output_override {
        return dir.to_string();
    }
    // Try recipe's output node directory param.
    if let Ok(def) = serde_json::from_str::<bnto_core::PipelineDefinition>(definition_json)
        && let Some(recipe_dir) = bnto_core::resolve_output_directory(&def)
    {
        // Two-pass: resolve {{ctx.*}} first, then {{node.*}} (input file metadata).
        let resolved = bnto_core::resolve_ctx_templates(&recipe_dir, process_ctx);
        let node_outputs = bnto_core::build_node_outputs_for_input(&def.nodes, files);
        return bnto_core::resolve_node_templates(&resolved, &node_outputs);
    }
    // Default: current directory.
    ".".to_string()
}

/// Print `<recipe> with N files...` (or the input mode label).
fn print_run_banner(recipe_path: &str, raw_json: &str, prepared: &input::PreparedInput) {
    let label = input::mode_label(raw_json);
    if prepared.files.is_empty() {
        eprintln!("{} with {label}...", recipe_path.bold());
    } else {
        let count = prepared.files.len();
        eprintln!(
            "{} with {count} file{}...",
            recipe_path.bold(),
            if count == 1 { "" } else { "s" }
        );
    }
}
