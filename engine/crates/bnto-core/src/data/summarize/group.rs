// Items into groups, in the order each group's first item arrived.
//
// An item's group is the text its `groupBy` values print as (the same text a
// `{{item.KEY}}` template gives), so CSV text "2024" and a JSON 2024 share a
// group, and absent, null and "" all print as nothing and share the blank one.
// A group keeps its first item's values, with a blank one as null.

use std::collections::HashMap;

use serde_json::{Map, Value};

use super::params::{COUNT_FIELD, SummarizeParams, sum_field};
use super::total::Total;
use crate::control::placeholder;
use crate::processor::BatchFile;

/// One group: its field values, how many items it has and their totals.
struct Group {
    values: Map<String, Value>,
    count: u64,
    /// One per `sum` field, in that order.
    totals: Vec<Total>,
}

/// The fields of each group's summary item, in first-seen order, or the
/// first summed value that isn't a number (naming its item).
pub(super) fn summarize(
    files: &[BatchFile],
    params: &SummarizeParams,
) -> Result<Vec<Map<String, Value>>, String> {
    let mut groups: Vec<Group> = Vec::new();
    let mut index: HashMap<Vec<String>, usize> = HashMap::new();
    for file in files {
        let key = key_of(file, &params.group_by);
        let at = *index.entry(key).or_insert_with(|| {
            groups.push(new_group(file, params));
            groups.len() - 1
        });
        add_to(&mut groups[at], file, &params.sum)?;
    }
    groups.into_iter().map(|g| fields_of(g, params)).collect()
}

/// The text each `groupBy` field of `file` prints as.
fn key_of(file: &BatchFile, group_by: &[String]) -> Vec<String> {
    group_by
        .iter()
        .map(|f| {
            file.metadata
                .get(f)
                .map(placeholder::text)
                .unwrap_or_default()
        })
        .collect()
}

/// An empty group holding `file`'s `groupBy` values, a blank one as null.
fn new_group(file: &BatchFile, params: &SummarizeParams) -> Group {
    let values = params
        .group_by
        .iter()
        .map(|f| (f.clone(), group_value(file.metadata.get(f))))
        .collect();
    Group {
        values,
        count: 0,
        totals: vec![Total::default(); params.sum.len()],
    }
}

fn group_value(value: Option<&Value>) -> Value {
    match value {
        Some(v) if !placeholder::text(v).is_empty() => v.clone(),
        _ => Value::Null,
    }
}

/// Count `file` in `group` and add its `sum` values to the totals.
fn add_to(group: &mut Group, file: &BatchFile, sum: &[String]) -> Result<(), String> {
    group.count += 1;
    for (field, total) in sum.iter().zip(group.totals.iter_mut()) {
        total
            .add(file.metadata.get(field))
            .map_err(|why| format!("{}: sum field \"{field}\": {why}", file.filename))?;
    }
    Ok(())
}

/// A group's summary fields: its values, `count`, then each `sum_<field>`.
fn fields_of(group: Group, params: &SummarizeParams) -> Result<Map<String, Value>, String> {
    let mut fields = group.values;
    fields.insert(COUNT_FIELD.to_string(), group.count.into());
    for (field, total) in params.sum.iter().zip(group.totals) {
        let value = total
            .into_value()
            .map_err(|why| format!("sum field \"{field}\": {why}"))?;
        fields.insert(sum_field(field), value);
    }
    Ok(fields)
}
