// Definition JSON Schema — Validates `.bnto.json` files.
//
// Generates a JSON Schema (Draft 2020-12) describing the Definition format.
// Built by hand with `serde_json::json!()` — simpler than a codegen library
// since the Definition shape is stable. Makes the engine the single source
// of truth for both execution and validation.

use serde_json::Value;

/// Build the JSON Schema for a `Port` object (connection point on a node).
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

/// Build the JSON Schema for an `Edge` (connection between two nodes).
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

/// Build the JSON Schema for the `metadata` object (description, timestamps, tags).
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

/// Build the JSON Schema for the optional `fields` block (edit-fields nodes).
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

/// Build the JSON Schema for `PipelineSettings` (recipe-level configuration).
fn pipeline_settings_schema() -> Value {
    serde_json::json!({
        "type": "object",
        "description": "Recipe-level settings that control execution behavior.",
        "properties": {
            "iteration": {
                "type": "string",
                "description": "How the executor handles iteration over multiple input files. 'auto' wraps primitive sequences in implicit per-file loops; 'explicit' (default) executes exactly what's defined.",
                "enum": ["auto", "explicit"],
                "default": "explicit"
            }
        },
        "additionalProperties": false
    })
}

// --- Definition Schema ---

/// Build the JSON Schema for a canvas position (`{ x, y }`).
fn position_schema() -> Value {
    serde_json::json!({
        "type": "object",
        "description": "The node's position on the visual editor canvas.",
        "required": ["x", "y"],
        "properties": {
            "x": { "type": "number" },
            "y": { "type": "number" }
        },
        "additionalProperties": false
    })
}

/// Build the properties map for the Definition schema.
fn definition_properties() -> Value {
    serde_json::json!({
        "id":         { "type": "string", "description": "Unique identifier for this node within the recipe." },
        "type":       { "type": "string", "description": "The node type (e.g., 'image-compress', 'spreadsheet-clean', 'file-rename')." },
        "version":    { "type": "string", "description": "Format version (semver, e.g., '1.0.0')." },
        "parentId":   { "type": "string", "description": "Optional parent node id (for nested nodes)." },
        "name":       { "type": "string", "description": "Human-readable name for this node." },
        "position":   position_schema(),
        "metadata":   { "$ref": "#/$defs/Metadata" },
        "parameters": { "type": "object", "description": "Configuration parameters (key-value pairs).", "additionalProperties": true },
        "fields":     { "$ref": "#/$defs/Fields" },
        "inputPorts": { "type": "array", "description": "Input connection ports.", "items": { "$ref": "#/$defs/Port" } },
        "outputPorts":{ "type": "array", "description": "Output connection ports.", "items": { "$ref": "#/$defs/Port" } },
        "settings":   { "$ref": "#/$defs/PipelineSettings" },
        "nodes":      { "type": "array", "description": "Child nodes (recursive).", "items": { "$ref": "#/$defs/Definition" } },
        "edges":      { "type": "array", "description": "Connections between child nodes.", "items": { "$ref": "#/$defs/Edge" } }
    })
}

/// Generate a JSON Schema (Draft 2020-12) for the `.bnto.json` Definition format.
///
/// The Definition is recursive — child `nodes` reference the same schema via `$ref`.
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
            "PipelineSettings": pipeline_settings_schema(),
            "Definition": {
                "type": "object",
                "description": "A single node in a .bnto.json recipe. Can contain child nodes (recursive).",
                "required": ["id", "type", "version", "name", "position", "metadata", "parameters", "inputPorts", "outputPorts"],
                "properties": definition_properties(),
                "additionalProperties": true
            }
        }
    })
}

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
        // The $defs block should contain all 6 type definitions.
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
        assert!(
            defs.contains_key("PipelineSettings"),
            "Missing PipelineSettings in $defs"
        );
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
        let required_strs: Vec<&str> = required.iter().map(|v| v.as_str().unwrap()).collect();
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
        let required_strs: Vec<&str> = required.iter().map(|v| v.as_str().unwrap()).collect();
        assert!(required_strs.contains(&"id"));
        assert!(required_strs.contains(&"source"));
        assert!(required_strs.contains(&"target"));
    }

    #[test]
    fn test_definition_schema_has_settings_property() {
        let schema = definition_json_schema();
        let props = schema["$defs"]["Definition"]["properties"]
            .as_object()
            .expect("Definition properties should be an object");
        assert!(
            props.contains_key("settings"),
            "Definition should have a settings property"
        );
        assert_eq!(
            props["settings"]["$ref"], "#/$defs/PipelineSettings",
            "settings should reference PipelineSettings"
        );
    }

    #[test]
    fn test_settings_schema_has_iteration_enum() {
        let schema = definition_json_schema();
        let settings = &schema["$defs"]["PipelineSettings"];
        let iteration = &settings["properties"]["iteration"];
        let enum_values = iteration["enum"]
            .as_array()
            .expect("iteration should have an enum");
        let values: Vec<&str> = enum_values.iter().map(|v| v.as_str().unwrap()).collect();
        assert!(values.contains(&"auto"));
        assert!(values.contains(&"explicit"));
        assert_eq!(iteration["default"], "explicit");
    }

    #[test]
    fn test_definition_schema_has_pipeline_settings_def() {
        let schema = definition_json_schema();
        let defs = schema["$defs"]
            .as_object()
            .expect("$defs should be an object");
        let settings = defs
            .get("PipelineSettings")
            .expect("PipelineSettings should exist in $defs");
        assert_eq!(settings["type"], "object");
        assert!(
            settings["properties"]["iteration"].is_object(),
            "PipelineSettings should have an iteration property"
        );
    }
}
