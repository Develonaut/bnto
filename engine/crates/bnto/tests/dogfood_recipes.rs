// The repo's dogfood recipes (recipes/, see .claude/strategy/dogfooding.md),
// run through the real CLI against a temp copy of the layout they check, so
// a recipe that stops catching what it's for fails here, not only in CI.
// Each run gets its own temp HOME and BNTO_HOME, never under its working
// directory (release-verify runs from the repo itself).
#![cfg(unix)]

mod helpers;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use helpers::{bnto_bin, temp_output_dir};

/// A dogfood recipe from the repo's recipes/ folder.
fn dogfood_recipe(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../recipes")
        .join(format!("{name}.bnto.json"))
}

/// `bnto run <recipe> --yes` with `root` as the working directory.
fn run_in(root: &Path, recipe: &Path) -> Output {
    run_with(root, recipe, &[])
}

/// `run_in`, plus `--param` pairs.
fn run_with(root: &Path, recipe: &Path, params: &[&str]) -> Output {
    let mut args = vec!["run", recipe.to_str().unwrap(), "--yes"];
    for param in params {
        args.extend(["--param", param]);
    }
    let homes = temp_output_dir();
    Command::new(bnto_bin())
        .args(args)
        .current_dir(root)
        .env("HOME", homes.path().join("home"))
        .env("BNTO_HOME", homes.path().join("bnto-home"))
        .env("BNTO_TELEMETRY_DISABLED", "1")
        .env("BNTO_NO_UPDATE_CHECK", "1")
        .output()
        .unwrap()
}

fn write(root: &Path, rel: &str, text: &str) {
    let path = root.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, text).unwrap();
}

fn text(output: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

#[test]
fn unused_assets_fails_naming_only_the_files_nothing_references() {
    let root = temp_output_dir();
    let root = root.path();
    write(root, "apps/web/public/used.png", "png");
    write(root, "apps/web/public/mascots/fish.svg", "svg");
    write(root, "apps/web/public/orphan.webp", "webp");
    write(root, "apps/web/public/old/forgotten.svg", "svg");
    // Served by convention, never referenced: not checked.
    write(root, "apps/web/public/robots.txt", "User-agent: *");
    write(root, "apps/web/public/favicon/favicon.ico", "ico");
    write(root, "apps/web/public/wasm/bnto_wasm_bg.wasm", "wasm");
    // Never in git: Finder litter on a Mac developer's checkout.
    write(root, "apps/web/public/.DS_Store", "finder");
    write(root, "apps/web/public/mascots/.DS_Store", "finder");
    write(root, "apps/web/app/page.tsx", r#"<img src="/used.png" />"#);
    write(
        root,
        "packages/ui/src/Mascot.tsx",
        r#"const src = `/mascots/fish.svg`;"#,
    );

    let output = run_in(root, &dogfood_recipe("unused-assets"));
    let all = text(&output);
    assert!(!output.status.success(), "orphans passed: {all}");
    assert!(all.contains("apps/web/public/orphan.webp"), "{all}");
    assert!(all.contains("apps/web/public/old/forgotten.svg"), "{all}");
    for referenced in [
        "used.png",
        "fish.svg",
        "robots.txt",
        "favicon.ico",
        "bnto_wasm_bg.wasm",
        ".DS_Store",
    ] {
        assert!(!all.contains(referenced), "{referenced} reported: {all}");
    }
}

#[test]
fn unused_assets_passes_when_everything_is_referenced() {
    let root = temp_output_dir();
    let root = root.path();
    write(root, "apps/web/public/used.png", "png");
    write(root, "apps/web/public/robots.txt", "User-agent: *");
    write(
        root,
        "apps/web/components/Hero.tsx",
        r#"<img src="/used.png" />"#,
    );

    let output = run_in(root, &dogfood_recipe("unused-assets"));
    assert!(output.status.success(), "{}", text(&output));
}

/// The repo root, where release-verify reads the crates it checks.
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

#[test]
fn release_verify_asks_for_a_version_before_any_request() {
    let output = run_in(&repo_root(), &dogfood_recipe("release-verify"));
    let all = text(&output);
    assert!(!output.status.success(), "{all}");
    // Refused before consent and any step, naming the -p (ISS-1).
    assert!(
        all.contains("field \"version\" (node \"checks\") is unset: pass -p version=..."),
        "{all}"
    );
    assert!(
        !all.contains("crates.io serves"),
        "reached the network: {all}"
    );
}

#[test]
fn release_verify_with_an_empty_version_asks_for_one() {
    let output = run_with(
        &repo_root(),
        &dogfood_recipe("release-verify"),
        &["version="],
    );
    let all = text(&output);
    assert!(!output.status.success(), "{all}");
    assert!(all.contains("give the version: -p version=X.Y.Z"), "{all}");
}

#[test]
fn release_verify_refuses_a_malformed_version_before_any_request() {
    for bad in ["v0.18.0", "0.18", "0.18.0-rc.1", "latest"] {
        let param = format!("version={bad}");
        let output = run_with(&repo_root(), &dogfood_recipe("release-verify"), &[&param]);
        let all = text(&output);
        assert!(!output.status.success(), "{bad}: {all}");
        assert!(
            all.contains(&format!("bad version '{bad}'")),
            "{bad}: {all}"
        );
        assert!(
            !all.contains("crates.io serves"),
            "{bad} reached the network: {all}"
        );
    }
}

/// Live: needs the network. `task release:verify -- 0.18.0` runs the same.
#[test]
#[ignore = "talks to crates.io, GitHub and bnto.io"]
fn release_verify_passes_a_shipped_release_and_fails_an_unshipped_one() {
    let shipped = run_with(
        &repo_root(),
        &dogfood_recipe("release-verify"),
        &["version=0.18.0"],
    );
    assert!(shipped.status.success(), "{}", text(&shipped));
    let unshipped = run_with(
        &repo_root(),
        &dogfood_recipe("release-verify"),
        &["version=0.0.1"],
    );
    let all = text(&unshipped);
    assert!(!unshipped.status.success(), "{all}");
    for check in [
        "crates.io has no",
        "GitHub has no release",
        "doesn't show 0.0.1",
    ] {
        assert!(all.contains(check), "{check}: {all}");
    }
}

/// A Rust file of `production` code lines with a `test_lines` test module
/// after them, then `trailing` more code lines.
fn rust_file(production: usize, test_lines: usize, trailing: usize) -> String {
    let code = |n: usize| "fn f() {}\n".repeat(n);
    let tests = format!(
        "#[cfg(test)]\nmod tests {{\n{}}}\n",
        "    #[test]\n".repeat(test_lines)
    );
    format!("{}{tests}{}", code(production), code(trailing))
}

#[test]
fn rust_file_size_fails_naming_each_file_over_400_production_lines() {
    let root = temp_output_dir();
    let root = root.path();
    write(
        root,
        "engine/crates/demo/src/big.rs",
        &rust_file(401, 10, 0),
    );
    // Code after the test module still counts.
    write(
        root,
        "engine/crates/demo/src/split.rs",
        &rust_file(300, 200, 101),
    );
    write(
        root,
        "engine/crates/demo/src/at_limit.rs",
        &rust_file(400, 500, 0),
    );
    // Test-only files are not production code.
    write(
        root,
        "engine/crates/demo/src/nested/tests.rs",
        &rust_file(900, 0, 0),
    );
    write(
        root,
        "engine/crates/demo/tests/journey.rs",
        &rust_file(900, 0, 0),
    );

    let output = run_in(root, &dogfood_recipe("rust-file-size"));
    let all = text(&output);
    assert!(!output.status.success(), "oversized files passed: {all}");
    assert!(all.contains("engine/crates/demo/src/big.rs: 401"), "{all}");
    assert!(
        all.contains("engine/crates/demo/src/split.rs: 401"),
        "{all}"
    );
    for fine in ["at_limit.rs", "tests.rs", "journey.rs"] {
        assert!(!all.contains(fine), "{fine} reported: {all}");
    }
}

#[test]
fn rust_file_size_passes_when_every_file_is_within_the_limit() {
    let root = temp_output_dir();
    let root = root.path();
    write(
        root,
        "engine/crates/demo/src/lib.rs",
        &rust_file(400, 300, 0),
    );
    write(
        root,
        "engine/crates/demo/src/small.rs",
        &rust_file(20, 0, 0),
    );

    let output = run_in(root, &dogfood_recipe("rust-file-size"));
    assert!(output.status.success(), "{}", text(&output));
}

/// A Rust line pointing at the made-up absolute path `/<path>`. Built at run
/// time so this file never holds one itself (the check scans it too).
fn made_up_path_line(path: &str) -> String {
    format!("let p = Path::new(\"/{path}\");\n")
}

#[test]
fn nonexistent_paths_fails_naming_each_made_up_path() {
    let root = temp_output_dir();
    let root = root.path();
    write(
        root,
        "engine/crates/demo/src/lib.rs",
        &format!("fn f() {{}}\n{}", made_up_path_line("nonexistent")),
    );
    write(
        root,
        "engine/crates/demo/tests/journey.rs",
        &made_up_path_line("tmp/NonExistent_dir"),
    );
    // Not a path, or not Rust: not checked.
    write(
        root,
        "engine/crates/demo/src/ok.rs",
        r#"let slug = "nonexistent-recipe";"#,
    );
    write(
        root,
        "engine/crates/demo/README.md",
        &made_up_path_line("nonexistent"),
    );

    let output = run_in(root, &dogfood_recipe("nonexistent-paths"));
    let all = text(&output);
    assert!(!output.status.success(), "made-up paths passed: {all}");
    assert!(all.contains("engine/crates/demo/src/lib.rs:2:"), "{all}");
    assert!(
        all.contains("engine/crates/demo/tests/journey.rs:1:"),
        "{all}"
    );
    for fine in ["ok.rs", "README.md"] {
        assert!(!all.contains(fine), "{fine} reported: {all}");
    }
}

#[test]
fn nonexistent_paths_passes_when_every_path_is_real() {
    let root = temp_output_dir();
    let root = root.path();
    write(
        root,
        "engine/crates/demo/src/lib.rs",
        "let missing = tmp.path().join(\"missing\");\nlet slug = \"nonexistent-recipe\";\n",
    );

    let output = run_in(root, &dogfood_recipe("nonexistent-paths"));
    assert!(output.status.success(), "{}", text(&output));
}
