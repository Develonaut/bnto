// Routed files — each source's files, held until its consumers take them.
//
// A source (the level's input, or a node's output) can feed several
// consumers: nodes at the level and the level's output. Every consumer but
// the last, in execution order, gets a clone; the last takes the files, so a
// linear level never clones. A `FileData::Path` clone copies only the path,
// so consumers share the file on disk.

use std::collections::HashMap;

use crate::graph::{LevelGraph, Source};
use crate::pipeline::PipelineFile;

/// A source as a map key: `None` is the level's input, `Some(id)` a node.
pub(super) type SourceKey<'a> = Option<&'a str>;

/// The key a source is held under.
pub(super) fn key(source: Source<'_>) -> SourceKey<'_> {
    match source {
        Source::LevelInput => None,
        Source::Node(id) => Some(id),
    }
}

/// How many times each source is read: by the nodes it feeds and by the
/// level's output.
pub(super) fn consumer_counts<'a>(graph: &LevelGraph<'a>) -> HashMap<SourceKey<'a>, usize> {
    let mut counts = HashMap::new();
    for source in graph.inputs.values().flatten().chain(&graph.outputs) {
        *counts.entry(key(*source)).or_insert(0) += 1;
    }
    counts
}

/// The files of every source that still has a consumer to come.
pub(super) struct Routed<'a> {
    counts: HashMap<SourceKey<'a>, usize>,
    held: HashMap<SourceKey<'a>, Held>,
}

/// One source's files and how many consumers are left to take them.
struct Held {
    files: Vec<PipelineFile>,
    left: usize,
}

impl<'a> Routed<'a> {
    /// Routing for `graph`, holding the level's input `files`.
    pub(super) fn new(graph: &LevelGraph<'a>, files: Vec<PipelineFile>) -> Self {
        let mut routed = Self {
            counts: consumer_counts(graph),
            held: HashMap::new(),
        };
        routed.hold(None, files);
        routed
    }

    /// Hold a node's output for its consumers. With none, it's dropped.
    pub(super) fn hand_over(&mut self, node_id: &'a str, files: Vec<PipelineFile>) {
        self.hold(Some(node_id), files);
    }

    fn hold(&mut self, key: SourceKey<'a>, files: Vec<PipelineFile>) {
        let left = self.counts.get(&key).copied().unwrap_or(0);
        if left > 0 {
            self.held.insert(key, Held { files, left });
        }
    }

    /// The files of `sources`, joined in the order given (edge order).
    pub(super) fn take(&mut self, sources: &[Source<'a>]) -> Vec<PipelineFile> {
        let mut files = Vec::new();
        for source in sources {
            files.extend(self.take_one(key(*source)));
        }
        files
    }

    /// One consumer's share: a clone, or the files themselves for the last.
    fn take_one(&mut self, key: SourceKey<'a>) -> Vec<PipelineFile> {
        let Some(held) = self.held.get_mut(&key) else {
            return Vec::new();
        };
        held.left -= 1;
        if held.left > 0 {
            return held.files.clone();
        }
        self.held.remove(&key).map(|h| h.files).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::processor::FileData;

    fn file(name: &str) -> PipelineFile {
        PipelineFile {
            name: name.to_string(),
            data: FileData::Bytes(name.as_bytes().to_vec()),
            mime_type: "text/plain".to_string(),
            metadata: serde_json::Map::new(),
        }
    }

    /// A level graph with only the routing parts set.
    fn graph<'a>(
        inputs: &[(&'a str, Vec<Source<'a>>)],
        outputs: Vec<Source<'a>>,
    ) -> LevelGraph<'a> {
        LevelGraph {
            order: Vec::new(),
            inputs: inputs.iter().cloned().collect::<BTreeMap<_, _>>(),
            outputs,
        }
    }

    #[test]
    fn test_every_consumer_gets_the_files_and_the_last_empties_the_slot() {
        let g = graph(&[("a", vec![Source::LevelInput])], vec![Source::LevelInput]);
        let mut routed = Routed::new(&g, vec![file("x")]);
        assert_eq!(routed.take(&[Source::LevelInput])[0].name, "x");
        assert_eq!(routed.take(&[Source::LevelInput])[0].name, "x");
        assert!(routed.held.is_empty());
    }

    #[test]
    fn test_fan_in_joins_sources_in_the_order_given() {
        let sources = vec![Source::Node("b"), Source::Node("a")];
        let g = graph(&[("j", sources.clone())], vec![Source::Node("j")]);
        let mut routed = Routed::new(&g, vec![]);
        routed.hand_over("a", vec![file("from-a")]);
        routed.hand_over("b", vec![file("from-b")]);
        let names: Vec<String> = routed.take(&sources).into_iter().map(|f| f.name).collect();
        assert_eq!(names, vec!["from-b", "from-a"]);
    }

    #[test]
    fn test_a_source_nobody_reads_is_dropped() {
        let g = graph(&[("a", vec![])], vec![Source::Node("a")]);
        let mut routed = Routed::new(&g, vec![file("unread")]);
        routed.hand_over("lost", vec![file("lost")]);
        assert!(routed.held.is_empty());
    }
}
