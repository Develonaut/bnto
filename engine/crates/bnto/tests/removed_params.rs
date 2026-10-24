// A recipe that still gives `vector-optimize` the `precision` param it no
// longer has (removed because it never did anything, ISS-15): `bnto check`
// and `bnto run` refuse it naming `bnto migrate`, and after `bnto migrate`
// the same recipe runs.

mod helpers;

use std::path::{Path, PathBuf};
use std::process::Output;

use helpers::{Sandbox, fixture_vector, output_files, recipe_path, temp_output_dir};

const HINT: &str = "vector-optimize has no param \"precision\" \
                    (removed because it never did anything; run `bnto migrate` on the recipe)";

/// The bundled `optimize-svg` recipe as it shipped before the removal,
/// with `"precision": 3` on its `vector-optimize` step, written into `dir`.
fn write_old_optimize_svg(dir: &Path) -> PathBuf {
    let text = std::fs::read_to_string(recipe_path("optimize-svg")).unwrap();
    let mut recipe: serde_json::Value = serde_json::from_str(&text).unwrap();
    let step = recipe["nodes"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|n| n["type"] == "vector-optimize")
        .unwrap();
    step["parameters"]["precision"] = serde_json::json!(3);
    let path = dir.join("optimize-svg.bnto.json");
    std::fs::write(&path, serde_json::to_string_pretty(&recipe).unwrap()).unwrap();
    path
}

fn bnto(sandbox: &Sandbox, args: &[&str]) -> Output {
    sandbox.command().args(args).output().unwrap()
}

fn run_on_verbose_svg(sandbox: &Sandbox, recipe: &Path, out: &Path) -> Output {
    let svg = fixture_vector("verbose.svg");
    let args = ["run", path_str(recipe), "--yes", &svg, "-o", path_str(out)];
    bnto(sandbox, &args)
}

fn path_str(path: &Path) -> &str {
    path.to_str().unwrap()
}

#[test]
fn test_check_names_bnto_migrate_for_the_removed_precision() {
    let (sandbox, dir) = (Sandbox::new(), temp_output_dir());
    let recipe = write_old_optimize_svg(dir.path());

    let output = bnto(&sandbox, &["check", path_str(&recipe)]);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!output.status.success(), "stdout: {stdout}");
    assert!(stdout.contains(HINT), "stdout: {stdout}");
}

#[test]
fn test_run_refuses_the_removed_precision_before_any_step() {
    let (sandbox, dir, out) = (Sandbox::new(), temp_output_dir(), temp_output_dir());
    let recipe = write_old_optimize_svg(dir.path());

    let output = run_on_verbose_svg(&sandbox, &recipe, out.path());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "stderr: {stderr}");
    assert!(stderr.contains(HINT), "stderr: {stderr}");
    assert!(
        output_files(&out).is_empty(),
        "a refused run writes nothing"
    );
}

#[test]
fn test_migrate_drops_precision_and_the_recipe_runs() {
    let (sandbox, dir, out) = (Sandbox::new(), temp_output_dir(), temp_output_dir());
    let recipe = write_old_optimize_svg(dir.path());

    let migrated = bnto(&sandbox, &["migrate", path_str(&recipe)]);
    let stderr = String::from_utf8_lossy(&migrated.stderr);
    assert!(migrated.status.success(), "stderr: {stderr}");
    assert!(
        stderr.contains("Remove params their nodes no longer have"),
        "stderr: {stderr}"
    );
    assert!(
        !std::fs::read_to_string(&recipe)
            .unwrap()
            .contains("precision")
    );

    let run = run_on_verbose_svg(&sandbox, &recipe, out.path());
    let stderr = String::from_utf8_lossy(&run.stderr);
    assert!(run.status.success(), "stderr: {stderr}");
    assert!(!output_files(&out).is_empty(), "stderr: {stderr}");
}
