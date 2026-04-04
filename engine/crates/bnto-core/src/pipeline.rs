// Pipeline definition types — deserialized from JSON recipe definitions.
// Mirrors the TypeScript `PipelineDefinition` / `PipelineNode` types exactly.
// I/O nodes are structural markers (skipped by executor); container nodes
// (loop, group, parallel) hold child nodes for nested execution.

use serde::{Deserialize, Serialize};

// =============================================================================
// Pipeline Settings — Recipe-Level Configuration
// =============================================================================

/// How the executor handles iteration over multiple input files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum IterationMode {
    /// Execute exactly what's defined — containers control iteration.
    /// This is the existing behavior and the default for backward compatibility.
    #[default]
    Explicit,
    /// Wrap contiguous per-file processor sequences in implicit per-file loops.
    /// Flat recipes produce identical output to explicit-loop recipes.
    Auto,
}

/// Recipe-level settings on the root Definition. Extensible — new fields
/// can be added without changing the schema shape.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PipelineSettings {
    /// How the executor iterates over multiple input files.
    #[serde(default)]
    pub iteration: IterationMode,
}

// =============================================================================
// Pipeline Definition
// =============================================================================

/// The top-level pipeline definition that the executor receives.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PipelineDefinition {
    /// The ordered list of nodes in this pipeline.
    /// Nodes execute sequentially — output from node N feeds into node N+1.
    pub nodes: Vec<PipelineNode>,

    /// Recipe-level settings (iteration mode, etc.).
    /// Optional for backward compatibility — missing defaults to explicit iteration.
    #[serde(default)]
    pub settings: Option<PipelineSettings>,
}

impl PipelineDefinition {
    /// Returns the resolved iteration mode, defaulting to `Explicit`
    /// when settings are absent.
    pub fn resolved_iteration(&self) -> IterationMode {
        self.settings
            .as_ref()
            .map(|s| s.iteration)
            .unwrap_or_default()
    }
}

/// A single node in the pipeline.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PipelineNode {
    /// The unique identifier for this node (e.g., "node-abc123").
    /// Used in progress events so the UI knows which node to highlight.
    pub id: String,

    /// The per-operation type key (e.g., "image-compress", "spreadsheet-clean").
    /// I/O types ("input", "output") are skipped by the executor.
    #[serde(rename = "type")]
    pub node_type: String,

    /// Configuration parameters for this node.
    /// Operation-specific settings (quality, dimensions, format, etc.).
    /// Defaults to an empty Map when absent (I/O nodes often have no params).
    /// Accepts both `params` (Rust convention) and `parameters` (TypeScript
    /// convention) via serde alias.
    #[serde(default, alias = "parameters")]
    pub params: serde_json::Map<String, serde_json::Value>,

    /// Child nodes for container types (loop, group, parallel).
    /// `None` for primitive (leaf) nodes. `Some(vec![...])` for containers.
    /// Both `None` and `Some(vec![])` mean "no children."
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
#[derive(Debug, Clone)]
pub struct PipelineFile {
    /// The filename (e.g., "photo.jpg", "data.csv").
    pub name: String,

    /// The raw file data as bytes.
    pub data: Vec<u8>,

    /// The MIME type (e.g., "image/jpeg", "text/csv").
    pub mime_type: String,

    /// Metadata from the processor that created this file.
    /// Carries through the pipeline so the final result includes
    /// stats like compression ratio, original size, etc.
    /// Empty for files that haven't been processed yet (inputs).
    pub metadata: serde_json::Map<String, serde_json::Value>,
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
// InputMode — How data enters the recipe
// =============================================================================

/// How the recipe expects to receive its input data.
/// Read from the input node's `mode` parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputMode {
    /// User uploads files (default). CLI reads file paths from disk.
    #[default]
    FileUpload,
    /// User provides a URL. CLI accepts a URL string.
    Url,
    /// User provides text content. CLI accepts a text string.
    Text,
}

/// Walk the definition to find the input node and read its `mode` param.
/// Returns `FileUpload` if no input node or no mode param is found.
pub fn resolve_input_mode(def: &PipelineDefinition) -> InputMode {
    find_input_mode_in_nodes(&def.nodes)
}

fn find_input_mode_in_nodes(nodes: &[PipelineNode]) -> InputMode {
    for node in nodes {
        if node.node_type == "input" {
            return match node.params.get("mode").and_then(|v| v.as_str()) {
                Some("url") => InputMode::Url,
                Some("text") => InputMode::Text,
                _ => InputMode::FileUpload,
            };
        }
        // Recurse into container children
        if let Some(children) = &node.children {
            let mode = find_input_mode_in_nodes(children);
            if mode != InputMode::FileUpload {
                return mode;
            }
            // Check if we found an input node with default mode
            if children.iter().any(|c| c.node_type == "input") {
                return InputMode::FileUpload;
            }
        }
    }
    InputMode::FileUpload
}

/// Walk the definition to find the first processing node (not I/O, not container).
/// Used by the CLI to know where to inject params like URL.
pub fn first_processing_node_id(def: &PipelineDefinition) -> Option<String> {
    find_first_processing_in_nodes(&def.nodes)
}

fn find_first_processing_in_nodes(nodes: &[PipelineNode]) -> Option<String> {
    for node in nodes {
        if is_io_node(&node.node_type) {
            continue;
        }
        if is_container_node(&node.node_type) {
            // Look inside container children for a processing node
            if let Some(children) = &node.children
                && let Some(id) = find_first_processing_in_nodes(children)
            {
                return Some(id);
            }
            continue;
        }
        // Found a processing node
        return Some(node.id.clone());
    }
    None
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

    fn parse_definition(json: &str) -> PipelineDefinition {
        serde_json::from_str(json).unwrap()
    }

    // --- Deserialization Tests ---
    // Verify we can parse the same JSON shape that the TypeScript side produces.

    #[test]
    fn test_simple_definition_deserializes() {
        // A minimal pipeline: input → compress → output.
        let json = r#"{
            "nodes": [
                { "id": "n1", "type": "input" },
                { "id": "n2", "type": "image-compress", "params": { "quality": 80 } },
                { "id": "n3", "type": "output" }
            ]
        }"#;

        let def: PipelineDefinition = serde_json::from_str(json).unwrap();

        assert_eq!(def.nodes.len(), 3);
        assert_eq!(def.nodes[0].id, "n1");
        assert_eq!(def.nodes[0].node_type, "input");
        assert_eq!(def.nodes[1].id, "n2");
        assert_eq!(def.nodes[1].node_type, "image-compress");
        assert_eq!(def.nodes[2].id, "n3");
        assert_eq!(def.nodes[2].node_type, "output");
    }

    #[test]
    fn test_params_deserialize_correctly() {
        let json = r#"{
            "nodes": [
                {
                    "id": "n1",
                    "type": "image-compress",
                    "params": {
                        "quality": 80,
                        "preserveExif": true
                    }
                }
            ]
        }"#;

        let def: PipelineDefinition = serde_json::from_str(json).unwrap();
        let params = &def.nodes[0].params;

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
                        { "id": "child-1", "type": "image-compress" }
                    ]
                }
            ]
        }"#;

        let def: PipelineDefinition = serde_json::from_str(json).unwrap();
        let loop_node = &def.nodes[0];

        assert_eq!(loop_node.node_type, "loop");
        let children = loop_node.children.as_ref().unwrap();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].node_type, "image-compress");
    }

    #[test]
    fn test_no_children_is_none() {
        let json = r#"{
            "nodes": [
                { "id": "n1", "type": "image-compress" }
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
                                { "id": "proc-1", "type": "image-compress" }
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
        assert_eq!(proc_node.node_type, "image-compress");
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
                        { "id": "child-1", "type": "image-compress" }
                    ]
                }
            ]
        }"#;

        let def: PipelineDefinition = serde_json::from_str(json).unwrap();
        let loop_node = &def.nodes[0];
        let children = loop_node.children.as_ref().unwrap();

        assert_eq!(children.len(), 1);
        assert_eq!(children[0].id, "child-1");
        assert_eq!(children[0].node_type, "image-compress");
    }

    #[test]
    fn test_parameters_alias_deserializes_as_params() {
        // TypeScript recipes use "parameters" for node config.
        // The Rust struct uses "params". The alias bridges this gap.
        let json = r#"{
            "nodes": [
                {
                    "id": "n1",
                    "type": "image-compress",
                    "parameters": { "quality": 80 }
                }
            ]
        }"#;

        let def: PipelineDefinition = serde_json::from_str(json).unwrap();
        let params = &def.nodes[0].params;

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
                            "type": "image-compress",
                            "parameters": { "quality": 75 }
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
                        { "id": "child-1", "type": "image-compress" }
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
                    "type": "image-compress",
                    "version": "1.0.0",
                    "name": "Compress Image",
                    "position": { "x": 100, "y": 100 },
                    "metadata": { "description": "Compresses images" },
                    "parameters": { "quality": 80 },
                    "inputPorts": [{ "id": "in-1", "name": "files" }],
                    "outputPorts": [{ "id": "out-1", "name": "files" }]
                }
            ],
            "edges": [{ "id": "e1", "source": "input", "target": "compress-image" }]
        }"#;

        let def: PipelineDefinition = serde_json::from_str(json).unwrap();
        assert_eq!(def.nodes.len(), 1);
        assert_eq!(def.nodes[0].id, "compress-image");
        assert_eq!(def.nodes[0].params["quality"], 80);
    }

    // --- Full Recipe Deserialization Tests ---
    // Verify that the EXACT JSON shape from TS recipe definitions
    // deserializes correctly with all aliases and ignored fields.

    #[test]
    fn test_compress_images_recipe_deserializes() {
        // Compositional: Input → Group("Batch Compress") → Loop → [image-compress] → Output
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
                                    "id": "compress-image", "type": "image-compress", "version": "1.0.0",
                                    "name": "Compress Image", "position": {"x": 0, "y": 0},
                                    "metadata": {},
                                    "parameters": { "quality": 80 },
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
        assert_eq!(loop_children[0].node_type, "image-compress");
        assert_eq!(loop_children[0].params["quality"], 80);
    }

    #[test]
    fn test_clean_csv_recipe_deserializes() {
        // Compositional: Input → Group("CSV Cleaner") → [spreadsheet-clean] → Output
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
                            "id": "clean", "type": "spreadsheet-clean", "version": "1.0.0",
                            "name": "Clean CSV", "position": {"x": 0, "y": 0},
                            "metadata": {},
                            "parameters": {
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
        assert_eq!(group_children[0].node_type, "spreadsheet-clean");
    }

    #[test]
    fn test_rename_files_recipe_deserializes() {
        // Compositional: Input → Group("Batch Rename") → Loop → [file-rename] → Output
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
                                    "id": "rename-file", "type": "file-rename", "version": "1.0.0",
                                    "name": "Rename File", "position": {"x": 0, "y": 0},
                                    "metadata": {},
                                    "parameters": { "prefix": "renamed-" },
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
        assert_eq!(loop_children[0].node_type, "file-rename");
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
                                            "id": "processor", "type": "image-compress",
                                            "parameters": { "quality": 50 }
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
        assert_eq!(processor.node_type, "image-compress");
        assert_eq!(processor.params["quality"], 50);
    }

    // --- Helper Function Tests ---

    // --- PipelineSettings & IterationMode Tests ---

    #[test]
    fn test_definition_without_settings_deserializes() {
        let json = r#"{
            "nodes": [
                { "id": "n1", "type": "input" },
                { "id": "n2", "type": "image-compress" },
                { "id": "n3", "type": "output" }
            ]
        }"#;
        let def: PipelineDefinition = serde_json::from_str(json).unwrap();
        assert!(def.settings.is_none());
    }

    #[test]
    fn test_definition_with_auto_iteration_deserializes() {
        let json = r#"{
            "settings": { "iteration": "auto" },
            "nodes": [
                { "id": "n1", "type": "input" },
                { "id": "n2", "type": "image-compress" },
                { "id": "n3", "type": "output" }
            ]
        }"#;
        let def: PipelineDefinition = serde_json::from_str(json).unwrap();
        let settings = def.settings.as_ref().unwrap();
        assert_eq!(settings.iteration, IterationMode::Auto);
    }

    #[test]
    fn test_definition_with_explicit_iteration_deserializes() {
        let json = r#"{
            "settings": { "iteration": "explicit" },
            "nodes": [
                { "id": "n1", "type": "image-compress" }
            ]
        }"#;
        let def: PipelineDefinition = serde_json::from_str(json).unwrap();
        let settings = def.settings.as_ref().unwrap();
        assert_eq!(settings.iteration, IterationMode::Explicit);
    }

    #[test]
    fn test_definition_with_unknown_iteration_fails() {
        let json = r#"{
            "settings": { "iteration": "garbage" },
            "nodes": [{ "id": "n1", "type": "input" }]
        }"#;
        let result = serde_json::from_str::<PipelineDefinition>(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_resolved_iteration_defaults_explicit() {
        let json = r#"{ "nodes": [] }"#;
        let def: PipelineDefinition = serde_json::from_str(json).unwrap();
        assert_eq!(def.resolved_iteration(), IterationMode::Explicit);
    }

    #[test]
    fn test_resolved_iteration_returns_auto() {
        let json = r#"{
            "settings": { "iteration": "auto" },
            "nodes": []
        }"#;
        let def: PipelineDefinition = serde_json::from_str(json).unwrap();
        assert_eq!(def.resolved_iteration(), IterationMode::Auto);
    }

    #[test]
    fn test_settings_with_default_iteration_field() {
        // Settings object present but iteration field absent — defaults to explicit.
        let json = r#"{
            "settings": {},
            "nodes": []
        }"#;
        let def: PipelineDefinition = serde_json::from_str(json).unwrap();
        let settings = def.settings.as_ref().unwrap();
        assert_eq!(settings.iteration, IterationMode::Explicit);
        assert_eq!(def.resolved_iteration(), IterationMode::Explicit);
    }

    // --- Serialization Round-Trip Tests ---
    // Verify PipelineDefinition and PipelineNode serialize and deserialize back.

    #[test]
    fn test_definition_round_trip_serialization() {
        let json = r#"{
            "nodes": [
                { "id": "n1", "type": "input" },
                { "id": "n2", "type": "image-compress", "params": { "quality": 80 } },
                { "id": "n3", "type": "output" }
            ],
            "settings": { "iteration": "auto" }
        }"#;

        let def: PipelineDefinition = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&def).unwrap();
        let round_tripped: PipelineDefinition = serde_json::from_str(&serialized).unwrap();

        assert_eq!(round_tripped.nodes.len(), def.nodes.len());
        for (orig, rt) in def.nodes.iter().zip(round_tripped.nodes.iter()) {
            assert_eq!(orig.id, rt.id);
            assert_eq!(orig.node_type, rt.node_type);
        }
        assert_eq!(round_tripped.resolved_iteration(), def.resolved_iteration());
    }

    #[test]
    fn test_definition_serialization_preserves_params() {
        let json = r#"{
            "nodes": [
                {
                    "id": "n1",
                    "type": "image-compress",
                    "params": { "quality": 80, "preserveExif": true, "name": "test" }
                }
            ]
        }"#;

        let def: PipelineDefinition = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&def).unwrap();
        let round_tripped: PipelineDefinition = serde_json::from_str(&serialized).unwrap();

        let params = &round_tripped.nodes[0].params;
        assert_eq!(params["quality"], 80);
        assert_eq!(params["preserveExif"], true);
        assert_eq!(params["name"], "test");
    }

    #[test]
    fn test_definition_serialization_preserves_children() {
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
                                { "id": "proc-1", "type": "image-compress", "params": { "quality": 50 } }
                            ]
                        }
                    ]
                }
            ]
        }"#;

        let def: PipelineDefinition = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&def).unwrap();
        let round_tripped: PipelineDefinition = serde_json::from_str(&serialized).unwrap();

        let group = &round_tripped.nodes[0];
        assert_eq!(group.node_type, "group");
        let loop_node = &group.children.as_ref().unwrap()[0];
        assert_eq!(loop_node.node_type, "loop");
        let proc_node = &loop_node.children.as_ref().unwrap()[0];
        assert_eq!(proc_node.node_type, "image-compress");
        assert_eq!(proc_node.params["quality"], 50);
    }

    #[test]
    fn test_iteration_mode_serializes_camel_case() {
        let auto_json = serde_json::to_string(&IterationMode::Auto).unwrap();
        assert_eq!(auto_json, r#""auto""#);

        let explicit_json = serde_json::to_string(&IterationMode::Explicit).unwrap();
        assert_eq!(explicit_json, r#""explicit""#);
    }

    #[test]
    fn test_pipeline_settings_serializes() {
        let settings = PipelineSettings {
            iteration: IterationMode::Auto,
        };
        let json = serde_json::to_string(&settings).unwrap();
        assert!(json.contains(r#""iteration":"auto""#));
    }

    // --- Helper Function Tests ---

    #[test]
    fn test_is_io_node() {
        assert!(is_io_node("input"));
        assert!(is_io_node("output"));
        assert!(!is_io_node("image-compress"));
        assert!(!is_io_node("spreadsheet-clean"));
        assert!(!is_io_node("loop"));
    }

    #[test]
    fn test_is_container_node() {
        assert!(is_container_node("loop"));
        assert!(is_container_node("group"));
        assert!(is_container_node("parallel"));
        assert!(!is_container_node("image-compress"));
        assert!(!is_container_node("input"));
        assert!(!is_container_node("output"));
    }

    // --- InputMode Resolution Tests ---

    #[test]
    fn test_resolve_input_mode_file_upload() {
        let def = parse_definition(
            r#"{
                "formatVersion": "1.0.0",
                "nodes": [
                    { "id": "in", "type": "input", "params": { "mode": "file-upload" } },
                    { "id": "proc", "type": "image-compress", "params": {} },
                    { "id": "out", "type": "output", "params": {} }
                ]
            }"#,
        );
        assert_eq!(resolve_input_mode(&def), InputMode::FileUpload);
    }

    #[test]
    fn test_resolve_input_mode_url() {
        let def = parse_definition(
            r#"{
                "formatVersion": "1.0.0",
                "nodes": [
                    { "id": "in", "type": "input", "params": { "mode": "url" } },
                    { "id": "proc", "type": "video-download", "params": {} },
                    { "id": "out", "type": "output", "params": {} }
                ]
            }"#,
        );
        assert_eq!(resolve_input_mode(&def), InputMode::Url);
    }

    #[test]
    fn test_resolve_input_mode_text() {
        let def = parse_definition(
            r#"{
                "formatVersion": "1.0.0",
                "nodes": [
                    { "id": "in", "type": "input", "params": { "mode": "text" } },
                    { "id": "proc", "type": "text-transform", "params": {} },
                    { "id": "out", "type": "output", "params": {} }
                ]
            }"#,
        );
        assert_eq!(resolve_input_mode(&def), InputMode::Text);
    }

    #[test]
    fn test_resolve_input_mode_missing_defaults() {
        let def = parse_definition(
            r#"{
                "formatVersion": "1.0.0",
                "nodes": [
                    { "id": "in", "type": "input", "params": {} },
                    { "id": "proc", "type": "image-compress", "params": {} }
                ]
            }"#,
        );
        assert_eq!(resolve_input_mode(&def), InputMode::FileUpload);
    }

    #[test]
    fn test_resolve_input_mode_no_input_node() {
        let def = parse_definition(
            r#"{
                "formatVersion": "1.0.0",
                "nodes": [
                    { "id": "proc", "type": "image-compress", "params": {} },
                    { "id": "out", "type": "output", "params": {} }
                ]
            }"#,
        );
        assert_eq!(resolve_input_mode(&def), InputMode::FileUpload);
    }

    #[test]
    fn test_resolve_input_mode_nested() {
        let def = parse_definition(
            r#"{
                "formatVersion": "1.0.0",
                "nodes": [
                    {
                        "id": "group-1",
                        "type": "group",
                        "params": {},
                        "children": [
                            { "id": "in", "type": "input", "params": { "mode": "url" } },
                            { "id": "proc", "type": "video-download", "params": {} }
                        ]
                    },
                    { "id": "out", "type": "output", "params": {} }
                ]
            }"#,
        );
        assert_eq!(resolve_input_mode(&def), InputMode::Url);
    }

    // --- first_processing_node_id Tests ---

    #[test]
    fn test_first_processing_node_id_simple() {
        let def = parse_definition(
            r#"{
                "formatVersion": "1.0.0",
                "nodes": [
                    { "id": "in", "type": "input", "params": {} },
                    { "id": "compress", "type": "image-compress", "params": {} },
                    { "id": "out", "type": "output", "params": {} }
                ]
            }"#,
        );
        assert_eq!(first_processing_node_id(&def), Some("compress".to_string()));
    }

    #[test]
    fn test_first_processing_node_id_none() {
        let def = parse_definition(
            r#"{
                "formatVersion": "1.0.0",
                "nodes": [
                    { "id": "in", "type": "input", "params": {} },
                    { "id": "out", "type": "output", "params": {} }
                ]
            }"#,
        );
        assert_eq!(first_processing_node_id(&def), None);
    }

    #[test]
    fn test_first_processing_node_id_nested() {
        let def = parse_definition(
            r#"{
                "formatVersion": "1.0.0",
                "nodes": [
                    { "id": "in", "type": "input", "params": {} },
                    {
                        "id": "loop-1",
                        "type": "loop",
                        "params": {},
                        "children": [
                            { "id": "resize", "type": "image-resize", "params": {} },
                            { "id": "compress", "type": "image-compress", "params": {} }
                        ]
                    },
                    { "id": "out", "type": "output", "params": {} }
                ]
            }"#,
        );
        assert_eq!(first_processing_node_id(&def), Some("resize".to_string()));
    }
}
