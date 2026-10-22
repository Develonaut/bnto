// Release versions as the update check compares them: `X.Y.Z`, numerically.
// Pre-releases (`1.0.0-beta.1`) never count as an update.

/// A stable release version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(pub u64, pub u64, pub u64);

impl Version {
    /// `X.Y.Z` with numeric parts, or `None` (pre-releases included).
    pub fn parse(text: &str) -> Option<Self> {
        let mut parts = text.split('.').map(|p| p.parse::<u64>().ok());
        let version = Version(parts.next()??, parts.next()??, parts.next()??);
        parts.next().is_none().then_some(version)
    }

    /// The version this binary was built as.
    pub fn current() -> Option<Self> {
        Self::parse(env!("CARGO_PKG_VERSION"))
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.0, self.1, self.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_stable_version() {
        assert_eq!(Version::parse("0.16.1"), Some(Version(0, 16, 1)));
    }

    #[test]
    fn a_pre_release_or_garbage_is_not_a_version() {
        assert_eq!(Version::parse("1.0.0-beta.1"), None);
        assert_eq!(Version::parse("1.2"), None);
        assert_eq!(Version::parse("one.two.three"), None);
    }

    #[test]
    fn compares_numerically_not_as_text() {
        assert!(Version::parse("0.10.0") > Version::parse("0.9.0"));
    }

    #[test]
    fn displays_as_written() {
        assert_eq!(Version(0, 16, 1).to_string(), "0.16.1");
    }
}
