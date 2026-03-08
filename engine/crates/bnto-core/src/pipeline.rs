// =============================================================================
// Pipeline Definition Types — What the Engine Receives to Execute
// =============================================================================
//
// WHAT IS THIS FILE?
// When a user clicks "Run" in the editor, the web app sends a recipe
// definition (JSON) to the engine. This file defines the Rust types that
// the engine deserializes that JSON into.
//
// These types mirror the TypeScript `PipelineDefinition` / `PipelineNode`
// types exactly — same JSON shape, same field names. A definition created
// in the web app can be serialized to JSON and deserialized here without
// any transformation.
//
// WHAT ARE I/O NODES?
// Recipe definitions include "input" and "output" nodes that serve as
// structural markers — they tell the editor where files enter and leave
// the pipeline. The executor silently skips these nodes because they
// don't perform any processing. Only "processing nodes" (image, csv,
// file-system, etc.) get dispatched to a NodeProcessor.
//
// WHAT ARE CONTAINER NODES?
// Some nodes contain child nodes:
// - "loop" — runs children once PER file (each iteration gets one file)
// - "group" — runs children once on the FULL batch of files
// - "parallel" — same as group for now (concurrent execution is future)
//
// Container nodes enable complex recipes like "for each image, compress
// then resize" without flattening the node tree.

use serde::Deserialize;

// =============================================================================
// Pipeline Definition
// =============================================================================

/// The top-level pipeline definition that the executor receives.
///
/// RUST CONCEPT: `#[derive(Deserialize)]`
/// The `Deserialize` derive macro generates code that can parse JSON
/// (or any serde-supported format) into this struct. Combined with
/// `serde_json::from_str()`, we can do:
///   let def: PipelineDefinition = serde_json::from_str(json_str)?;
/// and get a fully typed Rust struct from raw JSON.
#[derive(Debug, Clone, Deserialize)]
pub struct PipelineDefinition {
    /// The ordered list of nodes in this pipeline.
    /// Nodes execute sequentially — output from node N feeds into node N+1.
    pub nodes: Vec<PipelineNode>,
}

/// A single node in the pipeline.
///
/// RUST CONCEPT: `#[serde(rename = "type")]`
/// The JSON field is called `"type"`, but `type` is a reserved keyword
/// in Rust (used for type aliases). So we name the Rust field `node_type`
/// and tell serde to look for `"type"` in the JSON.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PipelineNode {
    /// The unique identifier for this node (e.g., "node-abc123").
    /// Used in progress events so the UI knows which node to highlight.
    pub id: String,

    /// The node type identifier (e.g., "image", "spreadsheet", "file-system").
    /// Combined with `params.operation` to form the compound dispatch key
    /// (e.g., "image:compress"). I/O types ("input", "output") are skipped.
    #[serde(rename = "type")]
    pub node_type: String,

    /// Configuration parameters for this node.
    /// Contains the `operation` field (e.g., "compress", "resize") plus
    /// any operation-specific settings (quality, dimensions, format, etc.).
    ///
    /// RUST CONCEPT: `#[serde(default)]`
    /// If the JSON doesn't have a `params` field, use the default value
    /// (an empty Map) instead of failing with a parse error. This is
    /// important because I/O nodes often don't have params.
    ///
    /// RUST CONCEPT: `#[serde(alias = "...")]`
    /// The TypeScript `Definition` type uses `parameters` for this field,
    /// but the Rust struct uses `params`. The `alias` attribute tells serde
    /// to accept EITHER name when deserializing. This means the same Rust
    /// code works with both:
    ///   - `{ "params": { ... } }` (Rust convention, used in tests)
    ///   - `{ "parameters": { ... } }` (TypeScript convention, used in recipes)
    #[serde(default, alias = "parameters")]
    pub params: serde_json::Map<String, serde_json::Value>,

    /// Child nodes for container types (loop, group, parallel).
    /// `None` for primitive (leaf) nodes. `Some(vec![...])` for containers.
    ///
    /// RUST CONCEPT: `Option<Vec<T>>`
    /// Doubly-optional: the field might be absent from JSON (`None`),
    /// or present but empty (`Some(vec![])`). Both mean "no children."
    ///
    /// The TypeScript `Definition` type uses `nodes` for child definitions,
    /// but the Rust struct uses `children`. The `alias` lets serde accept
    /// either name — so real recipe JSON (with `"nodes"`) and test JSON
    /// (with `"children"`) both work.
    #[serde(alias = "nodes")]
    pub children: Option<Vec<PipelineNode>>,
}

// =============================================================================
// Pipeline File Types
// =============================================================================

/// A file that enters the pipeline for processing.
///
/// This is the engine's internal file representation — it holds raw bytes,
/// not a browser File object or filesystem path. The adapter layer
/// (WASM bridge, CLI, Tauri) converts from its native file type to this.
#[derive(Clone)]
pub struct PipelineFile {
    /// The filename (e.g., "photo.jpg", "data.csv").
    pub name: String,

    /// The raw file data as bytes.
    pub data: Vec<u8>,

    /// The MIME type (e.g., "image/jpeg", "text/csv").
    pub mime_type: String,
}

/// A single output file produced by the pipeline.
///
/// Includes the processed data plus metadata about the processing
/// (compression ratio, dimensions, rows affected, etc.).
#[derive(Debug, Clone)]
pub struct PipelineFileResult {
    /// The filename of the output (e.g., "photo-compressed.jpg").
    pub name: String,

    /// The processed file data as bytes.
    pub data: Vec<u8>,

    /// The MIME type of the output.
    pub mime_type: String,

    /// Metadata about the processing (timing, stats, etc.).
    /// Each node can attach arbitrary key-value metadata to its output.
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

/// The result of executing an entire pipeline.
#[derive(Debug, Clone)]
pub struct PipelineResult {
    /// All output files produced by the pipeline's final processing node.
    pub files: Vec<PipelineFileResult>,

    /// Total wall-clock time for the entire pipeline, in milliseconds.
    pub duration_ms: u64,
}

// =============================================================================
// Helper: Check if a node type is an I/O marker
// =============================================================================

/// Returns true if the node type is an I/O structural marker
/// (input or output) that the executor should skip.
pub fn is_io_node(node_type: &str) -> bool {
    node_type == "input" || node_type == "output"
}

/// Returns true if the node type is a container that holds child nodes
/// (loop, group, or parallel).
pub fn is_container_node(node_type: &str) -> bool {
    node_type == "loop" || node_type == "group" || node_type == "parallel"
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // --- Deserialization Tests ---
    // Verify we can parse the same JSON shape that the TypeScript side produces.

    #[test]
    fn test_simple_definition_deserializes() {
        // A minimal pipeline: input → compress → output.
        let json = r#"{
            "nodes": [
                { "id": "n1", "type": "input" },
                { "id": "n2", "type": "image", "params": { "operation": "compress", "quality": 80 } },
                { "id": "n3", "type": "output" }
            ]
        }"#;

        let def: PipelineDefinition = serde_json::from_str(json).unwrap();

        assert_eq!(def.nodes.len(), 3);
        assert_eq!(def.nodes[0].id, "n1");
        assert_eq!(def.nodes[0].node_type, "input");
        assert_eq!(def.nodes[1].id, "n2");
        assert_eq!(def.nodes[1].node_type, "image");
        assert_eq!(def.nodes[2].id, "n3");
        assert_eq!(def.nodes[2].node_type, "output");
    }

    #[test]
    fn test_params_deserialize_correctly() {
        let json = r#"{
            "nodes": [
                {
                    "id": "n1",
                    "type": "image",
                    "params": {
                        "operation": "compress",
                        "quality": 80,
                        "preserveExif": true
                    }
                }
            ]
        }"#;

        let def: PipelineDefinition = serde_json::from_str(json).unwrap();
        let params = &def.nodes[0].params;

        assert_eq!(params["operation"], "compress");
        assert_eq!(params["quality"], 80);
        assert_eq!(params["preserveExif"], true);
    }

    #[test]
    fn test_missing_params_defaults_to_empty() {
        // I/O nodes often don't have params.
        let json = r#"{
            "nodes": [
                { "id": "n1", "type": "input" }
            ]
        }"#;

        let def: PipelineDefinition = serde_json::from_str(json).unwrap();
        assert!(def.nodes[0].params.is_empty());
    }

    #[test]
    fn test_container_node_with_children() {
        // A loop node containing a compress child.
        let json = r#"{
            "nodes": [
                {
                    "id": "loop-1",
                    "type": "loop",
                    "children": [
                        { "id": "child-1", "type": "image", "params": { "operation": "compress" } }
                    ]
                }
            ]
        }"#;

        let def: PipelineDefinition = serde_json::from_str(json).unwrap();
        let loop_node = &def.nodes[0];

        assert_eq!(loop_node.node_type, "loop");
        let children = loop_node.children.as_ref().unwrap();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].node_type, "image");
    }

    #[test]
    fn test_no_children_is_none() {
        let json = r#"{
            "nodes": [
                { "id": "n1", "type": "image", "params": { "operation": "compress" } }
            ]
        }"#;

        let def: PipelineDefinition = serde_json::from_str(json).unwrap();
        assert!(def.nodes[0].children.is_none());
    }

    #[test]
    fn test_nested_containers() {
        // Group containing a loop containing a processing node.
        let json = r#"{
            "nodes": [
                {
                    "id": "group-1",
                    "type": "group",
                    "children": [
                        {
                            "id": "loop-1",
                            "type": "loop",
                            "children": [
                                { "id": "proc-1", "type": "image", "params": { "operation": "compress" } }
                            ]
                        }
                    ]
                }
            ]
        }"#;

        let def: PipelineDefinition = serde_json::from_str(json).unwrap();
        let group = &def.nodes[0];
        let loop_node = &group.children.as_ref().unwrap()[0];
        let proc_node = &loop_node.children.as_ref().unwrap()[0];

        assert_eq!(group.node_type, "group");
        assert_eq!(loop_node.node_type, "loop");
        assert_eq!(proc_node.node_type, "image");
    }

    // --- Serde Alias Tests ---
    // Verify that the TypeScript field names ("nodes", "parameters") work
    // alongside the Rust field names ("children", "params").

    #[test]
    fn test_nodes_alias_deserializes_as_children() {
        // TypeScript recipes use "nodes" for child definitions.
        // The Rust struct uses "children". The alias bridges this gap.
        let json = r#"{
            "nodes": [
                {
                    "id": "loop-1",
                    "type": "loop",
                    "nodes": [
                        { "id": "child-1", "type": "image", "params": { "operation": "compress" } }
                    ]
                }
            ]
        }"#;

        let def: PipelineDefinition = serde_json::from_str(json).unwrap();
        let loop_node = &def.nodes[0];
        let children = loop_node.children.as_ref().unwrap();

        assert_eq!(children.len(), 1);
        assert_eq!(children[0].id, "child-1");
        assert_eq!(children[0].node_type, "image");
    }

    #[test]
    fn test_parameters_alias_deserializes_as_params() {
        // TypeScript recipes use "parameters" for node config.
        // The Rust struct uses "params". The alias bridges this gap.
        let json = r#"{
            "nodes": [
                {
                    "id": "n1",
                    "type": "image",
                    "parameters": {
                        "operation": "compress",
                        "quality": 80
                    }
                }
            ]
        }"#;

        let def: PipelineDefinition = serde_json::from_str(json).unwrap();
        let params = &def.nodes[0].params;

        assert_eq!(params["operation"], "compress");
        assert_eq!(params["quality"], 80);
    }

    #[test]
    fn test_both_aliases_together() {
        // Both TS field names used simultaneously in one definition.
        let json = r#"{
            "nodes": [
                {
                    "id": "loop-1",
                    "type": "loop",
                    "parameters": { "mode": "forEach" },
                    "nodes": [
                        {
                            "id": "child-1",
                            "type": "image",
                            "parameters": { "operation": "compress", "quality": 75 }
                        }
                    ]
                }
            ]
        }"#;

        let def: PipelineDefinition = serde_json::from_str(json).unwrap();
        let loop_node = &def.nodes[0];

        // "parameters" → params
        assert_eq!(loop_node.params["mode"], "forEach");

        // "nodes" → children
        let children = loop_node.children.as_ref().unwrap();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].params["operation"], "compress");
        assert_eq!(children[0].params["quality"], 75);
    }

    #[test]
    fn test_original_field_names_still_work() {
        // Backward compatibility: "children" and "params" still work.
        let json = r#"{
            "nodes": [
                {
                    "id": "loop-1",
                    "type": "loop",
                    "params": { "mode": "forEach" },
                    "children": [
                        { "id": "child-1", "type": "image", "params": { "operation": "compress" } }
                    ]
                }
            ]
        }"#;

        let def: PipelineDefinition = serde_json::from_str(json).unwrap();
        let loop_node = &def.nodes[0];

        assert_eq!(loop_node.params["mode"], "forEach");
        assert_eq!(loop_node.children.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_unknown_fields_silently_ignored() {
        // Real recipe JSON includes fields the Rust struct doesn't have:
        // version, name, position, metadata, inputPorts, outputPorts, edges.
        // Serde should ignore them without error.
        let json = r#"{
            "nodes": [
                {
                    "id": "compress-image",
                    "type": "image",
                    "version": "1.0.0",
                    "name": "Compress Image",
                    "position": { "x": 100, "y": 100 },
                    "metadata": { "description": "Compresses images" },
                    "parameters": { "operation": "compress", "quality": 80 },
                    "inputPorts": [{ "id": "in-1", "name": "files" }],
                    "outputPorts": [{ "id": "out-1", "name": "files" }]
                }
            ],
            "edges": [{ "id": "e1", "source": "input", "target": "compress-image" }]
        }"#;

        let def: PipelineDefinition = serde_json::from_str(json).unwrap();
        assert_eq!(def.nodes.len(), 1);
        assert_eq!(def.nodes[0].id, "compress-image");
        assert_eq!(def.nodes[0].params["operation"], "compress");
        assert_eq!(def.nodes[0].params["quality"], 80);
    }

    // --- Full Recipe Deserialization Tests ---
    // Verify that the EXACT JSON shape from TS recipe definitions
    // deserializes correctly with all aliases and ignored fields.

    #[test]
    fn test_compress_images_recipe_deserializes() {
        // Compositional: Input → Group("Batch Compress") → Loop → [image:compress] → Output
        let json = r#"{
            "nodes": [
                {
                    "id": "input", "type": "input", "version": "1.0.0",
                    "name": "Input Files", "position": {"x": 0, "y": 100},
                    "metadata": {},
                    "parameters": { "mode": "file-upload", "accept": ["image/jpeg"] },
                    "inputPorts": [], "outputPorts": [{"id": "out-1", "name": "files"}]
                },
                {
                    "id": "batch-compress", "type": "group", "version": "1.0.0",
                    "name": "Batch Compress", "position": {"x": 250, "y": 100},
                    "metadata": { "description": "Reusable sub-recipe." },
                    "parameters": {},
                    "inputPorts": [{"id": "in-1", "name": "files"}],
                    "outputPorts": [{"id": "out-1", "name": "files"}],
                    "nodes": [
                        {
                            "id": "compress-loop", "type": "loop", "version": "1.0.0",
                            "name": "Compress Each Image", "position": {"x": 0, "y": 0},
                            "metadata": {},
                            "parameters": { "mode": "forEach" },
                            "inputPorts": [{"id": "in-1", "name": "items"}], "outputPorts": [],
                            "nodes": [
                                {
                                    "id": "compress-image", "type": "image", "version": "1.0.0",
                                    "name": "Compress Image", "position": {"x": 0, "y": 0},
                                    "metadata": {},
                                    "parameters": { "operation": "compress", "quality": 80 },
                                    "inputPorts": [], "outputPorts": []
                                }
                            ],
                            "edges": []
                        }
                    ],
                    "edges": []
                },
                {
                    "id": "output", "type": "output", "version": "1.0.0",
                    "name": "Compressed Images", "position": {"x": 500, "y": 100},
                    "metadata": {},
                    "parameters": { "mode": "download", "zip": true },
                    "inputPorts": [{"id": "in-1", "name": "files"}], "outputPorts": []
                }
            ],
            "edges": [
                {"id": "e1", "source": "input", "target": "batch-compress"},
                {"id": "e2", "source": "batch-compress", "target": "output"}
            ]
        }"#;

        let def: PipelineDefinition = serde_json::from_str(json).unwrap();

        // Top level: 3 nodes (input, group, output).
        assert_eq!(def.nodes.len(), 3);
        assert_eq!(def.nodes[0].node_type, "input");
        assert_eq!(def.nodes[1].node_type, "group");
        assert_eq!(def.nodes[1].id, "batch-compress");
        assert_eq!(def.nodes[2].node_type, "output");

        // Group has 1 child (compress-loop).
        let group_children = def.nodes[1].children.as_ref().unwrap();
        assert_eq!(group_children.len(), 1);
        assert_eq!(group_children[0].node_type, "loop");

        // Loop has 1 child (compress-image processor).
        let loop_children = group_children[0].children.as_ref().unwrap();
        assert_eq!(loop_children.len(), 1);
        assert_eq!(loop_children[0].id, "compress-image");
        assert_eq!(loop_children[0].node_type, "image");
        assert_eq!(loop_children[0].params["operation"], "compress");
        assert_eq!(loop_children[0].params["quality"], 80);
    }

    #[test]
    fn test_clean_csv_recipe_deserializes() {
        // Compositional: Input → Group("CSV Cleaner") → [spreadsheet:clean] → Output
        let json = r#"{
            "nodes": [
                {
                    "id": "input", "type": "input", "version": "1.0.0",
                    "name": "Input Files", "position": {"x": 0, "y": 100},
                    "metadata": {},
                    "parameters": { "mode": "file-upload" },
                    "inputPorts": [], "outputPorts": [{"id": "out-1", "name": "files"}]
                },
                {
                    "id": "csv-cleaner", "type": "group", "version": "1.0.0",
                    "name": "CSV Cleaner", "position": {"x": 250, "y": 100},
                    "metadata": {},
                    "parameters": {},
                    "inputPorts": [{"id": "in-1", "name": "files"}],
                    "outputPorts": [{"id": "out-1", "name": "files"}],
                    "nodes": [
                        {
                            "id": "clean", "type": "spreadsheet", "version": "1.0.0",
                            "name": "Clean CSV", "position": {"x": 0, "y": 0},
                            "metadata": {},
                            "parameters": {
                                "operation": "clean",
                                "trimWhitespace": true,
                                "removeEmptyRows": true,
                                "removeDuplicates": true
                            },
                            "inputPorts": [{"id": "in-1", "name": "files"}],
                            "outputPorts": [{"id": "out-1", "name": "files"}]
                        }
                    ],
                    "edges": []
                },
                {
                    "id": "output", "type": "output", "version": "1.0.0",
                    "name": "Cleaned CSV", "position": {"x": 500, "y": 100},
                    "metadata": {},
                    "parameters": { "mode": "download" },
                    "inputPorts": [{"id": "in-1", "name": "files"}], "outputPorts": []
                }
            ],
            "edges": [
                {"id": "e1", "source": "input", "target": "csv-cleaner"},
                {"id": "e2", "source": "csv-cleaner", "target": "output"}
            ]
        }"#;

        let def: PipelineDefinition = serde_json::from_str(json).unwrap();

        assert_eq!(def.nodes.len(), 3);
        // Middle node is now a group, not a flat processor.
        assert_eq!(def.nodes[1].node_type, "group");
        assert_eq!(def.nodes[1].id, "csv-cleaner");

        // Group has 1 child (the clean processor).
        let group_children = def.nodes[1].children.as_ref().unwrap();
        assert_eq!(group_children.len(), 1);
        assert_eq!(group_children[0].node_type, "spreadsheet");
        assert_eq!(group_children[0].params["operation"], "clean");
    }

    #[test]
    fn test_rename_files_recipe_deserializes() {
        // Compositional: Input → Group("Batch Rename") → Loop → [file-system:rename] → Output
        let json = r#"{
            "nodes": [
                { "id": "input", "type": "input", "version": "1.0.0",
                  "name": "Input", "position": {"x": 0, "y": 0}, "metadata": {},
                  "parameters": {}, "inputPorts": [], "outputPorts": [] },
                {
                    "id": "batch-rename", "type": "group", "version": "1.0.0",
                    "name": "Batch Rename", "position": {"x": 250, "y": 100},
                    "metadata": {},
                    "parameters": {},
                    "inputPorts": [], "outputPorts": [],
                    "nodes": [
                        {
                            "id": "rename-loop", "type": "loop", "version": "1.0.0",
                            "name": "Rename Each File", "position": {"x": 0, "y": 0},
                            "metadata": {},
                            "parameters": { "mode": "forEach" },
                            "inputPorts": [], "outputPorts": [],
                            "nodes": [
                                {
                                    "id": "rename-file", "type": "file-system", "version": "1.0.0",
                                    "name": "Rename File", "position": {"x": 0, "y": 0},
                                    "metadata": {},
                                    "parameters": { "operation": "rename", "prefix": "renamed-" },
                                    "inputPorts": [], "outputPorts": []
                                }
                            ],
                            "edges": []
                        }
                    ],
                    "edges": []
                },
                { "id": "output", "type": "output", "version": "1.0.0",
                  "name": "Output", "position": {"x": 0, "y": 0}, "metadata": {},
                  "parameters": {}, "inputPorts": [], "outputPorts": [] }
            ],
            "edges": []
        }"#;

        let def: PipelineDefinition = serde_json::from_str(json).unwrap();

        // Middle node is the batch-rename group.
        let group_node = &def.nodes[1];
        assert_eq!(group_node.node_type, "group");
        assert_eq!(group_node.id, "batch-rename");

        // Group has 1 child (rename-loop).
        let group_children = group_node.children.as_ref().unwrap();
        assert_eq!(group_children.len(), 1);
        assert_eq!(group_children[0].node_type, "loop");

        // Loop has 1 child (rename-file processor).
        let loop_children = group_children[0].children.as_ref().unwrap();
        assert_eq!(loop_children.len(), 1);
        assert_eq!(loop_children[0].node_type, "file-system");
        assert_eq!(loop_children[0].params["operation"], "rename");
        assert_eq!(loop_children[0].params["prefix"], "renamed-");
    }

    #[test]
    fn test_deeply_nested_three_levels() {
        // Group → Group → Loop → processor — 3 levels of nesting.
        // All using TS field names ("nodes", "parameters").
        let json = r#"{
            "nodes": [
                {
                    "id": "outer-group", "type": "group",
                    "parameters": {},
                    "nodes": [
                        {
                            "id": "inner-group", "type": "group",
                            "parameters": {},
                            "nodes": [
                                {
                                    "id": "the-loop", "type": "loop",
                                    "parameters": { "mode": "forEach" },
                                    "nodes": [
                                        {
                                            "id": "processor", "type": "image",
                                            "parameters": { "operation": "compress", "quality": 50 }
                                        }
                                    ]
                                }
                            ]
                        }
                    ]
                }
            ]
        }"#;

        let def: PipelineDefinition = serde_json::from_str(json).unwrap();

        // Walk 3 levels deep.
        let outer = &def.nodes[0];
        assert_eq!(outer.node_type, "group");

        let inner = &outer.children.as_ref().unwrap()[0];
        assert_eq!(inner.node_type, "group");

        let loop_node = &inner.children.as_ref().unwrap()[0];
        assert_eq!(loop_node.node_type, "loop");

        let processor = &loop_node.children.as_ref().unwrap()[0];
        assert_eq!(processor.node_type, "image");
        assert_eq!(processor.params["operation"], "compress");
        assert_eq!(processor.params["quality"], 50);
    }

    // --- Helper Function Tests ---

    #[test]
    fn test_is_io_node() {
        assert!(is_io_node("input"));
        assert!(is_io_node("output"));
        assert!(!is_io_node("image"));
        assert!(!is_io_node("spreadsheet"));
        assert!(!is_io_node("loop"));
    }

    #[test]
    fn test_is_container_node() {
        assert!(is_container_node("loop"));
        assert!(is_container_node("group"));
        assert!(is_container_node("parallel"));
        assert!(!is_container_node("image"));
        assert!(!is_container_node("input"));
        assert!(!is_container_node("output"));
    }
}
