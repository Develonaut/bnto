// `bnto upgrade`: install the newest bnto release. A cargo install is
// upgraded by rerunning `cargo install bnto --locked`; anything else gets
// instructions instead of a guess (.claude/strategy/cli-upgrade.md).

mod install_method;
mod swap;

use std::path::Path;
use std::process::Command;

use crate::storage::BntoPaths;
use crate::update_check::{self, Version};
use install_method::{CARGO_INSTALL_ARGS, InstallMethod, install_method};

pub use swap::remove_leftover;

/// Run `bnto upgrade` (`check_only`: `bnto upgrade --check`).
pub fn run_upgrade(check_only: bool, paths: Option<&BntoPaths>) {
    let current = Version::current().expect("bnto's own version is X.Y.Z");
    let Some(latest) = update_check::latest_release() else {
        eprintln!("Couldn't reach crates.io to look for a newer bnto. Try again later.");
        std::process::exit(1);
    };
    if let Some(paths) = paths {
        update_check::record(paths, latest);
    }
    if latest <= current {
        println!("bnto {current} is the latest.");
    } else if check_only {
        println!("bnto {latest} is available (you have {current}). Run `bnto upgrade`.");
    } else {
        install(current, latest);
    }
}

/// Install `latest` over the running `current`, or explain how to.
fn install(current: Version, latest: Version) {
    let exe = std::env::current_exe().expect("the running binary's path");
    let cargo_home = std::env::var_os("CARGO_HOME").map(Into::into);
    if install_method(&exe, cargo_home, dirs::home_dir()) == InstallMethod::Other {
        println!("bnto {latest} is available (you have {current}).");
        println!("This bnto wasn't installed with cargo ({}).", exe.display());
        println!(
            "Upgrade it the way you installed it, or install with: cargo install bnto --locked"
        );
        std::process::exit(1);
    }
    println!("Upgrading bnto {current} → {latest}...");
    if cargo_install(&exe) {
        println!("Upgraded bnto {current} → {latest}.");
    } else {
        eprintln!("cargo install failed; bnto {current} is still installed.");
        std::process::exit(1);
    }
}

/// Run `cargo install bnto --locked` in the user's terminal. On Windows the
/// running binary is moved aside first, and put back if cargo fails.
fn cargo_install(exe: &Path) -> bool {
    let moved = cfg!(windows).then(|| swap::move_aside(exe).ok()).flatten();
    let ok = Command::new(cargo_program(exe))
        .args(CARGO_INSTALL_ARGS)
        .status()
        .is_ok_and(|status| status.success());
    if !ok && let Some(old) = &moved {
        swap::restore(old, exe);
    }
    ok
}

/// The cargo next to this bnto when there is one (the same toolchain that
/// installed it), else `cargo` from PATH.
fn cargo_program(exe: &Path) -> std::path::PathBuf {
    let sibling = exe.with_file_name(format!("cargo{}", std::env::consts::EXE_SUFFIX));
    if sibling.exists() {
        sibling
    } else {
        "cargo".into()
    }
}
