// `bnto doctor` — check external dependencies and recipe secrets.

use colored::Colorize;

use crate::context;

/// Run the doctor command — check dependencies and recipe secrets.
pub fn run_doctor() {
    let registry = bnto_engine::create_registry();
    let ctx = crate::unwrap_or_exit(context::NativeContext::current_dir());

    let mut has_issues = false;

    // Check binary dependencies.
    let deps = bnto_engine::deps::collect_all_dependencies(&registry);
    if deps.is_empty() {
        println!("{} All processors are self-contained.", "ok".green().bold());
    } else {
        let statuses = bnto_engine::deps::check_dependencies(&deps, &ctx);
        if print_dep_statuses(&statuses) {
            has_issues = true;
        }
    }

    // Check secrets for all known recipes that declare them.
    if check_all_recipe_secrets(&ctx) {
        has_issues = true;
    }

    if has_issues {
        eprintln!(
            "\n{}",
            "Some checks failed. See above for details.".yellow()
        );
        std::process::exit(1);
    }
    println!("\n{}", "All checks passed.".green());
}

/// Print each dependency status line. Returns true if any have issues.
fn print_dep_statuses(statuses: &[bnto_engine::deps::DependencyStatus]) -> bool {
    let mut has_issues = false;
    println!("{}\n", "Checking external dependencies...".bold());
    for status in statuses {
        if !status.found {
            has_issues = true;
            println!("  {} {}", "MISSING".red().bold(), status.dependency.binary);
            println!(
                "         Install: {}",
                status.dependency.install_hint.cyan()
            );
            if !status.dependency.homepage.is_empty() {
                println!("         Homepage: {}", status.dependency.homepage);
            }
        } else if status.version_satisfied == Some(false) {
            has_issues = true;
            let installed = status.installed_version.as_deref().unwrap_or("unknown");
            println!(
                "  {} {} (installed: {}, requires: {})",
                "OUTDATED".yellow().bold(),
                status.dependency.binary,
                installed,
                status.dependency.version
            );
            println!("         Update: {}", status.dependency.install_hint.cyan());
        } else {
            let version_info = match &status.installed_version {
                Some(v) => format!(" ({})", v.dimmed()),
                None => String::new(),
            };
            println!(
                "  {} {}{}",
                "ok".green(),
                status.dependency.binary,
                version_info
            );
        }
    }
    has_issues
}

/// Check secrets across all recipes that declare them.
/// Returns true if any required secrets are missing.
fn check_all_recipe_secrets(ctx: &dyn bnto_core::ProcessContext) -> bool {
    let catalog = bnto_engine::recipes::builtin_recipes();
    let mut has_missing = false;
    let mut checked_any = false;

    for entry in &catalog {
        let def: Result<bnto_core::PipelineDefinition, _> =
            serde_json::from_str(entry.definition_json);
        let Ok(def) = def else { continue };
        if def.secrets.is_empty() {
            continue;
        }

        if !checked_any {
            println!("\n{}\n", "Checking recipe secrets...".bold());
            checked_any = true;
        }

        has_missing |= print_recipe_secret_status(&entry.slug, &def, ctx);
    }

    has_missing
}

/// Check and print secret status for a single recipe. Returns true if missing.
fn print_recipe_secret_status(
    slug: &str,
    def: &bnto_core::PipelineDefinition,
    ctx: &dyn bnto_core::ProcessContext,
) -> bool {
    let statuses = bnto_core::secrets::check_secrets(&def.secrets, ctx);
    let missing = bnto_core::secrets::missing_required(&statuses);

    if missing.is_empty() {
        println!("  {} {}: all secrets present", "ok".green(), slug);
        return false;
    }

    println!("  {} {}", "MISSING".red().bold(), slug);
    for s in &missing {
        if s.description.is_empty() {
            println!("         - {}", s.key);
        } else {
            println!("         - {} ({})", s.key, s.description);
        }
    }
    true
}
