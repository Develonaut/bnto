// Read-only recipe commands: `bnto list`, `bnto info`, `bnto dry-run`.

use std::process;

use colored::Colorize;

use crate::catalog::RecipeCatalog;
use crate::consent::check_trust;
use crate::unfilled_fields::refuse_unfilled;
use crate::{dry_run, host_support, info, list, recipe_source};

/// `bnto list` — print the catalog grouped by category.
pub fn list_recipes(catalog: &RecipeCatalog) {
    let groups = list::group_recipes(catalog.all());
    list::print_recipe_list(&groups);
}

/// `bnto info <recipe>` — print recipe details and dependencies.
pub fn show_info(slug: &str, catalog: &RecipeCatalog) {
    let Some(recipe_info) = info::get_recipe_info(slug, catalog) else {
        exit_unknown_recipe(slug);
    };
    info::print_recipe_info(slug, &recipe_info);
}

/// `bnto dry-run <recipe>` — preview what a recipe (slug or path) will do.
pub fn show_dry_run(
    recipe_arg: &str,
    inputs: &[String],
    param_overrides: &[String],
    yes: bool,
    catalog: &RecipeCatalog,
) {
    let Ok(json) = recipe_source::read_recipe(recipe_arg, catalog) else {
        exit_unknown_recipe(recipe_arg);
    };
    refuse_unsupported_host(recipe_arg, &json);
    or_exit(refuse_unfilled(&json, param_overrides));
    // With inputs the preview executes the pipeline (shell commands
    // included), so it needs the same consent as `bnto run`.
    if !inputs.is_empty() {
        check_trust(recipe_arg, &json, yes, catalog);
        dry_run::print_shell_note(&json);
    }
    let result = or_exit(dry_run::dry_run_recipe(
        recipe_arg,
        &json,
        inputs,
        param_overrides,
    ));
    dry_run::print_dry_run(recipe_arg, &result);
}

/// Stop a recipe with no branch for this OS, as `bnto run` does: a preview
/// of a run that would be refused would mislead.
fn refuse_unsupported_host(recipe_arg: &str, json: &str) {
    let host = bnto_core::HostPlatform::current();
    if let Some(why) = host_support::unsupported_host(recipe_arg, json, host) {
        eprintln!("{} {why}", "Error:".red());
        process::exit(1);
    }
}

/// The value, or print the error and exit 1, as `bnto run` does.
fn or_exit<T>(result: Result<T, String>) -> T {
    result.unwrap_or_else(|why| {
        eprintln!("{} {why}", "Error:".red());
        process::exit(1);
    })
}

/// Report an unknown recipe slug, then exit.
fn exit_unknown_recipe(slug: &str) -> ! {
    eprintln!("{} Unknown recipe: {slug}", "Error:".red());
    eprintln!("Run {} to see available recipes.", "bnto list".cyan());
    process::exit(1);
}
