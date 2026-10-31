// The `{{item.*}}` a step reads when it is decided or filled once for all its
// files, inside a loop: a `when` guard, and a batch step the executor fills.
// It's the item a per-file step would read, from the files the step is
// about to get (ISS-110), so several files must agree on every key it reads.

use crate::control::placeholder;
use crate::pipeline::PipelineFile;

type Params = serde_json::Map<String, serde_json::Value>;

/// The item a step reads for `params` (or a guard's texts): inside a loop,
/// `step_item` over the keys they read; outside one, none, as before.
pub(super) fn for_step(
    loop_item: &Option<Params>,
    files: &[PipelineFile],
    params: &Params,
) -> Result<Option<Params>, String> {
    if loop_item.is_none() {
        return Ok(None);
    }
    let keys = super::unfilled_items::item_keys(params);
    step_item(loop_item.as_ref(), files, &keys)
}

/// The step's item: the loop's item with no files, the file's item with
/// one, and with several the keys in `keys` they agree on, over the loop's
/// item; or why they don't agree.
pub(super) fn step_item(
    loop_item: Option<&Params>,
    files: &[PipelineFile],
    keys: &[String],
) -> Result<Option<Params>, String> {
    match files {
        [] => Ok(loop_item.cloned()),
        [file] => Ok(Some(placeholder::file_item(&file.metadata, loop_item))),
        _ => agreed(loop_item, files, keys).map(Some),
    }
}

/// The loop's item with each of `keys` set to the one value `files` give it,
/// or why they give two.
fn agreed(
    loop_item: Option<&Params>,
    files: &[PipelineFile],
    keys: &[String],
) -> Result<Params, String> {
    let items: Vec<Params> = files
        .iter()
        .map(|f| placeholder::file_item(&f.metadata, loop_item))
        .collect();
    let mut item = loop_item.cloned().unwrap_or_default();
    for key in keys {
        if let Some(value) = one_value(&items, key)? {
            item.insert(key.clone(), value);
        }
    }
    Ok(item)
}

/// The value every item gives `key` (`None` when none has it), or why not.
fn one_value(items: &[Params], key: &str) -> Result<Option<serde_json::Value>, String> {
    let first = items[0].get(key);
    match items.iter().find(|item| item.get(key) != first) {
        None => Ok(first.cloned()),
        Some(other) => Err(disagree(key, items.len(), first, other.get(key))),
    }
}

fn disagree(
    key: &str,
    count: usize,
    a: Option<&serde_json::Value>,
    b: Option<&serde_json::Value>,
) -> String {
    let show = |v: Option<&serde_json::Value>| {
        v.map_or("no value".to_string(), |v| {
            format!("\"{}\"", placeholder::text(v))
        })
    };
    format!(
        "the step gets {count} files whose {{{{item.{key}}}}} differ ({} and {}): \
         split the files with file-filter, or read the loop's item with {{{{loop.{key}}}}} \
         if it has that key",
        show(a),
        show(b)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::processor::FileData;
    use serde_json::{Value, json};

    fn params(v: Value) -> Params {
        v.as_object().cloned().unwrap()
    }

    fn file(metadata: Value) -> PipelineFile {
        PipelineFile {
            name: "f".into(),
            data: FileData::Bytes(Vec::new()),
            mime_type: "text/plain".into(),
            metadata: params(metadata),
        }
    }

    fn keys(names: &[&str]) -> Vec<String> {
        names.iter().map(|k| k.to_string()).collect()
    }

    #[test]
    fn test_with_no_files_the_guard_reads_the_loop_item() {
        let row = params(json!({ "k": "row" }));
        let item = step_item(Some(&row), &[], &keys(&["k"])).unwrap();
        assert_eq!(item, Some(row));
    }

    #[test]
    fn test_with_one_file_its_value_wins_over_the_loop_item() {
        let row = params(json!({ "k": "row", "only": 1 }));
        let files = [file(json!({ "k": "step" }))];
        let item = step_item(Some(&row), &files, &keys(&["k"]))
            .unwrap()
            .unwrap();
        assert_eq!(item["k"], "step");
        assert_eq!(item["only"], 1, "a key only the row has still fills");
    }

    #[test]
    fn test_several_files_that_agree_give_their_value() {
        // Files a step made lack the row's column, so they read the row's.
        let row = params(json!({ "title": "Ada" }));
        let files = [file(json!({ "n": 1 })), file(json!({ "n": 2 }))];
        let item = step_item(Some(&row), &files, &keys(&["title"]))
            .unwrap()
            .unwrap();
        assert_eq!(item["title"], "Ada");
    }

    #[test]
    fn test_several_files_that_disagree_are_refused() {
        let files = [file(json!({ "k": "a" })), file(json!({ "k": "b" }))];
        let error = step_item(None, &files, &keys(&["k"])).unwrap_err();
        for part in ["{{item.k}}", "\"a\"", "\"b\"", "{{loop.k}}", "file-filter"] {
            assert!(error.contains(part), "{part}: {error}");
        }
    }
}
