// A home of its own for each `bnto` a test runs, so no test reads the
// developer's recipe library or writes session logs into their ~/.bnto.

use std::path::{Path, PathBuf};
use std::process::Command;

use super::bnto_bin;

/// Temp HOME, BNTO_HOME, config dir and working directory for `bnto` runs.
/// Keep it alive until the runs finish: dropping it deletes the dirs.
pub struct Sandbox {
    home: tempfile::TempDir,
    work: tempfile::TempDir,
}

impl Sandbox {
    pub fn new() -> Self {
        Self {
            home: tempfile::tempdir().unwrap(),
            work: tempfile::tempdir().unwrap(),
        }
    }

    /// `bnto` run inside the sandbox. Telemetry is off by env, which also
    /// keeps Windows (whose config dir ignores env vars) from writing state.
    pub fn command(&self) -> Command {
        let mut cmd = Command::new(bnto_bin());
        cmd.current_dir(self.work.path())
            .env("HOME", self.home.path())
            .env("BNTO_HOME", self.bnto_home())
            .env("XDG_CONFIG_HOME", self.home.path().join(".config"))
            .env("BNTO_TELEMETRY_DISABLED", "1");
        cmd
    }

    /// The sandbox's bnto home: put library recipes in its `recipes/`.
    pub fn bnto_home(&self) -> PathBuf {
        self.home.path().join(".bnto")
    }

    /// The working directory `command()` runs in, for relative paths.
    pub fn work_dir(&self) -> &Path {
        self.work.path()
    }
}
