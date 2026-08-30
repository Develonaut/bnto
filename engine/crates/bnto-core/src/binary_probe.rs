//! Platform-aware probing for whether a binary exists on PATH.
//!
//! Dependency checks shell out to a lookup command rather than walking
//! PATH themselves, so the command name has to match the host platform.

/// Name of the PATH-lookup command for the host platform.
///
/// Unix has `which`. Windows has no `which` on a stock PATH — the
/// equivalent is `where.exe`, which also applies PATHEXT, so
/// `where yt-dlp` resolves `yt-dlp.exe`. Spawning `where` directly
/// invokes the executable, not PowerShell's `Where-Object` alias.
pub const fn probe_command() -> &'static str {
    if cfg!(windows) { "where" } else { "which" }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_command_matches_host_platform() {
        if cfg!(windows) {
            assert_eq!(probe_command(), "where");
        } else {
            assert_eq!(probe_command(), "which");
        }
    }

    #[test]
    fn probe_command_is_never_empty() {
        assert!(!probe_command().is_empty());
    }
}
