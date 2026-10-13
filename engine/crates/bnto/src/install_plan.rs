// Per-dependency install steps for `bnto install`.
//
// A dependency with an `installHints` entry for the host platform runs that
// exact command through the platform shell; otherwise the detected package
// manager installs it by binary name, or the user installs it manually.

use bnto_core::context::ProcessContext;
use bnto_core::metadata::{Dependency, HostPlatform};

use crate::package_manager::{self, PackageManager};

/// How one missing dependency gets installed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallStep {
    /// Run the recipe author's host-platform command through the shell.
    HostHint(String),
    /// Run `<pm> install <binary>` (argument vector, no shell).
    PackageManager(PackageManager, String),
    /// Nothing runnable — the user installs it by following the hint.
    Manual(String),
}

impl InstallStep {
    /// Choose the step for `dep`: host hint, then package manager, then manual.
    pub fn plan(
        dep: &Dependency,
        platform: Option<HostPlatform>,
        pm: Option<PackageManager>,
    ) -> Self {
        let host_hint = platform.and_then(|p| dep.install_hints.for_platform(p));
        match (host_hint, pm) {
            (Some(hint), _) => Self::HostHint(hint.to_string()),
            (None, Some(pm)) => Self::PackageManager(pm, dep.binary.clone()),
            (None, None) => Self::Manual(dep.install_hint_for(platform).to_string()),
        }
    }

    /// The exact command this step runs, or the manual hint.
    pub fn display(&self) -> String {
        match self {
            Self::HostHint(command) | Self::Manual(command) => command.clone(),
            Self::PackageManager(pm, binary) => package_manager::display_command(*pm, binary),
        }
    }

    /// Whether `bnto install` can run this step itself.
    pub fn is_runnable(&self) -> bool {
        !matches!(self, Self::Manual(_))
    }

    /// Whether this step runs a recipe author's command string.
    pub fn is_author_command(&self) -> bool {
        matches!(self, Self::HostHint(_))
    }

    /// Run the step via `ctx`. Manual steps are an error — nothing to run.
    pub fn run(&self, ctx: &dyn ProcessContext) -> Result<(), String> {
        let (prog, args) = match self {
            Self::HostHint(command) => shell_invocation(command),
            Self::PackageManager(pm, binary) => package_manager::install_command(*pm, binary),
            Self::Manual(_) => return Err("no install command to run".to_string()),
        };
        let arg_refs: Vec<&str> = args.iter().map(String::as_str).collect();
        ctx.run_command(&prog, &arg_refs)
            .map(|_| ())
            .map_err(|e| e.to_string())
    }
}

/// The platform shell invocation that runs `command` verbatim. Hints are
/// whole command lines (`sudo apt install ffmpeg`), so they need a shell.
fn shell_invocation(command: &str) -> (String, Vec<String>) {
    let (shell, flag) = if cfg!(windows) {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };
    (
        shell.to_string(),
        vec![flag.to_string(), command.to_string()],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::errors::BntoError;
    use bnto_core::metadata::InstallHints;
    use std::path::{Path, PathBuf};
    use std::sync::Mutex;

    /// Records every command instead of spawning it.
    #[derive(Default)]
    struct RecordingContext {
        calls: Mutex<Vec<(String, Vec<String>)>>,
    }

    impl RecordingContext {
        fn calls(&self) -> Vec<(String, Vec<String>)> {
            self.calls.lock().unwrap().clone()
        }
    }

    impl ProcessContext for RecordingContext {
        fn run_command(&self, cmd: &str, args: &[&str]) -> Result<Vec<u8>, BntoError> {
            let args = args.iter().map(|a| a.to_string()).collect();
            self.calls.lock().unwrap().push((cmd.to_string(), args));
            Ok(Vec::new())
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

    fn ffmpeg(hints: InstallHints) -> Dependency {
        Dependency {
            binary: "ffmpeg".into(),
            version: String::new(),
            install_hint: "see https://ffmpeg.org/download.html".into(),
            install_hints: hints,
            homepage: String::new(),
        }
    }

    fn linux_hint() -> InstallHints {
        InstallHints {
            linux: "sudo apt install ffmpeg".into(),
            ..Default::default()
        }
    }

    #[test]
    fn plan_uses_host_hint_when_present() {
        let step = InstallStep::plan(
            &ffmpeg(linux_hint()),
            Some(HostPlatform::Linux),
            Some(PackageManager::Brew),
        );
        assert_eq!(
            step,
            InstallStep::HostHint("sudo apt install ffmpeg".into())
        );
        assert_eq!(step.display(), "sudo apt install ffmpeg");
    }

    #[test]
    fn plan_falls_back_to_package_manager_without_host_hint() {
        let step = InstallStep::plan(
            &ffmpeg(linux_hint()),
            Some(HostPlatform::Windows),
            Some(PackageManager::Choco),
        );
        assert_eq!(
            step,
            InstallStep::PackageManager(PackageManager::Choco, "ffmpeg".into())
        );
        assert_eq!(step.display(), "choco install -y ffmpeg");
    }

    #[test]
    fn plan_ignores_generic_hint_as_a_command() {
        // The generic `installHint` is prose ("see https://..."), never run.
        let step = InstallStep::plan(&ffmpeg(InstallHints::default()), None, None);
        assert_eq!(
            step,
            InstallStep::Manual("see https://ffmpeg.org/download.html".into())
        );
        assert!(!step.is_runnable());
    }

    #[test]
    fn hint_and_package_manager_steps_are_runnable() {
        assert!(InstallStep::HostHint("x".into()).is_runnable());
        assert!(InstallStep::PackageManager(PackageManager::Brew, "x".into()).is_runnable());
    }

    #[test]
    fn run_host_hint_passes_exact_string_to_platform_shell() {
        let ctx = RecordingContext::default();
        let step = InstallStep::HostHint("winget install Gyan.FFmpeg".into());
        step.run(&ctx).unwrap();
        let (shell, flag) = if cfg!(windows) {
            ("cmd", "/C")
        } else {
            ("sh", "-c")
        };
        assert_eq!(
            ctx.calls(),
            vec![(
                shell.to_string(),
                vec![flag.to_string(), "winget install Gyan.FFmpeg".to_string()]
            )]
        );
    }

    #[test]
    fn run_package_manager_uses_argument_vector() {
        let ctx = RecordingContext::default();
        InstallStep::PackageManager(PackageManager::Brew, "yt-dlp".into())
            .run(&ctx)
            .unwrap();
        assert_eq!(
            ctx.calls(),
            vec![(
                "brew".to_string(),
                vec!["install".to_string(), "yt-dlp".to_string()]
            )]
        );
    }

    #[test]
    fn run_manual_step_spawns_nothing() {
        let ctx = RecordingContext::default();
        assert!(InstallStep::Manual("see docs".into()).run(&ctx).is_err());
        assert!(ctx.calls().is_empty());
    }
}
