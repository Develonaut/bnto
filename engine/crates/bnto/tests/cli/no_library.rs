// When the bnto home's storage can't be set up, `bnto` runs with the
// bundled recipes alone. It used to load the library from an empty path,
// `read_dir("")`, which on Windows may list the working folder, so a
// recipe sitting there would have loaded as a library recipe. This guards
// only that Windows behaviour: elsewhere `read_dir("")` already fails.

use crate::helpers;

use helpers::Sandbox;

#[test]
fn a_failed_storage_setup_loads_no_library() {
    let sandbox = Sandbox::new();
    std::fs::create_dir_all(sandbox.bnto_home()).unwrap();
    // `state` is a file, so creating the home's folders fails.
    std::fs::write(sandbox.bnto_home().join("state"), "").unwrap();
    let recipe = r#"{"id":"planted-recipe","name":"Planted","nodes":[]}"#;
    std::fs::write(sandbox.work_dir().join("planted-recipe.bnto.json"), recipe).unwrap();
    let output = sandbox.command().arg("list").output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(stderr.contains("storage init"), "{stderr}");
    assert!(
        !stdout.contains("Planted") && !stdout.contains(" x "),
        "{stdout}"
    );
}
