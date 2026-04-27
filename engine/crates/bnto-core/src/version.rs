//! Version constraint checking for external dependencies.
//!
//! Parses `<binary> --version` output, extracts the version number, and
//! validates it against the constraint string on `Dependency.version`
//! (e.g., `">=6.0"`). Used by the pre-flight dependency check before
//! pipeline execution and by `bnto doctor`.

use crate::errors::BntoError;

// --- Types ---

/// A parsed version constraint (e.g., `>=6.0` → `Gte`, `[6, 0]`).
#[derive(Debug, Clone, PartialEq)]
pub struct VersionConstraint {
    pub op: ConstraintOp,
    pub segments: Vec<u64>,
}

/// Comparison operator for a version constraint.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConstraintOp {
    /// `>=` — installed version must be greater than or equal to the constraint.
    Gte,
}

// --- Parsing ---

/// Extract a version-like string from `--version` output.
///
/// Scans the output for the first sequence of dot-separated numbers
/// (e.g., `"6.1.1"` from `"ffmpeg version 6.1.1 Copyright..."`).
/// Returns `None` if no version pattern is found.
pub fn extract_version(output: &str) -> Option<&str> {
    // Find the first digit-dot-digit pattern.
    let bytes = output.as_bytes();
    let mut start = None;
    let mut end = 0;

    for (i, &b) in bytes.iter().enumerate() {
        match (start, b) {
            // No match yet — look for first digit.
            (None, b'0'..=b'9') => {
                start = Some(i);
                end = i + 1;
            }
            // Inside a version string — extend on digits or dots.
            (Some(_), b'0'..=b'9' | b'.') => {
                end = i + 1;
            }
            // Inside a version string — non-digit/dot ends it.
            (Some(_), _) => break,
            // Not started, not a digit — keep scanning.
            _ => {}
        }
    }

    let s = start?;
    let candidate = &output[s..end];

    // Must contain at least one dot (e.g., "6.0", not just "6").
    if !candidate.contains('.') {
        return None;
    }

    // Trim trailing dots (e.g., "6.1." → "6.1").
    Some(candidate.trim_end_matches('.'))
}

/// Parse a constraint string like `">=6.0"` into a `VersionConstraint`.
pub fn parse_constraint(constraint: &str) -> Result<VersionConstraint, BntoError> {
    let trimmed = constraint.trim();

    if trimmed.is_empty() {
        return Err(BntoError::InvalidInput(
            "Empty version constraint".to_string(),
        ));
    }

    // Extract operator and version string.
    let (op, version_str) = if let Some(rest) = trimmed.strip_prefix(">=") {
        (ConstraintOp::Gte, rest.trim())
    } else {
        return Err(BntoError::InvalidInput(format!(
            "Unsupported version constraint operator in '{trimmed}'. Only >= is supported."
        )));
    };

    let segments = parse_segments(version_str)?;

    Ok(VersionConstraint { op, segments })
}

/// Parse a dotted version string into numeric segments.
fn parse_segments(version: &str) -> Result<Vec<u64>, BntoError> {
    if version.is_empty() {
        return Err(BntoError::InvalidInput("Empty version string".to_string()));
    }

    version
        .split('.')
        .map(|part| {
            part.parse::<u64>().map_err(|_| {
                BntoError::InvalidInput(format!("Invalid version segment '{part}' in '{version}'"))
            })
        })
        .collect()
}

// --- Comparison ---

/// Check if an installed version satisfies a constraint.
///
/// Compares segment-by-segment. Missing trailing segments are treated as 0
/// (e.g., `"6.0"` == `"6.0.0"`). This handles both semver (`6.1.1`) and
/// calendar versioning (`2024.12.23`) since both are dot-separated numerics.
pub fn satisfies(installed: &str, constraint: &VersionConstraint) -> bool {
    let installed_segments = match parse_segments(installed) {
        Ok(s) => s,
        Err(_) => return false,
    };

    match constraint.op {
        ConstraintOp::Gte => compare_segments(&installed_segments, &constraint.segments) >= 0,
    }
}

/// Compare two version segment lists.
/// Returns negative if a < b, 0 if equal, positive if a > b.
fn compare_segments(a: &[u64], b: &[u64]) -> i64 {
    let max_len = a.len().max(b.len());
    for i in 0..max_len {
        let av = a.get(i).copied().unwrap_or(0);
        let bv = b.get(i).copied().unwrap_or(0);
        if av != bv {
            return av as i64 - bv as i64;
        }
    }
    0
}

/// Run `<binary> --version` and check if the installed version satisfies
/// the constraint. Returns the installed version string on success, or
/// `None` if version couldn't be determined (binary missing, no parseable
/// output, etc.).
pub fn check_version(
    binary: &str,
    constraint_str: &str,
    ctx: &dyn crate::ProcessContext,
) -> VersionCheckResult {
    // Parse the constraint first — if it's invalid, skip the check.
    let constraint = match parse_constraint(constraint_str) {
        Ok(c) => c,
        Err(_) => return VersionCheckResult::Skipped,
    };

    // Run `<binary> --version` and capture stdout.
    let output = match ctx.run_command(binary, &["--version"]) {
        Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
        Err(_) => return VersionCheckResult::Skipped,
    };

    // Extract version from output.
    let installed = match extract_version(&output) {
        Some(v) => v.to_string(),
        None => return VersionCheckResult::Skipped,
    };

    let satisfied = satisfies(&installed, &constraint);

    VersionCheckResult::Checked {
        installed,
        satisfied,
    }
}

/// Result of a version check against a constraint.
#[derive(Debug, Clone, PartialEq)]
pub enum VersionCheckResult {
    /// Version was checked — includes the installed version and whether
    /// it satisfies the constraint.
    Checked { installed: String, satisfied: bool },
    /// Version check was skipped (no constraint, unparseable output, etc.).
    Skipped,
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // --- extract_version ---

    #[test]
    fn test_extract_version_ffmpeg() {
        let output = "ffmpeg version 6.1.1 Copyright (c) 2000-2023";
        assert_eq!(extract_version(output), Some("6.1.1"));
    }

    #[test]
    fn test_extract_version_ytdlp() {
        // yt-dlp prints just the version string on a line.
        let output = "2024.12.23";
        assert_eq!(extract_version(output), Some("2024.12.23"));
    }

    #[test]
    fn test_extract_version_multiline() {
        let output = "yt-dlp 2024.12.23 [some extra info]\nMore lines here";
        assert_eq!(extract_version(output), Some("2024.12.23"));
    }

    #[test]
    fn test_extract_version_no_match() {
        let output = "some random output without version numbers";
        assert_eq!(extract_version(output), None);
    }

    #[test]
    fn test_extract_version_bare_number() {
        // Single number without dots should not match.
        let output = "version 6";
        assert_eq!(extract_version(output), None);
    }

    #[test]
    fn test_extract_version_trailing_dot() {
        let output = "tool 1.2.";
        assert_eq!(extract_version(output), Some("1.2"));
    }

    // --- parse_constraint ---

    #[test]
    fn test_parse_constraint_gte() {
        let c = parse_constraint(">=6.0").unwrap();
        assert_eq!(c.op, ConstraintOp::Gte);
        assert_eq!(c.segments, vec![6, 0]);
    }

    #[test]
    fn test_parse_constraint_gte_three_parts() {
        let c = parse_constraint(">=2024.0.0").unwrap();
        assert_eq!(c.op, ConstraintOp::Gte);
        assert_eq!(c.segments, vec![2024, 0, 0]);
    }

    #[test]
    fn test_parse_constraint_empty_is_error() {
        assert!(parse_constraint("").is_err());
    }

    #[test]
    fn test_parse_constraint_unsupported_op() {
        assert!(parse_constraint("<6.0").is_err());
        assert!(parse_constraint("~6.0").is_err());
        assert!(parse_constraint("^6.0").is_err());
    }

    #[test]
    fn test_parse_constraint_with_spaces() {
        let c = parse_constraint(">= 6.0").unwrap();
        assert_eq!(c.segments, vec![6, 0]);
    }

    // --- satisfies ---

    #[test]
    fn test_satisfies_gte_exact() {
        let c = parse_constraint(">=6.0").unwrap();
        assert!(satisfies("6.0", &c));
    }

    #[test]
    fn test_satisfies_gte_higher_major() {
        let c = parse_constraint(">=6.0").unwrap();
        assert!(satisfies("7.0.0", &c));
    }

    #[test]
    fn test_satisfies_gte_higher_minor() {
        let c = parse_constraint(">=6.0").unwrap();
        assert!(satisfies("6.1.1", &c));
    }

    #[test]
    fn test_satisfies_gte_fail_lower() {
        let c = parse_constraint(">=6.0").unwrap();
        assert!(!satisfies("5.1.2", &c));
    }

    #[test]
    fn test_satisfies_calver_pass() {
        let c = parse_constraint(">=2024.0.0").unwrap();
        assert!(satisfies("2024.12.23", &c));
    }

    #[test]
    fn test_satisfies_calver_fail() {
        let c = parse_constraint(">=2024.0.0").unwrap();
        assert!(!satisfies("2023.06.01", &c));
    }

    #[test]
    fn test_satisfies_missing_segments_treated_as_zero() {
        // "6.0" vs constraint ">=6.0.0" — the installed "6.0" becomes "6.0.0".
        let c = parse_constraint(">=6.0.0").unwrap();
        assert!(satisfies("6.0", &c));
    }

    #[test]
    fn test_satisfies_unparseable_installed_returns_false() {
        let c = parse_constraint(">=6.0").unwrap();
        assert!(!satisfies("not-a-version", &c));
    }

    // --- check_version ---

    #[test]
    fn test_check_version_satisfied() {
        let ctx = MockVersionContext("ffmpeg version 6.1.1 Copyright".to_string());
        let result = check_version("ffmpeg", ">=6.0", &ctx);
        assert_eq!(
            result,
            VersionCheckResult::Checked {
                installed: "6.1.1".to_string(),
                satisfied: true,
            }
        );
    }

    #[test]
    fn test_check_version_unsatisfied() {
        let ctx = MockVersionContext("ffmpeg version 5.0.2 Copyright".to_string());
        let result = check_version("ffmpeg", ">=6.0", &ctx);
        assert_eq!(
            result,
            VersionCheckResult::Checked {
                installed: "5.0.2".to_string(),
                satisfied: false,
            }
        );
    }

    #[test]
    fn test_check_version_empty_constraint_skipped() {
        let ctx = MockVersionContext("ffmpeg version 6.1.1".to_string());
        let result = check_version("ffmpeg", "", &ctx);
        assert_eq!(result, VersionCheckResult::Skipped);
    }

    #[test]
    fn test_check_version_command_fails_skipped() {
        let ctx = FailingContext;
        let result = check_version("nonexistent", ">=1.0", &ctx);
        assert_eq!(result, VersionCheckResult::Skipped);
    }

    #[test]
    fn test_check_version_no_version_in_output_skipped() {
        let ctx = MockVersionContext("no version here".to_string());
        let result = check_version("tool", ">=1.0", &ctx);
        assert_eq!(result, VersionCheckResult::Skipped);
    }

    // --- Test helpers ---

    /// Mock context that returns a fixed string from `run_command`.
    struct MockVersionContext(String);

    impl crate::ProcessContext for MockVersionContext {
        fn run_command(&self, _cmd: &str, _args: &[&str]) -> Result<Vec<u8>, BntoError> {
            Ok(self.0.as_bytes().to_vec())
        }
        fn run_command_streaming(
            &self,
            _cmd: &str,
            _args: &[&str],
            _on_output: &dyn Fn(&str),
        ) -> Result<Vec<u8>, BntoError> {
            Ok(self.0.as_bytes().to_vec())
        }
        fn temp_file(&self, _suffix: &str) -> Result<std::path::PathBuf, BntoError> {
            Err(BntoError::ProcessingFailed("mock".to_string()))
        }
        fn env_var(&self, _key: &str) -> Option<String> {
            None
        }
        fn work_dir(&self) -> Result<&std::path::Path, BntoError> {
            Err(BntoError::ProcessingFailed("mock".to_string()))
        }
    }

    /// Mock context where all commands fail.
    struct FailingContext;

    impl crate::ProcessContext for FailingContext {
        fn run_command(&self, _cmd: &str, _args: &[&str]) -> Result<Vec<u8>, BntoError> {
            Err(BntoError::ProcessingFailed("not found".to_string()))
        }
        fn run_command_streaming(
            &self,
            _cmd: &str,
            _args: &[&str],
            _on_output: &dyn Fn(&str),
        ) -> Result<Vec<u8>, BntoError> {
            Err(BntoError::ProcessingFailed("not found".to_string()))
        }
        fn temp_file(&self, _suffix: &str) -> Result<std::path::PathBuf, BntoError> {
            Err(BntoError::ProcessingFailed("mock".to_string()))
        }
        fn env_var(&self, _key: &str) -> Option<String> {
            None
        }
        fn work_dir(&self) -> Result<&std::path::Path, BntoError> {
            Err(BntoError::ProcessingFailed("mock".to_string()))
        }
    }
}
