// Document-shape types for `.bnto.json` files -- the authoring view.
//
// These mirror the TypeScript `Definition` types in
// `packages/@bnto/nodes/src/definition.ts`. They represent the full document
// users author and the editor displays: position, metadata, ports, edges.
//
// The execution-pruned view (what the pipeline executor receives) lives in
// `pipeline.rs` (`PipelineDefinition` / `PipelineNode`). That view strips
// presentation fields and flattens `nodes` -> `children` for runtime walks.
//
// JSON conventions preserved from the TS source of truth:
// - camelCase keys
// - Optional fields omitted when `None` (never emitted as `null`)
// - Empty port arrays emitted (not omitted) so the shape is regular
// - `nodes` and `edges` omitted on leaves, present on containers

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::pipeline::PipelineSettings;

/// Full document shape for a node in a `.bnto.json` file.
///
/// This is the authoring view -- richer than `PipelineNode` (which is the
/// execution-pruned view). The `settings` field is only meaningful at the
/// root of a recipe document; nested nodes typically omit it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Definition {
    pub id: String,
    #[serde(rename = "type")]
    pub node_type: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub name: String,
    pub position: Position,
    pub metadata: Metadata,
    #[serde(default = "default_parameters")]
    pub parameters: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<FieldsConfig>,
    pub input_ports: Vec<Port>,
    pub output_ports: Vec<Port>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<Definition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<Edge>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<PipelineSettings>,
}

/// 2D coordinate for a node on the editor canvas.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

/// Authoring metadata attached to every node (description, timestamps, tags).
/// All fields are optional; an empty `Metadata` serializes to `{}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<BTreeMap<String, String>>,
}

/// An input or output port on a node. `handle` is used when a node exposes
/// multiple logical slots (e.g. an `if` node with `then` / `else` outputs).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Port {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
}

/// A directed connection between two nodes in a container.
/// `source_handle` / `target_handle` disambiguate when ports have handles.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Edge {
    pub id: String,
    pub source: String,
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_handle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_handle: Option<String>,
}

/// Structured-data field overrides, used by nodes that operate on
/// record-style inputs (e.g. spreadsheets). `values` is arbitrary JSON so the
/// schema can stay permissive while individual node types enforce their own.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FieldsConfig {
    pub values: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_only_set: Option<bool>,
}

/// Default for `parameters` when the key is missing entirely. Produces `{}`,
/// matching the TS convention that parameters is always an object.
fn default_parameters() -> serde_json::Value {
    serde_json::Value::Object(serde_json::Map::new())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn minimal_leaf(id: &str, node_type: &str) -> Definition {
        Definition {
            id: id.to_string(),
            node_type: node_type.to_string(),
            version: "1.0.0".to_string(),
            parent_id: None,
            name: "Test".to_string(),
            position: Position { x: 0.0, y: 0.0 },
            metadata: Metadata::default(),
            parameters: default_parameters(),
            fields: None,
            input_ports: vec![],
            output_ports: vec![],
            nodes: None,
            edges: None,
            settings: None,
        }
    }

    // --- Shape tests: verify JSON key casing and None omission ---

    #[test]
    fn serializes_keys_in_camel_case() {
        let def = Definition {
            parent_id: Some("parent".to_string()),
            ..minimal_leaf("n1", "image-compress")
        };
        let json = serde_json::to_value(&def).unwrap();
        assert!(json.get("parentId").is_some(), "parent_id -> parentId");
        assert!(
            json.get("inputPorts").is_some(),
            "input_ports -> inputPorts"
        );
        assert!(
            json.get("outputPorts").is_some(),
            "output_ports -> outputPorts"
        );
        assert!(json.get("type").is_some(), "node_type -> type");
    }

    #[test]
    fn omits_none_fields_on_serialization() {
        let def = minimal_leaf("n1", "image-compress");
        let json = serde_json::to_value(&def).unwrap();
        assert!(json.get("parentId").is_none());
        assert!(json.get("fields").is_none());
        assert!(json.get("nodes").is_none());
        assert!(json.get("edges").is_none());
        assert!(json.get("settings").is_none());
    }

    #[test]
    fn emits_empty_port_arrays_rather_than_omitting() {
        let def = minimal_leaf("n1", "image-compress");
        let json = serde_json::to_value(&def).unwrap();
        assert_eq!(json["inputPorts"], json!([]));
        assert_eq!(json["outputPorts"], json!([]));
    }

    #[test]
    fn empty_metadata_serializes_to_empty_object() {
        let meta = Metadata::default();
        let json = serde_json::to_value(&meta).unwrap();
        assert_eq!(json, json!({}));
    }

    #[test]
    fn metadata_omits_none_fields() {
        let meta = Metadata {
            description: Some("hello".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_value(&meta).unwrap();
        assert_eq!(json, json!({ "description": "hello" }));
    }

    #[test]
    fn metadata_custom_data_serializes_as_nested_object() {
        let mut custom = BTreeMap::new();
        custom.insert("author".to_string(), "ryan".to_string());
        let meta = Metadata {
            custom_data: Some(custom),
            ..Default::default()
        };
        let json = serde_json::to_value(&meta).unwrap();
        assert_eq!(json, json!({ "customData": { "author": "ryan" } }));
    }

    #[test]
    fn position_round_trips() {
        let pos = Position { x: 12.5, y: -4.0 };
        let json = serde_json::to_string(&pos).unwrap();
        let back: Position = serde_json::from_str(&json).unwrap();
        assert_eq!(pos, back);
    }

    #[test]
    fn edge_with_handles_uses_camel_case() {
        let edge = Edge {
            id: "e1".to_string(),
            source: "a".to_string(),
            target: "b".to_string(),
            source_handle: Some("out".to_string()),
            target_handle: Some("in".to_string()),
        };
        let json = serde_json::to_value(&edge).unwrap();
        assert_eq!(
            json,
            json!({
                "id": "e1",
                "source": "a",
                "target": "b",
                "sourceHandle": "out",
                "targetHandle": "in",
            })
        );
    }

    #[test]
    fn edge_without_handles_omits_them() {
        let edge = Edge {
            id: "e1".to_string(),
            source: "a".to_string(),
            target: "b".to_string(),
            source_handle: None,
            target_handle: None,
        };
        let json = serde_json::to_value(&edge).unwrap();
        assert!(json.get("sourceHandle").is_none());
        assert!(json.get("targetHandle").is_none());
    }

    #[test]
    fn port_with_handle_round_trips() {
        let port = Port {
            id: "p1".to_string(),
            name: "data".to_string(),
            handle: Some("then".to_string()),
        };
        let json = serde_json::to_string(&port).unwrap();
        let back: Port = serde_json::from_str(&json).unwrap();
        assert_eq!(port, back);
    }

    #[test]
    fn fields_config_serializes_with_camel_case() {
        let fc = FieldsConfig {
            values: json!({ "name": "test" }),
            keep_only_set: Some(true),
        };
        let json = serde_json::to_value(&fc).unwrap();
        assert_eq!(json["keepOnlySet"], json!(true));
        assert!(json.get("keep_only_set").is_none());
    }

    // --- Round-trip tests against real fixtures ---

    #[test]
    fn deserializes_real_compress_images_recipe() {
        let raw = include_str!("../../bnto/tests/fixtures/explicit/compress-images.bnto.json");
        let def: Definition = serde_json::from_str(raw).expect("deserialization");

        assert_eq!(def.id, "compress-images");
        assert_eq!(def.node_type, "group");
        assert_eq!(def.name, "Compress Images");
        assert_eq!(
            def.metadata.description.as_deref(),
            Some("Accepts image files and compresses each one.")
        );

        let children = def.nodes.as_ref().expect("group has nodes");
        assert_eq!(children.len(), 3);

        let edges = def.edges.as_ref().expect("group has edges");
        assert_eq!(edges.len(), 2);
        assert_eq!(edges[0].source, "input");
        assert_eq!(edges[0].target, "compress-loop");
    }

    #[test]
    fn round_trip_preserves_shape_for_all_recipe_fixtures() {
        // Explicit-mode recipes exercise containers, I/O nodes, nested loops.
        // We verify the Rust representation is stable across serialize ->
        // deserialize, not that the emitted JSON is byte-identical to the
        // source. Fixtures store coordinates as JSON integers (e.g. `"x": 0`);
        // Position is `f64`, so re-serialization emits `0.0`. That's an
        // encoding artifact -- the Definition value itself is lossless.
        let fixtures = [
            include_str!("../../bnto/tests/fixtures/explicit/compress-images.bnto.json"),
            include_str!("../../bnto/tests/fixtures/explicit/optimize-images-for-web.bnto.json"),
            include_str!("../../bnto/tests/fixtures/explicit/merge-csv.bnto.json"),
            include_str!("../../bnto/tests/fixtures/explicit/rename-csv-columns.bnto.json"),
            include_str!("../../bnto/tests/fixtures/explicit/rename-files.bnto.json"),
            include_str!("../../bnto/tests/fixtures/explicit/strip-exif.bnto.json"),
            include_str!("../../bnto/tests/fixtures/explicit/svg-to-png.bnto.json"),
            include_str!("../../bnto/tests/fixtures/explicit/watermark-images.bnto.json"),
        ];

        for (i, raw) in fixtures.iter().enumerate() {
            let first: Definition =
                serde_json::from_str(raw).expect("fixture parses into Definition");
            let reemitted = serde_json::to_string(&first).expect("serializes back");
            let second: Definition =
                serde_json::from_str(&reemitted).expect("re-emitted JSON parses");
            assert_eq!(first, second, "fixture {} lost data through round-trip", i);
        }
    }

    #[test]
    fn container_node_preserves_nested_children() {
        let child = minimal_leaf("child", "image-compress");
        let container = Definition {
            nodes: Some(vec![child.clone()]),
            edges: Some(vec![]),
            ..minimal_leaf("parent", "loop")
        };

        let json = serde_json::to_string(&container).unwrap();
        let back: Definition = serde_json::from_str(&json).unwrap();
        assert_eq!(back.nodes.as_ref().unwrap().len(), 1);
        assert_eq!(back.nodes.as_ref().unwrap()[0], child);
        assert_eq!(back.edges.as_ref().unwrap().len(), 0);
    }

    #[test]
    fn missing_parameters_defaults_to_empty_object() {
        // `parameters` is required in TS but this guards against recipes
        // authored by older tooling that may omit it entirely.
        let raw = r#"{
            "id": "n1",
            "type": "image-compress",
            "version": "1.0.0",
            "name": "Test",
            "position": { "x": 0, "y": 0 },
            "metadata": {},
            "inputPorts": [],
            "outputPorts": []
        }"#;
        let def: Definition = serde_json::from_str(raw).expect("accepts missing parameters");
        assert_eq!(def.parameters, json!({}));
    }
}
