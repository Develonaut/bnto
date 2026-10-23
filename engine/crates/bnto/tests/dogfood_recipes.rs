// The repo's dogfood recipes (recipes/, see .claude/strategy/dogfooding.md),
// run through the real CLI against a temp copy of the layout they check, so
// a recipe that stops catching what it's for fails here, not only in CI.
// Each run uses a temp HOME and BNTO_HOME.
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
    Command::new(bnto_bin())
        .args(args)
        .current_dir(root)
        .env("HOME", root.join("home"))
        .env("BNTO_HOME", root.join("bnto-home"))
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
