// ISS-126: a relative or empty `BNTO_HOME`, or config `paths.home`, would
// make the working folder's `.bnto/` the bnto home, so a cloned repo's
// recipes would load as library recipes. bnto refuses it and stops.

use crate::helpers;

use std::process::Output;

use helpers::Sandbox;

/// A recipe the working folder offers, which a relative home would list.
fn plant_recipe(sandbox: &Sandbox) {
    let recipes = sandbox.work_dir().join(".bnto").join("recipes");
    std::fs::create_dir_all(&recipes).unwrap();
    let recipe = r#"{"id":"sneaky","name":"Sneaky","nodes":[]}"#;
    std::fs::write(recipes.join("sneaky.bnto.json"), recipe).unwrap();
}

fn list_with_home(sandbox: &Sandbox, bnto_home: &str) -> Output {
    let mut cmd = sandbox.command();
    cmd.arg("list").env("BNTO_HOME", bnto_home);
    cmd.output().unwrap()
}

/// `output` failed naming `setting`, and left the working folder alone.
fn assert_refused(sandbox: &Sandbox, output: &Output, setting: &str) {
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!output.status.success(), "ran: {stdout}");
    assert!(stderr.contains(setting), "{stderr}");
    assert!(stderr.contains("must be an absolute path"), "{stderr}");
    assert!(!stdout.contains("sneaky"), "{stdout}");
    let dot = sandbox.work_dir().join(".bnto");
    assert!(
        !dot.join("state").exists(),
        "a home was made in the working folder"
    );
}

#[test]
fn a_relative_bnto_home_is_refused() {
    let sandbox = Sandbox::new();
    plant_recipe(&sandbox);
    let output = list_with_home(&sandbox, ".bnto");
    assert_refused(&sandbox, &output, "BNTO_HOME");
}

#[test]
fn an_empty_bnto_home_is_refused() {
    let sandbox = Sandbox::new();
    plant_recipe(&sandbox);
    let output = list_with_home(&sandbox, "");
    assert_refused(&sandbox, &output, "BNTO_HOME");
}

#[test]
fn a_relative_config_home_is_refused() {
    let sandbox = Sandbox::new();
    plant_recipe(&sandbox);
    std::fs::create_dir_all(sandbox.bnto_home()).unwrap();
    let config = "version = 3\n\n[paths]\nhome = '.bnto'\n";
    std::fs::write(sandbox.bnto_home().join("config.toml"), config).unwrap();
    let output = sandbox.command().arg("list").output().unwrap();
    assert_refused(&sandbox, &output, "paths.home in ");
}

#[test]
fn a_relative_config_home_is_refused_when_storage_setup_fails() {
    // `state` is a file, so creating the home's folders fails: the refusal
    // must still come first, not be skipped with the setup's warning.
    let sandbox = Sandbox::new();
    plant_recipe(&sandbox);
    std::fs::create_dir_all(sandbox.bnto_home()).unwrap();
    std::fs::write(sandbox.bnto_home().join("state"), "").unwrap();
    let config = "version = 3\n\n[paths]\nhome = '.bnto'\n";
    std::fs::write(sandbox.bnto_home().join("config.toml"), config).unwrap();
    let output = sandbox.command().arg("list").output().unwrap();
    assert_refused(&sandbox, &output, "paths.home in ");
}

#[test]
fn a_home_starting_with_a_tilde_says_it_is_not_expanded() {
    let sandbox = Sandbox::new();
    let output = list_with_home(&sandbox, "~/bnto");
    assert_refused(&sandbox, &output, "BNTO_HOME");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("`~` isn't expanded"), "{stderr}");
}

#[test]
fn an_absolute_bnto_home_still_works() {
    let sandbox = Sandbox::new();
    let output = sandbox.command().arg("list").output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(sandbox.bnto_home().join("state").is_dir());
}

/// Where the v1 layout kept recipes and approvals, under a home folder.
#[cfg(target_os = "macos")]
const OLD_DATA_DIR: &str = "Library/Application Support/bnto";
#[cfg(all(unix, not(target_os = "macos")))]
const OLD_DATA_DIR: &str = ".local/share/bnto";

#[cfg(unix)]
#[test]
fn a_relative_home_folder_feeds_no_migration() {
    // `HOME=.` made the v1 → v2 migration read the working folder as the
    // old home and copy its recipes and approvals into a fresh bnto home.
    let sandbox = Sandbox::new();
    let old = sandbox.work_dir().join(OLD_DATA_DIR);
    std::fs::create_dir_all(old.join("recipes")).unwrap();
    let recipe = r#"{"id":"planted-recipe","name":"Planted","nodes":[]}"#;
    std::fs::write(old.join("recipes").join("planted-recipe.bnto.json"), recipe).unwrap();
    std::fs::write(old.join("trusted.json"), "{}").unwrap();
    let mut cmd = sandbox.command();
    cmd.arg("list")
        .env("HOME", ".")
        .env_remove("XDG_DATA_HOME")
        .env_remove("XDG_CONFIG_HOME");
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!stdout.contains("planted-recipe"), "{stdout}");
    let home = sandbox.bnto_home();
    assert!(
        !home
            .join("recipes")
            .join("planted-recipe.bnto.json")
            .exists()
    );
    assert!(!home.join("state").join("trusted.json").exists());
}
