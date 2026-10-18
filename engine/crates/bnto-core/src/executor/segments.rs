// Segments — a level graph's execution order cut into linear runs.
//
// A segment is a maximal run of nodes in which each node after the first
// takes the previous node's output and nothing else, and that output feeds
// nothing else. Each segment runs as a node chain (or, with `iteration:
// auto`, through auto-iteration), exactly as a level without edges does;
// fan-in and fan-out points start a new segment, like batch nodes do.

use std::collections::HashMap;
use std::ops::Range;

use crate::graph::{LevelGraph, Source};
use crate::pipeline::PipelineNode;

use super::routed::{SourceKey, consumer_counts};

/// The graph's segments, as ranges of its execution order.
pub(super) fn segments(graph: &LevelGraph<'_>) -> Vec<Range<usize>> {
    let counts = consumer_counts(graph);
    let mut segments: Vec<Range<usize>> = Vec::new();
    for (i, node) in graph.order.iter().enumerate() {
        match segments.last_mut() {
            Some(segment) if continues(graph, &counts, graph.order[i - 1], node) => {
                segment.end = i + 1;
            }
            _ => segments.push(i..i + 1),
        }
    }
    segments
}

/// True when `next` takes only `prev`'s output, and `prev` feeds only `next`.
fn continues(
    graph: &LevelGraph<'_>,
    counts: &HashMap<SourceKey<'_>, usize>,
    prev: &PipelineNode,
    next: &PipelineNode,
) -> bool {
    let only_prev = graph.inputs[next.id.as_str()] == [Source::Node(&prev.id)];
    only_prev && counts.get(&Some(prev.id.as_str())) == Some(&1)
}

/// The graph's nodes when its edges are today's chain: one segment, fed the
/// level's files, whose last node alone is the level's output. Such a level
/// runs exactly as it would with no edges.
pub(super) fn as_chain<'g, 'a>(graph: &'g LevelGraph<'a>) -> Option<&'g [&'a PipelineNode]> {
    let order = graph.order.as_slice();
    let (Some(first), Some(last)) = (order.first(), order.last()) else {
        return (graph.outputs == [Source::LevelInput]).then_some(order);
    };
    let fed = graph.inputs[first.id.as_str()] == [Source::LevelInput];
    let output = graph.outputs == [Source::Node(&last.id)];
    (fed && output && segments(graph).len() == 1).then_some(order)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{Flow, level_flow};
    use crate::pipeline::PipelineDefinition;

    fn definition(json: &str) -> PipelineDefinition {
        serde_json::from_str(json).unwrap()
    }

    fn with_graph<R>(json: &str, check: impl Fn(&LevelGraph<'_>) -> R) -> R {
        let def = definition(json);
        let Ok(Flow::Graph(graph)) = level_flow(&def.nodes, &def.edges) else {
            panic!("expected a graph");
        };
        check(&graph)
    }

    const CHAIN: &str = r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "a", "type": "x" }, { "id": "b", "type": "x" },
            { "id": "output", "type": "output" }
        ],
        "edges": [
            { "id": "e1", "source": "input", "target": "a" },
            { "id": "e2", "source": "a", "target": "b" },
            { "id": "e3", "source": "b", "target": "output" }
        ]
    }"#;

    const FAN_OUT: &str = r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "a", "type": "x" }, { "id": "b", "type": "x" },
            { "id": "c", "type": "x" }, { "id": "d", "type": "x" },
            { "id": "output", "type": "output" }
        ],
        "edges": [
            { "id": "e1", "source": "input", "target": "a" },
            { "id": "e2", "source": "a", "target": "b" },
            { "id": "e3", "source": "a", "target": "c" },
            { "id": "e4", "source": "c", "target": "d" },
            { "id": "e5", "source": "b", "target": "output" },
            { "id": "e6", "source": "d", "target": "output" }
        ]
    }"#;

    #[test]
    fn test_a_chain_is_one_segment_and_runs_as_today() {
        with_graph(CHAIN, |g| {
            assert_eq!(segments(g), vec![0..2]);
            assert_eq!(as_chain(g).map(<[_]>::len), Some(2));
        });
    }

    #[test]
    fn test_fan_out_ends_a_segment_and_a_linear_tail_joins_up() {
        with_graph(FAN_OUT, |g| {
            // Order a, b, c, d: a fans out; c → d is linear.
            assert_eq!(segments(g), vec![0..1, 1..2, 2..4]);
            assert!(as_chain(g).is_none());
        });
    }

    #[test]
    fn test_a_chain_that_skips_input_is_not_todays_chain() {
        let json = CHAIN.replace(
            r#""source": "input", "target": "a""#,
            r#""source": "a", "target": "output""#,
        );
        with_graph(&json, |g| assert!(as_chain(g).is_none()));
    }

    #[test]
    fn test_input_straight_to_output_runs_as_an_empty_chain() {
        let json = r#"{
            "nodes": [ { "id": "input", "type": "input" }, { "id": "output", "type": "output" } ],
            "edges": [ { "id": "e1", "source": "input", "target": "output" } ]
        }"#;
        with_graph(json, |g| assert_eq!(as_chain(g).map(<[_]>::len), Some(0)));
    }
}
