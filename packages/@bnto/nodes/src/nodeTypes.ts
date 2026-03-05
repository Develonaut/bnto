/**
 * Node type registry — the canonical list of all node types and their metadata.
 *
 * This is the TypeScript equivalent of Go's `registry.DefaultRegistry()`.
 * It defines what each node type does, what category it belongs to,
 * and whether it can contain child nodes.
 *
 * Go source: engine/pkg/api/service.go → DefaultRegistry()
 */

/**
 * All registered node type names.
 *
 * Derived from `NODE_TYPES` constant — a single value that
 * produces both the runtime constant and the type.
 */
export const NODE_TYPES = {
  editFields: "edit-fields",
  fileSystem: "file-system",
  group: "group",
  httpRequest: "http-request",
  image: "image",
  input: "input",
  loop: "loop",
  output: "output",
  parallel: "parallel",
  shellCommand: "shell-command",
  spreadsheet: "spreadsheet",
  transform: "transform",
} as const;

/** Union of all valid node type name strings. */
export type NodeTypeName = (typeof NODE_TYPES)[keyof typeof NODE_TYPES];

/** All node type names as a readonly array. */
export const NODE_TYPE_NAMES: readonly NodeTypeName[] = Object.values(NODE_TYPES) as NodeTypeName[];

/** Node category for grouping in the UI and documentation. */
export type NodeCategory =
  | "data"
  | "file"
  | "image"
  | "io"
  | "network"
  | "spreadsheet"
  | "control"
  | "system";

/** Metadata describing a node type's behavior and capabilities. */
export interface NodeTypeInfo {
  /** The node type name as used in `.bnto.json` definitions. */
  name: NodeTypeName;

  /** Human-readable display label. */
  label: string;

  /** One-sentence description of what the node does. */
  description: string;

  /** Category for grouping. */
  category: NodeCategory;

  /** Whether this node can contain child nodes (group, loop). */
  isContainer: boolean;

  /**
   * Whether this node type is available for browser execution.
   *
   * Server-only nodes (shell-command, http-request with CORS issues)
   * are not available in browser context.
   */
  browserCapable: boolean;

  /**
   * Icon identifier — a Lucide icon name for visual consumers.
   *
   * Pure string metadata (no React/Lucide dependency). Consumers
   * resolve this to an actual icon component using their own icon system.
   * e.g. "image" → ImageIcon, "table" → TableIcon
   */
  icon: string;
}

/**
 * Metadata for all 12 registered node types.
 *
 * Maps node type name → info. The engine owns the implementations;
 * this metadata describes capabilities without coupling to any runtime.
 */
export const NODE_TYPE_INFO: Record<NodeTypeName, NodeTypeInfo> = {
  "edit-fields": {
    name: "edit-fields",
    label: "Edit Fields",
    description: "Set field values from static values or template expressions.",
    category: "data",
    isContainer: false,
    browserCapable: true,
    icon: "pen-line",
  },
  "file-system": {
    name: "file-system",
    label: "File System",
    description: "File operations: read, write, copy, move, delete, mkdir, exists, list.",
    category: "file",
    isContainer: false,
    browserCapable: false,
    icon: "folder-open",
  },
  group: {
    name: "group",
    label: "Group",
    description: "Container for child nodes. Orchestrates sequential or parallel execution.",
    category: "control",
    isContainer: true,
    browserCapable: true,
    icon: "braces",
  },
  "http-request": {
    name: "http-request",
    label: "HTTP Request",
    description: "Make HTTP requests to APIs (GET, POST, PUT, DELETE, etc.).",
    category: "network",
    isContainer: false,
    browserCapable: false,
    icon: "globe",
  },
  image: {
    name: "image",
    label: "Image",
    description: "Image processing: resize, convert formats, optimize, composite, batch.",
    category: "image",
    isContainer: false,
    browserCapable: true,
    icon: "image",
  },
  input: {
    name: "input",
    label: "Input",
    description:
      "Declares how data enters the recipe. Read by the environment to render the appropriate input widget.",
    category: "io",
    isContainer: false,
    browserCapable: true,
    icon: "upload",
  },
  loop: {
    name: "loop",
    label: "Loop",
    description: "Iterate over arrays (forEach), repeat N times, or loop while condition.",
    category: "control",
    isContainer: true,
    browserCapable: true,
    icon: "refresh-cw",
  },
  output: {
    name: "output",
    label: "Output",
    description:
      "Declares how results are delivered. Read by the environment to render the appropriate output widget.",
    category: "io",
    isContainer: false,
    browserCapable: true,
    icon: "download",
  },
  parallel: {
    name: "parallel",
    label: "Parallel",
    description: "Execute tasks concurrently with configurable worker pool and error strategy.",
    category: "control",
    isContainer: true,
    browserCapable: true,
    icon: "columns-3",
  },
  "shell-command": {
    name: "shell-command",
    label: "Shell Command",
    description: "Execute shell commands with stall detection, retry, and streaming output.",
    category: "system",
    isContainer: false,
    browserCapable: false,
    icon: "terminal-square",
  },
  spreadsheet: {
    name: "spreadsheet",
    label: "Spreadsheet",
    description: "Read and write CSV or Excel files.",
    category: "spreadsheet",
    isContainer: false,
    browserCapable: true,
    icon: "table",
  },
  transform: {
    name: "transform",
    label: "Transform",
    description: "Transform data using expressions (single value) or field mappings.",
    category: "data",
    isContainer: false,
    browserCapable: true,
    icon: "shuffle",
  },
} as const satisfies Record<NodeTypeName, NodeTypeInfo>;

// Functions live in individual files per one-export-per-file rule:
// isNodeType.ts, getNodeTypeInfo.ts, getBrowserCapableTypes.ts, getContainerTypes.ts
// Import them directly or via the package barrel (index.ts).
