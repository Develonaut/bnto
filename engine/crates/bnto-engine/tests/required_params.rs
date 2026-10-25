// A required param with no value refuses the run before any step starts,
// naming the node and the param. Watermark Images shipped `"overlay": ""`,
// which passed every check and failed at decode on every run (ISS-25), so
// this covers every required param of every bundled recipe: as shipped when
// the recipe leaves it without a value, and emptied or left out otherwise.

use std::cell::Cell;
use std::rc::Rc;

use bnto_core::pipeline::is_io_node;
use bnto_core::{
    NoopContext, PipelineDefinition, PipelineNode, PipelineReporter, execute_pipeline,
};
use bnto_engine::create_registry;
use bnto_engine::recipes::builtin_recipes;
use serde_json::Value;

/// One required param of one processing step in a bundled recipe.
struct RequiredParam {
    recipe: String,
    node: String,
    param: String,
    /// The recipe as shipped.
    definition: PipelineDefinition,
    /// True when the recipe ships it with no value the step can use.
    shipped_blank: bool,
}

#[test]
fn a_bundled_recipe_that_ships_a_required_param_blank_is_refused_before_the_run() {
    for case in required_params().iter().filter(|c| c.shipped_blank) {
        assert_refused(case, &case.definition, "as shipped");
    }
}

#[test]
fn watermark_images_as_shipped_is_refused_naming_its_overlay() {
    let all = required_params();
    let overlay = all
        .iter()
        .find(|c| c.recipe == "watermark-images" && c.param == "overlay")
        .expect("watermark-images has a required overlay");
    assert!(
        overlay.shipped_blank,
        "watermark-images ships an overlay now"
    );
    assert_refused(overlay, &overlay.definition, "as shipped");
}

#[test]
fn every_required_param_of_every_bundled_recipe_is_refused_when_emptied_or_left_out() {
    let all = required_params();
    assert!(!all.is_empty(), "no bundled recipe has a required param");
    for case in &all {
        assert_refused(case, &with_param(case, Some(Value::from(""))), "emptied");
        if !has_default(case) {
            assert_refused(case, &with_param(case, None), "left out");
        }
    }
}

/// Run `definition` with no files and assert it's refused before any event,
/// with an error naming the case's node and param.
fn assert_refused(case: &RequiredParam, definition: &PipelineDefinition, how: &str) {
    let events = Rc::new(Cell::new(0));
    let seen = Rc::clone(&events);
    let reporter = PipelineReporter::new(move |_| seen.set(seen.get() + 1));
    let at = format!("{}/{} param {} {how}", case.recipe, case.node, case.param);
    let result = execute_pipeline(
        definition,
        vec![],
        &create_registry(),
        &reporter,
        &NoopContext,
        || 0,
    );
    let err = result
        .err()
        .unwrap_or_else(|| panic!("{at}: the run wasn't refused"));
    let expected = format!("Node '{}': param \"{}\"", case.node, case.param);
    assert!(err.to_string().contains(&expected), "{at}: {err}");
    assert_eq!(
        events.get(),
        0,
        "{at}: the run started before it was refused"
    );
}

/// Every required param of every processing step in the bundled recipes.
fn required_params() -> Vec<RequiredParam> {
    let mut out = Vec::new();
    for recipe in builtin_recipes() {
        let definition: PipelineDefinition = serde_json::from_str(recipe.definition_json).unwrap();
        let mut found = Vec::new();
        collect(&definition.nodes, &mut found);
        out.extend(
            found
                .into_iter()
                .map(|(node, param, shipped_blank)| RequiredParam {
                    recipe: recipe.slug.clone(),
                    node,
                    param,
                    definition: definition.clone(),
                    shipped_blank,
                }),
        );
    }
    out
}

/// `(node id, param, shipped blank)` for each required param under `nodes`.
fn collect(nodes: &[PipelineNode], out: &mut Vec<(String, String, bool)>) {
    let registry = create_registry();
    for node in nodes {
        collect(node.children.as_deref().unwrap_or_default(), out);
        if is_io_node(&node.node_type) {
            continue;
        }
        let Some(declared) = registry
            .resolve(&node.node_type, &node.params)
            .and_then(|p| p.declared_params())
        else {
            continue;
        };
        for def in declared.iter().filter(|d| is_required(d)) {
            let blank = match node.params.get(&def.name) {
                None => def.default.is_none(),
                Some(value) => {
                    value.is_null() || value.as_str().is_some_and(|s| s.trim().is_empty())
                }
            };
            out.push((node.id.clone(), def.name.clone(), blank));
        }
    }
}

fn is_required(def: &bnto_core::ParameterDef) -> bool {
    def.constraints.as_ref().is_some_and(|c| c.required)
}

/// Whether the case's param has a default a step falls back on when left out.
fn has_default(case: &RequiredParam) -> bool {
    let node = find(&case.definition.nodes, &case.node).unwrap();
    create_registry()
        .resolve(&node.node_type, &node.params)
        .and_then(|p| p.declared_params())
        .into_iter()
        .flatten()
        .any(|d| d.name == case.param && d.default.is_some())
}

/// The shipped recipe with the case's param set to `value`, or removed.
fn with_param(case: &RequiredParam, value: Option<Value>) -> PipelineDefinition {
    let mut definition = case.definition.clone();
    let node = find_mut(&mut definition.nodes, &case.node).unwrap();
    match value {
        Some(value) => node.params.insert(case.param.clone(), value),
        None => node.params.remove(&case.param),
    };
    definition
}

fn find<'a>(nodes: &'a [PipelineNode], id: &str) -> Option<&'a PipelineNode> {
    nodes.iter().find_map(|node| {
        (node.id == id)
            .then_some(node)
            .or_else(|| find(node.children.as_deref().unwrap_or_default(), id))
    })
}

fn find_mut<'a>(nodes: &'a mut [PipelineNode], id: &str) -> Option<&'a mut PipelineNode> {
    for node in nodes {
        if node.id == id {
            return Some(node);
        }
        if let Some(found) = node.children.as_deref_mut().and_then(|c| find_mut(c, id)) {
            return Some(found);
        }
    }
    None
}
