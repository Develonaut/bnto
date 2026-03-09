/**
 * AUTO-GENERATED from engine/catalog.snapshot.json — DO NOT EDIT.
 *
 * JSON Schema (Draft 2020-12) for `.bnto.json` definition files.
 * Use this to validate definition files in any language or tool
 * that supports JSON Schema (VS Code, ajv, jsonschema, etc.).
 *
 * Run `task nodes:generate` to regenerate after engine changes.
 *
 * Engine catalog v1.0.0
 */

/** JSON Schema for .bnto.json definition files. */
export const DEFINITION_JSON_SCHEMA = {
  "$defs": {
    "Definition": {
      "additionalProperties": true,
      "description": "A single node in a .bnto.json recipe. Can contain child nodes (recursive).",
      "properties": {
        "edges": {
          "description": "Connections between child nodes.",
          "items": {
            "$ref": "#/$defs/Edge"
          },
          "type": "array"
        },
        "fields": {
          "$ref": "#/$defs/Fields"
        },
        "id": {
          "description": "Unique identifier for this node within the recipe.",
          "type": "string"
        },
        "inputPorts": {
          "description": "Input connection ports for this node.",
          "items": {
            "$ref": "#/$defs/Port"
          },
          "type": "array"
        },
        "metadata": {
          "$ref": "#/$defs/Metadata"
        },
        "name": {
          "description": "Human-readable name for this node.",
          "type": "string"
        },
        "nodes": {
          "description": "Child nodes (for container nodes like group, loop, parallel). Recursive.",
          "items": {
            "$ref": "#/$defs/Definition"
          },
          "type": "array"
        },
        "outputPorts": {
          "description": "Output connection ports for this node.",
          "items": {
            "$ref": "#/$defs/Port"
          },
          "type": "array"
        },
        "parameters": {
          "additionalProperties": true,
          "description": "Configuration parameters for this node (key-value pairs).",
          "type": "object"
        },
        "parentId": {
          "description": "Optional: the id of the parent node (for nested nodes inside groups/loops).",
          "type": "string"
        },
        "position": {
          "additionalProperties": false,
          "description": "The node's position on the visual editor canvas.",
          "properties": {
            "x": {
              "type": "number"
            },
            "y": {
              "type": "number"
            }
          },
          "required": [
            "x",
            "y"
          ],
          "type": "object"
        },
        "type": {
          "description": "The node type (e.g., 'image', 'spreadsheet', 'file-system', 'input', 'output').",
          "type": "string"
        },
        "version": {
          "description": "The format version of this definition (semver, e.g., '1.0.0').",
          "type": "string"
        }
      },
      "required": [
        "id",
        "type",
        "version",
        "name",
        "position",
        "metadata",
        "parameters",
        "inputPorts",
        "outputPorts"
      ],
      "type": "object"
    },
    "Edge": {
      "additionalProperties": false,
      "description": "A connection between two nodes in the pipeline graph.",
      "properties": {
        "id": {
          "description": "Unique identifier for this edge.",
          "type": "string"
        },
        "source": {
          "description": "The id of the source node (where data flows FROM).",
          "type": "string"
        },
        "sourceHandle": {
          "description": "Optional: which output port on the source node.",
          "type": "string"
        },
        "target": {
          "description": "The id of the target node (where data flows TO).",
          "type": "string"
        },
        "targetHandle": {
          "description": "Optional: which input port on the target node.",
          "type": "string"
        }
      },
      "required": [
        "id",
        "source",
        "target"
      ],
      "type": "object"
    },
    "Fields": {
      "additionalProperties": false,
      "description": "Field values for edit-fields nodes.",
      "properties": {
        "keepOnlySet": {
          "description": "If true, only fields listed in `values` are kept in the output.",
          "type": "boolean"
        },
        "values": {
          "additionalProperties": true,
          "description": "Map of field names to their values.",
          "type": "object"
        }
      },
      "required": [
        "values"
      ],
      "type": "object"
    },
    "Metadata": {
      "additionalProperties": false,
      "description": "Descriptive metadata about the node (description, timestamps, tags).",
      "properties": {
        "createdAt": {
          "description": "ISO 8601 timestamp of when this node was created.",
          "type": "string"
        },
        "customData": {
          "additionalProperties": {
            "type": "string"
          },
          "description": "Open-ended key-value pairs for consumer-specific data.",
          "type": "object"
        },
        "description": {
          "description": "Human-readable description of what this node does.",
          "type": "string"
        },
        "tags": {
          "description": "Tags for categorization and search.",
          "items": {
            "type": "string"
          },
          "type": "array"
        },
        "updatedAt": {
          "description": "ISO 8601 timestamp of the last modification.",
          "type": "string"
        }
      },
      "type": "object"
    },
    "Port": {
      "additionalProperties": false,
      "description": "A connection point on a node (input or output).",
      "properties": {
        "handle": {
          "description": "Optional handle identifier used by the visual editor for positioning.",
          "type": "string"
        },
        "id": {
          "description": "Unique identifier for this port within its node.",
          "type": "string"
        },
        "name": {
          "description": "Human-readable display name for the port.",
          "type": "string"
        }
      },
      "required": [
        "id",
        "name"
      ],
      "type": "object"
    }
  },
  "$ref": "#/$defs/Definition",
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "description": "A .bnto.json recipe definition. Describes a pipeline of nodes that process data.",
  "title": "Bnto Definition"
} as const;
