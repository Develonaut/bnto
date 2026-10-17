// JSON parsing for spreadsheet-read: each element of a top-level array becomes one row.
//
// An object's fields become the row's keys, nested objects flattened into
// dotted paths (`{"meta": {"id": "x"}}` → `meta.id`), so `{{item.meta.id}}`
// reads them with no template support of its own. Values keep their JSON
// type. Arrays and empty objects are leaves, kept as JSON under their key.
// An element that isn't an object is kept whole under `value`.

use std::collections::BTreeSet;

use bnto_core::errors::BntoError;
use serde_json::Value;

use super::table::{Row, Table};

/// The key an element that isn't an object is kept under.
const VALUE_KEY: &str = "value";

const UTF8_BOM: &[u8] = &[0xEF, 0xBB, 0xBF];

/// Read a JSON array into a table, one row per element. Errors name `filename`.
pub(super) fn read(data: &[u8], filename: &str) -> Result<Table, BntoError> {
    let rows = parse_array(data, filename)?
        .into_iter()
        .enumerate()
        .map(|(i, element)| element_row(element).map_err(|key| collision(filename, i, &key)))
        .collect::<Result<Vec<_>, _>>()?;
    let column_count = rows
        .iter()
        .flat_map(Row::keys)
        .collect::<BTreeSet<_>>()
        .len();
    Ok(Table { column_count, rows })
}

/// The file's top-level array, or why it isn't one.
fn parse_array(data: &[u8], filename: &str) -> Result<Vec<Value>, BntoError> {
    let data = data.strip_prefix(UTF8_BOM).unwrap_or(data);
    let value: Value = serde_json::from_slice(data)
        .map_err(|e| invalid(format!("{filename} is not valid JSON: {e}")))?;
    match value {
        Value::Array(elements) => Ok(elements),
        other => Err(invalid(format!(
            "{filename} is not a JSON array (it holds {}). \
             spreadsheet-read reads an array, one item per element.",
            kind(&other)
        ))),
    }
}

fn kind(value: &Value) -> &'static str {
    match value {
        Value::Object(_) => "an object",
        Value::Array(_) => "an array",
        Value::String(_) => "a string",
        Value::Number(_) => "a number",
        Value::Bool(_) => "a boolean",
        Value::Null => "null",
    }
}

/// One element's row. Err is a key two of its fields both flatten to.
fn element_row(element: Value) -> Result<Row, String> {
    let mut row = Row::new();
    match element {
        Value::Object(fields) => flatten_into(&mut row, None, fields)?,
        other => {
            row.insert(VALUE_KEY.to_string(), other);
        }
    }
    Ok(row)
}

/// Insert `fields` into `row` under `prefix.`, recursing into non-empty objects.
fn flatten_into(row: &mut Row, prefix: Option<&str>, fields: Row) -> Result<(), String> {
    for (key, value) in fields {
        let path = match prefix {
            Some(prefix) => format!("{prefix}.{key}"),
            None => key,
        };
        match value {
            Value::Object(inner) if !inner.is_empty() => flatten_into(row, Some(&path), inner)?,
            _ if row.contains_key(&path) => return Err(path),
            leaf => {
                row.insert(path, leaf);
            }
        }
    }
    Ok(())
}

/// `{"a.b": 1, "a": {"b": 2}}` has two fields on `a.b`; neither silently wins.
fn collision(filename: &str, index: usize, key: &str) -> BntoError {
    invalid(format!(
        "{filename}: element {index} has two fields that both read as \"{key}\" \
         (a key with a dot and a nested object). Rename one of them."
    ))
}

fn invalid(message: String) -> BntoError {
    BntoError::InvalidInput(message)
}
