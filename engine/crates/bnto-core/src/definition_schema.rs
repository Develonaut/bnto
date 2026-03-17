// Definition JSON Schema — validates `.bnto.json` files.
// Generates a JSON Schema (Draft 2020-12) describing the Definition structure
// so any consumer can validate recipe files without reimplementing TS types.

use serde_json::Value;

// =============================================================================
// Port Schema
// =============================================================================

/// JSON Schema for a Port (input or output connection point on a node).
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
        "additionalProperties": false
    })
}

// =============================================================================
// Edge Schema
// =============================================================================

/// JSON Schema for an Edge (connection between two nodes).
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
// Metadata Schema
// =============================================================================

/// JSON Schema for the metadata block inside a Definition.
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
        "additionalProperties": false
    })
}

// =============================================================================
// Fields Schema
// =============================================================================

/// JSON Schema for the optional `fields` block (edit-fields nodes).
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
// Definition Schema
// =============================================================================

/// Generate a JSON Schema (Draft 2020-12) for the `.bnto.json` Definition format.
///
/// The Definition is recursive — child `nodes` are themselves Definitions.
/// Uses `$ref` + `$defs` to handle the recursion.
pub fn definition_json_schema() -> Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "title": "Bnto Definition",
        "description": "A .bnto.json recipe definition. Describes a pipeline of nodes that process data.",

        "$ref": "#/$defs/Definition",

        "$defs": {
            "Port": port_schema(),
            "Edge": edge_schema(),
            "Metadata": metadata_schema(),
            "Fields": fields_schema(),

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
                // Allow additional properties for forward compatibility.
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
        let schema = definition_json_schema();
        assert!(schema.is_object(), "Schema should be a JSON object");
    }

    #[test]
    fn test_definition_schema_has_correct_meta_fields() {
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
        let schema = definition_json_schema();
        assert_eq!(schema["$ref"], "#/$defs/Definition");
    }

    #[test]
    fn test_definition_schema_has_all_defs() {
        let schema = definition_json_schema();
        let defs = schema["$defs"]
            .as_object()
            .expect("$defs should be an object");
        assert!(
            defs.contains_key("Definition"),
            "Missing Definition in $defs"
        );
        assert!(defs.contains_key("Port"), "Missing Port in $defs");
        assert!(defs.contains_key("Edge"), "Missing Edge in $defs");
        assert!(defs.contains_key("Metadata"), "Missing Metadata in $defs");
        assert!(defs.contains_key("Fields"), "Missing Fields in $defs");
    }

    #[test]
    fn test_definition_has_required_properties() {
        let schema = definition_json_schema();
        let def = &schema["$defs"]["Definition"];
        let required = def["required"]
            .as_array()
            .expect("required should be an array");

        let required_strs: Vec<&str> = required.iter().map(|v| v.as_str().unwrap()).collect();

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
        let schema = definition_json_schema();
        let nodes_items = &schema["$defs"]["Definition"]["properties"]["nodes"]["items"];
        assert_eq!(
            nodes_items["$ref"], "#/$defs/Definition",
            "nodes items should reference Definition recursively"
        );
    }

    #[test]
    fn test_port_schema_has_required_fields() {
        let schema = definition_json_schema();
        let port = &schema["$defs"]["Port"];
        let required = port["required"]
            .as_array()
            .expect("Port required should be an array");
        let required_strs: Vec<&str> = required.iter().map(|v| v.as_str().unwrap()).collect();
        assert!(required_strs.contains(&"id"));
        assert!(required_strs.contains(&"name"));
    }

    #[test]
    fn test_edge_schema_has_required_fields() {
        let schema = definition_json_schema();
        let edge = &schema["$defs"]["Edge"];
        let required = edge["required"]
            .as_array()
            .expect("Edge required should be an array");
        let required_strs: Vec<&str> = required.iter().map(|v| v.as_str().unwrap()).collect();
        assert!(required_strs.contains(&"id"));
        assert!(required_strs.contains(&"source"));
        assert!(required_strs.contains(&"target"));
    }
}
