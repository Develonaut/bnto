// The values edit-fields sets on one item, every `{{item.KEY}}` filled from
// the fields that item arrived with (the loop's item on top) and every
// `{{file}}` with its name.
//
// The executor hands this node its params unfilled (`resolves_loop_items`),
// in a loop too, so it fills them here, once per file, in one pass over the
// recipe's text, the run's namespaces included, as assert and report fill
// them (`placeholder::fill_value`): a value that reads like a token is set
// as written. A key the item lacks is a recipe mistake: it fails the run
// instead of becoming "".

use serde_json::{Map, Value};

use crate::control::placeholder::{self, Scope};

/// `values` filled for the file `name`: `{{file}}` with `name`, every
/// `{{item.KEY}}` from `item`, the run's namespaces from `once`, or why one
/// can't be: the field being set, the key, and the fields `item` does have.
pub(super) fn fill_values(
    values: &Map<String, Value>,
    name: &str,
    item: &Map<String, Value>,
    once: Scope<'_>,
) -> Result<Map<String, Value>, String> {
    let scope = Scope {
        file: Some(name),
        item: Some(item),
        ..once
    };
    values
        .iter()
        .map(|(field, value)| {
            let filled = placeholder::fill_value(value, scope)
                .map_err(|missing| unknown(field, &missing.key, item))?;
            Ok((field.clone(), filled))
        })
        .collect()
}

fn unknown(field: &str, key: &str, item: &Map<String, Value>) -> String {
    let has = if item.is_empty() {
        "it has no fields".to_string()
    } else {
        let keys: Vec<&str> = item.keys().map(String::as_str).collect();
        format!("it has: {}", keys.join(", "))
    };
    format!("\"{field}\" reads {{{{item.{key}}}}}, but the item has no field \"{key}\" ({has})")
}
