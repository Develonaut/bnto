// `bnto install <recipe>` — auto-install recipe dependencies.
//
// Plans one step per missing dependency (host `installHints` command, else
// `<pm> install <binary>`, else a manual hint), prints the exact commands,
// prompts for confirmation, runs them, and verifies success.

use std::io::BufRead;

use bnto_core::context::ProcessContext;
use bnto_core::metadata::HostPlatform;
use bnto_engine::deps::DependencyStatus;
use colored::Colorize;

use crate::catalog::RecipeCatalog;
use crate::context::{self, NativeContext};
use crate::install_plan::InstallStep;
use crate::install_trust;
use crate::package_manager;

/// A missing dependency paired with how it will be installed.
struct PlannedStep {
    binary: String,
    step: InstallStep,
}

/// Whether the user declined, or which binaries failed to install.
#[derive(Debug, PartialEq)]
enum RunOutcome {
    Declined,
    Ran { failed: Vec<String> },
}

/// Run the install command for a single recipe or all recipes.
pub fn run_install(recipe_slug: Option<&str>, yes: bool, all: bool, catalog: &RecipeCatalog) {
    let registry = bnto_engine::create_registry();
    let ctx = crate::unwrap_or_exit(context::NativeContext::current_dir());

    let (deps, label) = collect_deps(recipe_slug, all, &registry, catalog);
    if deps.is_empty() {
        // "on Linux": another OS may still need something (`requires` on a
        // platform-guarded step).
        let here =
            HostPlatform::current().map_or(String::new(), |os| format!(" on {}", os.label()));
        println!(
            "{} {} has no external dependencies{here}.",
            "ok".green().bold(),
            label
        );
        return;
    }
    let statuses = bnto_engine::deps::check_dependencies(&deps, &ctx);
    let trusted = install_trust::hints_trusted(recipe_slug, all, catalog);
    install_missing(&statuses, &label, yes, trusted, &ctx);
}

/// Plan, confirm, install and verify whichever dependencies are missing.
#[expect(clippy::too_many_lines)]
fn install_missing(
    statuses: &[DependencyStatus],
    label: &str,
    yes: bool,
    trusted: bool,
    ctx: &NativeContext,
) {
    let missing: Vec<_> = statuses.iter().filter(|s| !s.found).collect();
    if missing.is_empty() {
        println!(
            "{} All dependencies for {} are already installed.",
            "ok".green().bold(),
            label
        );
        return;
    }
    let steps = plan_steps(
        &missing,
        HostPlatform::current(),
        package_manager::detect(ctx),
    );
    print_install_plan(&steps, label);
    let runnable = require_runnable(steps);
    let ask = should_ask(yes, trusted, &runnable);
    match confirm_and_run(&runnable, ask, std::io::stdin().lock(), ctx) {
        RunOutcome::Declined => exit_cancelled(),
        RunOutcome::Ran { failed } if !failed.is_empty() => exit_install_failed(&failed),
        RunOutcome::Ran { .. } => verify_installs(&missing, ctx),
    }
}

/// Whether to prompt; explains when `--yes` is overridden by an untrusted hint.
fn should_ask(yes: bool, trusted: bool, runnable: &[PlannedStep]) -> bool {
    let commands: Vec<InstallStep> = runnable.iter().map(|p| p.step.clone()).collect();
    let ask = install_trust::needs_prompt(yes, trusted, &commands);
    if ask && yes {
        warn_untrusted_yes();
    }
    ask
}

/// One step per missing dependency, in plan order.
fn plan_steps(
    missing: &[&DependencyStatus],
    platform: Option<HostPlatform>,
    pm: Option<package_manager::PackageManager>,
) -> Vec<PlannedStep> {
    missing
        .iter()
        .map(|status| PlannedStep {
            binary: status.dependency.binary.clone(),
            step: InstallStep::plan(&status.dependency, platform, pm),
        })
        .collect()
}

/// Resolve which dependencies to check — single recipe or all recipes.
fn collect_deps(
    recipe_slug: Option<&str>,
    all: bool,
    registry: &bnto_core::NodeRegistry,
    catalog: &RecipeCatalog,
) -> (Vec<bnto_core::metadata::Dependency>, String) {
    if all {
        let deps = collect_all_recipe_deps(registry, catalog);
        return (deps, "all recipes".to_string());
    }
    let slug = recipe_slug.unwrap_or_else(|| exit_missing_slug());
    let recipe_json = read_recipe_json(slug, catalog);
    let def: bnto_core::PipelineDefinition = crate::unwrap_or_exit(
        serde_json::from_str(&recipe_json).map_err(|e| format!("Invalid recipe JSON: {e}")),
    );
    let deps =
        bnto_engine::deps::collect_pipeline_dependencies(&def, registry, HostPlatform::current());
    (deps, slug.to_string())
}

/// Explain that a recipe slug (or `--all`) is required, then exit.
fn exit_missing_slug() -> ! {
    eprintln!(
        "{} Provide a recipe slug or use {}.",
        "Error:".red(),
        "--all".cyan()
    );
    eprintln!(
        "Usage: {} or {}",
        "bnto install <recipe>".cyan(),
        "bnto install --all".cyan()
    );
    std::process::exit(1);
}

/// Dependencies every catalog recipe needs on this OS — `requires` plus
/// processor-declared deps, deduplicated by binary.
fn collect_all_recipe_deps(
    registry: &bnto_core::NodeRegistry,
    catalog: &RecipeCatalog,
) -> Vec<bnto_core::metadata::Dependency> {
    bnto_engine::deps::collect_catalog_dependencies(
        registry,
        &catalog.definitions(),
        HostPlatform::current(),
    )
}

/// Print every planned step — the exact command that will run, or the
/// manual hint — so nothing runs without the user having seen it.
fn print_install_plan(steps: &[PlannedStep], label: &str) {
    println!(
        "
{} {} missing {} for {}:
",
        "Install".bold(),
        steps.len(),
        dependency_noun(steps.len()),
        label
    );
    for planned in steps {
        println!("  {} {}", "→".cyan(), plan_line(planned));
    }
    println!();
}

/// Plan text for one step: the command, or `binary (manual hint)`.
fn plan_line(planned: &PlannedStep) -> String {
    match &planned.step {
        InstallStep::Manual(hint) => format!("{} ({hint})", planned.binary),
        step => step.display(),
    }
}

/// "dependency" or "dependencies" for `count`.
fn dependency_noun(count: usize) -> &'static str {
    if count == 1 {
        "dependency"
    } else {
        "dependencies"
    }
}

/// Keep the runnable steps; exit when there is nothing `bnto` can run.
fn require_runnable(steps: Vec<PlannedStep>) -> Vec<PlannedStep> {
    let runnable: Vec<_> = steps.into_iter().filter(|p| p.step.is_runnable()).collect();
    if runnable.is_empty() {
        eprintln!(
            "{} No install command for this platform and no supported package manager detected (brew, apt, pacman, choco).",
            "Warning:".yellow()
        );
        eprintln!("Install the dependencies manually using the hints above.");
        std::process::exit(1);
    }
    runnable
}

/// Explain why `--yes` still prompts: a non-bundled recipe wrote a command.
fn warn_untrusted_yes() {
    eprintln!(
        "{} --yes does not auto-run install commands from recipes that aren't bundled with bnto. Review the commands above.",
        "Note:".yellow()
    );
}

/// Prompt (when `ask`), then run each step. Declining runs nothing.
fn confirm_and_run(
    steps: &[PlannedStep],
    ask: bool,
    reader: impl BufRead,
    ctx: &dyn ProcessContext,
) -> RunOutcome {
    if ask {
        eprint!(
            "Run the install {} above? [Y/n] ",
            command_noun(steps.len())
        );
        if !confirm_yes(reader) {
            return RunOutcome::Declined;
        }
    }
    let failed = steps
        .iter()
        .filter(|planned| !install_one(ctx, planned))
        .map(|planned| planned.binary.clone())
        .collect();
    RunOutcome::Ran { failed }
}

/// "command" or "commands" for `count`.
fn command_noun(count: usize) -> &'static str {
    if count == 1 { "command" } else { "commands" }
}

/// Run one step, echoing its exact command. Returns true on success.
fn install_one(ctx: &dyn ProcessContext, planned: &PlannedStep) -> bool {
    eprintln!("{} {}", "Running:".dimmed(), planned.step.display());
    match planned.step.run(ctx) {
        Ok(()) => true,
        Err(e) => {
            eprintln!("  {} {e}", "failed:".red());
            false
        }
    }
}

/// Report that the user declined, then exit without installing anything.
fn exit_cancelled() -> ! {
    eprintln!("Cancelled.");
    std::process::exit(1);
}

/// Report which installs failed, then exit.
fn exit_install_failed(failed: &[String]) -> ! {
    eprintln!(
        "
{} {} {} failed to install: {}",
        "Error:".red(),
        failed.len(),
        dependency_noun(failed.len()),
        failed.join(", ")
    );
    std::process::exit(1);
}

/// Re-check that previously missing dependencies are now found.
fn verify_installs(missing: &[&DependencyStatus], ctx: &dyn ProcessContext) {
    let deps: Vec<_> = missing.iter().map(|s| s.dependency.clone()).collect();
    let recheck = bnto_engine::deps::check_dependencies(&deps, ctx);
    let still_missing: Vec<_> = recheck.iter().filter(|s| !s.found).collect();
    if !still_missing.is_empty() {
        exit_still_missing(&still_missing);
    }
    println!(
        "
{} All dependencies installed successfully.",
        "ok".green().bold()
    );
}

/// List dependencies that are still missing after install, then exit.
fn exit_still_missing(still_missing: &[&DependencyStatus]) -> ! {
    eprintln!(
        "
{} {} {} still not found after install:",
        "Warning:".yellow(),
        still_missing.len(),
        dependency_noun(still_missing.len())
    );
    for s in still_missing {
        eprintln!("  {} {}", "MISSING".red(), s.dependency.binary);
    }
    std::process::exit(1);
}

/// Look up recipe JSON — try catalog slug first, then disk path.
fn read_recipe_json(slug: &str, catalog: &RecipeCatalog) -> String {
    if let Some(entry) = catalog.resolve(slug) {
        return entry.definition_json.clone();
    }
    match std::fs::read_to_string(slug) {
        Ok(json) => json,
        Err(_) => {
            eprintln!("{} Unknown recipe: {slug}", "Error:".red());
            eprintln!("Run {} to see available recipes.", "bnto list".cyan());
            std::process::exit(1);
        }
    }
}

/// Read a single line and return true if it's empty or "y"/"yes".
///
/// EOF (closed or non-interactive stdin) declines — only an explicit empty
/// line takes the default, so piped/CI runs never install unprompted.
fn confirm_yes(mut reader: impl std::io::BufRead) -> bool {
    let mut input = String::new();
    match reader.read_line(&mut input) {
        Ok(0) | Err(_) => return false,
        Ok(_) => {}
    }
    let trimmed = input.trim();
    trimmed.is_empty() || trimmed.eq_ignore_ascii_case("y") || trimmed.eq_ignore_ascii_case("yes")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::package_manager::PackageManager;
    use bnto_core::errors::BntoError;
    use bnto_core::metadata::Dependency;
    use std::path::{Path, PathBuf};

    /// Context where every binary lookup succeeds (all deps found).
    struct AllFoundContext;

    impl ProcessContext for AllFoundContext {
        fn run_command(&self, _cmd: &str, _args: &[&str]) -> Result<Vec<u8>, BntoError> {
            Ok(b"/usr/local/bin/found".to_vec())
        }
        fn find_binary(&self, name: &str) -> Option<PathBuf> {
            Some(PathBuf::from("/usr/local/bin").join(name))
        }
        fn temp_file(&self, _suffix: &str) -> Result<PathBuf, BntoError> {
            Err(BntoError::ProcessingFailed("n/a".into()))
        }
        fn env_var(&self, _key: &str) -> Option<String> {
            None
        }
        fn work_dir(&self) -> Result<&Path, BntoError> {
            Err(BntoError::ProcessingFailed("n/a".into()))
        }
    }

    /// Context where every binary lookup fails (all deps missing).
    struct AllMissingContext;

    impl ProcessContext for AllMissingContext {
        fn run_command(&self, _cmd: &str, _args: &[&str]) -> Result<Vec<u8>, BntoError> {
            Err(BntoError::ProcessingFailed("not found".into()))
        }
        fn temp_file(&self, _suffix: &str) -> Result<PathBuf, BntoError> {
            Err(BntoError::ProcessingFailed("n/a".into()))
        }
        fn env_var(&self, _key: &str) -> Option<String> {
            None
        }
        fn work_dir(&self) -> Result<&Path, BntoError> {
            Err(BntoError::ProcessingFailed("n/a".into()))
        }
    }

    fn make_dep(binary: &str) -> Dependency {
        Dependency {
            binary: binary.to_string(),
            version: String::new(),
            install_hint: format!("brew install {binary}"),
            install_hints: Default::default(),
            homepage: String::new(),
        }
    }

    #[test]
    fn no_deps_detected() {
        let deps = vec![];
        let statuses = bnto_engine::deps::check_dependencies(&deps, &AllFoundContext);
        assert!(statuses.is_empty());
    }

    #[test]
    fn all_deps_already_satisfied() {
        let deps = vec![make_dep("ffmpeg"), make_dep("yt-dlp")];
        let statuses = bnto_engine::deps::check_dependencies(&deps, &AllFoundContext);
        let missing: Vec<_> = statuses.iter().filter(|s| !s.found).collect();
        assert!(missing.is_empty());
    }

    #[test]
    fn missing_deps_are_listed() {
        let deps = vec![make_dep("ffmpeg"), make_dep("yt-dlp")];
        let statuses = bnto_engine::deps::check_dependencies(&deps, &AllMissingContext);
        let missing: Vec<_> = statuses.iter().filter(|s| !s.found).collect();
        assert_eq!(missing.len(), 2);
        assert_eq!(missing[0].dependency.binary, "ffmpeg");
        assert_eq!(missing[1].dependency.binary, "yt-dlp");
    }

    /// Records every spawned command; never installs anything.
    #[derive(Default)]
    struct RecordingContext {
        calls: std::sync::Mutex<Vec<(String, Vec<String>)>>,
    }

    impl RecordingContext {
        fn calls(&self) -> Vec<(String, Vec<String>)> {
            self.calls.lock().unwrap().clone()
        }
    }

    impl ProcessContext for RecordingContext {
        fn run_command(&self, cmd: &str, args: &[&str]) -> Result<Vec<u8>, BntoError> {
            let args = args.iter().map(|a| a.to_string()).collect();
            self.calls.lock().unwrap().push((cmd.to_string(), args));
            Ok(Vec::new())
        }
        fn temp_file(&self, _suffix: &str) -> Result<PathBuf, BntoError> {
            Err(BntoError::ProcessingFailed("n/a".into()))
        }
        fn env_var(&self, _key: &str) -> Option<String> {
            None
        }
        fn work_dir(&self) -> Result<&Path, BntoError> {
            Err(BntoError::ProcessingFailed("n/a".into()))
        }
    }

    fn dep_with_linux_hint(binary: &str, hint: &str) -> Dependency {
        let mut dep = make_dep(binary);
        dep.install_hints.linux = hint.to_string();
        dep
    }

    fn missing(dep: Dependency) -> DependencyStatus {
        bnto_engine::deps::check_dependencies(&[dep], &AllMissingContext).remove(0)
    }

    fn planned(dep: Dependency, pm: Option<PackageManager>) -> PlannedStep {
        let status = missing(dep);
        plan_steps(&[&status], Some(HostPlatform::Linux), pm).remove(0)
    }

    fn hint_step() -> PlannedStep {
        planned(
            dep_with_linux_hint("ffmpeg", "sudo apt install ffmpeg"),
            Some(PackageManager::Brew),
        )
    }

    #[test]
    fn plan_shows_host_hint_when_present() {
        assert_eq!(plan_line(&hint_step()), "sudo apt install ffmpeg");
    }

    #[test]
    fn plan_falls_back_to_package_manager_install() {
        let step = planned(make_dep("yt-dlp"), Some(PackageManager::Brew));
        assert_eq!(plan_line(&step), "brew install yt-dlp");
    }

    #[test]
    fn plan_shows_manual_hint_without_package_manager() {
        let step = planned(make_dep("yt-dlp"), None);
        assert_eq!(plan_line(&step), "yt-dlp (brew install yt-dlp)");
        assert!(!step.step.is_runnable());
    }

    #[test]
    fn confirmed_hint_runs_exact_string_through_shell() {
        let ctx = RecordingContext::default();
        let outcome = confirm_and_run(
            &[hint_step()],
            true,
            &b"y
"[..],
            &ctx,
        );
        assert_eq!(outcome, RunOutcome::Ran { failed: vec![] });
        let (shell, flag) = if cfg!(windows) {
            ("cmd", "/C")
        } else {
            ("sh", "-c")
        };
        assert_eq!(
            ctx.calls(),
            vec![(
                shell.to_string(),
                vec![flag.to_string(), "sudo apt install ffmpeg".to_string()]
            )]
        );
    }

    #[test]
    fn declined_confirmation_runs_nothing() {
        let ctx = RecordingContext::default();
        let outcome = confirm_and_run(
            &[hint_step()],
            true,
            &b"n
"[..],
            &ctx,
        );
        assert_eq!(outcome, RunOutcome::Declined);
        assert!(ctx.calls().is_empty());
    }

    #[test]
    fn closed_stdin_runs_nothing() {
        let ctx = RecordingContext::default();
        let outcome = confirm_and_run(&[hint_step()], true, &b""[..], &ctx);
        assert_eq!(outcome, RunOutcome::Declined);
        assert!(ctx.calls().is_empty());
    }

    #[test]
    fn skipped_prompt_runs_without_reading_stdin() {
        let ctx = RecordingContext::default();
        let outcome = confirm_and_run(&[hint_step()], false, &b""[..], &ctx);
        assert_eq!(outcome, RunOutcome::Ran { failed: vec![] });
        assert_eq!(ctx.calls().len(), 1);
    }

    #[test]
    fn failed_install_is_reported_by_binary() {
        let step = planned(make_dep("yt-dlp"), Some(PackageManager::Brew));
        let outcome = confirm_and_run(&[step], false, &b""[..], &AllMissingContext);
        assert_eq!(
            outcome,
            RunOutcome::Ran {
                failed: vec!["yt-dlp".to_string()]
            }
        );
    }

    #[test]
    fn all_deps_include_recipe_level_requires() {
        // yt-dlp is declared only in download-video's recipe-level `requires`
        // (no processor declares it), so it must come from recipe scanning.
        let registry = bnto_engine::create_registry();
        let catalog = RecipeCatalog::load(Path::new("/nonexistent"));
        let deps = collect_all_recipe_deps(&registry, &catalog);
        assert!(deps.iter().any(|d| d.binary == "yt-dlp"), "got: {deps:?}");
    }

    #[test]
    fn confirm_declines_on_eof() {
        // Closed/non-interactive stdin must never auto-approve an install.
        assert!(!confirm_yes(&b""[..]));
    }

    #[test]
    fn confirm_empty_line_defaults_to_yes() {
        assert!(confirm_yes(&b"\n"[..]));
    }

    #[test]
    fn confirm_accepts_y() {
        assert!(confirm_yes(&b"y\n"[..]));
        assert!(confirm_yes(&b"YES\n"[..]));
    }

    #[test]
    fn confirm_rejects_n() {
        assert!(!confirm_yes(&b"n\n"[..]));
    }

    #[test]
    fn read_recipe_json_from_catalog() {
        let catalog = RecipeCatalog::load(Path::new("/nonexistent"));
        let json = read_recipe_json("download-video", &catalog);
        assert!(json.contains("download-video") || json.contains("shell-run"));
    }
}
