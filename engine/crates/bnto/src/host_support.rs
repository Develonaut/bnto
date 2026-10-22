// Whether a recipe runs on this machine: `bnto run` stops a recipe with no
// branch for the host before consent and before any step runs, and listings
// tag a recipe that misses an OS.

use bnto_core::HostPlatform;
use bnto_core::os_support::{describe_os, recipe_os};

/// Why `name` can't run on `host`, or `None` when it can. An OS bnto has no
/// name for (`host` is `None`) and a definition that doesn't parse are let
/// through: the run reports those its own way.
pub fn unsupported_host(
    name: &str,
    definition_json: &str,
    host: Option<HostPlatform>,
) -> Option<String> {
    let host = host?;
    let def = serde_json::from_str(definition_json).ok()?;
    let os = recipe_os(&def);
    (!os.contains(&host)).then(|| {
        format!(
            "{name} runs on {}. This machine runs {}.",
            describe_os(&os),
            host.label()
        )
    })
}

/// A short tag for listings, like `[macOS]`, or `None` for a recipe that
/// runs everywhere.
pub fn os_badge(os: &[HostPlatform]) -> Option<String> {
    (os.len() < HostPlatform::ALL.len()).then(|| format!("[{}]", describe_os(os)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEEP_AWAKE: &str = include_str!("../../bnto-engine/recipes/keep-awake.bnto.json");

    #[test]
    fn a_macos_recipe_is_refused_elsewhere() {
        let why = unsupported_host("keep-awake", KEEP_AWAKE, Some(HostPlatform::Linux));
        assert_eq!(
            why.as_deref(),
            Some("keep-awake runs on macOS. This machine runs Linux.")
        );
    }

    #[test]
    fn a_macos_recipe_runs_on_macos() {
        assert_eq!(
            unsupported_host("keep-awake", KEEP_AWAKE, Some(HostPlatform::MacOs)),
            None
        );
    }

    #[test]
    fn only_a_recipe_that_misses_an_os_gets_a_badge() {
        assert_eq!(os_badge(&HostPlatform::ALL), None);
        let some = [HostPlatform::MacOs, HostPlatform::Linux];
        assert_eq!(os_badge(&some).as_deref(), Some("[macOS and Linux]"));
    }

    #[test]
    fn an_unnamed_os_or_an_unreadable_recipe_is_let_through() {
        assert_eq!(unsupported_host("keep-awake", KEEP_AWAKE, None), None);
        let host = Some(HostPlatform::Windows);
        assert_eq!(unsupported_host("broken", "{", host), None);
    }
}
