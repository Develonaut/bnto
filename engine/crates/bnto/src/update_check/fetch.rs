// Ask crates.io's sparse index for the newest bnto. Small, unauthenticated,
// no rate limit; the request carries nothing about the user.

use std::time::Duration;

use super::index::latest_in_index;
use super::version::Version;

const INDEX_URL: &str = "https://index.crates.io/bn/to/bnto";
const TIMEOUT: Duration = Duration::from_secs(5);

/// The newest stable bnto on crates.io, or `None` offline or on any error.
pub fn fetch_latest() -> Option<Version> {
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .timeout_global(Some(TIMEOUT))
        .build()
        .into();
    let user_agent = concat!("bnto/", env!("CARGO_PKG_VERSION"), " (update check)");
    let mut response = agent
        .get(INDEX_URL)
        .header("User-Agent", user_agent)
        .call()
        .ok()?;
    latest_in_index(&response.body_mut().read_to_string().ok()?)
}
