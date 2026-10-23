// `cli_recipe_run` names only bundled recipes: a recipe file's path, a
// library recipe's slug and a custom recipe's field names never leave the
// machine. BNTO_TELEMETRY_DEBUG prints each event to stderr, sent or not.
//
// Each run gets fresh HOME / BNTO_HOME / XDG_CONFIG_HOME temp dirs and a temp
// working directory, so it never reads or writes the developer's own.

mod helpers;

use std::process::Command;

use helpers::{bnto_bin, fixture_image, recipe_path};

/// Run `bnto run <recipe> small.jpg [extra...]` in a sandbox whose library
/// holds `library` (file name, recipe JSON), returning the run's
/// `cli_recipe_run` debug line.
fn recipe_run_event(recipe: &str, extra: &[&str], library: Option<(&str, &str)>) -> String {
    let home = tempfile::tempdir().unwrap();
    let work = tempfile::tempdir().unwrap();
    let bnto_home = home.path().join(".bnto");
    if let Some((name, json)) = library {
        std::fs::create_dir_all(bnto_home.join("recipes")).unwrap();
        std::fs::write(bnto_home.join("recipes").join(name), json).unwrap();
    }
    let output = Command::new(bnto_bin())
        .args(["run", recipe, "--yes", &fixture_image("small.jpg")])
        .args(["-o", work.path().join("out").to_str().unwrap()])
        .args(extra)
        .current_dir(work.path())
        .env("HOME", home.path())
        .env("BNTO_HOME", &bnto_home)
        .env("XDG_CONFIG_HOME", home.path().join(".config"))
        .env("BNTO_TELEMETRY_DEBUG", "1")
        .env_remove("BNTO_POSTHOG_KEY")
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    assert!(output.status.success(), "bnto run failed: {stderr}");
    stderr
        .lines()
        .find(|line| line.contains("cli_recipe_run"))
        .unwrap_or_else(|| panic!("no cli_recipe_run event in: {stderr}"))
        .to_string()
}

fn copy_of_compress_images() -> String {
    std::fs::read_to_string(recipe_path("compress-images")).unwrap()
}

#[test]
fn a_bundled_recipe_is_named() {
    let event = recipe_run_event("compress-images", &["-p", "quality=50"], None);
    assert!(
        event.contains(r#""slug": String("compress-images")"#),
        "{event}"
    );
    assert!(event.contains(r#"String("quality")"#), "{event}");
}

#[test]
fn a_recipe_path_is_sent_as_custom() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("acme-client.bnto.json");
    std::fs::write(&path, copy_of_compress_images()).unwrap();
    let path = path.to_str().unwrap();

    let event = recipe_run_event(path, &["-p", "quality=50"], None);
    assert!(event.contains(r#""slug": String("custom")"#), "{event}");
    assert!(!event.contains("acme-client"), "path leaked: {event}");
    assert!(!event.contains("quality"), "param name leaked: {event}");
    assert!(event.contains(r#""param_count": Number(1)"#), "{event}");
}

#[test]
fn a_library_recipe_is_sent_as_custom() {
    let library = ("acme-client.bnto.json", copy_of_compress_images());
    let event = recipe_run_event("acme-client", &[], Some((library.0, &library.1)));
    assert!(event.contains(r#""slug": String("custom")"#), "{event}");
    assert!(
        !event.contains("acme-client"),
        "library slug leaked: {event}"
    );
}
