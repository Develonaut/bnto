// SEC-32..37: PowerShell script mode — a literal script run as
// `<abs path> -NoProfile -NonInteractive -EncodedCommand <script>`, values
// only as $env:BNTO_ARG_1..N (decisions/powershell-script-mode.md).
//
// Each spec runs once per PowerShell installed at its fixed path
// (`powershell` on Windows, `pwsh` anywhere). Without one they skip, unless
// BNTO_REQUIRE_PWSH=1 (set in CI), which makes a missing PowerShell fail.

use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use crate::support::{Lab, step};

/// The installed PowerShells, by the paths bnto itself tries.
fn installed() -> Vec<&'static str> {
    let mut found = Vec::new();
    if cfg!(windows) {
        let root = std::env::var("SystemRoot").unwrap_or_else(|_| r"C:\Windows".into());
        let powershell =
            PathBuf::from(root).join(r"System32\WindowsPowerShell\v1.0\powershell.exe");
        if powershell.exists() {
            found.push("powershell");
        }
    }
    if pwsh_paths().iter().any(|p| Path::new(p).exists()) {
        found.push("pwsh");
    }
    if found.is_empty() && std::env::var("BNTO_REQUIRE_PWSH").as_deref() == Ok("1") {
        panic!("BNTO_REQUIRE_PWSH=1 but no PowerShell is installed");
    }
    found
}

fn pwsh_paths() -> &'static [&'static str] {
    if cfg!(windows) {
        &[r"C:\Program Files\PowerShell\7\pwsh.exe"]
    } else if cfg!(target_os = "macos") {
        &[
            "/usr/local/bin/pwsh",
            "/opt/homebrew/bin/pwsh",
            "/usr/local/microsoft/powershell/7/pwsh",
        ]
    } else {
        &[
            "/usr/bin/pwsh",
            "/opt/microsoft/powershell/7/pwsh",
            "/snap/bin/pwsh",
        ]
    }
}

fn ps_step(shell: &str, script: &str, args: Value, fields: &[&str]) -> Value {
    step(
        json!({ "shell": shell, "script": script, "args": args }),
        fields,
    )
}

/// PowerShell-injection strings, each trying to create `canary`.
fn payloads(canary: &Path) -> Vec<String> {
    let c = canary.display();
    let new_item = format!("New-Item -ItemType File -Path '{c}'");
    vec![
        format!("'; {new_item}; '"),
        format!("\"; {new_item}; \""),
        format!("$({new_item})"),
        format!("@({new_item})"),
        format!("& {{ {new_item} }}"),
        format!("`\"; {new_item}"),
        format!("x\n{new_item}"),
        format!("x\r\n{new_item}"),
        format!("x | {new_item}"),
        format!("-Command {new_item}"),
    ]
}

/// SEC-32: hostile values arrive as $env:BNTO_ARG_n and stay data, even
/// interpolated into a string.
#[test]
fn sec32_values_arrive_only_as_environment_variables() {
    for shell in installed() {
        let lab = Lab::new();
        let recipe = lab.recipe(ps_step(
            shell,
            "Write-Output \"[$env:BNTO_ARG_1|$env:BNTO_ARG_2]\"",
            json!(["{{fields.v}}", "second"]),
            &["v"],
        ));
        for payload in payloads(&lab.canary()) {
            let out = lab.run(&recipe, &[&format!("v={payload}")]);
            assert!(out.ok, "{shell} {payload:?}: {}", out.text);
            assert!(!lab.canary_fired(), "{shell}: {payload:?} executed");
        }
        let out = lab.run(&recipe, &["v=$(Get-Date)"]);
        assert!(
            out.text.contains("[$(Get-Date)|second]"),
            "{shell}: {}",
            out.text
        );
    }
}

/// SEC-33: nothing follows `-EncodedCommand`, so a value never reaches the
/// command line PowerShell parses.
#[test]
fn sec33_a_value_is_never_on_the_command_line() {
    for shell in installed() {
        let lab = Lab::new();
        let script = "Write-Output \"cmd=[$([Environment]::CommandLine)]\"\n\
                      Write-Output \"env=[$env:BNTO_ARG_1]\"";
        let recipe = lab.recipe(ps_step(shell, script, json!(["{{fields.v}}"]), &["v"]));
        let out = lab.run(&recipe, &["v=MARKER-7f3a"]);
        assert!(out.ok, "{shell}: {}", out.text);
        assert!(
            out.text.contains("env=[MARKER-7f3a]"),
            "{shell}: {}",
            out.text
        );
        let command_line = out.text.split("cmd=[").nth(1).unwrap_or_default();
        let command_line = command_line.split(']').next().unwrap_or_default();
        assert!(
            command_line.contains("-EncodedCommand"),
            "{shell}: {command_line}"
        );
        assert!(
            !command_line.contains("MARKER-7f3a"),
            "{shell}: {command_line}"
        );
    }
}

/// SEC-34: a profile planted in the user's home never runs (`-NoProfile`).
/// Not on Windows: its profile lives in the real Documents folder, which a
/// test must not write to.
#[cfg(not(windows))]
#[test]
fn sec34_a_planted_profile_never_runs() {
    if !installed().contains(&"pwsh") {
        return;
    }
    let lab = Lab::new();
    let home = lab.path().join("user-home");
    let profile_dir = home.join(".config").join("powershell");
    std::fs::create_dir_all(&profile_dir).unwrap();
    let plant = format!(
        "New-Item -ItemType File -Path '{}'\n",
        lab.canary().display()
    );
    for name in ["profile.ps1", "Microsoft.PowerShell_profile.ps1"] {
        std::fs::write(profile_dir.join(name), &plant).unwrap();
    }
    let recipe = lab.recipe(ps_step("pwsh", "Write-Output ok", json!([]), &[]));
    let home = home.to_string_lossy().into_owned();
    let out = lab.run_with(&recipe, &[], &[], &[("HOME", &home)]);
    assert!(out.ok && out.text.contains("ok"), "{}", out.text);
    assert!(!lab.canary_fired(), "the profile ran: {}", out.text);
}

/// SEC-35: PowerShell runs by fixed absolute path; one planted on PATH, or
/// named in `searchPath`, never runs.
#[test]
fn sec35_powershell_is_not_looked_up_on_path() {
    for shell in installed() {
        let lab = Lab::new();
        let fake_bin = lab.path().join("fakebin");
        plant_fake(&fake_bin, shell, &lab.canary());
        let sep = if cfg!(windows) { ";" } else { ":" };
        let path = format!(
            "{}{sep}{}",
            fake_bin.display(),
            std::env::var("PATH").unwrap()
        );
        let mut planted = ps_step(shell, "Write-Output real-ps-5d2", json!([]), &[]);
        let out = lab.run_with(&lab.recipe(planted.clone()), &[], &[], &[("PATH", &path)]);
        assert!(
            out.ok && out.text.contains("real-ps-5d2"),
            "{shell}: {}",
            out.text
        );
        assert!(
            !lab.canary_fired(),
            "{shell}: ran the planted one from PATH"
        );

        planted["parameters"]["searchPath"] = json!([fake_bin.to_str().unwrap()]);
        let out = lab.run(&lab.recipe(planted), &[]);
        assert!(!out.ok, "{shell}: searchPath accepted for a script");
        assert!(
            !lab.canary_fired(),
            "{shell}: ran the planted one from searchPath"
        );
    }
}

/// A fake `shell` in `dir`: on Unix a script that creates `canary`; on
/// Windows `.cmd`/`.bat` shims that do, plus a stand-in `.exe` (a copy of
/// `hostname.exe`, which can't print the real script's marker).
fn plant_fake(dir: &Path, shell: &str, canary: &Path) {
    std::fs::create_dir_all(dir).unwrap();
    if cfg!(windows) {
        let body = format!("@echo off\r\ntype nul > \"{}\"\r\n", canary.display());
        for ext in ["cmd", "bat"] {
            std::fs::write(dir.join(format!("{shell}.{ext}")), &body).unwrap();
        }
        let root = std::env::var("SystemRoot").unwrap_or_else(|_| r"C:\Windows".into());
        let hostname = PathBuf::from(root).join(r"System32\hostname.exe");
        std::fs::copy(hostname, dir.join(format!("{shell}.exe"))).unwrap();
    } else {
        let path = dir.join(shell);
        std::fs::write(&path, format!("#!/bin/sh\ntouch '{}'\n", canary.display())).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
        }
    }
}

/// SEC-36: the cleared environment applies: an exported secret isn't visible
/// and an exported PSModulePath doesn't reach the script.
#[test]
fn sec36_powershell_starts_from_a_cleared_environment() {
    for shell in installed() {
        let lab = Lab::new();
        let mods = lab.path().join("planted-modules");
        std::fs::create_dir_all(&mods).unwrap();
        let script = "Write-Output \"secret=[$env:BNTO_PROBE_SECRET]\"\n\
                      Write-Output \"modules=[$env:PSModulePath]\"";
        let recipe = lab.recipe(ps_step(shell, script, json!([]), &[]));
        let mods = mods.to_string_lossy().into_owned();
        let env = [
            ("BNTO_PROBE_SECRET", "leaked"),
            ("PSModulePath", mods.as_str()),
        ];
        let out = lab.run_with(&recipe, &[], &[], &env);
        assert!(out.ok, "{shell}: {}", out.text);
        assert!(out.text.contains("secret=[]"), "{shell}: {}", out.text);
        assert!(
            !out.text.contains("planted-modules"),
            "{shell}: {}",
            out.text
        );
    }
}

/// SEC-37: a step's own `env` can't set the variables that carry its values,
/// and `BNTO_ARG_COUNT` matches the args.
#[test]
fn sec37_the_value_variables_belong_to_args() {
    let lab = Lab::new();
    let mut shadowing = ps_step("pwsh", "Write-Output $env:BNTO_ARG_1", json!(["real"]), &[]);
    shadowing["parameters"]["env"] = json!({ "BNTO_ARG_1": "forged" });
    let out = lab.check(&lab.recipe(shadowing));
    assert!(!out.ok, "env set BNTO_ARG_1: {}", out.text);
    assert!(out.text.contains("BNTO_ARG_"), "{}", out.text);

    for shell in installed() {
        let lab = Lab::new();
        let script =
            "Write-Output \"count=[$env:BNTO_ARG_COUNT] empty=[$([string]$env:BNTO_ARG_2)]\"";
        let recipe = lab.recipe(ps_step(shell, script, json!(["a", "", "c"]), &[]));
        let out = lab.run(&recipe, &[]);
        assert!(out.ok, "{shell}: {}", out.text);
        assert!(
            out.text.contains("count=[3] empty=[]"),
            "{shell}: {}",
            out.text
        );
    }
}

/// SEC-21 and SEC-25 for PowerShell: a template in the script is refused,
/// and both PowerShells are accepted as `shell` (static, so on every OS).
#[test]
fn sec21_sec25_powershell_scripts_are_checked_like_posix_ones() {
    let lab = Lab::new();
    for shell in ["powershell", "pwsh"] {
        let ok = lab.check(&lab.recipe(ps_step(shell, "Write-Output 1", json!([]), &[])));
        assert!(ok.ok, "{shell} refused: {}", ok.text);
        let templated = ps_step(shell, "Write-Output {{fields.v}}", json!([]), &["v"]);
        let out = lab.check(&lab.recipe(templated));
        assert!(!out.ok, "{shell}: a template in the script was accepted");
    }
}

/// Not a SEC row: what a script prints, how it fails and how it exits.
#[test]
fn a_powershell_step_keeps_utf8_exit_codes_and_plain_stderr() {
    for shell in installed() {
        let lab = Lab::new();
        let echo = lab.recipe(ps_step(
            shell,
            "Write-Output \"[$env:BNTO_ARG_1]\"",
            json!(["héllo ✓"]),
            &[],
        ));
        let out = lab.run(&echo, &[]);
        assert!(out.ok, "{shell}: {}", out.text);
        assert!(out.text.contains("[héllo ✓]"), "{shell}: {}", out.text);

        for script in [
            "[Console]::Error.WriteLine('boom-9c1')\nexit 3",
            "Write-Error 'boom-9c1'\nexit 3",
        ] {
            let failing = lab.recipe(ps_step(shell, script, json!([]), &[]));
            let out = lab.run(&failing, &[]);
            assert!(!out.ok, "{shell}: exit 3 passed");
            assert!(out.text.contains("boom-9c1"), "{shell}: {}", out.text);
            assert!(!out.text.contains("CLIXML"), "{shell}: {}", out.text);
        }
    }
}
