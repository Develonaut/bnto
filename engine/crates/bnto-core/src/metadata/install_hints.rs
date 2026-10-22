// Per-platform install hints for external dependencies.
//
// Package managers and package IDs differ by OS (`brew install yt-dlp` vs
// `winget install yt-dlp.yt-dlp`), so a single hint string can't serve
// every platform. Unset platforms fall back to `Dependency::install_hint`,
// then to `GENERIC_INSTALL_HINT`.

use serde::{Deserialize, Serialize};
#[cfg(feature = "ts")]
use ts_rs::TS;

/// Operating-system family an install hint targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostPlatform {
    MacOs,
    Linux,
    Windows,
}

impl HostPlatform {
    /// Every platform, in the order bnto lists them.
    pub const ALL: [Self; 3] = [Self::MacOs, Self::Linux, Self::Windows];

    /// The name `{{ctx.platform}}` reads, and a platform guard compares.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MacOs => "macos",
            Self::Linux => "linux",
            Self::Windows => "windows",
        }
    }

    /// The name people know it by.
    pub const fn label(self) -> &'static str {
        match self {
            Self::MacOs => "macOS",
            Self::Linux => "Linux",
            Self::Windows => "Windows",
        }
    }

    /// The platform this binary was compiled for. `None` on targets with
    /// no host OS (WASM) or OSes without hints (BSDs), which use the fallback.
    pub const fn current() -> Option<Self> {
        if cfg!(target_os = "macos") {
            Some(Self::MacOs)
        } else if cfg!(target_os = "linux") {
            Some(Self::Linux)
        } else if cfg!(windows) {
            Some(Self::Windows)
        } else {
            None
        }
    }
}

/// Install commands keyed by platform. Empty entries mean "no specific
/// hint" and defer to the dependency's generic `installHint`.
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(
    feature = "ts",
    ts(
        export,
        export_to = "../../../../packages/@bnto/nodes/src/generated/definitionTypes/"
    )
)]
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct InstallHints {
    /// Install command for macOS (e.g. `brew install yt-dlp`).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub macos: String,
    /// Install command for Linux (e.g. `sudo apt install yt-dlp`).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub linux: String,
    /// Install command for Windows (e.g. `winget install yt-dlp.yt-dlp`).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub windows: String,
}

impl InstallHints {
    /// The hint for `platform`, or `None` if that platform has none set.
    pub fn for_platform(&self, platform: HostPlatform) -> Option<&str> {
        let hint = match platform {
            HostPlatform::MacOs => &self.macos,
            HostPlatform::Linux => &self.linux,
            HostPlatform::Windows => &self.windows,
        };
        (!hint.is_empty()).then_some(hint.as_str())
    }

    /// True when no platform has a hint — lets serde omit the field.
    pub fn is_empty(&self) -> bool {
        self.macos.is_empty() && self.linux.is_empty() && self.windows.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hints() -> InstallHints {
        InstallHints {
            macos: "brew install yt-dlp".to_string(),
            linux: String::new(),
            windows: "winget install yt-dlp.yt-dlp".to_string(),
        }
    }

    #[test]
    fn names_match_what_ctx_platform_reads() {
        let names = HostPlatform::ALL.map(HostPlatform::as_str);
        assert_eq!(names, ["macos", "linux", "windows"]);
        assert_eq!(HostPlatform::MacOs.label(), "macOS");
    }

    #[test]
    fn for_platform_returns_matching_hint() {
        let h = hints();
        assert_eq!(
            h.for_platform(HostPlatform::MacOs),
            Some("brew install yt-dlp")
        );
        assert_eq!(
            h.for_platform(HostPlatform::Windows),
            Some("winget install yt-dlp.yt-dlp")
        );
    }

    #[test]
    fn for_platform_returns_none_when_unset() {
        assert_eq!(hints().for_platform(HostPlatform::Linux), None);
    }

    #[test]
    fn is_empty_only_when_no_platform_set() {
        assert!(InstallHints::default().is_empty());
        assert!(!hints().is_empty());
    }

    #[test]
    fn deserializes_partial_object() {
        let h: InstallHints = serde_json::from_str(r#"{ "windows": "winget install x" }"#).unwrap();
        assert_eq!(h.windows, "winget install x");
        assert!(h.macos.is_empty() && h.linux.is_empty());
    }

    #[test]
    fn serializes_without_empty_platforms() {
        let json = serde_json::to_string(&hints()).unwrap();
        assert!(!json.contains("linux"), "empty platforms omitted: {json}");
    }

    #[test]
    fn current_matches_compile_target() {
        let expected = if cfg!(target_os = "macos") {
            Some(HostPlatform::MacOs)
        } else if cfg!(target_os = "linux") {
            Some(HostPlatform::Linux)
        } else if cfg!(windows) {
            Some(HostPlatform::Windows)
        } else {
            None
        };
        assert_eq!(HostPlatform::current(), expected);
    }
}
