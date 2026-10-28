// Pre-flight: each processor checks its params as the recipe wrote them,
// before templates fill data in: no param it doesn't declare, no required
// param without a value, and nothing `NodeProcessor::validate_raw` refuses.
// Run before any step starts, so a refused recipe runs nothing. The CLI and
// TUI make the same check before consent (`raw_param_problems`, ISS-74).

use std::fmt;

use crate::errors::BntoError;
use crate::metadata::ParameterDef;
use crate::pipeline::PipelineNode;
use crate::processor::NodeProcessor;
use crate::registry::NodeRegistry;
use crate::required_params::missing_required;
use crate::undeclared_params::undeclared_params;

/// The first param a node's processor refuses as the recipe wrote it.
#[derive(Debug, Clone)]
pub struct RawParamProblem<'a> {
    pub node: &'a PipelineNode,
    /// What's wrong, without the node: `param "logo" (Logo) is required`.
    pub message: String,
    /// The param's definition, when the problem is a required param with no
    /// value, so a caller can say how to give it.
    pub missing: Option<ParameterDef>,
}

impl fmt::Display for RawParamProblem<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node '{}': {}", self.node.id, self.message)
    }
}

/// Refuse the run when any node's processor refuses its unfilled params.
pub(super) fn check_raw_params(
    nodes: &[PipelineNode],
    registry: &NodeRegistry,
) -> Result<(), BntoError> {
    match raw_param_problems(nodes, registry).into_iter().next() {
        Some(problem) => Err(BntoError::InvalidInput(problem.to_string())),
        None => Ok(()),
    }
}

/// Each node's first problem, in the order pre-flight walks them: each node,
/// then its children, before the next node.
pub fn raw_param_problems<'a>(
    nodes: &'a [PipelineNode],
    registry: &NodeRegistry,
) -> Vec<RawParamProblem<'a>> {
    let mut problems = Vec::new();
    for node in nodes {
        if let Some(processor) = registry.resolve(&node.node_type, &node.params)
            && let Some((message, missing)) = first_problem(node, processor)
        {
            problems.push(RawParamProblem {
                node,
                message,
                missing,
            });
        }
        let children = node.children.as_deref().unwrap_or_default();
        problems.extend(raw_param_problems(children, registry));
    }
    problems
}

/// The first undeclared param, else the first required param with no value
/// (with its definition), else the first raw-param refusal.
fn first_problem(
    node: &PipelineNode,
    processor: &dyn NodeProcessor,
) -> Option<(String, Option<ParameterDef>)> {
    if let Some(declared) = processor.declared_params() {
        let undeclared = undeclared_params(&node.node_type, &node.params, &declared);
        if let Some(first) = undeclared.into_iter().next() {
            return Some((first.message, None));
        }
        if let Some(first) = missing_required(&node.params, &declared).into_iter().next() {
            let def = declared.into_iter().find(|def| def.name == first.param);
            return Some((first.message, def));
        }
    }
    let refused = processor.validate_raw(&node.params).into_iter().next();
    refused.map(|message| (message, None))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::processor::{NodeInput, NodeOutput, NodeProcessor};
    use crate::progress::ProgressReporter;

    /// Declares one param, `secret`, and refuses it written with a placeholder.
    struct Picky;

    impl NodeProcessor for Picky {
        fn name(&self) -> &str {
            "picky"
        }
        fn metadata(&self) -> crate::metadata::NodeMetadata {
            crate::metadata::NodeMetadata {
                node_type: "picky".to_string(),
                name: "Picky".to_string(),
                description: String::new(),
                category: crate::metadata::NodeCategory::Data,
                accepts: vec![],
                platforms: vec![],
                parameters: vec![crate::metadata::ParameterDef {
                    name: "secret".to_string(),
                    ..Default::default()
                }],
                input_cardinality: Default::default(),
                requires: vec![],
            }
        }
        fn process(
            &self,
            _: NodeInput,
            _: &ProgressReporter,
            _: &dyn crate::ProcessContext,
        ) -> Result<NodeOutput, BntoError> {
            unreachable!("pre-flight must refuse before any step runs")
        }
        fn validate_raw(&self, raw: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
            let secret = raw
                .get("secret")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            if crate::placeholders::mentions_data(secret) {
                vec!["'secret' must be literal".into()]
            } else {
                vec![]
            }
        }
    }

    /// Declares one required param, `logo`, with no default.
    struct Needy;

    impl NodeProcessor for Needy {
        fn name(&self) -> &str {
            "needy"
        }
        fn metadata(&self) -> crate::metadata::NodeMetadata {
            crate::metadata::NodeMetadata {
                node_type: "needy".to_string(),
                parameters: vec![crate::metadata::ParameterDef {
                    name: "logo".to_string(),
                    label: "Logo".to_string(),
                    constraints: Some(crate::metadata::Constraints {
                        min: None,
                        max: None,
                        required: true,
                    }),
                    ..Default::default()
                }],
                ..Picky.metadata()
            }
        }
        fn process(
            &self,
            _: NodeInput,
            _: &ProgressReporter,
            _: &dyn crate::ProcessContext,
        ) -> Result<NodeOutput, BntoError> {
            unreachable!("pre-flight must refuse before any step runs")
        }
    }

    fn registry() -> NodeRegistry {
        let mut registry = NodeRegistry::new();
        registry.register("picky", Box::new(Picky));
        registry.register("needy", Box::new(Needy));
        registry
    }

    fn nodes(json: serde_json::Value) -> Vec<PipelineNode> {
        serde_json::from_value(json).unwrap()
    }

    #[test]
    fn literal_params_pass() {
        let ns = nodes(serde_json::json!([
            { "id": "p", "type": "picky", "parameters": { "secret": "x" } }
        ]));
        assert!(check_raw_params(&ns, &registry()).is_ok());
    }

    #[test]
    fn a_refused_raw_param_names_the_node() {
        let ns = nodes(serde_json::json!([
            { "id": "p", "type": "picky", "parameters": { "secret": "{{fields.s}}" } }
        ]));
        let err = check_raw_params(&ns, &registry()).unwrap_err().to_string();
        assert!(err.contains("Node 'p': 'secret' must be literal"), "{err}");
    }

    #[test]
    fn an_undeclared_param_refuses_the_run_naming_node_and_param() {
        let ns = nodes(serde_json::json!([
            { "id": "p", "type": "picky", "parameters": { "secret": "x", "secert": "y" } }
        ]));
        let err = check_raw_params(&ns, &registry()).unwrap_err().to_string();
        assert!(
            err.contains("Node 'p': picky has no param \"secert\""),
            "{err}"
        );
    }

    #[test]
    fn an_undeclared_param_nested_in_a_container_refuses_the_run() {
        let ns = nodes(serde_json::json!([{
            "id": "g", "type": "group",
            "children": [{ "id": "p", "type": "picky", "parameters": { "extra": 1 } }]
        }]));
        let err = check_raw_params(&ns, &registry()).unwrap_err().to_string();
        assert!(
            err.contains("Node 'p': picky has no param \"extra\""),
            "{err}"
        );
    }

    #[test]
    fn a_required_param_left_empty_refuses_the_run_naming_node_and_param() {
        let ns = nodes(serde_json::json!([
            { "id": "stamp", "type": "needy", "parameters": { "logo": "" } }
        ]));
        let err = check_raw_params(&ns, &registry()).unwrap_err().to_string();
        assert!(
            err.contains("Node 'stamp': param \"logo\" (Logo) is required"),
            "{err}"
        );
    }

    #[test]
    fn a_required_param_left_out_refuses_the_run() {
        let ns = nodes(serde_json::json!([{
            "id": "g", "type": "group",
            "children": [{ "id": "stamp", "type": "needy", "parameters": {} }]
        }]));
        let err = check_raw_params(&ns, &registry()).unwrap_err().to_string();
        assert!(err.contains("Node 'stamp': param \"logo\""), "{err}");
    }

    #[test]
    fn a_required_param_with_a_value_passes() {
        let ns = nodes(serde_json::json!([
            { "id": "stamp", "type": "needy", "parameters": { "logo": "{{fields.logo}}" } }
        ]));
        assert!(check_raw_params(&ns, &registry()).is_ok());
    }

    #[test]
    fn a_guard_that_wont_hold_doesnt_hide_a_refused_step() {
        // Checked before any guard is decided: a branch for another OS
        // can't carry a bad step past pre-flight.
        let ns = nodes(serde_json::json!([{
            "id": "g", "type": "group",
            "when": { "value": "a", "op": "equals", "expected": "b" },
            "children": [{ "id": "p", "type": "picky", "parameters": { "secret": "{{fields.s}}" } }]
        }]));
        let err = check_raw_params(&ns, &registry()).unwrap_err().to_string();
        assert!(err.contains("Node 'p'"), "{err}");
    }

    // --- Problems as data, for checks made before a run starts (ISS-74) ---

    #[test]
    fn a_required_param_problem_carries_its_definition() {
        let ns = nodes(serde_json::json!([
            { "id": "stamp", "type": "needy", "parameters": { "logo": "" } }
        ]));
        let problems = raw_param_problems(&ns, &registry());
        assert_eq!(problems.len(), 1);
        assert_eq!(problems[0].node.id, "stamp");
        assert_eq!(problems[0].message, "param \"logo\" (Logo) is required");
        let missing = problems[0].missing.as_ref().expect("a required param");
        assert_eq!(missing.name, "logo");
        assert_eq!(
            problems[0].to_string(),
            "Node 'stamp': param \"logo\" (Logo) is required"
        );
    }

    #[test]
    fn other_problems_carry_no_definition() {
        let ns = nodes(serde_json::json!([
            { "id": "p", "type": "picky", "parameters": { "secret": "{{fields.s}}" } }
        ]));
        let problems = raw_param_problems(&ns, &registry());
        assert_eq!(problems.len(), 1);
        assert!(problems[0].missing.is_none());
    }

    #[test]
    fn every_node_with_a_problem_is_listed_in_walk_order() {
        let ns = nodes(serde_json::json!([
            { "id": "a", "type": "needy", "parameters": {} },
            { "id": "g", "type": "group", "children": [
                { "id": "b", "type": "picky", "parameters": { "extra": 1 } }
            ] },
            { "id": "c", "type": "needy", "parameters": { "logo": "x" } }
        ]));
        let ids: Vec<String> = raw_param_problems(&ns, &registry())
            .into_iter()
            .map(|p| p.node.id.clone())
            .collect();
        assert_eq!(ids, ["a", "b"]);
    }

    #[test]
    fn nested_nodes_and_unknown_types_are_walked() {
        let ns = nodes(serde_json::json!([{
            "id": "g", "type": "group",
            "children": [{ "id": "p", "type": "picky", "parameters": { "secret": "{{item.k}}" } }]
        }]));
        assert!(check_raw_params(&ns, &registry()).is_err());
    }
}
