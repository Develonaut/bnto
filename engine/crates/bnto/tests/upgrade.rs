// `bnto upgrade`, run as the real binary. The index is a local file
// (`BNTO_UPDATE_INDEX_URL=file://…`) and `cargo` is a fake that records its
// args, so nothing reaches the network and nothing really installs.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use tempfile::TempDir;

const CURRENT: &str = env!("CARGO_PKG_VERSION");

/// A sandbox: homes, an index listing `versions`, and a cargo home.
struct Sandbox {
    root: TempDir,
}

impl Sandbox {
    fn new(versions: &[&str]) -> Self {
        let root = tempfile::tempdir().unwrap();
        let index: String = versions
            .iter()
            .map(|v| format!("{{\"name\":\"bnto\",\"vers\":\"{v}\",\"yanked\":false}}\n"))
            .collect();
        std::fs::write(root.path().join("index"), index).unwrap();
        std::fs::create_dir_all(root.path().join("cargo/bin")).unwrap();
        Self { root }
    }

    fn path(&self, rel: &str) -> PathBuf {
        self.root.path().join(rel)
    }

    /// Run `binary upgrade <args>` with every home in the sandbox.
    fn upgrade(&self, binary: &Path, args: &[&str]) -> Output {
        Command::new(binary)
            .arg("upgrade")
            .args(args)
            .env("HOME", self.path("home"))
            .env("BNTO_HOME", self.path("bnto"))
            .env("CARGO_HOME", self.path("cargo"))
            .env("BNTO_TELEMETRY_DISABLED", "1")
            .env(
                "BNTO_UPDATE_INDEX_URL",
                format!("file://{}", self.path("index").display()),
            )
            .output()
            .unwrap()
    }

    /// The real bnto, copied into the sandbox's cargo bin dir, as
    /// `cargo install` would have put it. Unix-only, like its tests.
    #[cfg(unix)]
    fn cargo_installed_bnto(&self) -> PathBuf {
        let exe = self.path("cargo/bin/bnto");
        std::fs::copy(env!("CARGO_BIN_EXE_bnto"), &exe).unwrap();
        exe
    }

    /// The args the fake cargo was called with, if it was.
    fn cargo_calls(&self) -> Option<String> {
        std::fs::read_to_string(self.path("cargo-calls")).ok()
    }
}

fn text(output: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

#[cfg(unix)]
/// A fake `cargo` beside the installed bnto that records its args and exits
/// with `code`.
fn fake_cargo(sandbox: &Sandbox, code: u8) {
    use std::os::unix::fs::PermissionsExt;
    let calls = sandbox.path("cargo-calls");
    let script = format!(
        "#!/bin/sh\necho \"$@\" > '{}'\nexit {code}\n",
        calls.display()
    );
    let cargo = sandbox.path("cargo/bin/cargo");
    std::fs::write(&cargo, script).unwrap();
    std::fs::set_permissions(&cargo, std::fs::Permissions::from_mode(0o755)).unwrap();
}

#[test]
fn already_the_latest_installs_nothing() {
    let sandbox = Sandbox::new(&["0.1.0", CURRENT]);
    let out = sandbox.upgrade(Path::new(env!("CARGO_BIN_EXE_bnto")), &[]);
    assert!(out.status.success(), "{}", text(&out));
    assert!(
        text(&out).contains(&format!("bnto {CURRENT} is the latest.")),
        "{}",
        text(&out)
    );
    assert_eq!(sandbox.cargo_calls(), None);
}

#[test]
fn check_reports_a_newer_release_and_caches_it() {
    let sandbox = Sandbox::new(&[CURRENT, "99.0.0"]);
    let out = sandbox.upgrade(Path::new(env!("CARGO_BIN_EXE_bnto")), &["--check"]);
    assert!(out.status.success(), "{}", text(&out));
    assert!(
        text(&out).contains("bnto 99.0.0 is available"),
        "{}",
        text(&out)
    );
    assert_eq!(sandbox.cargo_calls(), None);
    let cached = std::fs::read_to_string(sandbox.path("bnto/cache/update-check.json")).unwrap();
    assert!(cached.contains("99.0.0"), "{cached}");
}

#[test]
#[cfg(unix)]
fn a_cargo_install_is_upgraded_with_cargo() {
    let sandbox = Sandbox::new(&[CURRENT, "99.0.0"]);
    fake_cargo(&sandbox, 0);
    let out = sandbox.upgrade(&sandbox.cargo_installed_bnto(), &[]);
    assert!(out.status.success(), "{}", text(&out));
    assert_eq!(
        sandbox.cargo_calls().as_deref(),
        Some("install bnto --locked\n")
    );
    assert!(
        text(&out).contains(&format!("Upgraded bnto {CURRENT} → 99.0.0.")),
        "{}",
        text(&out)
    );
}

#[test]
#[cfg(unix)]
fn a_failed_cargo_install_says_so() {
    let sandbox = Sandbox::new(&[CURRENT, "99.0.0"]);
    fake_cargo(&sandbox, 101);
    let exe = sandbox.cargo_installed_bnto();
    let out = sandbox.upgrade(&exe, &[]);
    assert!(!out.status.success(), "{}", text(&out));
    assert!(text(&out).contains("still installed"), "{}", text(&out));
    assert!(exe.exists(), "the old binary must stay");
}

#[test]
fn another_install_gets_instructions_not_a_guess() {
    // The test binary lives in target/, not in a cargo bin dir.
    let sandbox = Sandbox::new(&[CURRENT, "99.0.0"]);
    let out = sandbox.upgrade(Path::new(env!("CARGO_BIN_EXE_bnto")), &[]);
    assert!(!out.status.success(), "{}", text(&out));
    assert!(
        text(&out).contains("wasn't installed with cargo"),
        "{}",
        text(&out)
    );
    assert_eq!(sandbox.cargo_calls(), None);
}

#[test]
fn an_unreachable_index_is_an_error() {
    let sandbox = Sandbox::new(&[]);
    std::fs::remove_file(sandbox.path("index")).unwrap();
    let out = sandbox.upgrade(Path::new(env!("CARGO_BIN_EXE_bnto")), &[]);
    assert!(!out.status.success(), "{}", text(&out));
    assert!(
        text(&out).contains("Couldn't reach crates.io"),
        "{}",
        text(&out)
    );
}
