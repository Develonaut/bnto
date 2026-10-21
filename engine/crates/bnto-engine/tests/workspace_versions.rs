// Every bnto crate carries the release version from `[workspace.package]`, so a
// release tag, crates.io and `bnto --version` show one number (releases.md).
// A crate that pins its own version, or depends on a sibling by path, drifts.

use std::path::{Path, PathBuf};

fn crates_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

/// `(crate dir name, Cargo.toml text)` for every bnto crate.
fn bnto_manifests() -> Vec<(String, String)> {
    let mut manifests: Vec<_> = std::fs::read_dir(crates_dir())
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|dir| dir.join("Cargo.toml").exists())
        .map(|dir| dir.file_name().unwrap().to_string_lossy().into_owned())
        .filter(|name| name.starts_with("bnto"))
        .map(|name| {
            let toml = std::fs::read_to_string(crates_dir().join(&name).join("Cargo.toml"));
            (name, toml.unwrap())
        })
        .collect();
    manifests.sort();
    manifests
}

#[test]
fn every_bnto_crate_inherits_the_release_version() {
    let manifests = bnto_manifests();
    assert!(manifests.len() >= 10, "found only {manifests:?}");
    for (name, toml) in manifests {
        assert!(
            toml.lines().any(|l| l.trim() == "version.workspace = true"),
            "{name} sets its own version; use `version.workspace = true`"
        );
    }
}

#[test]
fn bnto_crates_depend_on_siblings_through_the_workspace() {
    // `[workspace.dependencies]` holds each sibling's version once, so
    // `task version:bump` can move them all together.
    for (name, toml) in bnto_manifests() {
        let by_path = toml.lines().find(|l| l.contains("path = \"../"));
        assert!(
            by_path.is_none(),
            "{name} depends on a sibling by path: {by_path:?}"
        );
    }
}
