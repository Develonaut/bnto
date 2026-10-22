// Multi-namespace template resolution — extends {{fields.*}} with env, ctx, node, and item namespaces.
//
// Entry point: `resolve_templates()` walks params and substitutes placeholders from
// all namespaces. Falls back to `resolve_fields()` for the {{fields.*}} namespace,
// then handles {{env.*}}, {{ctx.*}}, {{node.<id>.*}}, and {{item.*}} additionally.

use std::collections::BTreeMap;

use serde_json::Value;

use crate::context::ProcessContext;
use crate::metadata::HostPlatform;

use super::interpolate::interpolate;
use super::node_outputs::NodeOutputSource;

/// All template namespace sources bundled for resolution.
pub struct TemplateContext<'a> {
    /// Field values from node field definitions + user overrides.
    pub field_values: &'a BTreeMap<String, Value>,
    /// System access for env vars and working directory.
    pub process_ctx: &'a dyn ProcessContext,
    /// Output metadata of the nodes that ran before this one, by node id.
    pub node_outputs: &'a dyn NodeOutputSource,
    /// The {{item.*}} source: the metadata of the file being processed, under the
    /// innermost loop's item. `None` when there is neither.
    pub loop_item: &'a Option<serde_json::Map<String, Value>>,
    /// The run's start (Unix epoch ms) for `{{ctx.started}}`; `None` outside a run.
    pub run_started_ms: Option<u64>,
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
        "ctx" => resolve_ctx(key, ctx.process_ctx, ctx.run_started_ms),
        "item" => ctx.loop_item.as_ref().and_then(|m| m.get(key).cloned()),
        p if p.starts_with("node.") => {
            let node_id = &p[5..]; // strip "node."
            resolve_node_ref(node_id, key, ctx.node_outputs)
        }
        _ => None,
    }
}

/// Resolve {{ctx.*}} properties from ProcessContext.
///
/// Path-related variables live under the `paths.*` sub-namespace:
///   - `paths.home_dir`   → ~/.bnto/
///   - `paths.output_dir` → ~/.bnto/output/
///   - `paths.work_dir`   → current working directory
///   - `paths.temp_dir`   → system temp directory
///
/// Old flat names (`work_dir`, `temp_dir`) are kept as backward-compat aliases.
fn resolve_ctx(
    key: &str,
    process_ctx: &dyn ProcessContext,
    run_started_ms: Option<u64>,
) -> Option<Value> {
    match key {
        // --- Paths namespace ---
        "paths.home_dir" => process_ctx
            .home_dir()
            .map(|p| Value::String(p.to_string_lossy().into_owned())),
        "paths.output_dir" => process_ctx
            .output_dir()
            .map(|p| Value::String(p.to_string_lossy().into_owned())),
        "paths.work_dir" | "work_dir" => process_ctx
            .work_dir()
            .ok()
            .map(|p| Value::String(p.to_string_lossy().into_owned())),
        "paths.temp_dir" | "temp_dir" => {
            let dir = std::env::temp_dir();
            Some(Value::String(dir.to_string_lossy().into_owned()))
        }
        // --- Flat namespace ---
        "platform" => Some(Value::String(current_platform().to_string())),
        "started" => run_started_ms.map(|ms| Value::String(crate::time::rfc3339_utc(ms))),
        "date" | "time" | "timestamp" => {
            let (y, m, d, hh, mm, ss) = crate::time::now_civil();
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

/// Resolve only `{{ctx.*}}` variables in a string.
///
/// The environment (CLI, TUI) calls this to expand template variables in
/// the output directory path before deciding where to write files.
/// Only resolves ctx namespace — ignores fields, env, item, node placeholders.
pub fn resolve_ctx_templates(s: &str, ctx: &dyn ProcessContext) -> String {
    if !s.contains("{{ctx.") {
        return s.to_string();
    }
    interpolate(s, |inner| {
        let key = inner.strip_prefix("ctx.")?;
        Some(
            resolve_ctx(key, ctx, None)
                .map(|v| value_to_string(&v))
                .unwrap_or_default(),
        )
    })
}

/// The platform name a platform guard compares (`HostPlatform::as_str`),
/// or "unknown" on an OS bnto has no name for.
fn current_platform() -> &'static str {
    HostPlatform::current().map_or("unknown", HostPlatform::as_str)
}

/// Resolve {{node.<id>.<prop>}} from previously executed node outputs.
fn resolve_node_ref(
    node_id: &str,
    prop: &str,
    node_outputs: &dyn NodeOutputSource,
) -> Option<Value> {
    let output = node_outputs.node_output(node_id)?;
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

    // Multiple or partial placeholders — one pass, values never rescanned.
    Value::String(interpolate(s, |inner| interpolated_value(inner, ctx)))
}

/// The text one `{{ns.key}}` fills in during interpolation, or `None` to
/// leave it as written: unknown fields, item outside a loop, other
/// namespaces. Unset env, ctx and node values fill in as empty text.
fn interpolated_value(inner: &str, ctx: &TemplateContext) -> Option<String> {
    let (namespace, key) = inner.split_once('.')?;
    let text = |value: Option<Value>| value.map(|v| value_to_string(&v)).unwrap_or_default();
    match namespace {
        "fields" => ctx.field_values.get(key).map(value_to_string),
        "env" => Some(ctx.process_ctx.env_var(key).unwrap_or_default()),
        "ctx" => Some(text(resolve_ctx(key, ctx.process_ctx, ctx.run_started_ms))),
        "item" => ctx
            .loop_item
            .as_ref()
            .map(|item| text(item.get(key).cloned())),
        "node" => {
            let (node_id, prop) = key.rsplit_once('.')?;
            Some(text(resolve_node_ref(node_id, prop, ctx.node_outputs)))
        }
        _ => None,
    }
}

// --- Input File Metadata ---

use crate::pipeline::{PipelineFile, PipelineNode};

/// Build the initial `node_outputs` map, seeded with input file metadata.
///
/// Finds the `input` node in the pipeline and populates its namespace
/// from the first file's name. This enables `{{node.input.filename_title}}`
/// and related templates before any processing starts.
pub fn build_node_outputs_for_input(
    nodes: &[PipelineNode],
    files: &[PipelineFile],
) -> BTreeMap<String, Value> {
    let mut outputs = BTreeMap::new();
    if let Some(input_node) = nodes.iter().find(|n| n.node_type == "input")
        && let Some(first_file) = files.first()
    {
        outputs.insert(
            input_node.id.clone(),
            build_input_metadata(&first_file.name),
        );
    }
    outputs
}

/// Build a JSON object with filename metadata for template resolution.
///
/// Produces `filename`, `filename_stem`, `filename_title`, and `filename_ext`
/// from a filename string. `filename_title` applies deslug + title case,
/// e.g. `"vehicles-and-monsters"` → `"Vehicles And Monsters"`.
pub fn build_input_metadata(filename: &str) -> Value {
    let path = std::path::Path::new(filename);
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(filename);
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    serde_json::json!({
        "filename": filename,
        "filename_stem": stem,
        "filename_title": crate::case::deslug_title(stem),
        "filename_ext": ext,
    })
}

/// Resolve `{{node.*}}` placeholders in a plain string.
///
/// Used outside the full template system — e.g. for output directory paths
/// that need node-derived values before pipeline execution starts.
pub fn resolve_node_templates(s: &str, node_outputs: &BTreeMap<String, Value>) -> String {
    if !s.contains("{{node.") {
        return s.to_string();
    }
    interpolate(s, |inner| {
        let (node_id, prop) = inner.strip_prefix("node.")?.rsplit_once('.')?;
        Some(
            resolve_node_ref(node_id, prop, node_outputs)
                .map(|v| value_to_string(&v))
                .unwrap_or_default(),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::NoopContext;
    use serde_json::json;
    use std::path::{Path, PathBuf};

    // --- {{ctx.started}} ---

    fn started_ctx<'a>(
        fields: &'a BTreeMap<String, Value>,
        outputs: &'a BTreeMap<String, Value>,
        started: Option<u64>,
    ) -> TemplateContext<'a> {
        TemplateContext {
            field_values: fields,
            process_ctx: &NoopContext,
            node_outputs: outputs,
            loop_item: &None,
            run_started_ms: started,
        }
    }

    #[test]
    fn test_ctx_started_resolves_the_run_start_as_rfc3339() {
        let (fields, outputs) = (BTreeMap::new(), BTreeMap::new());
        let ctx = started_ctx(&fields, &outputs, Some(1_700_000_000_123));
        let params = json!({ "a": "{{ctx.started}}", "b": "after {{ctx.started}}" });
        let resolved = resolve_templates(params.as_object().unwrap(), &ctx);
        assert_eq!(resolved["a"], "2023-11-14T22:13:20.123Z");
        assert_eq!(resolved["b"], "after 2023-11-14T22:13:20.123Z");
    }

    #[test]
    fn test_ctx_started_stays_unresolved_outside_a_run() {
        let (fields, outputs) = (BTreeMap::new(), BTreeMap::new());
        let ctx = started_ctx(&fields, &outputs, None);
        let params = json!({ "a": "{{ctx.started}}" });
        let resolved = resolve_templates(params.as_object().unwrap(), &ctx);
        assert_eq!(resolved["a"], "{{ctx.started}}");
    }

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

    /// Test context that returns controlled values for env vars and paths.
    struct MockContext {
        env_vars: BTreeMap<String, String>,
        work_dir: PathBuf,
        home_dir: PathBuf,
        output_dir: PathBuf,
    }

    impl MockContext {
        fn new() -> Self {
            Self {
                env_vars: BTreeMap::new(),
                work_dir: PathBuf::from("/mock/work"),
                home_dir: PathBuf::from("/mock/home/.bnto"),
                output_dir: PathBuf::from("/mock/home/.bnto/output"),
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
        fn home_dir(&self) -> Option<&Path> {
            Some(&self.home_dir)
        }
        fn output_dir(&self) -> Option<PathBuf> {
            Some(self.output_dir.clone())
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
            run_started_ms: None,
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
            run_started_ms: None,
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
            run_started_ms: None,
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
            run_started_ms: None,
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
            run_started_ms: None,
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
            run_started_ms: None,
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
            run_started_ms: None,
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
            run_started_ms: None,
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
            run_started_ms: None,
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
            run_started_ms: None,
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
            run_started_ms: None,
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
            run_started_ms: None,
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
            run_started_ms: None,
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
    fn resolve_ctx_templates_paths_namespace() {
        let mock = MockContext::new();
        let result = resolve_ctx_templates("{{ctx.paths.output_dir}}/{{ctx.date}}-compress", &mock);
        assert!(result.starts_with("/mock/home/.bnto/output/"));
        assert!(result.ends_with("-compress"));
        assert!(!result.contains("{{"));
    }

    // --- {{ctx.paths.*}} organized namespace ---

    #[test]
    fn ctx_paths_home_dir_resolves() {
        let params = make_params(&[("dir", json!("{{ctx.paths.home_dir}}"))]);
        let mock = MockContext::new();
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &mock,
            node_outputs: &empty_outputs(),
            loop_item: &None,
            run_started_ms: None,
        };
        let resolved = resolve_templates(&params, &ctx);
        assert_eq!(resolved["dir"], json!("/mock/home/.bnto"));
    }

    #[test]
    fn ctx_paths_output_dir_resolves() {
        let params = make_params(&[("dir", json!("{{ctx.paths.output_dir}}"))]);
        let mock = MockContext::new();
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &mock,
            node_outputs: &empty_outputs(),
            loop_item: &None,
            run_started_ms: None,
        };
        let resolved = resolve_templates(&params, &ctx);
        assert_eq!(resolved["dir"], json!("/mock/home/.bnto/output"));
    }

    #[test]
    fn ctx_paths_work_dir_resolves() {
        let params = make_params(&[("dir", json!("{{ctx.paths.work_dir}}"))]);
        let mock = MockContext::new();
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &mock,
            node_outputs: &empty_outputs(),
            loop_item: &None,
            run_started_ms: None,
        };
        let resolved = resolve_templates(&params, &ctx);
        assert_eq!(resolved["dir"], json!("/mock/work"));
    }

    #[test]
    fn ctx_paths_work_dir_alias() {
        // Old flat form still works.
        let params = make_params(&[
            ("old", json!("{{ctx.work_dir}}")),
            ("new", json!("{{ctx.paths.work_dir}}")),
        ]);
        let mock = MockContext::new();
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &mock,
            node_outputs: &empty_outputs(),
            loop_item: &None,
            run_started_ms: None,
        };
        let resolved = resolve_templates(&params, &ctx);
        assert_eq!(resolved["old"], resolved["new"]);
    }

    #[test]
    fn ctx_paths_temp_dir_resolves() {
        let params = make_params(&[("dir", json!("{{ctx.paths.temp_dir}}"))]);
        let mock = MockContext::new();
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &mock,
            node_outputs: &empty_outputs(),
            loop_item: &None,
            run_started_ms: None,
        };
        let resolved = resolve_templates(&params, &ctx);
        let val = resolved["dir"].as_str().unwrap();
        assert!(!val.is_empty());
        assert!(!val.contains("{{"));
    }

    #[test]
    fn ctx_paths_temp_dir_alias() {
        // Old flat form still works.
        let params = make_params(&[
            ("old", json!("{{ctx.temp_dir}}")),
            ("new", json!("{{ctx.paths.temp_dir}}")),
        ]);
        let mock = MockContext::new();
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &mock,
            node_outputs: &empty_outputs(),
            loop_item: &None,
            run_started_ms: None,
        };
        let resolved = resolve_templates(&params, &ctx);
        assert_eq!(resolved["old"], resolved["new"]);
    }

    #[test]
    fn ctx_paths_unknown_left_asis() {
        let params = make_params(&[("x", json!("{{ctx.paths.unknown}}"))]);
        let mock = MockContext::new();
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &mock,
            node_outputs: &empty_outputs(),
            loop_item: &None,
            run_started_ms: None,
        };
        let resolved = resolve_templates(&params, &ctx);
        assert_eq!(resolved["x"], json!("{{ctx.paths.unknown}}"));
    }

    #[test]
    fn ctx_paths_output_dir_in_interpolation() {
        let params = make_params(&[("dir", json!("{{ctx.paths.output_dir}}/{{ctx.date}}-images"))]);
        let mock = MockContext::new();
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &mock,
            node_outputs: &empty_outputs(),
            loop_item: &None,
            run_started_ms: None,
        };
        let resolved = resolve_templates(&params, &ctx);
        let val = resolved["dir"].as_str().unwrap();
        assert!(val.starts_with("/mock/home/.bnto/output/"));
        assert!(val.ends_with("-images"));
        assert!(!val.contains("{{"));
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
            run_started_ms: None,
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
            run_started_ms: None,
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
            run_started_ms: None,
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
            run_started_ms: None,
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
            run_started_ms: None,
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
            run_started_ms: None,
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
            run_started_ms: None,
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
            run_started_ms: None,
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
            run_started_ms: None,
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
            run_started_ms: None,
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
            run_started_ms: None,
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
            run_started_ms: None,
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
            run_started_ms: None,
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
            run_started_ms: None,
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
            run_started_ms: None,
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
            run_started_ms: None,
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
            run_started_ms: None,
        };
        let resolved = resolve_templates(&params, &ctx);
        assert_eq!(resolved["x"], json!("mp4_Beta"));
    }

    // --- build_input_metadata ---

    #[test]
    fn input_metadata_with_extension() {
        let meta = build_input_metadata("vehicles-and-monsters.csv");
        assert_eq!(meta["filename"], json!("vehicles-and-monsters.csv"));
        assert_eq!(meta["filename_stem"], json!("vehicles-and-monsters"));
        assert_eq!(meta["filename_title"], json!("Vehicles And Monsters"));
        assert_eq!(meta["filename_ext"], json!("csv"));
    }

    #[test]
    fn input_metadata_multi_dot_extension() {
        let meta = build_input_metadata("archive.tar.gz");
        assert_eq!(meta["filename"], json!("archive.tar.gz"));
        assert_eq!(meta["filename_stem"], json!("archive.tar"));
        assert_eq!(meta["filename_ext"], json!("gz"));
    }

    #[test]
    fn input_metadata_no_extension() {
        let meta = build_input_metadata("Makefile");
        assert_eq!(meta["filename"], json!("Makefile"));
        assert_eq!(meta["filename_stem"], json!("Makefile"));
        assert_eq!(meta["filename_ext"], json!(""));
    }

    #[test]
    fn input_metadata_deslug_title_transform() {
        let meta = build_input_metadata("my_project-name.txt");
        // deslug replaces both hyphens and underscores with spaces
        assert_eq!(meta["filename_title"], json!("My Project Name"));
    }

    // --- build_node_outputs_for_input ---

    use crate::pipeline::PipelineNode;

    fn make_input_node(id: &str) -> PipelineNode {
        PipelineNode {
            id: id.to_string(),
            node_type: "input".to_string(),
            params: serde_json::Map::new(),
            fields: BTreeMap::new(),
            always: false,
            when: None,
            children: None,
            edges: vec![],
        }
    }

    fn make_processing_node(id: &str, node_type: &str) -> PipelineNode {
        PipelineNode {
            id: id.to_string(),
            node_type: node_type.to_string(),
            params: serde_json::Map::new(),
            fields: BTreeMap::new(),
            always: false,
            when: None,
            children: None,
            edges: vec![],
        }
    }

    fn make_pipeline_file(name: &str) -> PipelineFile {
        PipelineFile {
            name: name.to_string(),
            data: crate::file_data::FileData::Bytes(Vec::new()),
            mime_type: "text/plain".to_string(),
            metadata: serde_json::Map::new(),
        }
    }

    #[test]
    fn node_outputs_for_input_finds_input_node() {
        let nodes = vec![
            make_input_node("input"),
            make_processing_node("compress", "image-compress"),
        ];
        let files = vec![make_pipeline_file("photos.zip")];
        let outputs = build_node_outputs_for_input(&nodes, &files);
        assert!(outputs.contains_key("input"));
        assert_eq!(outputs["input"]["filename"], json!("photos.zip"));
    }

    #[test]
    fn node_outputs_for_input_empty_files() {
        let nodes = vec![make_input_node("input")];
        let files: Vec<PipelineFile> = Vec::new();
        let outputs = build_node_outputs_for_input(&nodes, &files);
        assert!(outputs.is_empty());
    }

    #[test]
    fn node_outputs_for_input_no_input_node() {
        let nodes = vec![make_processing_node("compress", "image-compress")];
        let files = vec![make_pipeline_file("photo.jpg")];
        let outputs = build_node_outputs_for_input(&nodes, &files);
        assert!(outputs.is_empty());
    }

    #[test]
    fn node_outputs_for_input_uses_first_file() {
        let nodes = vec![make_input_node("input")];
        let files = vec![
            make_pipeline_file("first.csv"),
            make_pipeline_file("second.csv"),
        ];
        let outputs = build_node_outputs_for_input(&nodes, &files);
        assert_eq!(outputs["input"]["filename"], json!("first.csv"));
    }

    // --- resolve_node_templates (public function) ---

    #[test]
    fn resolve_node_templates_resolves_placeholders() {
        let mut outputs = BTreeMap::new();
        outputs.insert("input".to_string(), build_input_metadata("data-set.csv"));
        let result = resolve_node_templates("/downloads/{{node.input.filename_title}}", &outputs);
        assert_eq!(result, "/downloads/Data Set");
    }

    #[test]
    fn resolve_node_templates_no_placeholders() {
        let outputs = BTreeMap::new();
        let result = resolve_node_templates("plain-string", &outputs);
        assert_eq!(result, "plain-string");
    }

    #[test]
    fn resolve_node_templates_empty_string() {
        let outputs = BTreeMap::new();
        let result = resolve_node_templates("", &outputs);
        assert_eq!(result, "");
    }

    #[test]
    fn resolve_node_templates_multiple_placeholders() {
        let mut outputs = BTreeMap::new();
        outputs.insert(
            "input".to_string(),
            build_input_metadata("vehicles-and-monsters.csv"),
        );
        let result = resolve_node_templates(
            "/downloads/{{node.input.filename_title}}/{{node.input.filename_ext}}",
            &outputs,
        );
        assert_eq!(result, "/downloads/Vehicles And Monsters/csv");
    }

    // --- {{node.input.*}} through full resolve_templates ---

    #[test]
    fn node_input_filename_title_through_full_resolution() {
        let mut outputs = BTreeMap::new();
        outputs.insert(
            "input".to_string(),
            build_input_metadata("vehicles-and-monsters.csv"),
        );
        let params = make_params(&[("dest", json!("/downloads/{{node.input.filename_title}}"))]);
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &NoopContext,
            node_outputs: &outputs,
            loop_item: &None,
            run_started_ms: None,
        };
        let resolved = resolve_templates(&params, &ctx);
        assert_eq!(resolved["dest"], json!("/downloads/Vehicles And Monsters"));
    }

    #[test]
    fn node_input_filename_stem_sole_placeholder() {
        let mut outputs = BTreeMap::new();
        outputs.insert("input".to_string(), build_input_metadata("my-data.csv"));
        let params = make_params(&[("stem", json!("{{node.input.filename_stem}}"))]);
        let ctx = TemplateContext {
            field_values: &empty_fields(),
            process_ctx: &NoopContext,
            node_outputs: &outputs,
            loop_item: &None,
            run_started_ms: None,
        };
        let resolved = resolve_templates(&params, &ctx);
        assert_eq!(resolved["stem"], json!("my-data"));
    }

    // --- Single pass: a filled value is data, never a template (SEC-17/18) ---

    fn interp_ctx<'a>(
        fields: &'a BTreeMap<String, Value>,
        process_ctx: &'a MockContext,
        outputs: &'a BTreeMap<String, Value>,
        item: &'a Option<serde_json::Map<String, Value>>,
    ) -> TemplateContext<'a> {
        TemplateContext {
            field_values: fields,
            process_ctx,
            node_outputs: outputs,
            loop_item: item,
            run_started_ms: None,
        }
    }

    #[test]
    fn test_a_filled_field_value_is_not_read_as_a_template() {
        let fields = BTreeMap::from([("v".to_string(), json!("{{env.SECRET}}"))]);
        let mock = MockContext::new().with_env("SECRET", "hunter2");
        let (outputs, item) = (empty_outputs(), None);
        let ctx = interp_ctx(&fields, &mock, &outputs, &item);
        let params = make_params(&[("a", json!("x={{fields.v}}"))]);
        let resolved = resolve_templates(&params, &ctx);
        assert_eq!(resolved["a"], "x={{env.SECRET}}");
    }

    #[test]
    fn test_a_filled_item_value_is_not_read_as_a_template() {
        let fields = empty_fields();
        let mock = MockContext::new();
        let outputs = BTreeMap::from([("n".to_string(), json!({ "p": "leaked" }))]);
        let item = Some(
            json!({ "name": "{{node.n.p}}" })
                .as_object()
                .unwrap()
                .clone(),
        );
        let ctx = interp_ctx(&fields, &mock, &outputs, &item);
        let params = make_params(&[("a", json!("file {{item.name}}"))]);
        assert_eq!(resolve_templates(&params, &ctx)["a"], "file {{node.n.p}}");
    }

    #[test]
    fn test_a_self_referencing_env_value_terminates() {
        let fields = empty_fields();
        let mock = MockContext::new().with_env("LOOP", "{{env.LOOP}}");
        let (outputs, item) = (empty_outputs(), None);
        let ctx = interp_ctx(&fields, &mock, &outputs, &item);
        let params = make_params(&[("a", json!("x={{env.LOOP}}"))]);
        assert_eq!(resolve_templates(&params, &ctx)["a"], "x={{env.LOOP}}");
    }

    #[test]
    fn test_a_self_referencing_item_value_terminates() {
        let fields = empty_fields();
        let mock = MockContext::new();
        let outputs = empty_outputs();
        let item = Some(json!({ "k": "{{item.k}}" }).as_object().unwrap().clone());
        let ctx = interp_ctx(&fields, &mock, &outputs, &item);
        let params = make_params(&[("a", json!("x={{item.k}}"))]);
        assert_eq!(resolve_templates(&params, &ctx)["a"], "x={{item.k}}");
    }

    #[test]
    fn test_a_self_referencing_node_value_terminates() {
        let outputs = BTreeMap::from([("n".to_string(), json!({ "p": "{{node.n.p}}" }))]);
        assert_eq!(
            resolve_node_templates("d/{{node.n.p}}", &outputs),
            "d/{{node.n.p}}"
        );
    }

    #[test]
    fn test_a_stray_open_brace_does_not_hide_a_placeholder() {
        let fields = BTreeMap::from([("x".to_string(), json!("1"))]);
        let mock = MockContext::new();
        let (outputs, item) = (empty_outputs(), None);
        let ctx = interp_ctx(&fields, &mock, &outputs, &item);
        let params = make_params(&[("a", json!("{{a {{fields.x}} b"))]);
        assert_eq!(resolve_templates(&params, &ctx)["a"], "{{a 1 b");
    }

    #[test]
    fn test_unfinished_and_unknown_placeholders_are_kept() {
        let fields = BTreeMap::from([("x".to_string(), json!("1"))]);
        let mock = MockContext::new();
        let (outputs, item) = (empty_outputs(), None);
        let ctx = interp_ctx(&fields, &mock, &outputs, &item);
        let params = make_params(&[("a", json!("{{fields.x}} {{vars.y}} {{fields.x"))]);
        assert_eq!(
            resolve_templates(&params, &ctx)["a"],
            "1 {{vars.y}} {{fields.x"
        );
    }
}
