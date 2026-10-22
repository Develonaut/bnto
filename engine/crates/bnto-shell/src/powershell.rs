// PowerShell script mode: a literal script run as `<abs path> -NoLogo
// -NoProfile -NonInteractive -OutputFormat Text -EncodedCommand <base64>`,
// with nothing after it. `-Command <script> <args>` would append the args to the code, so values
// reach the script only as $env:BNTO_ARG_1..N (SEC-32..37). See
// decisions/powershell-script-mode.md.

use std::path::{Path, PathBuf};

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use bnto_core::HostPlatform;

/// Shells that run a PowerShell script.
pub(crate) const SHELLS: &[&str] = &["powershell", "pwsh"];

/// The prefix of the variables that carry a step's values.
pub(crate) const ARG_PREFIX: &str = "BNTO_ARG_";

/// Fixed engine text run before every script: no progress records on
/// stderr, UTF-8 stdout like every other step, and no ANSI colors in error
/// text (`$PSStyle` exists from pwsh 7.2).
const PRELUDE: &str = "$ProgressPreference = 'SilentlyContinue'\n\
                       [Console]::OutputEncoding = [System.Text.UTF8Encoding]::new($false)\n\
                       if ($PSStyle) { $PSStyle.OutputRendering = 'PlainText' }\n";

/// Flags before the encoded script: no banner, no profile, no prompts.
/// `-OutputFormat Text` must be explicit: an encoded, non-interactive
/// command with stderr redirected otherwise writes its errors as `#< CLIXML`.
const FLAGS: &[&str] = &[
    "-NoLogo",
    "-NoProfile",
    "-NonInteractive",
    "-OutputFormat",
    "Text",
    "-EncodedCommand",
];

/// The longest command line allowed, under Windows' 32,767 characters with
/// room for the program path.
const MAX_COMMAND_LINE: usize = 30_000;

/// Whether `shell` runs PowerShell.
pub fn is_powershell(shell: &str) -> bool {
    SHELLS.contains(&shell)
}

/// The program and every arg for a PowerShell script: `<abs path> -NoLogo
/// -NoProfile -NonInteractive -OutputFormat Text -EncodedCommand <script>`.
/// Nothing may follow.
pub(crate) fn invocation(shell: &str, script: &str) -> Result<(PathBuf, Vec<String>), String> {
    let system_root = std::env::var("SystemRoot").ok();
    let candidates = candidates(shell, HostPlatform::current(), system_root.as_deref())?;
    let path = crate::script::first_installed(shell, &candidates, Path::exists)?;
    Ok((path, args(script)))
}

fn args(script: &str) -> Vec<String> {
    let mut args: Vec<String> = FLAGS.iter().map(|flag| flag.to_string()).collect();
    args.push(encode(script));
    args
}

/// The script as `-EncodedCommand` takes it: base64 of UTF-16LE, prelude first.
fn encode(script: &str) -> String {
    let utf16: Vec<u8> = PRELUDE
        .encode_utf16()
        .chain(script.encode_utf16())
        .flat_map(u16::to_le_bytes)
        .collect();
    STANDARD.encode(utf16)
}

/// A script too long for a Windows command line. Checked on every OS, so a
/// recipe acts the same everywhere.
pub(crate) fn length_problem(script: &str) -> Option<String> {
    let length: usize = args(script).iter().map(|arg| arg.len() + 1).sum();
    (length > MAX_COMMAND_LINE).then(|| {
        format!(
            "the PowerShell script is too long for a command line once encoded \
             ({length} of {MAX_COMMAND_LINE} characters); make it shorter"
        )
    })
}

/// A step's values as the variables its script reads: `BNTO_ARG_1`…`N`, then
/// `BNTO_ARG_COUNT`.
pub(crate) fn value_env(values: &[String]) -> Vec<(String, String)> {
    let numbered = values
        .iter()
        .enumerate()
        .map(|(i, value)| (format!("{ARG_PREFIX}{}", i + 1), value.clone()));
    let count = (format!("{ARG_PREFIX}COUNT"), values.len().to_string());
    numbered.chain([count]).collect()
}

/// Where `shell` may be on `os`, by absolute path only — never PATH.
fn candidates(
    shell: &str,
    os: Option<HostPlatform>,
    system_root: Option<&str>,
) -> Result<Vec<PathBuf>, String> {
    match (shell, os) {
        ("powershell", Some(HostPlatform::Windows)) => Ok(windows_powershell(system_root)),
        ("powershell", _) => Err("'powershell' is Windows PowerShell, which only Windows \
                                  has; use 'pwsh' (PowerShell 7) elsewhere"
            .to_string()),
        ("pwsh", Some(os)) => Ok(pwsh_paths(os).iter().map(PathBuf::from).collect()),
        ("pwsh", None) => Err("bnto doesn't know where pwsh lives on this OS".to_string()),
        _ => Err(format!("unknown shell '{shell}'")),
    }
}

/// `%SystemRoot%`'s copy first, then the default Windows folder's.
fn windows_powershell(system_root: Option<&str>) -> Vec<PathBuf> {
    let under =
        |root: &str| PathBuf::from(root).join(r"System32\WindowsPowerShell\v1.0\powershell.exe");
    let mut paths: Vec<PathBuf> = system_root.into_iter().map(under).collect();
    paths.push(under(r"C:\Windows"));
    paths.dedup();
    paths
}

fn pwsh_paths(os: HostPlatform) -> &'static [&'static str] {
    match os {
        HostPlatform::Windows => &[r"C:\Program Files\PowerShell\7\pwsh.exe"],
        HostPlatform::MacOs => &[
            "/usr/local/bin/pwsh",
            "/opt/homebrew/bin/pwsh",
            "/usr/local/microsoft/powershell/7/pwsh",
        ],
        HostPlatform::Linux => &[
            "/usr/bin/pwsh",
            "/opt/microsoft/powershell/7/pwsh",
            "/snap/bin/pwsh",
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decode(encoded: &str) -> String {
        let bytes = STANDARD.decode(encoded).unwrap();
        let units: Vec<u16> = bytes
            .chunks(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect();
        String::from_utf16(&units).unwrap()
    }

    #[test]
    fn the_script_is_the_last_arg_and_nothing_follows_it() {
        let args = args("Write-Output $env:BNTO_ARG_1");
        assert_eq!(
            args[..6],
            [
                "-NoLogo",
                "-NoProfile",
                "-NonInteractive",
                "-OutputFormat",
                "Text",
                "-EncodedCommand"
            ]
        );
        assert_eq!(args.len(), 7);
        assert!(!args.iter().any(|a| a == "-Command" || a == "-File"));
    }

    #[test]
    fn the_encoded_script_is_utf16le_base64_of_the_prelude_then_the_script() {
        let script = "Write-Output \"héllo ✓\"";
        let decoded = decode(&encode(script));
        assert_eq!(decoded, format!("{PRELUDE}{script}"));
        assert!(decoded.starts_with("$ProgressPreference = 'SilentlyContinue'\n"));
    }

    #[test]
    fn values_become_numbered_variables_and_a_count() {
        let values = vec!["a b".to_string(), String::new(), "$(rm -r /)".to_string()];
        assert_eq!(
            value_env(&values),
            [
                ("BNTO_ARG_1".to_string(), "a b".to_string()),
                ("BNTO_ARG_2".to_string(), String::new()),
                ("BNTO_ARG_3".to_string(), "$(rm -r /)".to_string()),
                ("BNTO_ARG_COUNT".to_string(), "3".to_string()),
            ]
        );
        assert_eq!(
            value_env(&[]),
            [("BNTO_ARG_COUNT".to_string(), "0".to_string())]
        );
    }

    #[test]
    fn a_script_too_long_for_a_windows_command_line_is_refused() {
        assert!(length_problem(&"x".repeat(10_000)).is_none());
        let problem = length_problem(&"x".repeat(12_000)).unwrap();
        assert!(problem.contains("too long"), "{problem}");
    }

    #[test]
    fn windows_powershell_runs_only_on_windows_from_system_root() {
        let found = candidates("powershell", Some(HostPlatform::Windows), Some(r"D:\Win")).unwrap();
        assert_eq!(
            found,
            [
                PathBuf::from(r"D:\Win").join(r"System32\WindowsPowerShell\v1.0\powershell.exe"),
                PathBuf::from(r"C:\Windows")
                    .join(r"System32\WindowsPowerShell\v1.0\powershell.exe"),
            ]
        );
        for os in [HostPlatform::MacOs, HostPlatform::Linux] {
            let err = candidates("powershell", Some(os), None).unwrap_err();
            assert!(err.contains("use 'pwsh'"), "{err}");
        }
    }

    #[test]
    fn pwsh_is_looked_for_at_fixed_absolute_paths_on_every_os() {
        for os in HostPlatform::ALL {
            let found = candidates("pwsh", Some(os), None).unwrap();
            assert!(!found.is_empty(), "{os:?}");
            for path in found {
                let text = path.to_string_lossy().into_owned();
                assert!(text.starts_with('/') || text.starts_with(r"C:\"), "{text}");
            }
        }
        assert!(candidates("pwsh", None, None).is_err());
        assert!(candidates("sh", Some(HostPlatform::Linux), None).is_err());
    }
}
