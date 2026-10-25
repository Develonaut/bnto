// Read-only recipe commands: `bnto list`, `bnto info`, `bnto dry-run`.

use std::process;

use colored::Colorize;

use crate::catalog::RecipeCatalog;
use crate::cli::DryRunArgs;
use crate::consent::check_trust;
use crate::dry_run_hint::DryRunHint;
use crate::unfilled_fields::refuse_unfilled;
use crate::{dry_run, host_support, info, list, recipe_source, vars};

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
pub fn show_dry_run(args: &DryRunArgs, catalog: &RecipeCatalog) {
    let (recipe_arg, params) = (args.recipe.as_str(), args.param.as_slice());
    let json = read_recipe(recipe_arg, catalog);
    refuse_unsupported_host(recipe_arg, &json);
    // Vars fill in where `bnto run` fills them, so the preview, its unset
    // checks and its consent see the values the run would use.
    let json = or_exit(vars::apply(&json, &args.vars.files));
    or_exit(refuse_unfilled(&json, params));
    // With inputs the preview executes the pipeline (shell commands
    // included), so it needs the same consent as `bnto run`.
    if !args.inputs.is_empty() {
        let given = DryRunHint::new(recipe_arg, params, &args.vars.files);
        check_trust(&json, &given, args.yes, catalog);
        dry_run::print_shell_note(&json);
    }
    let result = dry_run::dry_run_recipe(recipe_arg, &json, &args.inputs, params);
    dry_run::print_dry_run(recipe_arg, &or_exit(result));
}

/// The recipe's JSON (a slug, else a path), or exit: "Unknown recipe" when
/// there's no such slug or file, else the read error, as `bnto run` gives.
fn read_recipe(recipe_arg: &str, catalog: &RecipeCatalog) -> String {
    match recipe_source::read_recipe(recipe_arg, catalog) {
        Ok(json) => json,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => exit_unknown_recipe(recipe_arg),
        Err(e) => {
            eprintln!("{} {e}", "Error reading recipe:".red());
            process::exit(1);
        }
    }
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

/// Report an unknown recipe (for dry-run: no such slug or file), then exit.
fn exit_unknown_recipe(recipe_arg: &str) -> ! {
    eprintln!("{} Unknown recipe: {recipe_arg}", "Error:".red());
    eprintln!("Run {} to see available recipes.", "bnto list".cyan());
    process::exit(1);
}
