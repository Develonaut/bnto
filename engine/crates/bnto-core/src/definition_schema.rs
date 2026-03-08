// =============================================================================
// Definition JSON Schema — Validates `.bnto.json` Files
// =============================================================================
//
// WHAT IS THIS FILE?
// This module generates a JSON Schema that describes the structure of a
// `.bnto.json` Definition file. Any consumer (editor, CLI, web app, CI tool)
// can use this schema to validate Definition files WITHOUT reimplementing
// the TypeScript types from `@bnto/nodes`.
//
// WHY DO WE NEED THIS?
// The Definition shape is currently defined in TypeScript (`packages/@bnto/nodes/
// src/definition.ts`). By generating the schema from Rust, the engine becomes
// the single source of truth — the same binary that EXECUTES definitions also
// DESCRIBES their expected shape. No sync issues between TS types and Rust.
//
// HOW IT WORKS:
// We use the `serde_json::json!()` macro to build a JSON Schema object by hand.
// This is simpler than pulling in a schema-generation library — the Definition
// shape is stable and well-known, so a hand-written schema is easy to maintain.
//
// The schema follows JSON Schema Draft 2020-12 (https://json-schema.org/).

use serde_json::Value;

// =============================================================================
// Port Schema — Describes an input or output port on a node
// =============================================================================

/// Build the JSON Schema for a `Port` object.
///
/// A Port is a connection point on a node — either an input or an output.
/// It has an `id`, a `name`, and an optional `handle` string.
///
/// RUST CONCEPT: `fn` returning `Value`
/// This function returns a `serde_json::Value`, which is Rust's representation
/// of an arbitrary JSON value. The `json!()` macro below creates one using
/// a JSON-like syntax that's very readable.
fn port_schema() -> Value {
    serde_json::json!({
        "type": "object",
        "description": "A connection point on a node (input or output).",
        "required": ["id", "name"],
        "properties": {
            "id": {
                "type": "string",
                "description": "Unique identifier for this port within its node."
            },
            "name": {
                "type": "string",
                "description": "Human-readable display name for the port."
            },
            "handle": {
                "type": "string",
                "description": "Optional handle identifier used by the visual editor for positioning."
            }
        },
        // `additionalProperties: false` means no extra keys beyond what we defined.
        // This catches typos (e.g., "naem" instead of "name").
        "additionalProperties": false
    })
}

// =============================================================================
// Edge Schema — Describes a connection between two nodes
// =============================================================================

/// Build the JSON Schema for an `Edge` object.
///
/// An Edge connects an output port of one node to an input port of another.
/// It has `id`, `source`, `target`, and optional handle fields.
fn edge_schema() -> Value {
    serde_json::json!({
        "type": "object",
        "description": "A connection between two nodes in the pipeline graph.",
        "required": ["id", "source", "target"],
        "properties": {
            "id": {
                "type": "string",
                "description": "Unique identifier for this edge."
            },
            "source": {
                "type": "string",
                "description": "The id of the source node (where data flows FROM)."
            },
            "target": {
                "type": "string",
                "description": "The id of the target node (where data flows TO)."
            },
            "sourceHandle": {
                "type": "string",
                "description": "Optional: which output port on the source node."
            },
            "targetHandle": {
                "type": "string",
                "description": "Optional: which input port on the target node."
            }
        },
        "additionalProperties": false
    })
}

// =============================================================================
// Metadata Schema — Describes a node's metadata block
// =============================================================================

/// Build the JSON Schema for the `metadata` object inside a Definition.
///
/// Metadata contains descriptive information about the node — its description,
/// creation timestamp, tags, and an open-ended customData map.
fn metadata_schema() -> Value {
    serde_json::json!({
        "type": "object",
        "description": "Descriptive metadata about the node (description, timestamps, tags).",
        "properties": {
            "description": {
                "type": "string",
                "description": "Human-readable description of what this node does."
            },
            "createdAt": {
                "type": "string",
                "description": "ISO 8601 timestamp of when this node was created."
            },
            "updatedAt": {
                "type": "string",
                "description": "ISO 8601 timestamp of the last modification."
            },
            "tags": {
                "type": "array",
                "description": "Tags for categorization and search.",
                "items": { "type": "string" }
            },
            "customData": {
                "type": "object",
                "description": "Open-ended key-value pairs for consumer-specific data.",
                "additionalProperties": { "type": "string" }
            }
        },
        // Metadata doesn't require any specific fields — all are optional.
        "additionalProperties": false
    })
}

// =============================================================================
// Fields Schema — Describes the fields block for edit-fields nodes
// =============================================================================

/// Build the JSON Schema for the optional `fields` block.
///
/// The `fields` object contains a `values` map (field name → value) and an
/// optional `keepOnlySet` boolean that controls whether unmentioned fields
/// are passed through or dropped.
fn fields_schema() -> Value {
    serde_json::json!({
        "type": "object",
        "description": "Field values for edit-fields nodes.",
        "required": ["values"],
        "properties": {
            "values": {
                "type": "object",
                "description": "Map of field names to their values.",
                "additionalProperties": true
            },
            "keepOnlySet": {
                "type": "boolean",
                "description": "If true, only fields listed in `values` are kept in the output."
            }
        },
        "additionalProperties": false
    })
}

// =============================================================================
// Definition Schema — The top-level `.bnto.json` structure
// =============================================================================

/// Generate a JSON Schema for the `.bnto.json` Definition format.
///
/// This is the main public function of this module. It returns a complete
/// JSON Schema (Draft 2020-12) that describes the structure of a Definition
/// node, including its recursive `nodes` array.
///
/// RUST CONCEPT: `pub fn` = public function
/// `pub` means this function can be called from outside this module.
/// Other crates (like `bnto-wasm`) import it to include in the catalog.
///
/// RUST CONCEPT: `serde_json::json!()` macro
/// The `json!()` macro lets you write JSON using a Rust-like syntax.
/// It creates a `serde_json::Value` at runtime. It's like writing
/// `JSON.parse('...')` in JavaScript, but checked at compile time.
///
/// # Returns
/// A `serde_json::Value` containing the full JSON Schema object.
pub fn definition_json_schema() -> Value {
    // --- Build the main Definition object schema ---
    //
    // A Definition is recursive — it can contain child `nodes` that are
    // themselves Definitions. JSON Schema handles this with `$ref` and
    // `$defs` (definitions block). We define the Definition shape once
    // in `$defs/Definition` and reference it from both the top level
    // and the `nodes` array items.

    serde_json::json!({
        // --- Schema metadata ---
        // `$schema` declares which version of JSON Schema we're using.
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "title": "Bnto Definition",
        "description": "A .bnto.json recipe definition. Describes a pipeline of nodes that process data.",

        // --- The top-level Definition is a reference to our shared definition ---
        //
        // CONCEPT: `$ref`
        // Instead of duplicating the Definition shape, we define it once in
        // `$defs/Definition` and reference it with `$ref`. This is especially
        // important because Definition is RECURSIVE (nodes contain Definitions).
        "$ref": "#/$defs/Definition",

        // --- Shared type definitions ---
        //
        // CONCEPT: `$defs`
        // This is where we define reusable schemas. Any schema in here can
        // be referenced from anywhere else using `$ref: "#/$defs/TypeName"`.
        "$defs": {
            "Port": port_schema(),
            "Edge": edge_schema(),
            "Metadata": metadata_schema(),
            "Fields": fields_schema(),

            // --- The Definition schema (the main event) ---
            "Definition": {
                "type": "object",
                "description": "A single node in a .bnto.json recipe. Can contain child nodes (recursive).",
                "required": ["id", "type", "version", "name", "position", "metadata", "parameters", "inputPorts", "outputPorts"],
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "Unique identifier for this node within the recipe."
                    },
                    "type": {
                        "type": "string",
                        "description": "The node type (e.g., 'image', 'spreadsheet', 'file-system', 'input', 'output')."
                    },
                    "version": {
                        "type": "string",
                        "description": "The format version of this definition (semver, e.g., '1.0.0')."
                    },
                    "parentId": {
                        "type": "string",
                        "description": "Optional: the id of the parent node (for nested nodes inside groups/loops)."
                    },
                    "name": {
                        "type": "string",
                        "description": "Human-readable name for this node."
                    },
                    "position": {
                        "type": "object",
                        "description": "The node's position on the visual editor canvas.",
                        "required": ["x", "y"],
                        "properties": {
                            "x": { "type": "number" },
                            "y": { "type": "number" }
                        },
                        "additionalProperties": false
                    },
                    "metadata": {
                        "$ref": "#/$defs/Metadata"
                    },
                    "parameters": {
                        "type": "object",
                        "description": "Configuration parameters for this node (key-value pairs).",
                        "additionalProperties": true
                    },
                    "fields": {
                        "$ref": "#/$defs/Fields"
                    },
                    "inputPorts": {
                        "type": "array",
                        "description": "Input connection ports for this node.",
                        "items": { "$ref": "#/$defs/Port" }
                    },
                    "outputPorts": {
                        "type": "array",
                        "description": "Output connection ports for this node.",
                        "items": { "$ref": "#/$defs/Port" }
                    },
                    "nodes": {
                        "type": "array",
                        "description": "Child nodes (for container nodes like group, loop, parallel). Recursive.",
                        "items": { "$ref": "#/$defs/Definition" }
                    },
                    "edges": {
                        "type": "array",
                        "description": "Connections between child nodes.",
                        "items": { "$ref": "#/$defs/Edge" }
                    }
                },
                // Allow additional properties for forward compatibility —
                // new fields added to Definition won't break older validators.
                "additionalProperties": true
            }
        }
    })
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_definition_schema_is_valid_json_object() {
        // The schema should be a valid JSON object (not null, not an array).
        let schema = definition_json_schema();
        assert!(schema.is_object(), "Schema should be a JSON object");
    }

    #[test]
    fn test_definition_schema_has_correct_meta_fields() {
        // Verify the schema metadata fields are present and correct.
        let schema = definition_json_schema();
        assert_eq!(
            schema["$schema"],
            "https://json-schema.org/draft/2020-12/schema"
        );
        assert_eq!(schema["title"], "Bnto Definition");
        assert!(schema["description"].is_string());
    }

    #[test]
    fn test_definition_schema_uses_ref_to_defs() {
        // The top-level schema should reference $defs/Definition.
        let schema = definition_json_schema();
        assert_eq!(schema["$ref"], "#/$defs/Definition");
    }

    #[test]
    fn test_definition_schema_has_all_defs() {
        // The $defs block should contain all 5 type definitions.
        let schema = definition_json_schema();
        let defs = schema["$defs"].as_object().expect("$defs should be an object");
        assert!(defs.contains_key("Definition"), "Missing Definition in $defs");
        assert!(defs.contains_key("Port"), "Missing Port in $defs");
        assert!(defs.contains_key("Edge"), "Missing Edge in $defs");
        assert!(defs.contains_key("Metadata"), "Missing Metadata in $defs");
        assert!(defs.contains_key("Fields"), "Missing Fields in $defs");
    }

    #[test]
    fn test_definition_has_required_properties() {
        // The Definition schema should list the correct required fields.
        let schema = definition_json_schema();
        let def = &schema["$defs"]["Definition"];
        let required = def["required"]
            .as_array()
            .expect("required should be an array");

        // Convert to strings for comparison.
        let required_strs: Vec<&str> = required
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect();

        assert!(required_strs.contains(&"id"));
        assert!(required_strs.contains(&"type"));
        assert!(required_strs.contains(&"version"));
        assert!(required_strs.contains(&"name"));
        assert!(required_strs.contains(&"position"));
        assert!(required_strs.contains(&"metadata"));
        assert!(required_strs.contains(&"parameters"));
        assert!(required_strs.contains(&"inputPorts"));
        assert!(required_strs.contains(&"outputPorts"));
    }

    #[test]
    fn test_definition_has_recursive_nodes() {
        // The `nodes` field should reference $defs/Definition (recursive).
        let schema = definition_json_schema();
        let nodes_items = &schema["$defs"]["Definition"]["properties"]["nodes"]["items"];
        assert_eq!(
            nodes_items["$ref"], "#/$defs/Definition",
            "nodes items should reference Definition recursively"
        );
    }

    #[test]
    fn test_port_schema_has_required_fields() {
        // Port should require id and name.
        let schema = definition_json_schema();
        let port = &schema["$defs"]["Port"];
        let required = port["required"]
            .as_array()
            .expect("Port required should be an array");
        let required_strs: Vec<&str> = required
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect();
        assert!(required_strs.contains(&"id"));
        assert!(required_strs.contains(&"name"));
    }

    #[test]
    fn test_edge_schema_has_required_fields() {
        // Edge should require id, source, and target.
        let schema = definition_json_schema();
        let edge = &schema["$defs"]["Edge"];
        let required = edge["required"]
            .as_array()
            .expect("Edge required should be an array");
        let required_strs: Vec<&str> = required
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect();
        assert!(required_strs.contains(&"id"));
        assert!(required_strs.contains(&"source"));
        assert!(required_strs.contains(&"target"));
    }
}
