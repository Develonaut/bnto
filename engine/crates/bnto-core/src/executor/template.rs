// Multi-namespace template resolution — extends {{fields.*}} with env, ctx, node, and item namespaces.
//
// Entry point: `resolve_templates()` walks params and substitutes placeholders from
// all namespaces. Falls back to `resolve_fields()` for the {{fields.*}} namespace,
// then handles {{env.*}}, {{ctx.*}}, {{node.<id>.*}}, and {{item.*}} additionally.

use std::collections::BTreeMap;

use serde_json::Value;

use crate::context::ProcessContext;

/// All template namespace sources bundled for resolution.
pub struct TemplateContext<'a> {
    /// Field values from node field definitions + user overrides.
    pub field_values: &'a BTreeMap<String, Value>,
    /// System access for env vars and working directory.
    pub process_ctx: &'a dyn ProcessContext,
    /// Per-node output params from previously executed nodes.
    pub node_outputs: &'a BTreeMap<String, Value>,
    /// Current loop iteration's per-file metadata for {{item.*}} templates.
    pub loop_item: &'a Option<serde_json::Map<String, Value>>,
}

/// Resolve all template namespaces in params: fields, env, ctx, node, item.
///
/// Walks the params map and replaces placeholders from all namespaces.
/// Namespace priority: {{fields.*}} first, then {{env.*}}, {{ctx.*}}, {{node.*}}, {{item.*}}.
pub fn resolve_templates(
    params: &serde_json::Map<String, Value>,
    ctx: &TemplateContext,
) -> serde_json::Map<String, Value> {
    params
        .iter()
        .map(|(k, v)| (k.clone(), resolve_value(v, ctx)))
        .collect()
}

/// Resolve a single JSON value, recursing into arrays and objects.
fn resolve_value(value: &Value, ctx: &TemplateContext) -> Value {
    match value {
        Value::String(s) => resolve_string(s, ctx),
        Value::Array(arr) => Value::Array(arr.iter().map(|v| resolve_value(v, ctx)).collect()),
        Value::Object(map) => Value::Object(resolve_templates(map, ctx)),
        other => other.clone(),
    }
}

/// Check if a string contains any template placeholder.
fn has_placeholder(s: &str) -> bool {
    s.contains("{{fields.")
        || s.contains("{{env.")
        || s.contains("{{ctx.")
        || s.contains("{{node.")
        || s.contains("{{item.")
}

/// If the string is exactly one `{{namespace.key}}` placeholder, return (prefix, key).
/// prefix is "fields", "env", "ctx", or the full "node.<id>" part.
fn extract_sole_placeholder(s: &str) -> Option<(&str, &str)> {
    let trimmed = s.trim();
    if !trimmed.starts_with("{{") || !trimmed.ends_with("}}") {
        return None;
    }
    let inner = &trimmed[2..trimmed.len() - 2];
    // Must have no nested braces.
    if inner.contains('{') || inner.contains('}') {
        return None;
    }
    // Split on first dot: "fields.format" → ("fields", "format")
    let (prefix, key) = inner.split_once('.')?;
    // For node refs, prefix is "node" but we need to split further:
    // "node.compress.format" → prefix="node", rest="compress.format"
    // We return ("node.compress", "format") so the caller can extract node_id.
    if prefix == "node" {
        // key is "compress.format" — split on last dot for property
        let (node_id, prop) = key.rsplit_once('.')?;
        return Some((&inner[..5 + node_id.len()], prop)); // "node.compress"
    }
    // "item" is a simple namespace like "fields" — "item.url" → ("item", "url")
    Some((prefix, key))
}

/// Resolve a single placeholder key from the appropriate namespace.
fn resolve_placeholder(prefix: &str, key: &str, ctx: &TemplateContext) -> Option<Value> {
    match prefix {
        "fields" => ctx.field_values.get(key).cloned(),
        "env" => {
            let val = ctx.process_ctx.env_var(key).unwrap_or_default();
            Some(Value::String(val))
        }
        "ctx" => resolve_ctx(key, ctx.process_ctx),
        "item" => ctx.loop_item.as_ref().and_then(|m| m.get(key).cloned()),
        p if p.starts_with("node.") => {
            let node_id = &p[5..]; // strip "node."
            resolve_node_ref(node_id, key, ctx.node_outputs)
        }
        _ => None,
    }
}

/// Resolve {{ctx.*}} properties from ProcessContext.
fn resolve_ctx(key: &str, process_ctx: &dyn ProcessContext) -> Option<Value> {
    match key {
        "work_dir" => process_ctx
            .work_dir()
            .ok()
            .map(|p| Value::String(p.to_string_lossy().into_owned())),
        "temp_dir" => {
            let dir = std::env::temp_dir();
            Some(Value::String(dir.to_string_lossy().into_owned()))
        }
        "platform" => Some(Value::String(current_platform().to_string())),
        "date" | "time" | "timestamp" => {
            let (y, m, d, hh, mm, ss) = now_civil();
            let val = match key {
                "date" => format!("{y:04}-{m:02}-{d:02}"),
                "time" => format!("{hh:02}-{mm:02}-{ss:02}"),
                "timestamp" => format!("{y:04}{m:02}{d:02}-{hh:02}{mm:02}{ss:02}"),
                _ => unreachable!(),
            };
            Some(Value::String(val))
        }
        _ => None, // Unknown ctx property — leave as-is
    }
}

/// Convert current wall-clock time to civil (year, month, day, hour, min, sec).
///
/// Uses Howard Hinnant's civil calendar algorithm on the Unix epoch.
/// No `chrono` dependency — pure arithmetic. `SystemTime::now()` works in
/// WASM (backed by `Date.now()`).
fn now_civil() -> (i32, u32, u32, u32, u32, u32) {
    use std::time::{SystemTime, UNIX_EPOCH};

    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    epoch_to_civil(secs)
}

/// Convert Unix epoch seconds to (year, month, day, hour, minute, second) in UTC.
///
/// Hinnant's algorithm: http://howardhinnant.github.io/date_algorithms.html
fn epoch_to_civil(epoch_secs: u64) -> (i32, u32, u32, u32, u32, u32) {
    let total_secs = epoch_secs;
    let day_secs = (total_secs % 86400) as u32;
    let hh = day_secs / 3600;
    let mm = (day_secs % 3600) / 60;
    let ss = day_secs % 60;

    // Days since epoch, shifted so day 0 = 0000-03-01
    let z = (total_secs / 86400) as i64 + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u32; // day of era [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };

    (y as i32, m, d, hh, mm, ss)
}

/// Resolve only `{{ctx.*}}` variables in a string.
///
/// The environment (CLI, TUI) calls this to expand template variables in
/// the output directory path before deciding where to write files.
/// Only resolves ctx namespace — ignores fields, env, item, node placeholders.
pub fn resolve_ctx_templates(s: &str, ctx: &dyn ProcessContext) -> String {
    if !s.contains("{{ctx.") {
        return s.to_string();
    }
    let mut result = s.to_string();
    resolve_ctx_interpolation(&mut result, ctx);
    result
}

/// Returns the current platform name.
fn current_platform() -> &'static str {
    if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else {
        "unknown"
    }
}

/// Resolve {{node.<id>.<prop>}} from previously executed node outputs.
fn resolve_node_ref(
    node_id: &str,
    prop: &str,
    node_outputs: &BTreeMap<String, Value>,
) -> Option<Value> {
    let output = node_outputs.get(node_id)?;
    match output {
        Value::Object(map) => map.get(prop).cloned(),
        _ => None,
    }
}

/// Convert a JSON value to a string for interpolation.
fn value_to_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

/// Resolve all template placeholders in a string.
///
/// Sole placeholder (entire string is one `{{ns.key}}`) preserves the original
/// JSON type. Partial or multiple placeholders use string interpolation.
fn resolve_string(s: &str, ctx: &TemplateContext) -> Value {
    if !has_placeholder(s) {
        return Value::String(s.to_string());
    }

    // Sole placeholder — preserve type.
    if let Some((prefix, key)) = extract_sole_placeholder(s) {
        if let Some(value) = resolve_placeholder(prefix, key, ctx) {
            return value;
        }
        // Not resolved — leave placeholder as-is.
        return Value::String(s.to_string());
    }

    // Multiple or partial placeholders — string interpolation.
    let mut result = s.to_string();

    // Resolve {{fields.*}}
    for (key, value) in ctx.field_values {
        let placeholder = format!("{{{{fields.{key}}}}}");
        if result.contains(&placeholder) {
            result = result.replace(&placeholder, &value_to_string(value));
        }
    }

    // Resolve {{env.*}} — scan for remaining env placeholders.
    resolve_env_interpolation(&mut result, ctx.process_ctx);

    // Resolve {{ctx.*}}
    resolve_ctx_interpolation(&mut result, ctx.process_ctx);

    // Resolve {{item.*}}
    resolve_item_interpolation(&mut result, ctx.loop_item);

    // Resolve {{node.*}}
    resolve_node_interpolation(&mut result, ctx.node_outputs);

    Value::String(result)
}

/// Replace all `{{env.KEY}}` occurrences in-place via string interpolation.
fn resolve_env_interpolation(result: &mut String, process_ctx: &dyn ProcessContext) {
    while let Some(start) = result.find("{{env.") {
        let rest = &result[start + 6..];
        let Some(end) = rest.find("}}") else { break };
        let key = &result[start + 6..start + 6 + end];
        let val = process_ctx.env_var(key).unwrap_or_default();
        let placeholder = format!("{{{{env.{key}}}}}");
        *result = result.replace(&placeholder, &val);
    }
}

/// Replace all `{{ctx.KEY}}` occurrences in-place via string interpolation.
fn resolve_ctx_interpolation(result: &mut String, process_ctx: &dyn ProcessContext) {
    while let Some(start) = result.find("{{ctx.") {
        let rest = &result[start + 6..];
        let Some(end) = rest.find("}}") else { break };
        let key = &result[start + 6..start + 6 + end];
        let val = resolve_ctx(key, process_ctx)
            .map(|v| value_to_string(&v))
            .unwrap_or_default();
        let placeholder = format!("{{{{ctx.{key}}}}}");
        *result = result.replace(&placeholder, &val);
    }
}

/// Replace all `{{item.KEY}}` occurrences in-place via string interpolation.
fn resolve_item_interpolation(
    result: &mut String,
    loop_item: &Option<serde_json::Map<String, Value>>,
) {
    let Some(item) = loop_item.as_ref() else {
        return;
    };
    while let Some(start) = result.find("{{item.") {
        let rest = &result[start + 7..];
        let Some(end) = rest.find("}}") else { break };
        let key = &result[start + 7..start + 7 + end];
        let val = item.get(key).map(value_to_string).unwrap_or_default();
        let placeholder = format!("{{{{item.{key}}}}}");
        *result = result.replace(&placeholder, &val);
    }
}

/// Replace all `{{node.<id>.<prop>}}` occurrences in-place via string interpolation.
fn resolve_node_interpolation(result: &mut String, node_outputs: &BTreeMap<String, Value>) {
    while let Some(start) = result.find("{{node.") {
        let rest = &result[start + 7..];
        let Some(end) = rest.find("}}") else { break };
        let ref_str = &result[start + 7..start + 7 + end]; // "compress.format"
        let Some((node_id, prop)) = ref_str.rsplit_once('.') else {
            break;
        };
        let val = resolve_node_ref(node_id, prop, node_outputs)
            .map(|v| value_to_string(&v))
            .unwrap_or_default();
        let placeholder = format!("{{{{node.{ref_str}}}}}");
        *result = result.replace(&placeholder, &val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::NoopContext;
    use serde_json::json;
    use std::path::{Path, PathBuf};

    fn make_params(pairs: &[(&str, Value)]) -> serde_json::Map<String, Value> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect()
    }

    fn make_fields(pairs: &[(&str, Value)]) -> BTreeMap<String, Value> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect()
    }

    /// Test context that returns controlled values for env vars and work_dir.
    struct MockContext {
        env_vars: BTreeMap<String, String>,
        work_dir: PathBuf,
    }

    impl MockContext {
        fn new() -> Self {
            Self {
                env_vars: BTreeMap::new(),
                work_dir: PathBuf::from("/mock/work"),
            }
        }

        fn with_env(mut self, key: &str, val: &str) -> Self {
            self.env_vars.insert(key.to_string(), val.to_string());
            self
        }
    }

    impl ProcessContext for MockContext {
        fn run_command(&self, _cmd: &str, _args: &[&str]) -> Result<Vec<u8>, crate::BntoError> {
            Err(crate::BntoError::ProcessingFailed("mock".into()))
        }
        fn temp_file(&self, _suffix: &str) -> Result<PathBuf, crate::BntoError> {
            Ok(PathBuf::from("/tmp/mock"))
        }
        fn env_var(&self, key: &str) -> Option<String> {
            self.env_vars.get(key).cloned()
        }
        fn work_dir(&self) -> Result<&Path, crate::BntoError> {
            Ok(&self.work_dir)
        }
    }

    fn empty_fields() -> BTreeMap<String, Value> {
        BTreeMap::new()
    }

    fn empty_outputs() -> BTreeMap<String, Value> {
        BTreeMap::new()
    }

    // --- {{fields.*}} backward compatibility ---

    #[test]
    fn fields_simple_substitution() {
        let params = make_params(&[("format", json!("{{fields.format}}"))]);
        let fields = make_fields(&[("format", json!("mp4"))]);
        let ctx = TemplateContext {
            field_values: &fields,
            process_ctx: &NoopContext,
            node_outputs: &empty_outputs(),
            loop_item: &None,
        };
        let resolved = resolve_templates(&params, &ctx);
        assert_eq!(resolved["format"], json!("mp4"));
    }

    #[test]
    fn fields_preserves_number_type() {
        let params = make_params(&[("quality", json!("{{fields.quality}}"))]);
        let fields = make_fields(&[("quality", json!(80))]);
        let ctx = TemplateContext {
            field_values: &fields,
            process_ctx: &NoopContext,
            node_outputs: &empty_outputs(),
            loop_item: &None,
        };
        let resolved = resolve_templates(&params, &ctx);
        assert_eq!(resolved["quality"], json!(80));
        assert!(resolved["quality"].is_number());
    }

    #[test]
    fn fields_missing_leaves_placeholder() {
        let params = make_params(&[("x", json!("{{fields.missing}}"))]);
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &NoopContext,
            node_outputs: &empty_outputs(),
            loop_item: &None,
        };
        let resolved = resolve_templates(&params, &ctx);
        assert_eq!(resolved["x"], json!("{{fields.missing}}"));
    }

    // --- {{env.*}} ---

    #[test]
    fn env_simple_substitution() {
        let params = make_params(&[("home", json!("{{env.HOME}}"))]);
        let mock = MockContext::new().with_env("HOME", "/Users/test");
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &mock,
            node_outputs: &empty_outputs(),
            loop_item: &None,
        };
        let resolved = resolve_templates(&params, &ctx);
        assert_eq!(resolved["home"], json!("/Users/test"));
    }

    #[test]
    fn env_missing_var_returns_empty() {
        let params = make_params(&[("x", json!("{{env.NONEXISTENT}}"))]);
        let mock = MockContext::new();
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &mock,
            node_outputs: &empty_outputs(),
            loop_item: &None,
        };
        let resolved = resolve_templates(&params, &ctx);
        assert_eq!(resolved["x"], json!(""));
    }

    #[test]
    fn env_in_interpolation() {
        let params = make_params(&[("path", json!("{{env.HOME}}/output"))]);
        let mock = MockContext::new().with_env("HOME", "/Users/test");
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &mock,
            node_outputs: &empty_outputs(),
            loop_item: &None,
        };
        let resolved = resolve_templates(&params, &ctx);
        assert_eq!(resolved["path"], json!("/Users/test/output"));
    }

    // --- {{ctx.*}} ---

    #[test]
    fn ctx_work_dir() {
        let params = make_params(&[("dir", json!("{{ctx.work_dir}}"))]);
        let mock = MockContext::new();
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &mock,
            node_outputs: &empty_outputs(),
            loop_item: &None,
        };
        let resolved = resolve_templates(&params, &ctx);
        assert_eq!(resolved["dir"], json!("/mock/work"));
    }

    #[test]
    fn ctx_platform() {
        let params = make_params(&[("os", json!("{{ctx.platform}}"))]);
        let mock = MockContext::new();
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &mock,
            node_outputs: &empty_outputs(),
            loop_item: &None,
        };
        let resolved = resolve_templates(&params, &ctx);
        let os = resolved["os"].as_str().unwrap();
        assert!(
            ["macos", "linux", "windows", "unknown"].contains(&os),
            "Unexpected platform: {os}"
        );
    }

    #[test]
    fn ctx_temp_dir_resolves() {
        let params = make_params(&[("tmp", json!("{{ctx.temp_dir}}"))]);
        let mock = MockContext::new();
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &mock,
            node_outputs: &empty_outputs(),
            loop_item: &None,
        };
        let resolved = resolve_templates(&params, &ctx);
        let tmp = resolved["tmp"].as_str().unwrap();
        assert!(
            !tmp.is_empty(),
            "temp_dir should resolve to a non-empty path"
        );
        assert!(
            !tmp.contains("{{"),
            "Should not contain unresolved placeholder"
        );
    }

    #[test]
    fn ctx_date_returns_iso_format() {
        let params = make_params(&[("d", json!("{{ctx.date}}"))]);
        let mock = MockContext::new();
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &mock,
            node_outputs: &empty_outputs(),
            loop_item: &None,
        };
        let resolved = resolve_templates(&params, &ctx);
        let val = resolved["d"].as_str().unwrap();
        // ISO 8601 date: YYYY-MM-DD
        assert!(
            val.len() == 10 && val.chars().nth(4) == Some('-') && val.chars().nth(7) == Some('-'),
            "Expected YYYY-MM-DD, got: {val}"
        );
        // Year should be reasonable (2020+)
        let year: u32 = val[..4].parse().unwrap();
        assert!(year >= 2020, "Year should be >= 2020, got: {year}");
    }

    #[test]
    fn ctx_time_returns_hyphen_separated() {
        let params = make_params(&[("t", json!("{{ctx.time}}"))]);
        let mock = MockContext::new();
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &mock,
            node_outputs: &empty_outputs(),
            loop_item: &None,
        };
        let resolved = resolve_templates(&params, &ctx);
        let val = resolved["t"].as_str().unwrap();
        // HH-MM-SS format (filesystem-safe)
        assert_eq!(val.len(), 8, "Expected HH-MM-SS (8 chars), got: {val}");
        let parts: Vec<&str> = val.split('-').collect();
        assert_eq!(parts.len(), 3, "Expected 3 parts separated by hyphens");
        let hour: u32 = parts[0].parse().unwrap();
        let minute: u32 = parts[1].parse().unwrap();
        let second: u32 = parts[2].parse().unwrap();
        assert!(hour < 24, "Hour out of range: {hour}");
        assert!(minute < 60, "Minute out of range: {minute}");
        assert!(second < 60, "Second out of range: {second}");
    }

    #[test]
    fn ctx_timestamp_returns_compact_sortable() {
        let params = make_params(&[("ts", json!("{{ctx.timestamp}}"))]);
        let mock = MockContext::new();
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &mock,
            node_outputs: &empty_outputs(),
            loop_item: &None,
        };
        let resolved = resolve_templates(&params, &ctx);
        let val = resolved["ts"].as_str().unwrap();
        // YYYYMMDD-HHMMSS format
        assert_eq!(
            val.len(),
            15,
            "Expected YYYYMMDD-HHMMSS (15 chars), got: {val}"
        );
        assert_eq!(
            val.chars().nth(8),
            Some('-'),
            "Expected hyphen at position 8"
        );
        // Date part should be all digits
        assert!(val[..8].chars().all(|c| c.is_ascii_digit()));
        // Time part should be all digits
        assert!(val[9..].chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn ctx_date_in_interpolation() {
        let params = make_params(&[("dir", json!("{{ctx.date}}-bulk-download"))]);
        let mock = MockContext::new();
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &mock,
            node_outputs: &empty_outputs(),
            loop_item: &None,
        };
        let resolved = resolve_templates(&params, &ctx);
        let val = resolved["dir"].as_str().unwrap();
        assert!(
            val.ends_with("-bulk-download"),
            "Expected suffix, got: {val}"
        );
        assert!(
            !val.contains("{{"),
            "Should not contain unresolved placeholder"
        );
    }

    #[test]
    fn resolve_ctx_templates_public_function() {
        let mock = MockContext::new();
        let result = resolve_ctx_templates("{{ctx.date}}-output", &mock);
        assert!(
            !result.contains("{{"),
            "Should not contain unresolved placeholder"
        );
        assert!(result.ends_with("-output"), "Suffix should be preserved");
        assert!(result.len() > "-output".len(), "Date should be prepended");
    }

    #[test]
    fn resolve_ctx_templates_no_placeholders() {
        let mock = MockContext::new();
        let result = resolve_ctx_templates("plain-string", &mock);
        assert_eq!(result, "plain-string");
    }

    #[test]
    fn resolve_ctx_templates_empty_string() {
        let mock = MockContext::new();
        let result = resolve_ctx_templates("", &mock);
        assert_eq!(result, "");
    }

    #[test]
    fn ctx_unknown_property_left_asis() {
        let params = make_params(&[("x", json!("{{ctx.bogus}}"))]);
        let mock = MockContext::new();
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &mock,
            node_outputs: &empty_outputs(),
            loop_item: &None,
        };
        let resolved = resolve_templates(&params, &ctx);
        assert_eq!(resolved["x"], json!("{{ctx.bogus}}"));
    }

    // --- {{node.<id>.*}} ---

    #[test]
    fn node_ref_resolves_from_output_map() {
        let params = make_params(&[("fmt", json!("{{node.compress.format}}"))]);
        let mut outputs = BTreeMap::new();
        outputs.insert("compress".to_string(), json!({"format": "webp"}));
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &NoopContext,
            node_outputs: &outputs,
            loop_item: &None,
        };
        let resolved = resolve_templates(&params, &ctx);
        assert_eq!(resolved["fmt"], json!("webp"));
    }

    #[test]
    fn node_ref_missing_node_left_asis() {
        let params = make_params(&[("x", json!("{{node.missing.prop}}"))]);
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &NoopContext,
            node_outputs: &empty_outputs(),
            loop_item: &None,
        };
        let resolved = resolve_templates(&params, &ctx);
        assert_eq!(resolved["x"], json!("{{node.missing.prop}}"));
    }

    #[test]
    fn node_ref_missing_property_left_asis() {
        let mut outputs = BTreeMap::new();
        outputs.insert("compress".to_string(), json!({"format": "webp"}));
        let params = make_params(&[("x", json!("{{node.compress.missing}}"))]);
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &NoopContext,
            node_outputs: &outputs,
            loop_item: &None,
        };
        let resolved = resolve_templates(&params, &ctx);
        assert_eq!(resolved["x"], json!("{{node.compress.missing}}"));
    }

    // --- Mixed namespaces ---

    #[test]
    fn mixed_namespaces_in_one_string() {
        let params = make_params(&[("cmd", json!("{{env.HOME}}/{{ctx.platform}}/out"))]);
        let mock = MockContext::new().with_env("HOME", "/Users/test");
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &mock,
            node_outputs: &empty_outputs(),
            loop_item: &None,
        };
        let resolved = resolve_templates(&params, &ctx);
        let val = resolved["cmd"].as_str().unwrap();
        assert!(val.starts_with("/Users/test/"));
        assert!(val.ends_with("/out"));
        assert!(!val.contains("{{"), "All placeholders should be resolved");
    }

    #[test]
    fn fields_and_env_mixed() {
        let params = make_params(&[("x", json!("{{fields.name}}-{{env.SUFFIX}}"))]);
        let fields = make_fields(&[("name", json!("hello"))]);
        let mock = MockContext::new().with_env("SUFFIX", "world");
        let ctx = TemplateContext {
            field_values: &fields,
            process_ctx: &mock,
            node_outputs: &empty_outputs(),
            loop_item: &None,
        };
        let resolved = resolve_templates(&params, &ctx);
        assert_eq!(resolved["x"], json!("hello-world"));
    }

    #[test]
    fn non_string_values_pass_through() {
        let params = make_params(&[("n", json!(42)), ("b", json!(true))]);
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &NoopContext,
            node_outputs: &empty_outputs(),
            loop_item: &None,
        };
        let resolved = resolve_templates(&params, &ctx);
        assert_eq!(resolved["n"], json!(42));
        assert_eq!(resolved["b"], json!(true));
    }

    #[test]
    fn array_elements_resolved() {
        let params = make_params(&[("args", json!(["--dir", "{{env.HOME}}", "-v"]))]);
        let mock = MockContext::new().with_env("HOME", "/test");
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &mock,
            node_outputs: &empty_outputs(),
            loop_item: &None,
        };
        let resolved = resolve_templates(&params, &ctx);
        assert_eq!(resolved["args"], json!(["--dir", "/test", "-v"]));
    }

    #[test]
    fn nested_object_resolved() {
        let params = make_params(&[("config", json!({"dir": "{{env.HOME}}"}))]);
        let mock = MockContext::new().with_env("HOME", "/test");
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &mock,
            node_outputs: &empty_outputs(),
            loop_item: &None,
        };
        let resolved = resolve_templates(&params, &ctx);
        assert_eq!(resolved["config"]["dir"], json!("/test"));
    }

    #[test]
    fn no_placeholder_passes_through() {
        let params = make_params(&[("cmd", json!("yt-dlp"))]);
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &NoopContext,
            node_outputs: &empty_outputs(),
            loop_item: &None,
        };
        let resolved = resolve_templates(&params, &ctx);
        assert_eq!(resolved["cmd"], json!("yt-dlp"));
    }

    #[test]
    fn node_ref_in_interpolation() {
        let params = make_params(&[("label", json!("Format: {{node.compress.format}}"))]);
        let mut outputs = BTreeMap::new();
        outputs.insert("compress".to_string(), json!({"format": "webp"}));
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &NoopContext,
            node_outputs: &outputs,
            loop_item: &None,
        };
        let resolved = resolve_templates(&params, &ctx);
        assert_eq!(resolved["label"], json!("Format: webp"));
    }

    // --- {{item.*}} — loop iteration metadata ---

    fn make_item(pairs: &[(&str, Value)]) -> Option<serde_json::Map<String, Value>> {
        Some(
            pairs
                .iter()
                .map(|(k, v)| (k.to_string(), v.clone()))
                .collect(),
        )
    }

    #[test]
    fn item_simple_substitution() {
        let params = make_params(&[("url", json!("{{item.url}}"))]);
        let item = make_item(&[("url", json!("https://example.com/video"))]);
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &NoopContext,
            node_outputs: &empty_outputs(),
            loop_item: &item,
        };
        let resolved = resolve_templates(&params, &ctx);
        assert_eq!(resolved["url"], json!("https://example.com/video"));
    }

    #[test]
    fn item_preserves_number_type() {
        let params = make_params(&[("count", json!("{{item.count}}"))]);
        let item = make_item(&[("count", json!(42))]);
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &NoopContext,
            node_outputs: &empty_outputs(),
            loop_item: &item,
        };
        let resolved = resolve_templates(&params, &ctx);
        assert_eq!(resolved["count"], json!(42));
        assert!(resolved["count"].is_number());
    }

    #[test]
    fn item_missing_key_left_asis() {
        let params = make_params(&[("x", json!("{{item.missing}}"))]);
        let item = make_item(&[("url", json!("https://example.com"))]);
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &NoopContext,
            node_outputs: &empty_outputs(),
            loop_item: &item,
        };
        let resolved = resolve_templates(&params, &ctx);
        assert_eq!(resolved["x"], json!("{{item.missing}}"));
    }

    #[test]
    fn item_no_context_left_asis() {
        let params = make_params(&[("url", json!("{{item.url}}"))]);
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &NoopContext,
            node_outputs: &empty_outputs(),
            loop_item: &None,
        };
        let resolved = resolve_templates(&params, &ctx);
        assert_eq!(resolved["url"], json!("{{item.url}}"));
    }

    #[test]
    fn item_in_interpolation() {
        let params = make_params(&[("path", json!("dir/{{item.group}}/out"))]);
        let item = make_item(&[("group", json!("Alpha"))]);
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &NoopContext,
            node_outputs: &empty_outputs(),
            loop_item: &item,
        };
        let resolved = resolve_templates(&params, &ctx);
        assert_eq!(resolved["path"], json!("dir/Alpha/out"));
    }

    #[test]
    fn item_mixed_with_fields() {
        let params = make_params(&[("x", json!("{{fields.format}}_{{item.group}}"))]);
        let fields = make_fields(&[("format", json!("mp4"))]);
        let item = make_item(&[("group", json!("Beta"))]);
        let ctx = TemplateContext {
            field_values: &fields,
            process_ctx: &NoopContext,
            node_outputs: &empty_outputs(),
            loop_item: &item,
        };
        let resolved = resolve_templates(&params, &ctx);
        assert_eq!(resolved["x"], json!("mp4_Beta"));
    }
}
