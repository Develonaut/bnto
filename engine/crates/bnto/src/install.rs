// `bnto install <recipe>` — auto-install recipe dependencies.
//
// Detects the system's package manager, shows what needs to be installed,
// prompts for confirmation, runs the install commands, and verifies success.

use colored::Colorize;

use crate::context;
use crate::package_manager::{self, PackageManager};

/// Run the install command for a single recipe or all recipes.
pub fn run_install(recipe_slug: Option<&str>, yes: bool, all: bool) {
    let registry = bnto_engine::create_registry();
    let ctx = crate::unwrap_or_exit(context::NativeContext::current_dir());

    let (deps, label) = collect_deps(recipe_slug, all, &registry);

    if deps.is_empty() {
        println!(
            "{} {} has no external dependencies.",
            "ok".green().bold(),
            label
        );
        return;
    }

    let statuses = bnto_engine::deps::check_dependencies(&deps, &ctx);
    let missing: Vec<_> = statuses.iter().filter(|s| !s.found).collect();

    if missing.is_empty() {
        println!(
            "{} All dependencies for {} are already installed.",
            "ok".green().bold(),
            label
        );
        return;
    }

    let pm = package_manager::detect(&ctx);
    print_install_plan(&missing, pm, &label);

    let pm = require_package_manager(pm);
    confirm_or_exit(yes, pm);
    execute_installs(&ctx, pm, &missing);
    verify_installs(&missing, &ctx);
}

/// Resolve which dependencies to check — single recipe or all recipes.
fn collect_deps(
    recipe_slug: Option<&str>,
    all: bool,
    registry: &bnto_core::NodeRegistry,
) -> (Vec<bnto_core::metadata::Dependency>, String) {
    if all {
        let deps = bnto_engine::deps::collect_all_dependencies(registry);
        return (deps, "all recipes".to_string());
    }
    let slug = recipe_slug.unwrap_or_else(|| {
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
    });
    let recipe_json = read_recipe_json(slug);
    let def: bnto_core::PipelineDefinition = crate::unwrap_or_exit(
        serde_json::from_str(&recipe_json).map_err(|e| format!("Invalid recipe JSON: {e}")),
    );
    let deps = bnto_engine::deps::collect_pipeline_dependencies(&def, registry);
    (deps, slug.to_string())
}

/// Print the list of missing dependencies and how they'll be installed.
fn print_install_plan(
    missing: &[&bnto_engine::deps::DependencyStatus],
    pm: Option<PackageManager>,
    label: &str,
) {
    let noun = if missing.len() == 1 {
        "dependency"
    } else {
        "dependencies"
    };
    println!(
        "\n{} {} missing {} for {}:\n",
        "Install".bold(),
        missing.len(),
        noun,
        label
    );
    for status in missing {
        if let Some(pm) = pm {
            println!(
                "  {} {}",
                "→".cyan(),
                package_manager::display_command(pm, &status.dependency.binary)
            );
        } else {
            println!(
                "  {} {} ({})",
                "→".cyan(),
                status.dependency.binary,
                status.dependency.install_hint.dimmed()
            );
        }
    }
    println!();
}

/// Exit with an error if no package manager was detected.
fn require_package_manager(pm: Option<PackageManager>) -> PackageManager {
    pm.unwrap_or_else(|| {
        eprintln!(
            "{} No supported package manager detected (brew, apt, pacman, choco).",
            "Warning:".yellow()
        );
        eprintln!("Install the dependencies manually using the hints above.");
        std::process::exit(1);
    })
}

/// Prompt for confirmation unless --yes was passed.
fn confirm_or_exit(yes: bool, pm: PackageManager) {
    if !yes {
        eprint!("Install with {}? [Y/n] ", pm.name().cyan());
        if !confirm_yes() {
            eprintln!("Cancelled.");
            std::process::exit(1);
        }
    }
}

/// Run install commands for each missing dependency. Exit on failure.
fn execute_installs(
    ctx: &dyn bnto_core::context::ProcessContext,
    pm: PackageManager,
    missing: &[&bnto_engine::deps::DependencyStatus],
) {
    let mut failed = Vec::new();
    for status in missing {
        let binary = &status.dependency.binary;
        eprintln!(
            "{} {}",
            "Running:".dimmed(),
            package_manager::display_command(pm, binary)
        );
        if let Err(e) = run_install_command(ctx, pm, binary) {
            eprintln!("  {} {e}", "failed:".red());
            failed.push(binary.as_str());
        }
    }
    if !failed.is_empty() {
        let noun = if failed.len() == 1 {
            "dependency"
        } else {
            "dependencies"
        };
        eprintln!(
            "\n{} {} {} failed to install: {}",
            "Error:".red(),
            failed.len(),
            noun,
            failed.join(", ")
        );
        std::process::exit(1);
    }
}

/// Re-check that previously missing dependencies are now found.
fn verify_installs(
    missing: &[&bnto_engine::deps::DependencyStatus],
    ctx: &dyn bnto_core::context::ProcessContext,
) {
    let deps: Vec<_> = missing.iter().map(|s| s.dependency.clone()).collect();
    let recheck = bnto_engine::deps::check_dependencies(&deps, ctx);
    let still_missing: Vec<_> = recheck.iter().filter(|s| !s.found).collect();
    if still_missing.is_empty() {
        println!(
            "\n{} All dependencies installed successfully.",
            "ok".green().bold()
        );
    } else {
        let noun = if still_missing.len() == 1 {
            "dependency"
        } else {
            "dependencies"
        };
        eprintln!(
            "\n{} {} {} still not found after install:",
            "Warning:".yellow(),
            still_missing.len(),
            noun
        );
        for s in &still_missing {
            eprintln!("  {} {}", "MISSING".red(), s.dependency.binary);
        }
        std::process::exit(1);
    }
}

/// Look up recipe JSON — try built-in slug first, then disk path.
fn read_recipe_json(slug: &str) -> String {
    if let Some(recipe) = bnto_engine::recipes::builtin_recipe_by_slug(slug) {
        return recipe.definition_json.to_string();
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

/// Run a single install command via the package manager.
fn run_install_command(
    ctx: &dyn bnto_core::context::ProcessContext,
    pm: PackageManager,
    binary: &str,
) -> Result<(), String> {
    let (prog, args) = package_manager::install_command(pm, binary);
    let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    ctx.run_command(&prog, &arg_refs)
        .map(|_| ())
        .map_err(|e| e.to_string())
}

/// Read a single line from stdin and return true if it's empty or starts with 'y'/'Y'.
fn confirm_yes() -> bool {
    let mut input = String::new();
    if std::io::stdin().read_line(&mut input).is_err() {
        return false;
    }
    let trimmed = input.trim();
    trimmed.is_empty() || trimmed.eq_ignore_ascii_case("y") || trimmed.eq_ignore_ascii_case("yes")
}

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::context::ProcessContext;
    use bnto_core::errors::BntoError;
    use bnto_core::metadata::Dependency;
    use std::path::{Path, PathBuf};

    /// Context where `which` succeeds for everything (all deps found).
    struct AllFoundContext;

    impl ProcessContext for AllFoundContext {
        fn run_command(&self, _cmd: &str, _args: &[&str]) -> Result<Vec<u8>, BntoError> {
            Ok(b"/usr/local/bin/found".to_vec())
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

    /// Context where `which` fails for everything (all deps missing).
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

    #[test]
    fn run_install_command_success() {
        let result = run_install_command(&AllFoundContext, PackageManager::Brew, "yt-dlp");
        assert!(result.is_ok());
    }

    #[test]
    fn run_install_command_failure() {
        let result = run_install_command(&AllMissingContext, PackageManager::Brew, "yt-dlp");
        assert!(result.is_err());
    }

    #[test]
    fn read_recipe_json_builtin() {
        // download-video is a built-in recipe that has dependencies.
        let json = read_recipe_json("download-video");
        assert!(json.contains("download-video") || json.contains("shell-command"));
    }
}
