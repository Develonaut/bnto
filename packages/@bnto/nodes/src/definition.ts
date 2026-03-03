/**
 * Core recipe definition types — 1:1 mapping with Go engine's `node` package.
 *
 * These types represent the JSON-serializable structure of `.bnto.json` files.
 * Every execution target (Rust WASM, JS, Go, desktop) reads and validates
 * against these types.
 *
 * Go source: engine/pkg/node/definition.go
 */

/**
 * A single node in a bnto recipe definition.
 *
 * Each node has a unique ID, a type (e.g., "http-request", "image"),
 * type-specific parameters, and input/output ports for connecting to
 * other nodes.
 *
 * Definitions can be nested — group and loop nodes contain child `nodes`
 * and `edges`, forming a recursive tree structure.
 */
export interface Definition {
  /** Unique identifier within the definition. */
  id: string;

  /** Node type (e.g., "http-request", "image", "loop"). */
  type: string;

  /** Schema version for compatibility (e.g., "1.0.0"). */
  version: string;

  /** Parent group ID if this node is nested inside a group. */
  parentId?: string;

  /** Human-readable name. */
  name: string;

  /** Visual editor position (pixels). Zero values for CLI-only usage. */
  position: Position;

  /** Additional metadata (description, tags, timestamps). */
  metadata: Metadata;

  /** Node-specific configuration. Keys and value types depend on the node type. */
  parameters: Record<string, unknown>;

  /** Field configuration for edit-fields nodes. */
  fields?: FieldsConfig;

  /** Input connection points. */
  inputPorts: Port[];

  /** Output connection points. */
  outputPorts: Port[];

  /** Child nodes (for group and loop nodes). */
  nodes?: Definition[];

  /** Connections between child nodes. */
  edges?: Edge[];
}

/**
 * Visual location of a node in the editor canvas.
 *
 * Go source: engine/pkg/node/definition.go → Position
 */
export interface Position {
  /** X coordinate in pixels. */
  x: number;

  /** Y coordinate in pixels. */
  y: number;
}

/**
 * Additional node metadata — extensible without changing the core Definition.
 *
 * Go source: engine/pkg/node/definition.go → Metadata
 */
export interface Metadata {
  /** Human-readable description. */
  description?: string;

  /** ISO 8601 creation timestamp. */
  createdAt?: string;

  /** ISO 8601 last-updated timestamp. */
  updatedAt?: string;

  /** User-defined tags. */
  tags?: string[];

  /** Custom key-value pairs. */
  customData?: Record<string, string>;
}

/**
 * An input or output connection point on a node.
 *
 * Ports are where edges attach. A node can have multiple input and
 * output ports for complex data flow patterns.
 *
 * Go source: engine/pkg/node/definition.go → Port
 */
export interface Port {
  /** Unique port identifier within the node. */
  id: string;

  /** Human-readable port name (e.g., "items", "rows", "files"). */
  name: string;

  /** Handle type for the visual editor (e.g., "source", "target"). */
  handle?: string;
}

/**
 * A connection between two nodes defining data flow.
 *
 * Data flows from the source node's output port to the target
 * node's input port.
 *
 * Go source: engine/pkg/node/definition.go → Edge
 */
export interface Edge {
  /** Unique edge identifier. */
  id: string;

  /** Source node ID. */
  source: string;

  /** Target node ID. */
  target: string;

  /** Source port handle. */
  sourceHandle?: string;

  /** Target port handle. */
  targetHandle?: string;
}

/**
 * Field editor configuration for edit-fields nodes.
 *
 * Values can be static or use Go template expressions (e.g., `{{.item}}`).
 *
 * Go source: engine/pkg/node/definition.go → FieldsConfig
 */
export interface FieldsConfig {
  /** Field values — static values or template strings. */
  values: Record<string, unknown>;

  /** When true, only output fields that are explicitly set. */
  keepOnlySet?: boolean;
}
