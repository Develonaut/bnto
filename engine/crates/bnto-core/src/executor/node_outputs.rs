// Node output records — what `{{node.<id>.<key>}}` reads.
//
// Once a processing node runs, its node-level `NodeOutput.metadata` is recorded
// under its id. Records stack in layers: a node chain owns a layer for what its
// nodes record, over the layer it was given, so each later node sees every
// earlier record without the run copying them into every chain or iteration.

use std::collections::BTreeMap;

use serde_json::{Map, Value};

/// Node id → that node's output metadata (a JSON object).
pub type NodeRecords = BTreeMap<String, Value>;

/// Looks a node's output record up by node id, for `{{node.<id>.*}}`.
pub trait NodeOutputSource {
    /// The record for `node_id`, if that node has run and recorded one.
    fn node_output(&self, node_id: &str) -> Option<&Value>;
}

impl NodeOutputSource for NodeRecords {
    fn node_output(&self, node_id: &str) -> Option<&Value> {
        self.get(node_id)
    }
}

/// One layer of records over the layer it inherits.
///
/// A node id lives in one layer only: a chain's nodes record into the chain's
/// own layer, so a lookup takes the first layer that has the id.
pub(super) struct NodeOutputs<'a> {
    inherited: Option<&'a NodeOutputs<'a>>,
    own: NodeRecords,
}

impl NodeOutputs<'_> {
    /// The bottom layer, holding the records seeded before the run.
    pub(super) fn root(records: NodeRecords) -> Self {
        NodeOutputs {
            inherited: None,
            own: records,
        }
    }

    /// A new, empty layer over this one.
    pub(super) fn layer(&self) -> NodeOutputs<'_> {
        NodeOutputs {
            inherited: Some(self),
            own: NodeRecords::new(),
        }
    }

    /// Merge `records` into this layer (see [`merge_records`]).
    pub(super) fn add(&mut self, records: NodeRecords) {
        merge_records(&mut self.own, records);
    }

    /// The records made in this layer, without the ones it inherited.
    pub(super) fn into_own(self) -> NodeRecords {
        self.own
    }
}

impl NodeOutputSource for NodeOutputs<'_> {
    fn node_output(&self, node_id: &str) -> Option<&Value> {
        match self.own.get(node_id) {
            Some(record) => Some(record),
            None => self.inherited?.node_output(node_id),
        }
    }
}

/// Records holding one node's metadata under its id.
pub(super) fn single(node_id: &str, metadata: Map<String, Value>) -> NodeRecords {
    NodeRecords::from([(node_id.to_string(), Value::Object(metadata))])
}

/// Merge `from` into `into`, key by key within each node's record.
///
/// When the same node records twice (a per-file node's later file, a later
/// loop iteration), a key it sets again takes the new value and a key only
/// the earlier run set is kept.
pub(super) fn merge_records(into: &mut NodeRecords, from: NodeRecords) {
    for (node_id, record) in from {
        match (into.get_mut(&node_id), record) {
            (Some(Value::Object(existing)), Value::Object(new)) => existing.extend(new),
            (_, record) => {
                into.insert(node_id, record);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn records(value: Value) -> NodeRecords {
        serde_json::from_value(value).unwrap()
    }

    #[test]
    fn test_merge_keeps_earlier_keys_and_overwrites_shared_ones() {
        let mut into = records(json!({ "pf": { "last": "a", "seen_a": true } }));
        merge_records(&mut into, records(json!({ "pf": { "last": "b" } })));
        assert_eq!(into["pf"], json!({ "last": "b", "seen_a": true }));
    }

    #[test]
    fn test_merge_adds_new_nodes() {
        let mut into = records(json!({ "a": { "x": 1 } }));
        merge_records(&mut into, records(json!({ "b": { "y": 2 } })));
        assert_eq!(into.len(), 2);
    }

    #[test]
    fn test_layer_reads_through_to_what_it_inherits() {
        let root = NodeOutputs::root(records(json!({ "in": { "filename": "a" } })));
        let mut layer = root.layer();
        layer.add(single(
            "count",
            Map::from_iter([("n".to_string(), json!(2))]),
        ));
        assert_eq!(layer.node_output("in"), Some(&json!({ "filename": "a" })));
        assert_eq!(layer.node_output("count"), Some(&json!({ "n": 2 })));
        assert_eq!(root.node_output("count"), None);
    }

    #[test]
    fn test_into_own_leaves_inherited_records_out() {
        let root = NodeOutputs::root(records(json!({ "in": {} })));
        let mut layer = root.layer();
        layer.add(single("count", Map::new()));
        assert_eq!(layer.into_own().keys().collect::<Vec<_>>(), vec!["count"]);
    }
}
