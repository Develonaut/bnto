// SEC-30: BatBadBut (CVE-2024-24576, bypass CVE-2024-43402). A `.bat` /
// `.cmd` found on the search path runs through cmd.exe, whose quoting
// std must escape or refuse. bnto builds on Rust >= 1.81, which has both
// fixes; this guards the toolchain floor and the shim resolution path.
#![cfg(windows)]

use serde_json::json;

use crate::support::{Lab, step};

/// SEC-30: cmd.exe metacharacters in a value never run as commands.
#[test]
fn sec30_batch_shim_args_cannot_break_out_of_cmd_quoting() {
    let lab = Lab::new();
    let bin = lab.path().join("bin");
    std::fs::create_dir_all(&bin).unwrap();
    std::fs::write(bin.join("echoargs.bat"), "@echo off\r\necho %*\r\n").unwrap();
    let recipe = lab.recipe(step(
        json!({
            "command": "echoargs",
            "args": ["{{fields.v}}"],
            "search_path": [bin.to_str().unwrap()]
        }),
        &["v"],
    ));
    for payload in [
        "x\" & type nul > canary & \"",
        "x & type nul > canary",
        "x | type nul > canary",
        "\"&type nul>canary&\"",
        "%COMSPEC% /c type nul > canary",
        "x\" ^& type nul ^> canary",
    ] {
        // std may refuse an arg it can't quote safely; either way, no canary.
        lab.run(&recipe, &[&format!("v={payload}")]);
        assert!(
            !lab.canary_fired(),
            "{payload:?} broke out of cmd.exe quoting"
        );
    }
}
