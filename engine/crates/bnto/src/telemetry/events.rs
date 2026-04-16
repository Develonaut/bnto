// Structured telemetry events for the CLI.

use std::collections::HashMap;

use super::bucket;

/// A telemetry event ready to send to PostHog.
#[derive(Debug, Clone)]
pub struct Event {
    pub name: String,
    pub properties: HashMap<String, serde_json::Value>,
}

impl Event {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            properties: HashMap::new(),
        }
    }

    fn prop(mut self, key: &str, value: impl Into<serde_json::Value>) -> Self {
        self.properties.insert(key.to_string(), value.into());
        self
    }
}

/// CLI command invoked (e.g. "run", "list", "info", "tui").
pub fn cli_command(command: &str) -> Event {
    Event::new("cli_command").prop("command", command)
}

/// Recipe run completed or failed via CLI.
pub fn cli_recipe_run(
    slug: &str,
    duration_ms: u64,
    file_count: usize,
    total_bytes: u64,
    output_file_count: usize,
    success: bool,
) -> Event {
    Event::new("cli_recipe_run")
        .prop("slug", slug)
        .prop("duration_bucket", bucket::bucket_duration(duration_ms))
        .prop("file_count_bucket", bucket::bucket_file_count(file_count))
        .prop("size_bucket", bucket::bucket_file_size(total_bytes))
        .prop(
            "output_file_count_bucket",
            bucket::bucket_file_count(output_file_count),
        )
        .prop("success", success)
}

/// Recipe run with per-node param overrides captured.
pub fn cli_recipe_run_with_params(
    slug: &str,
    duration_ms: u64,
    file_count: usize,
    total_bytes: u64,
    output_file_count: usize,
    success: bool,
    param_names: &[String],
) -> Event {
    let mut event = cli_recipe_run(
        slug,
        duration_ms,
        file_count,
        total_bytes,
        output_file_count,
        success,
    );
    if !param_names.is_empty() {
        let names: Vec<serde_json::Value> = param_names.iter().map(|n| n.clone().into()).collect();
        event.properties.insert("param_names".into(), names.into());
        event
            .properties
            .insert("param_count".into(), param_names.len().into());
    }
    event
}

/// CLI error (sanitized — no paths or PII).
pub fn cli_error(command: &str, error: &str) -> Event {
    Event::new("cli_error")
        .prop("command", command)
        .prop("error_category", categorize_error(error))
}

/// TUI session ended.
pub fn cli_tui_session(duration_ms: u64, theme: &str) -> Event {
    Event::new("cli_tui_session")
        .prop("duration_bucket", bucket::bucket_duration(duration_ms))
        .prop("theme", theme)
}

/// Categorize an error message into a safe bucket (no paths or PII).
fn categorize_error(error: &str) -> &'static str {
    let lower = error.to_lowercase();
    if lower.contains("not found") || lower.contains("no such file") {
        "file_not_found"
    } else if lower.contains("permission denied") {
        "permission_denied"
    } else if lower.contains("unsupported") {
        "unsupported_format"
    } else if lower.contains("timeout") || lower.contains("timed out") {
        "timeout"
    } else if lower.contains("recipe") && lower.contains("unknown") {
        "unknown_recipe"
    } else {
        "unknown"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_command_event_shape() {
        let e = cli_command("list");
        assert_eq!(e.name, "cli_command");
        assert_eq!(e.properties["command"], "list");
    }

    #[test]
    fn cli_command_run() {
        let e = cli_command("run");
        assert_eq!(e.properties["command"], "run");
    }

    #[test]
    fn cli_recipe_run_buckets_values() {
        let e = cli_recipe_run("compress-images", 3000, 5, 2_000_000, 5, true);
        assert_eq!(e.name, "cli_recipe_run");
        assert_eq!(e.properties["slug"], "compress-images");
        assert_eq!(e.properties["duration_bucket"], "1-5s");
        assert_eq!(e.properties["file_count_bucket"], "2-5");
        assert_eq!(e.properties["size_bucket"], "1-10MB");
        assert_eq!(e.properties["output_file_count_bucket"], "2-5");
        assert_eq!(e.properties["success"], true);
    }

    #[test]
    fn cli_recipe_run_with_params_captures_names() {
        let params = vec!["quality".to_string(), "format".to_string()];
        let e = cli_recipe_run_with_params("compress-images", 500, 1, 50_000, 1, true, &params);
        assert_eq!(e.properties["param_count"], 2);
        let names = e.properties["param_names"].as_array().unwrap();
        assert_eq!(names[0], "quality");
        assert_eq!(names[1], "format");
    }

    #[test]
    fn cli_recipe_run_with_no_params_omits_field() {
        let e = cli_recipe_run_with_params("compress-images", 500, 1, 50_000, 1, true, &[]);
        assert!(!e.properties.contains_key("param_names"));
        assert!(!e.properties.contains_key("param_count"));
    }

    #[test]
    fn cli_error_categorizes() {
        let e = cli_error("run", "No such file or directory: input.jpg");
        assert_eq!(e.properties["command"], "run");
        assert_eq!(e.properties["error_category"], "file_not_found");
    }

    #[test]
    fn cli_error_unknown_category() {
        let e = cli_error("run", "something unexpected happened");
        assert_eq!(e.properties["error_category"], "unknown");
    }

    #[test]
    fn cli_error_permission_denied() {
        let e = cli_error("run", "Permission denied: /etc/shadow");
        assert_eq!(e.properties["error_category"], "permission_denied");
    }

    #[test]
    fn cli_tui_session_shape() {
        let e = cli_tui_session(45_000, "tokyo");
        assert_eq!(e.name, "cli_tui_session");
        assert_eq!(e.properties["duration_bucket"], "30s-2m");
        assert_eq!(e.properties["theme"], "tokyo");
    }

    #[test]
    fn categorize_unsupported() {
        assert_eq!(
            categorize_error("unsupported format: .bmp"),
            "unsupported_format"
        );
    }

    #[test]
    fn categorize_timeout() {
        assert_eq!(categorize_error("operation timed out"), "timeout");
    }

    #[test]
    fn categorize_unknown_recipe() {
        assert_eq!(
            categorize_error("Unknown recipe: foo-bar"),
            "unknown_recipe"
        );
    }
}
