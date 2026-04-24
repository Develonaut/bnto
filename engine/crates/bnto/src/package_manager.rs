// Package manager detection and install command generation.
//
// Probes the host system's PATH for known package managers and builds
// the correct install invocation for each one.

use bnto_core::context::ProcessContext;

/// Supported system package managers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageManager {
    Brew,
    Apt,
    Pacman,
    Choco,
}

impl PackageManager {
    /// Human-readable name for display.
    pub fn name(self) -> &'static str {
        match self {
            Self::Brew => "Homebrew",
            Self::Apt => "apt",
            Self::Pacman => "pacman",
            Self::Choco => "Chocolatey",
        }
    }
}

/// Detect the first available package manager on the system.
///
/// Probes in preference order: brew → apt → pacman → choco.
/// Returns `None` if no known package manager is found.
pub fn detect(ctx: &dyn ProcessContext) -> Option<PackageManager> {
    let candidates = [
        ("brew", PackageManager::Brew),
        ("apt", PackageManager::Apt),
        ("pacman", PackageManager::Pacman),
        ("choco", PackageManager::Choco),
    ];
    for (binary, pm) in candidates {
        if ctx.run_command("which", &[binary]).is_ok() {
            return Some(pm);
        }
    }
    None
}

/// Build the install command for a given package manager and binary name.
///
/// Returns `(program, args)` — the caller runs `program` with `args`.
pub fn install_command(pm: PackageManager, binary: &str) -> (String, Vec<String>) {
    match pm {
        PackageManager::Brew => ("brew".into(), vec!["install".into(), binary.into()]),
        PackageManager::Apt => (
            "sudo".into(),
            vec!["apt".into(), "install".into(), "-y".into(), binary.into()],
        ),
        PackageManager::Pacman => (
            "sudo".into(),
            vec![
                "pacman".into(),
                "-S".into(),
                "--noconfirm".into(),
                binary.into(),
            ],
        ),
        PackageManager::Choco => (
            "choco".into(),
            vec!["install".into(), "-y".into(), binary.into()],
        ),
    }
}

/// Format an install command as a display string for the confirmation prompt.
pub fn display_command(pm: PackageManager, binary: &str) -> String {
    let (prog, args) = install_command(pm, binary);
    format!("{prog} {}", args.join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::errors::BntoError;
    use std::path::{Path, PathBuf};

    /// Mock context where only specific binaries are "found" by `which`.
    struct SelectiveContext {
        found: Vec<String>,
    }

    impl SelectiveContext {
        fn with(binaries: &[&str]) -> Self {
            Self {
                found: binaries.iter().map(|s| s.to_string()).collect(),
            }
        }
        fn none() -> Self {
            Self { found: vec![] }
        }
    }

    impl ProcessContext for SelectiveContext {
        fn run_command(&self, _cmd: &str, args: &[&str]) -> Result<Vec<u8>, BntoError> {
            if let Some(binary) = args.first()
                && self.found.contains(&binary.to_string())
            {
                return Ok(format!("/usr/bin/{binary}").into_bytes());
            }
            Err(BntoError::ProcessingFailed("not found".into()))
        }
        fn temp_file(&self, _suffix: &str) -> Result<PathBuf, BntoError> {
            Err(BntoError::ProcessingFailed("n/a".into()))
        }
        fn env_var(&self, _key: &str) -> Option<String> {
            None
        }
        fn work_dir(&self) -> Result<&Path, BntoError> {
            Err(BntoError::ProcessingFailed("n/a".into()))
        }
    }

    #[test]
    fn detect_brew() {
        let ctx = SelectiveContext::with(&["brew"]);
        assert_eq!(detect(&ctx), Some(PackageManager::Brew));
    }

    #[test]
    fn detect_apt() {
        let ctx = SelectiveContext::with(&["apt"]);
        assert_eq!(detect(&ctx), Some(PackageManager::Apt));
    }

    #[test]
    fn detect_pacman() {
        let ctx = SelectiveContext::with(&["pacman"]);
        assert_eq!(detect(&ctx), Some(PackageManager::Pacman));
    }

    #[test]
    fn detect_choco() {
        let ctx = SelectiveContext::with(&["choco"]);
        assert_eq!(detect(&ctx), Some(PackageManager::Choco));
    }

    #[test]
    fn detect_none_when_no_pm_available() {
        let ctx = SelectiveContext::none();
        assert_eq!(detect(&ctx), None);
    }

    #[test]
    fn detect_prefers_brew_over_apt() {
        let ctx = SelectiveContext::with(&["brew", "apt"]);
        assert_eq!(detect(&ctx), Some(PackageManager::Brew));
    }

    #[test]
    fn install_command_brew() {
        let (prog, args) = install_command(PackageManager::Brew, "yt-dlp");
        assert_eq!(prog, "brew");
        assert_eq!(args, vec!["install", "yt-dlp"]);
    }

    #[test]
    fn install_command_apt() {
        let (prog, args) = install_command(PackageManager::Apt, "ffmpeg");
        assert_eq!(prog, "sudo");
        assert_eq!(args, vec!["apt", "install", "-y", "ffmpeg"]);
    }

    #[test]
    fn install_command_pacman() {
        let (prog, args) = install_command(PackageManager::Pacman, "ffmpeg");
        assert_eq!(prog, "sudo");
        assert_eq!(args, vec!["pacman", "-S", "--noconfirm", "ffmpeg"]);
    }

    #[test]
    fn install_command_choco() {
        let (prog, args) = install_command(PackageManager::Choco, "yt-dlp");
        assert_eq!(prog, "choco");
        assert_eq!(args, vec!["install", "-y", "yt-dlp"]);
    }

    #[test]
    fn display_command_formats_correctly() {
        assert_eq!(
            display_command(PackageManager::Brew, "yt-dlp"),
            "brew install yt-dlp"
        );
        assert_eq!(
            display_command(PackageManager::Apt, "ffmpeg"),
            "sudo apt install -y ffmpeg"
        );
    }

    #[test]
    fn name_returns_human_readable() {
        assert_eq!(PackageManager::Brew.name(), "Homebrew");
        assert_eq!(PackageManager::Apt.name(), "apt");
        assert_eq!(PackageManager::Pacman.name(), "pacman");
        assert_eq!(PackageManager::Choco.name(), "Chocolatey");
    }
}
