// Pre-flight: each processor checks its params as the recipe wrote them,
// before templates fill data in: no param it doesn't declare, and nothing
// `NodeProcessor::validate_raw` refuses. Run before any step starts, so a
// refused recipe runs nothing.

use crate::errors::BntoError;
use crate::pipeline::PipelineNode;
use crate::processor::NodeProcessor;
use crate::registry::NodeRegistry;
use crate::undeclared_params::undeclared_params;

/// Refuse the run when any node's processor refuses its unfilled params.
pub(super) fn check_raw_params(
    nodes: &[PipelineNode],
    registry: &NodeRegistry,
) -> Result<(), BntoError> {
    for node in nodes {
        if let Some(processor) = registry.resolve(&node.node_type, &node.params)
            && let Some(problem) = first_problem(node, processor)
        {
            return Err(BntoError::InvalidInput(format!(
                "Node '{}': {problem}",
                node.id
            )));
        }
        if let Some(children) = &node.children {
            check_raw_params(children, registry)?;
        }
    }
    Ok(())
}

/// The first undeclared param, else the first raw-param refusal.
fn first_problem(node: &PipelineNode, processor: &dyn NodeProcessor) -> Option<String> {
    let undeclared = processor.declared_params().and_then(|declared| {
        undeclared_params(&node.node_type, &node.params, &declared)
            .into_iter()
            .next()
    });
    undeclared
        .map(|u| u.message)
        .or_else(|| processor.validate_raw(&node.params).into_iter().next())
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

    fn registry() -> NodeRegistry {
        let mut registry = NodeRegistry::new();
        registry.register("picky", Box::new(Picky));
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
    fn nested_nodes_and_unknown_types_are_walked() {
        let ns = nodes(serde_json::json!([{
            "id": "g", "type": "group",
            "children": [{ "id": "p", "type": "picky", "parameters": { "secret": "{{item.k}}" } }]
        }]));
        assert!(check_raw_params(&ns, &registry()).is_err());
    }
}
