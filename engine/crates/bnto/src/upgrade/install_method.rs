// How this bnto was installed, decided from where the running binary lives:
// in cargo's bin dir means `cargo install`, so `bnto upgrade` can rerun it.

use std::path::{Path, PathBuf};

/// How the running bnto was installed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallMethod {
    /// `cargo install bnto`: the binary is in cargo's bin dir.
    Cargo,
    /// Anything else (a release download, a package manager, a local build).
    Other,
}

/// The args `bnto upgrade` runs cargo with. `--locked` builds with the
/// dependency versions bnto was released with.
pub const CARGO_INSTALL_ARGS: [&str; 3] = ["install", "bnto", "--locked"];

/// `Cargo` when `exe` sits in cargo's bin dir: `$CARGO_HOME/bin`, or
/// `~/.cargo/bin` when CARGO_HOME isn't set.
pub fn install_method(
    exe: &Path,
    cargo_home: Option<PathBuf>,
    home: Option<PathBuf>,
) -> InstallMethod {
    let cargo_home = cargo_home.or_else(|| home.map(|h| h.join(".cargo")));
    let in_cargo_bin =
        cargo_home.is_some_and(|dir| exe.parent() == Some(dir.join("bin").as_path()));
    if in_cargo_bin {
        InstallMethod::Cargo
    } else {
        InstallMethod::Other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_binary_in_cargo_home_bin_is_a_cargo_install() {
        let method = install_method(
            Path::new("/opt/cargo/bin/bnto"),
            Some(PathBuf::from("/opt/cargo")),
            Some(PathBuf::from("/home/me")),
        );
        assert_eq!(method, InstallMethod::Cargo);
    }

    #[test]
    fn without_cargo_home_the_default_cargo_dir_counts() {
        let method = install_method(
            Path::new("/home/me/.cargo/bin/bnto"),
            None,
            Some(PathBuf::from("/home/me")),
        );
        assert_eq!(method, InstallMethod::Cargo);
    }

    #[test]
    fn a_binary_anywhere_else_is_not() {
        for exe in [
            "/usr/local/bin/bnto",
            "/home/me/code/bnto/target/debug/bnto",
        ] {
            let method = install_method(
                Path::new(exe),
                Some(PathBuf::from("/opt/cargo")),
                Some(PathBuf::from("/home/me")),
            );
            assert_eq!(method, InstallMethod::Other, "{exe}");
        }
    }
}
