// The one-line "a newer bnto is out" notice, shown on stderr.

use super::version::Version;

/// "bnto 0.17.0 is available (you have 0.16.1). Run `bnto upgrade`."
pub fn notice(current: Version, latest: Version) -> String {
    format!("bnto {latest} is available (you have {current}). Run `bnto upgrade`.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_both_versions_and_the_command() {
        assert_eq!(
            notice(Version(0, 16, 1), Version(0, 17, 0)),
            "bnto 0.17.0 is available (you have 0.16.1). Run `bnto upgrade`."
        );
    }
}
