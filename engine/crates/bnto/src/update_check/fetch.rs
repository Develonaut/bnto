// Ask crates.io's sparse index for the newest bnto. Small, unauthenticated,
// no rate limit; the request carries nothing about the user.

use std::time::Duration;

use super::index::latest_in_index;
use super::version::Version;

const INDEX_URL: &str = "https://index.crates.io/bn/to/bnto";
const TIMEOUT: Duration = Duration::from_secs(5);

/// Where the index is read from: crates.io, unless `BNTO_UPDATE_INDEX_URL`
/// points elsewhere (a mirror, or a `file://` path in tests).
fn index_url() -> String {
    std::env::var("BNTO_UPDATE_INDEX_URL").unwrap_or_else(|_| INDEX_URL.to_string())
}

/// The newest stable bnto on crates.io, or `None` offline or on any error.
pub fn fetch_latest() -> Option<Version> {
    let url = index_url();
    if let Some(path) = url.strip_prefix("file://") {
        return latest_in_index(&std::fs::read_to_string(path).ok()?);
    }
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .timeout_global(Some(TIMEOUT))
        .build()
        .into();
    let user_agent = concat!("bnto/", env!("CARGO_PKG_VERSION"), " (update check)");
    let mut response = agent
        .get(&url)
        .header("User-Agent", user_agent)
        .call()
        .ok()?;
    latest_in_index(&response.body_mut().read_to_string().ok()?)
}
