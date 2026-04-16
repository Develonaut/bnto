// CI environment detection — tag telemetry events with CI context.

use std::env;

/// Check if we're running in a CI environment.
pub fn is_ci() -> bool {
    env::var("CI").is_ok_and(|v| !v.is_empty())
        || env::var("GITHUB_ACTIONS").is_ok()
        || env::var("GITLAB_CI").is_ok()
        || env::var("CIRCLECI").is_ok()
        || env::var("TRAVIS").is_ok()
        || env::var("BUILDKITE").is_ok()
        || env::var("JENKINS_URL").is_ok()
}

/// Identify which CI provider is running, if any.
pub fn ci_provider() -> Option<&'static str> {
    if env::var("GITHUB_ACTIONS").is_ok() {
        Some("github-actions")
    } else if env::var("GITLAB_CI").is_ok() {
        Some("gitlab-ci")
    } else if env::var("CIRCLECI").is_ok() {
        Some("circleci")
    } else if env::var("TRAVIS").is_ok() {
        Some("travis-ci")
    } else if env::var("BUILDKITE").is_ok() {
        Some("buildkite")
    } else if env::var("JENKINS_URL").is_ok() {
        Some("jenkins")
    } else if env::var("CI").is_ok_and(|v| !v.is_empty()) {
        Some("unknown")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // CI env var tests mutate process-wide state, so they must run serially.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    /// Helper: clear all CI env vars, run closure, then restore.
    fn with_clean_env(f: impl FnOnce()) {
        let _guard = ENV_LOCK.lock().unwrap();
        let vars = [
            "CI",
            "GITHUB_ACTIONS",
            "GITLAB_CI",
            "CIRCLECI",
            "TRAVIS",
            "BUILDKITE",
            "JENKINS_URL",
        ];
        let saved: Vec<_> = vars.iter().map(|k| (*k, env::var(k).ok())).collect();
        // SAFETY: tests run serially under ENV_LOCK.
        unsafe {
            for k in &vars {
                env::remove_var(k);
            }
        }

        f();

        // SAFETY: tests run serially under ENV_LOCK.
        unsafe {
            for (k, v) in saved {
                match v {
                    Some(val) => env::set_var(k, val),
                    None => env::remove_var(k),
                }
            }
        }
    }

    #[test]
    fn no_ci_vars_means_not_ci() {
        with_clean_env(|| {
            assert!(!is_ci());
            assert_eq!(ci_provider(), None);
        });
    }

    #[test]
    fn ci_true_detected() {
        with_clean_env(|| {
            // SAFETY: tests run serially under ENV_LOCK.
            unsafe { env::set_var("CI", "true") };
            assert!(is_ci());
            assert_eq!(ci_provider(), Some("unknown"));
        });
    }

    #[test]
    fn github_actions_detected() {
        with_clean_env(|| {
            unsafe { env::set_var("GITHUB_ACTIONS", "true") };
            assert!(is_ci());
            assert_eq!(ci_provider(), Some("github-actions"));
        });
    }

    #[test]
    fn gitlab_ci_detected() {
        with_clean_env(|| {
            unsafe { env::set_var("GITLAB_CI", "true") };
            assert!(is_ci());
            assert_eq!(ci_provider(), Some("gitlab-ci"));
        });
    }

    #[test]
    fn empty_ci_var_not_detected() {
        with_clean_env(|| {
            unsafe { env::set_var("CI", "") };
            assert!(!is_ci());
        });
    }
}
