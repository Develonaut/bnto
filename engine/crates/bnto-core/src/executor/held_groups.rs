// Filling the params of a node that fills some `{{item.*}}` itself, inside a
// loop (shell-run's `each` groups, ISS-111). Values the node marks
// (`NodeProcessor::holds_item_templates`) keep their `{{item.*}}` for the
// node, which fills them per file over the loop's item; every other token in
// them is filled here. The rest of the params read the step's item
// (`step_item`), and only their keys must be filled and agree. SEC-27 counts
// a node's own tokens in each marked value alone too: across the whole params
// a token the executor used up outside could mask one a value brought into a
// marked value, which the node would then fill.

use serde_json::Value;

use super::field_scope::FieldValues;
use super::template::{TemplateContext, resolve_value};
use super::{PipelineContext, PipelineNodeRef};
use crate::errors::BntoError;
use crate::pipeline::PipelineFile;
use crate::processor::NodeProcessor;

type Params = serde_json::Map<String, Value>;

/// `node`'s params filled for `files` inside a loop, its marked values
/// keeping their `{{item.*}}`; or why they can't be.
pub(super) fn resolve<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    node: PipelineNodeRef,
    processor: &dyn NodeProcessor,
    files: &[PipelineFile],
    field_values: &FieldValues,
) -> Result<Params, BntoError> {
    let holds = |v: &Value| processor.holds_item_templates(v);
    let outside = without_held(&node.params, &holds);
    let invalid = |why: String| BntoError::InvalidInput(format!("Node '{}': {why}", node.id));
    let item = super::step_item::for_step(&ctx.loop_item, files, &outside).map_err(invalid)?;
    if let Some(key) = super::unfilled_items::unfilled_item_keys(&outside, &item).first() {
        let why = crate::control::placeholder::no_value(key, None, item.as_ref());
        return Err(invalid(why));
    }
    let filled = template(ctx, field_values, &item);
    let held = template(ctx, field_values, &None);
    let resolved = fill_holding(&node.params, &filled, &held, &holds);
    check_each_held(processor, node, &resolved, &holds)?;
    Ok(resolved)
}

/// SEC-27 within each marked value: filling it may not add a token the
/// node fills itself.
fn check_each_held(
    processor: &dyn NodeProcessor,
    node: PipelineNodeRef,
    resolved: &Params,
    holds: &dyn Fn(&Value) -> bool,
) -> Result<(), BntoError> {
    let (mut raw, mut filled) = (Vec::new(), Vec::new());
    for (key, value) in &node.params {
        held_pairs(value, &resolved[key], holds, &mut raw, &mut filled);
    }
    for (raw, filled) in raw.into_iter().zip(filled) {
        let one = |v: Value| Params::from_iter([("held".to_string(), v)]);
        let reserved = processor.reserved_tokens();
        super::reserved_tokens::check_reserved(reserved, &one(raw), &one(filled), &node.id)?;
    }
    Ok(())
}

/// Every marked value under `raw`, with its filled copy from `filled` (the
/// same shape: filling keeps arrays and maps as they are).
fn held_pairs(
    raw: &Value,
    filled: &Value,
    holds: &dyn Fn(&Value) -> bool,
    raws: &mut Vec<Value>,
    filleds: &mut Vec<Value>,
) {
    if holds(raw) {
        raws.push(raw.clone());
        filleds.push(filled.clone());
        return;
    }
    match (raw, filled) {
        (Value::Array(a), Value::Array(b)) => {
            a.iter()
                .zip(b)
                .for_each(|(a, b)| held_pairs(a, b, holds, raws, filleds));
        }
        (Value::Object(a), Value::Object(b)) => {
            for (k, v) in a {
                if let Some(w) = b.get(k) {
                    held_pairs(v, w, holds, raws, filleds);
                }
            }
        }
        _ => {}
    }
}

fn template<'a, F: Fn() -> u64 + Copy>(
    ctx: &'a PipelineContext<F>,
    field_values: &'a FieldValues,
    item: &'a Option<Params>,
) -> TemplateContext<'a> {
    TemplateContext {
        field_values,
        process_ctx: ctx.process_ctx,
        node_outputs: &ctx.node_outputs,
        loop_item: item,
        loop_row: ctx.loop_item.as_ref(),
        run_started_ms: Some(ctx.started_ms),
    }
}

/// `params` filled from `filled`, but a value `holds` marks from `held`.
fn fill_holding(
    params: &Params,
    filled: &TemplateContext,
    held: &TemplateContext,
    holds: &dyn Fn(&Value) -> bool,
) -> Params {
    let one = |v: &Value| fill_value(v, filled, held, holds);
    params.iter().map(|(k, v)| (k.clone(), one(v))).collect()
}

fn fill_value(
    value: &Value,
    filled: &TemplateContext,
    held: &TemplateContext,
    holds: &dyn Fn(&Value) -> bool,
) -> Value {
    if holds(value) {
        return resolve_value(value, held);
    }
    match value {
        Value::Array(items) => Value::Array(
            items
                .iter()
                .map(|v| fill_value(v, filled, held, holds))
                .collect(),
        ),
        Value::Object(map) => Value::Object(fill_holding(map, filled, held, holds)),
        other => resolve_value(other, filled),
    }
}

/// `params` with every value `holds` marks left out, at any depth.
fn without_held(params: &Params, holds: &dyn Fn(&Value) -> bool) -> Params {
    params
        .iter()
        .filter(|(_, v)| !holds(v))
        .map(|(k, v)| (k.clone(), strip(v, holds)))
        .collect()
}

fn strip(value: &Value, holds: &dyn Fn(&Value) -> bool) -> Value {
    match value {
        Value::Array(items) => Value::Array(
            items
                .iter()
                .filter(|v| !holds(v))
                .map(|v| strip(v, holds))
                .collect(),
        ),
        Value::Object(map) => Value::Object(without_held(map, holds)),
        other => other.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn is_group(v: &Value) -> bool {
        v.as_object().is_some_and(|m| m.contains_key("each"))
    }

    fn params(v: Value) -> Params {
        v.as_object().cloned().unwrap()
    }

    #[test]
    fn test_without_held_leaves_out_marked_values_at_any_depth() {
        let p = params(json!({
            "args": ["{{item.a}}", { "each": ["{{item.b}}"] }, ["x", { "each": [] }]],
            "env": { "K": { "each": ["{{item.c}}"] }, "L": "{{item.d}}" }
        }));
        let out = without_held(&p, &is_group);
        let expected = json!({ "args": ["{{item.a}}", ["x"]], "env": { "L": "{{item.d}}" } });
        assert_eq!(Value::Object(out), expected);
    }

    #[test]
    fn test_held_pairs_finds_each_marked_value_with_its_filled_copy() {
        let raw = json!(["a", { "each": ["{{loop.n}}"] }, { "x": { "each": ["b"] } }]);
        let filled = json!(["A", { "each": ["{{item.s}}"] }, { "x": { "each": ["b"] } }]);
        let (mut raws, mut filleds) = (Vec::new(), Vec::new());
        held_pairs(&raw, &filled, &is_group, &mut raws, &mut filleds);
        assert_eq!(
            raws,
            [json!({ "each": ["{{loop.n}}"] }), json!({ "each": ["b"] })]
        );
        assert_eq!(
            filleds,
            [json!({ "each": ["{{item.s}}"] }), json!({ "each": ["b"] })]
        );
    }
}
