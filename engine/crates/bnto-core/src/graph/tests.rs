use super::*;
use crate::pipeline::PipelineDefinition;

/// Parse one level: `nodes` as `[id, type]` pairs, `edges` as
/// `[id, source, target]` triples.
fn level(nodes: &[(&str, &str)], edges: &[(&str, &str, &str)]) -> PipelineDefinition {
    let nodes: Vec<_> = nodes
        .iter()
        .map(|(id, t)| serde_json::json!({ "id": id, "type": t }))
        .collect();
    let edges: Vec<_> = edges
        .iter()
        .map(|(id, s, t)| serde_json::json!({ "id": id, "source": s, "target": t }))
        .collect();
    serde_json::from_value(serde_json::json!({ "nodes": nodes, "edges": edges })).unwrap()
}

fn ids<'a>(nodes: &[&'a PipelineNode]) -> Vec<&'a str> {
    nodes.iter().map(|n| n.id.as_str()).collect()
}

fn graph(def: &PipelineDefinition) -> LevelGraph<'_> {
    match level_flow(&def.nodes, &def.edges) {
        Ok(Flow::Graph(graph)) => graph,
        other => panic!("expected a graph, got {other:?}"),
    }
}

fn error(def: &PipelineDefinition) -> GraphError {
    level_flow(&def.nodes, &def.edges).expect_err("expected a graph error")
}

const IO: [(&str, &str); 2] = [("input", "input"), ("output", "output")];

fn with_io<'a>(middle: &[(&'a str, &'a str)]) -> Vec<(&'a str, &'a str)> {
    let mut nodes = vec![IO[0]];
    nodes.extend_from_slice(middle);
    nodes.push(IO[1]);
    nodes
}

// --- Linear fallback ---

#[test]
fn missing_edges_give_a_sequence_in_node_order_without_io_nodes() {
    let def: PipelineDefinition = serde_json::from_str(
        r#"{ "nodes": [ { "id": "input", "type": "input" }, { "id": "b", "type": "x" },
                        { "id": "a", "type": "x" }, { "id": "output", "type": "output" } ] }"#,
    )
    .unwrap();
    match level_flow(&def.nodes, &def.edges) {
        Ok(Flow::Sequence(nodes)) => assert_eq!(ids(&nodes), ["b", "a"]),
        other => panic!("expected a sequence, got {other:?}"),
    }
}

#[test]
fn empty_edges_give_a_sequence() {
    let def = level(&with_io(&[("a", "x")]), &[]);
    assert!(
        matches!(level_flow(&def.nodes, &def.edges), Ok(Flow::Sequence(n)) if ids(&n) == ["a"])
    );
}

// --- Order ---

#[test]
fn edges_set_the_order_over_node_order() {
    let def = level(
        &with_io(&[("b", "x"), ("a", "x")]),
        &[
            ("e1", "input", "a"),
            ("e2", "a", "b"),
            ("e3", "b", "output"),
        ],
    );
    assert_eq!(ids(&graph(&def).order), ["a", "b"]);
}

#[test]
fn ready_nodes_are_taken_in_node_order_not_edge_order() {
    let def = level(
        &with_io(&[("c", "x"), ("b", "x"), ("a", "x")]),
        &[
            ("e1", "input", "a"),
            ("e2", "a", "b"),
            ("e3", "a", "c"),
            ("e4", "b", "output"),
            ("e5", "c", "output"),
        ],
    );
    assert_eq!(ids(&graph(&def).order), ["a", "c", "b"]);
}

// --- Inputs and outputs ---

#[test]
fn input_and_output_edges_map_to_the_level_input_and_outputs() {
    let def = level(
        &with_io(&[("a", "x")]),
        &[("e1", "input", "a"), ("e2", "a", "output")],
    );
    let graph = graph(&def);
    assert_eq!(graph.inputs["a"], [Source::LevelInput]);
    assert_eq!(graph.outputs, [Source::Node("a")]);
}

#[test]
fn a_node_with_no_incoming_edge_starts_empty_at_a_level_with_input() {
    let def = level(
        &with_io(&[("a", "x"), ("b", "x")]),
        &[
            ("e1", "input", "a"),
            ("e2", "a", "output"),
            ("e3", "b", "output"),
        ],
    );
    let graph = graph(&def);
    assert!(graph.inputs["b"].is_empty());
    assert_eq!(graph.outputs, [Source::Node("a"), Source::Node("b")]);
}

#[test]
fn an_unwired_input_node_is_allowed() {
    let def = level(
        &[("input", "input"), ("a", "x"), ("b", "x")],
        &[("e1", "a", "b")],
    );
    let graph = graph(&def);
    assert!(graph.inputs["a"].is_empty());
    assert_eq!(graph.outputs, [Source::Node("b")]);
}

#[test]
fn without_an_input_node_the_first_root_in_node_order_gets_the_level_input() {
    let def = level(
        &[("c", "x"), ("a", "x"), ("b", "x")],
        &[("e1", "a", "c"), ("e2", "b", "c")],
    );
    let graph = graph(&def);
    assert_eq!(graph.inputs["a"], [Source::LevelInput]);
    assert!(graph.inputs["b"].is_empty());
    assert_eq!(graph.inputs["c"], [Source::Node("a"), Source::Node("b")]);
}

#[test]
fn fan_in_follows_edge_order_not_node_order() {
    let def = level(
        &with_io(&[("a", "x"), ("b", "x"), ("join", "x")]),
        &[
            ("e1", "input", "a"),
            ("e2", "input", "b"),
            ("e3", "b", "join"),
            ("e4", "a", "join"),
            ("e5", "join", "output"),
        ],
    );
    assert_eq!(
        graph(&def).inputs["join"],
        [Source::Node("b"), Source::Node("a")]
    );
}

#[test]
fn outputs_follow_the_order_of_edges_into_output() {
    let def = level(
        &with_io(&[("a", "x"), ("b", "x")]),
        &[
            ("e1", "input", "a"),
            ("e2", "input", "b"),
            ("e3", "b", "output"),
            ("e4", "a", "output"),
            ("e5", "input", "output"),
        ],
    );
    let expected = [Source::Node("b"), Source::Node("a"), Source::LevelInput];
    assert_eq!(graph(&def).outputs, expected);
}

#[test]
fn without_an_output_node_the_sinks_are_the_outputs_in_node_order() {
    let def = level(
        &[("c", "x"), ("a", "x"), ("b", "x")],
        &[("e1", "a", "b"), ("e2", "a", "c")],
    );
    let graph = graph(&def);
    assert_eq!(ids(&graph.order), ["a", "c", "b"]);
    assert_eq!(graph.outputs, [Source::Node("c"), Source::Node("b")]);
}

// --- Errors ---

#[test]
fn a_cycle_is_refused_and_named() {
    let def = level(
        &with_io(&[("a", "x"), ("b", "x")]),
        &[
            ("e1", "input", "a"),
            ("e2", "a", "b"),
            ("e3", "b", "a"),
            ("e4", "b", "output"),
        ],
    );
    let err = error(&def);
    assert_eq!(
        err,
        GraphError::Cycle {
            path: vec!["a".into(), "b".into(), "a".into()]
        }
    );
    assert_eq!(err.to_string(), "edges form a cycle: a → b → a");
}

#[test]
fn a_longer_cycle_is_named_in_edge_direction() {
    let def = level(
        &[("a", "x"), ("b", "x"), ("c", "x")],
        &[("e1", "a", "b"), ("e2", "b", "c"), ("e3", "c", "a")],
    );
    assert_eq!(error(&def).to_string(), "edges form a cycle: a → b → c → a");
}

#[test]
fn a_self_loop_is_a_cycle() {
    let def = level(
        &with_io(&[("a", "x")]),
        &[
            ("e1", "input", "a"),
            ("e2", "a", "a"),
            ("e3", "a", "output"),
        ],
    );
    assert_eq!(
        error(&def),
        GraphError::Cycle {
            path: vec!["a".into(), "a".into()]
        }
    );
}

#[test]
fn an_edge_to_an_unknown_node_is_refused() {
    let def = level(
        &with_io(&[("a", "x")]),
        &[("e1", "input", "a"), ("e2", "a", "zz")],
    );
    let err = error(&def);
    assert_eq!(
        err,
        GraphError::UnknownNode {
            edge: "e2".into(),
            id: "zz".into()
        }
    );
    assert_eq!(
        err.to_string(),
        "edge \"e2\" connects to \"zz\", which is not a node at this level"
    );
}

#[test]
fn an_edge_from_an_unknown_node_is_refused() {
    let def = level(&with_io(&[("a", "x")]), &[("e1", "ghost", "a")]);
    let expected = GraphError::UnknownNode {
        edge: "e1".into(),
        id: "ghost".into(),
    };
    assert_eq!(error(&def), expected);
}

#[test]
fn an_edge_into_another_containers_child_is_refused() {
    let def: PipelineDefinition = serde_json::from_str(
        r#"{ "nodes": [
                { "id": "input", "type": "input" },
                { "id": "g", "type": "group", "nodes": [ { "id": "c", "type": "x" } ] },
                { "id": "output", "type": "output" } ],
             "edges": [ { "id": "e1", "source": "input", "target": "c" } ] }"#,
    )
    .unwrap();
    let expected = GraphError::UnknownNode {
        edge: "e1".into(),
        id: "c".into(),
    };
    assert_eq!(error(&def), expected);
}

#[test]
fn an_edge_into_input_is_refused() {
    let def = level(&with_io(&[("a", "x")]), &[("e1", "a", "input")]);
    let err = error(&def);
    assert_eq!(err, GraphError::IntoInput { edge: "e1".into() });
    assert_eq!(err.to_string(), "edge \"e1\" leads into an input node");
}

#[test]
fn an_edge_out_of_output_is_refused() {
    let def = level(&with_io(&[("a", "x")]), &[("e1", "output", "a")]);
    let err = error(&def);
    assert_eq!(err, GraphError::FromOutput { edge: "e1".into() });
    assert_eq!(err.to_string(), "edge \"e1\" leads out of an output node");
}

#[test]
fn a_duplicate_edge_is_refused() {
    let def = level(
        &with_io(&[("a", "x")]),
        &[
            ("e1", "input", "a"),
            ("e2", "a", "output"),
            ("e3", "input", "a"),
        ],
    );
    let err = error(&def);
    assert_eq!(err, GraphError::DuplicateEdge { edge: "e3".into() });
    assert_eq!(
        err.to_string(),
        "edge \"e3\" repeats an earlier edge between the same two nodes"
    );
}

#[test]
fn a_processing_node_touching_no_edge_is_refused() {
    let def = level(
        &with_io(&[("a", "x"), ("b", "x")]),
        &[("e1", "input", "a"), ("e2", "a", "output")],
    );
    let err = error(&def);
    assert_eq!(err, GraphError::Disconnected { node: "b".into() });
    assert_eq!(err.to_string(), "node \"b\" is not connected to any edge");
}

#[test]
fn an_output_node_touching_no_edge_is_refused() {
    let def = level(&with_io(&[("a", "x"), ("b", "x")]), &[("e1", "a", "b")]);
    assert_eq!(
        error(&def),
        GraphError::Disconnected {
            node: "output".into()
        }
    );
}
