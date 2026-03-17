// Pipeline definition types — what the engine receives when the user clicks "Run".
// These mirror the TypeScript `PipelineDefinition` / `PipelineNode` types exactly.

use serde::Deserialize;

// =============================================================================
// Pipeline Definition
// =============================================================================

/// The top-level pipeline definition that the executor receives.
#[derive(Debug, Clone, Deserialize)]
pub struct PipelineDefinition {
    /// Ordered list of nodes. Output from node N feeds into node N+1.
    pub nodes: Vec<PipelineNode>,
}

/// A single node in the pipeline.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PipelineNode {
    /// Unique identifier (e.g., "node-abc123"). Used in progress events.
    pub id: String,

    /// Node type (e.g., "image", "spreadsheet", "file-system").
    /// Combined with `params.operation` for compound dispatch key.
    /// `type` is a reserved keyword in Rust, so we rename from JSON.
    #[serde(rename = "type")]
    pub node_type: String,

    /// Configuration parameters including `operation` and type-specific settings.
    /// Accepts both "params" (Rust convention) and "parameters" (TS convention).
    #[serde(default, alias = "parameters")]
    pub params: serde_json::Map<String, serde_json::Value>,

    /// Child nodes for container types (loop, group, parallel).
    /// Accepts both "children" (Rust) and "nodes" (TS) field names.
    #[serde(alias = "nodes")]
    pub children: Option<Vec<PipelineNode>>,
}

// =============================================================================
// Pipeline File Types
// =============================================================================

/// Engine's internal file representation — raw bytes, not browser File objects.
/// The adapter layer converts from native file types to this.
#[derive(Debug, Clone)]
pub struct PipelineFile {
    pub name: String,
    pub data: Vec<u8>,
    pub mime_type: String,
    /// Metadata from the processor that created this file (e.g., compression ratio).
    /// Empty for unprocessed input files.
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

/// A single output file produced by the pipeline.
#[derive(Debug, Clone)]
pub struct PipelineFileResult {
    pub name: String,
    pub data: Vec<u8>,
    pub mime_type: String,
    /// Processing metadata (timing, stats, etc.) from the processor.
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

/// The result of executing an entire pipeline.
#[derive(Debug, Clone)]
pub struct PipelineResult {
    pub files: Vec<PipelineFileResult>,
    /// Total wall-clock time in milliseconds.
    pub duration_ms: u64,
}

// =============================================================================
// Helpers
// =============================================================================

/// Returns true if the node type is an I/O structural marker (input/output)
/// that the executor should skip.
pub fn is_io_node(node_type: &str) -> bool {
    node_type == "input" || node_type == "output"
}

/// Returns true if the node type is a container (loop, group, parallel).
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

    #[test]
    fn test_simple_definition_deserializes() {
        // A minimal pipeline: input -> compress -> output.
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
        // Group -> Loop -> processor (3 levels of nesting).
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
    // Verify that TS field names ("nodes", "parameters") work alongside
    // Rust field names ("children", "params").

    #[test]
    fn test_nodes_alias_deserializes_as_children() {
        // TS recipes use "nodes" for child definitions; Rust uses "children".
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
        // TS recipes use "parameters"; Rust uses "params".
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

        assert_eq!(loop_node.params["mode"], "forEach");

        let children = loop_node.children.as_ref().unwrap();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].params["operation"], "compress");
        assert_eq!(children[0].params["quality"], 75);
    }

    #[test]
    fn test_original_field_names_still_work() {
        // Backward compat: "children" and "params" still work.
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
        // Real recipe JSON includes fields the Rust struct doesn't have
        // (version, name, position, metadata, inputPorts, outputPorts, edges).
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
    // Verify exact JSON shape from TS recipe definitions deserializes correctly.

    #[test]
    fn test_compress_images_recipe_deserializes() {
        // Input -> Group("Batch Compress") -> Loop -> [image:compress] -> Output
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

        assert_eq!(def.nodes.len(), 3);
        assert_eq!(def.nodes[0].node_type, "input");
        assert_eq!(def.nodes[1].node_type, "group");
        assert_eq!(def.nodes[1].id, "batch-compress");
        assert_eq!(def.nodes[2].node_type, "output");

        let group_children = def.nodes[1].children.as_ref().unwrap();
        assert_eq!(group_children.len(), 1);
        assert_eq!(group_children[0].node_type, "loop");

        let loop_children = group_children[0].children.as_ref().unwrap();
        assert_eq!(loop_children.len(), 1);
        assert_eq!(loop_children[0].id, "compress-image");
        assert_eq!(loop_children[0].node_type, "image");
        assert_eq!(loop_children[0].params["operation"], "compress");
        assert_eq!(loop_children[0].params["quality"], 80);
    }

    #[test]
    fn test_clean_csv_recipe_deserializes() {
        // Input -> Group("CSV Cleaner") -> [spreadsheet:clean] -> Output
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
        assert_eq!(def.nodes[1].node_type, "group");
        assert_eq!(def.nodes[1].id, "csv-cleaner");

        let group_children = def.nodes[1].children.as_ref().unwrap();
        assert_eq!(group_children.len(), 1);
        assert_eq!(group_children[0].node_type, "spreadsheet");
        assert_eq!(group_children[0].params["operation"], "clean");
    }

    #[test]
    fn test_rename_files_recipe_deserializes() {
        // Input -> Group("Batch Rename") -> Loop -> [file-system:rename] -> Output
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

        let group_node = &def.nodes[1];
        assert_eq!(group_node.node_type, "group");
        assert_eq!(group_node.id, "batch-rename");

        let group_children = group_node.children.as_ref().unwrap();
        assert_eq!(group_children.len(), 1);
        assert_eq!(group_children[0].node_type, "loop");

        let loop_children = group_children[0].children.as_ref().unwrap();
        assert_eq!(loop_children.len(), 1);
        assert_eq!(loop_children[0].node_type, "file-system");
        assert_eq!(loop_children[0].params["operation"], "rename");
        assert_eq!(loop_children[0].params["prefix"], "renamed-");
    }

    #[test]
    fn test_deeply_nested_three_levels() {
        // Group -> Group -> Loop -> processor (all using TS field names).
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
