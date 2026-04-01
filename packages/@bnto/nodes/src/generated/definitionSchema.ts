/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
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
  $defs: {
    Definition: {
      additionalProperties: true,
      description: "A single node in a .bnto.json recipe. Can contain child nodes (recursive).",
      properties: {
        edges: {
          description: "Connections between child nodes.",
          items: {
            $ref: "#/$defs/Edge",
          },
          type: "array",
        },
        fields: {
          $ref: "#/$defs/Fields",
        },
        id: {
          description: "Unique identifier for this node within the recipe.",
          type: "string",
        },
        inputPorts: {
          description: "Input connection ports.",
          items: {
            $ref: "#/$defs/Port",
          },
          type: "array",
        },
        metadata: {
          $ref: "#/$defs/Metadata",
        },
        name: {
          description: "Human-readable name for this node.",
          type: "string",
        },
        nodes: {
          description: "Child nodes (recursive).",
          items: {
            $ref: "#/$defs/Definition",
          },
          type: "array",
        },
        outputPorts: {
          description: "Output connection ports.",
          items: {
            $ref: "#/$defs/Port",
          },
          type: "array",
        },
        parameters: {
          additionalProperties: true,
          description: "Configuration parameters (key-value pairs).",
          type: "object",
        },
        parentId: {
          description: "Optional parent node id (for nested nodes).",
          type: "string",
        },
        position: {
          additionalProperties: false,
          description: "The node's position on the visual editor canvas.",
          properties: {
            x: {
              type: "number",
            },
            y: {
              type: "number",
            },
          },
          required: ["x", "y"],
          type: "object",
        },
        settings: {
          $ref: "#/$defs/PipelineSettings",
        },
        type: {
          description:
            "The node type (e.g., 'image-compress', 'spreadsheet-clean', 'file-rename').",
          type: "string",
        },
        version: {
          description: "Format version (semver, e.g., '1.0.0').",
          type: "string",
        },
      },
      required: [
        "id",
        "type",
        "version",
        "name",
        "position",
        "metadata",
        "parameters",
        "inputPorts",
        "outputPorts",
      ],
      type: "object",
    },
    Edge: {
      additionalProperties: false,
      description: "A connection between two nodes in the pipeline graph.",
      properties: {
        id: {
          description: "Unique identifier for this edge.",
          type: "string",
        },
        source: {
          description: "The id of the source node (where data flows FROM).",
          type: "string",
        },
        sourceHandle: {
          description: "Optional: which output port on the source node.",
          type: "string",
        },
        target: {
          description: "The id of the target node (where data flows TO).",
          type: "string",
        },
        targetHandle: {
          description: "Optional: which input port on the target node.",
          type: "string",
        },
      },
      required: ["id", "source", "target"],
      type: "object",
    },
    Fields: {
      additionalProperties: false,
      description: "Field values for edit-fields nodes.",
      properties: {
        keepOnlySet: {
          description: "If true, only fields listed in `values` are kept in the output.",
          type: "boolean",
        },
        values: {
          additionalProperties: true,
          description: "Map of field names to their values.",
          type: "object",
        },
      },
      required: ["values"],
      type: "object",
    },
    Metadata: {
      additionalProperties: false,
      description: "Descriptive metadata about the node (description, timestamps, tags).",
      properties: {
        createdAt: {
          description: "ISO 8601 timestamp of when this node was created.",
          type: "string",
        },
        customData: {
          additionalProperties: {
            type: "string",
          },
          description: "Open-ended key-value pairs for consumer-specific data.",
          type: "object",
        },
        description: {
          description: "Human-readable description of what this node does.",
          type: "string",
        },
        tags: {
          description: "Tags for categorization and search.",
          items: {
            type: "string",
          },
          type: "array",
        },
        updatedAt: {
          description: "ISO 8601 timestamp of the last modification.",
          type: "string",
        },
      },
      type: "object",
    },
    PipelineSettings: {
      additionalProperties: false,
      description: "Recipe-level settings that control execution behavior.",
      properties: {
        iteration: {
          default: "explicit",
          description:
            "How the executor handles iteration over multiple input files. 'auto' wraps primitive sequences in implicit per-file loops; 'explicit' (default) executes exactly what's defined.",
          enum: ["auto", "explicit"],
          type: "string",
        },
      },
      type: "object",
    },
    Port: {
      additionalProperties: false,
      description: "A connection point on a node (input or output).",
      properties: {
        handle: {
          description: "Optional handle identifier used by the visual editor for positioning.",
          type: "string",
        },
        id: {
          description: "Unique identifier for this port within its node.",
          type: "string",
        },
        name: {
          description: "Human-readable display name for the port.",
          type: "string",
        },
      },
      required: ["id", "name"],
      type: "object",
    },
  },
  $ref: "#/$defs/Definition",
  $schema: "https://json-schema.org/draft/2020-12/schema",
  description: "A .bnto.json recipe definition. Describes a pipeline of nodes that process data.",
  title: "Bnto Definition",
} as const;
