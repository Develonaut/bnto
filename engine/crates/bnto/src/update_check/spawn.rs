// Refresh the cached check in a detached `bnto __check-for-updates`, so the
// command that noticed the stale cache never waits on the network and the
// refresh survives that command exiting.

use std::process::{Command, Stdio};

/// The hidden subcommand that runs one check and saves the result.
pub const CHECK_COMMAND: &str = "__check-for-updates";

/// Start the refresh and return at once. Failure to start is silent: the
/// next command tries again.
pub fn spawn_refresh() {
    let Ok(exe) = std::env::current_exe() else {
        return;
    };
    let mut command = Command::new(exe);
    command
        .arg(CHECK_COMMAND)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    detach(&mut command);
    let _ = command.spawn();
}

/// Its own process group (Unix) or no console (Windows), so the terminal's
/// Ctrl+C or closing window doesn't take it down mid-write.
#[cfg(unix)]
fn detach(command: &mut Command) {
    use std::os::unix::process::CommandExt;
    command.process_group(0);
}

#[cfg(windows)]
fn detach(command: &mut Command) {
    use std::os::windows::process::CommandExt;
    const DETACHED_PROCESS: u32 = 0x0000_0008;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    command.creation_flags(DETACHED_PROCESS | CREATE_NO_WINDOW);
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn the_cli_runs_the_check_under_this_name() {
        let cli = crate::cli::Cli::try_parse_from(["bnto", CHECK_COMMAND]).unwrap();
        assert!(matches!(
            cli.command,
            Some(crate::cli::Command::CheckForUpdates)
        ));
    }
}
