// `{{<namespace>.KEY}}` placeholders a batch node (assert, report, edit-fields,
// file-copy, file-render) fills itself, and the per-file `{{file}}`.
//
// The executor hands these nodes their params unfilled, in a loop too
// (`resolves_loop_items`). The node fills every token in one pass over the
// recipe's template (`fill_text`, `fill_value`, `fill_params`): its own
// (`{{file}}`, `{{item.*}}`, report's `{{entry.*}}`) and the run's
// (`{{fields.*}}`, `{{env.*}}`, `{{ctx.*}}`, `{{node.*}}`, answered by
// `ProcessContext::run_placeholder`). A filled value is never scanned again,
// so one that reads like a token is kept as written (ISS-35). A file's item
// is its metadata with the enclosing loop's item on top (`file_item`), as
// the executor builds it for a per-file node, so a key a step inside the
// iteration adds is there (ISS-58).

use serde_json::{Map, Value};

use crate::context::ProcessContext;

/// The per-file namespace: `{{item.KEY}}` reads the incoming file's metadata.
pub const ITEM: &str = "item";

/// The per-file name token: `{{file}}` reads the incoming file's name, as
/// file-render and shell-run's `each` read it.
pub(crate) const FILE: &str = "{{file}}";

/// `{{<namespace>.`, the start of every placeholder in that namespace.
fn open(namespace: &str) -> String {
    format!("{{{{{namespace}.")
}

/// Whether `text` reads any `{{<namespace>.KEY}}`.
pub fn reads(text: &str, namespace: &str) -> bool {
    text.contains(&open(namespace))
}

/// Why `{{item.KEY}}` can't be filled: no value for `file`, and the keys
/// `item` does have. The executor and nodes that fill items themselves share it.
pub fn no_value(key: &str, file: Option<&str>, item: Option<&Map<String, Value>>) -> String {
    let target = file.map(|f| format!(" for {f}")).unwrap_or_default();
    let available = match item {
        Some(item) if !item.is_empty() => {
            let keys: Vec<&str> = item.keys().map(String::as_str).collect();
            format!("it has: {}", keys.join(", "))
        }
        _ => "it has no item values".to_string(),
    };
    format!("{{{{item.{key}}}}} has no value{target} ({available})")
}

/// Whether `text` reads the file it is filled for: `{{file}}` or `{{item.*}}`.
pub(crate) fn reads_file(text: &str) -> bool {
    text.contains(FILE) || reads(text, ITEM)
}

/// What one fill reads: the file's name for `{{file}}`, an item for
/// `{{item.KEY}}`, report's entry for `{{entry}}` and `{{entry.KEY}}`, and
/// the run (`run_placeholder`) for any other namespace. A token whose source
/// isn't set is kept as written.
#[derive(Default, Clone, Copy)]
pub struct Scope<'a> {
    pub file: Option<&'a str>,
    pub item: Option<&'a Map<String, Value>>,
    pub entry: Option<&'a Value>,
    pub run: Option<&'a dyn ProcessContext>,
}

/// A key the scope's item or entry doesn't have.
#[derive(Debug, PartialEq, Eq)]
pub struct Missing {
    /// `item` or `entry`.
    pub namespace: &'static str,
    pub key: String,
}

impl Scope<'_> {
    /// The value `{{inner}}` reads as a whole value (a run value keeps its
    /// JSON type), `None` to keep it as written, or the key the item or entry
    /// lacks.
    pub fn value(&self, inner: &str) -> Result<Option<Value>, Missing> {
        self.lookup(inner, true)
    }

    /// The text `{{inner}}` fills in among other text.
    pub fn text(&self, inner: &str) -> Result<Option<String>, Missing> {
        Ok(self.lookup(inner, false)?.map(|v| text(&v)))
    }

    fn lookup(&self, inner: &str, whole: bool) -> Result<Option<Value>, Missing> {
        if inner == "file" {
            return Ok(self.file.map(|name| Value::String(name.to_string())));
        }
        if inner == ENTRY {
            return Ok(self.entry.cloned());
        }
        if let Some(key) = inner.strip_prefix("item.") {
            return lookup(self.item, ITEM, key);
        }
        if let Some(key) = inner.strip_prefix("entry.") {
            // An entry that isn't a map (text, a number) has no fields.
            let no_fields = Map::new();
            let fields = self.entry.map(|e| e.as_object().unwrap_or(&no_fields));
            return lookup(fields, ENTRY, key);
        }
        Ok(self.run.and_then(|run| run.run_placeholder(inner, whole)))
    }
}

const ENTRY: &str = "entry";

fn lookup(
    values: Option<&Map<String, Value>>,
    namespace: &'static str,
    key: &str,
) -> Result<Option<Value>, Missing> {
    let Some(values) = values else {
        return Ok(None);
    };
    let missing = || Missing {
        namespace,
        key: key.to_string(),
    };
    values.get(key).cloned().map(Some).ok_or_else(missing)
}

/// `text` filled from `scope` in one pass: a filled value is never scanned.
pub fn fill_text(text: &str, scope: Scope<'_>) -> Result<String, Missing> {
    fill_with(text, |inner| scope.text(inner))
}

/// `text` filled in one pass by `fill` (`None` keeps a token as written),
/// for a node with tokens of its own (file-render's `{{count}}`).
pub fn fill_with<E>(
    text: &str,
    fill: impl FnMut(&str) -> Result<Option<String>, E>,
) -> Result<String, E> {
    crate::executor::interpolate::try_interpolate(text, fill)
}

/// `text` with a node's own tokens filled in one pass by `fill` (`None`
/// keeps a token as written), such as file-rename's `{{name}}` and `{{ext}}`.
pub fn fill_tokens(text: &str, fill: impl Fn(&str) -> Option<String>) -> String {
    crate::executor::interpolate::interpolate(text, fill)
}

/// `value` filled from `scope`, recursing into lists and maps. Text that is
/// one whole token is the value itself, type and all, as the executor fills
/// a sole placeholder; other text stays text.
pub fn fill_value(value: &Value, scope: Scope<'_>) -> Result<Value, Missing> {
    match value {
        Value::String(text) => fill_string(text, scope),
        Value::Array(values) => values.iter().map(|v| fill_value(v, scope)).collect(),
        Value::Object(map) => map
            .iter()
            .map(|(k, v)| Ok((k.clone(), fill_value(v, scope)?)))
            .collect::<Result<Map<_, _>, _>>()
            .map(Value::Object),
        other => Ok(other.clone()),
    }
}

fn fill_string(text: &str, scope: Scope<'_>) -> Result<Value, Missing> {
    if let Some(inner) = sole_token(text)
        && let Some(value) = scope.value(inner)?
    {
        return Ok(value);
    }
    fill_text(text, scope).map(Value::String)
}

/// `params` filled from `scope`, all but those named in `per_file`, which
/// the node fills itself, once per file; or the param and key that can't be.
pub fn fill_params(
    params: &Map<String, Value>,
    scope: Scope<'_>,
    per_file: &[&str],
) -> Result<Map<String, Value>, (String, Missing)> {
    let fill = |(name, value): (&String, &Value)| {
        if per_file.contains(&name.as_str()) {
            return Ok((name.clone(), value.clone()));
        }
        let filled = fill_value(value, scope).map_err(|m| (name.clone(), m))?;
        Ok((name.clone(), filled))
    };
    params.iter().map(fill).collect()
}

/// The inside of `text` (trimmed) when it is exactly one `{{…}}`.
fn sole_token(text: &str) -> Option<&str> {
    let inner = text.trim().strip_prefix("{{")?.strip_suffix("}}")?;
    (!inner.contains('{') && !inner.contains('}')).then_some(inner)
}

/// A file's `{{item.*}}` source: its metadata, with the enclosing loop's
/// item on top. The loop item wins a shared key, so a loop over CSV rows
/// keeps reading its row's columns.
pub fn file_item(
    metadata: &Map<String, Value>,
    loop_item: Option<&Map<String, Value>>,
) -> Map<String, Value> {
    let mut item = metadata.clone();
    if let Some(loop_item) = loop_item {
        item.extend(loop_item.clone());
    }
    item
}

/// A value as text: strings as written, other JSON (numbers, lists) as JSON.
pub(crate) fn text(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn values(v: Value) -> Map<String, Value> {
        v.as_object().cloned().unwrap()
    }

    #[test]
    fn test_reads_needs_the_namespace_and_a_dot() {
        assert!(reads("x {{item.k}}", "item"));
        assert!(!reads("x {{items.k}} {{item}}", "item"));
    }

    #[test]
    fn test_reads_file_needs_the_file_token_or_an_item_key() {
        assert!(reads_file("{{file}}") && reads_file("{{item.k}}"));
        assert!(!reads_file("{{files}} {{node.a.file}}"));
    }

    fn file_scope<'a>(name: &'a str, item: &'a Map<String, Value>) -> Scope<'a> {
        Scope {
            file: Some(name),
            item: Some(item),
            ..Scope::default()
        }
    }

    #[test]
    fn test_fill_text_fills_the_name_and_item_keys() {
        let v = values(json!({ "k": "x", "n": 2 }));
        let out = fill_text(
            "{{file}}={{item.k}}{{item.n}} {{file}}",
            file_scope("a.ts", &v),
        );
        assert_eq!(out.unwrap(), "a.ts=x2 a.ts");
    }

    #[test]
    fn test_fill_text_keeps_filled_values_that_read_like_tokens() {
        let v = values(json!({ "k": "{{file}}" }));
        let out = fill_text("{{item.k}}", file_scope("{{item.k}}.ts", &v)).unwrap();
        assert_eq!(out, "{{file}}");
        let out = fill_text("{{file}}", file_scope("{{item.k}}.ts", &v)).unwrap();
        assert_eq!(out, "{{item.k}}.ts");
    }

    #[test]
    fn test_fill_text_returns_a_missing_key() {
        let v = values(json!({}));
        let missing = Missing {
            namespace: "item",
            key: "gone".into(),
        };
        assert_eq!(
            fill_text("{{file}} {{item.gone}}", file_scope("a", &v)),
            Err(missing)
        );
    }

    #[test]
    fn test_a_token_with_no_source_is_left_as_written() {
        let out = fill_text(
            "{{file}} {{item.k}} {{entry.k}} {{env.X}}",
            Scope::default(),
        );
        assert_eq!(out.unwrap(), "{{file}} {{item.k}} {{entry.k}} {{env.X}}");
    }

    #[test]
    fn test_entry_fields_and_the_whole_entry_fill_with_the_item() {
        let item = values(json!({ "g": "team" }));
        let entry = json!({ "name": "{{item.g}}" });
        let scope = Scope {
            item: Some(&item),
            entry: Some(&entry),
            ..Scope::default()
        };
        let out = fill_text("{{item.g}}: {{entry.name}}", scope).unwrap();
        assert_eq!(out, "team: {{item.g}}");
        let plain = json!("x");
        let scope = Scope {
            entry: Some(&plain),
            ..Scope::default()
        };
        assert_eq!(fill_text("- {{entry}}", scope).unwrap(), "- x");
    }

    #[test]
    fn test_fill_value_keeps_a_sole_tokens_type_and_recurses() {
        let v = values(json!({ "n": 2, "list": [1, 2] }));
        let value = json!({ "a": "{{item.n}}", "b": [" {{item.list}} ", "n={{item.n}}"] });
        let out = fill_value(&value, file_scope("f", &v)).unwrap();
        assert_eq!(out, json!({ "a": 2, "b": [[1, 2], "n=2"] }));
    }

    #[test]
    fn test_file_item_puts_the_loop_item_on_top() {
        let metadata = values(json!({ "k": "file", "added": 1 }));
        let loop_item = values(json!({ "k": "row" }));
        let item = file_item(&metadata, Some(&loop_item));
        assert_eq!(Value::Object(item), json!({ "k": "row", "added": 1 }));
    }

    /// A run whose `fields.x` holds a token and whose `fields.n` is a number.
    struct Run;

    impl ProcessContext for Run {
        fn run_command(&self, cmd: &str, args: &[&str]) -> Result<Vec<u8>, crate::BntoError> {
            crate::context::NoopContext.run_command(cmd, args)
        }
        fn temp_file(&self, suffix: &str) -> Result<std::path::PathBuf, crate::BntoError> {
            crate::context::NoopContext.temp_file(suffix)
        }
        fn env_var(&self, _key: &str) -> Option<String> {
            None
        }
        fn work_dir(&self) -> Result<&std::path::Path, crate::BntoError> {
            crate::context::NoopContext.work_dir()
        }
        fn run_placeholder(&self, inner: &str, whole: bool) -> Option<Value> {
            match inner {
                "fields.x" => Some(json!("{{file}}")),
                "fields.n" if whole => Some(json!(2)),
                "fields.n" => Some(json!("2")),
                _ => None,
            }
        }
    }

    #[test]
    fn test_the_runs_namespaces_fill_in_the_same_pass() {
        let v = values(json!({}));
        let scope = Scope {
            run: Some(&Run),
            ..file_scope("a.ts", &v)
        };
        let out = fill_text("{{fields.x}} {{file}} n={{fields.n}} {{fields.y}}", scope);
        assert_eq!(out.unwrap(), "{{file}} a.ts n=2 {{fields.y}}");
        assert_eq!(fill_value(&json!("{{fields.n}}"), scope).unwrap(), json!(2));
    }

    #[test]
    fn test_fill_params_leaves_the_per_file_ones_as_written() {
        let params = values(json!({ "a": "{{fields.n}}", "b": "{{fields.n}}" }));
        let scope = Scope {
            run: Some(&Run),
            ..Scope::default()
        };
        let out = fill_params(&params, scope, &["b"]).unwrap();
        assert_eq!(Value::Object(out), json!({ "a": 2, "b": "{{fields.n}}" }));
    }

    #[test]
    fn test_an_unclosed_placeholder_is_left_as_written() {
        let v = values(json!({ "k": "x" }));
        assert_eq!(
            fill_text("{{item.k", file_scope("f", &v)).unwrap(),
            "{{item.k"
        );
    }
}
