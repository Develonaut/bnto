// PostHog HTTP client — fire-and-forget event capture with 2s timeout.

use std::time::Duration;

use super::events::Event;

const POSTHOG_HOST: &str = "https://us.i.posthog.com";
const TIMEOUT: Duration = Duration::from_secs(2);

/// Send an event to PostHog. Returns Ok(()) on success, Err on failure.
/// Callers should ignore errors (fire-and-forget).
pub fn send_event(api_key: &str, distinct_id: &str, event: &Event) -> Result<(), String> {
    let payload = serde_json::json!({
        "api_key": api_key,
        "event": event.name,
        "distinct_id": distinct_id,
        "properties": event.properties,
    });

    let body =
        serde_json::to_string(&payload).map_err(|e| format!("Failed to serialize event: {e}"))?;

    let agent: ureq::Agent = ureq::Agent::config_builder()
        .timeout_global(Some(TIMEOUT))
        .build()
        .into();

    agent
        .post(&format!("{POSTHOG_HOST}/capture/"))
        .header("Content-Type", "application/json")
        .send(body.as_bytes())
        .map_err(|e| format!("Failed to send event: {e}"))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn send_event_to_invalid_key_does_not_panic() {
        let event = Event {
            name: "test_event".into(),
            properties: HashMap::new(),
        };
        // Don't assert success — CI may not have network access.
        let _ = send_event("phc_test_invalid_key", "test-distinct-id", &event);
    }
}
