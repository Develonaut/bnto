/**
 * AUTO-GENERATED from engine/catalog.snapshot.json — DO NOT EDIT.
 *
 * Run `task nodes:generate` to regenerate after engine changes.
 * Source: engine/crates/bnto-core/src/metadata.rs
 *
 * The Rust engine is the single source of truth for all node metadata.
 * To add or modify a node type, edit `all_node_types()` in the engine,
 * then run `task wasm:build` → `task nodes:generate`.
 *
 * Engine catalog v1.0.0
 */

// =============================================================================
// Node Types — All 12 registered node types
// =============================================================================

/**
 * All registered node type names.
 * Maps camelCase keys to kebab-case type name strings.
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
  | "control"
  | "data"
  | "file"
  | "image"
  | "io"
  | "network"
  | "spreadsheet"
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
  /** Whether this node type is available for browser execution. */
  browserCapable: boolean;
  /** Lucide icon name for visual consumers. */
  icon: string;
}

/**
 * Metadata for all 12 registered node types.
 * Maps node type name → info.
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
    description: "File operations: rename, copy, move, delete, mkdir, exists, list.",
    category: "file",
    isContainer: false,
    browserCapable: true,
    icon: "folder-open",
  },
  group: {
    name: "group",
    label: "Group",
    description: "Container for child nodes. Orchestrates sequential or parallel execution.",
    category: "control",
    isContainer: true,
    browserCapable: true,
    icon: "box",
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
    description: "Image processing: resize, convert formats, compress, composite, batch.",
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
    icon: "file-up",
  },
  loop: {
    name: "loop",
    label: "Loop",
    description: "Iterate over arrays (forEach), repeat N times, or loop while condition.",
    category: "control",
    isContainer: true,
    browserCapable: true,
    icon: "repeat",
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
    icon: "git-fork",
  },
  "shell-command": {
    name: "shell-command",
    label: "Shell Command",
    description: "Execute shell commands with stall detection, retry, and streaming output.",
    category: "system",
    isContainer: false,
    browserCapable: false,
    icon: "terminal",
  },
  spreadsheet: {
    name: "spreadsheet",
    label: "Spreadsheet",
    description: "Read and write CSV or Excel files.",
    category: "spreadsheet",
    isContainer: false,
    browserCapable: true,
    icon: "sheet",
  },
  transform: {
    name: "transform",
    label: "Transform",
    description: "Transform data using expressions (single value) or field mappings.",
    category: "data",
    isContainer: false,
    browserCapable: true,
    icon: "arrow-left-right",
  },
} as const satisfies Record<NodeTypeName, NodeTypeInfo>;

// =============================================================================
// Processors — 6 implemented operations
// =============================================================================

export type ParamType = "number" | "string" | "boolean" | "enum" | "object";

export interface ProcessorParam {
  readonly name: string;
  readonly label: string;
  readonly description: string;
  readonly type: ParamType;
  readonly options?: readonly string[];
  readonly default?: unknown;
  readonly constraints?: {
    readonly min?: number;
    readonly max?: number;
    readonly required?: boolean;
  };
}

export interface ProcessorDef {
  readonly nodeType: string;
  readonly operation: string;
  readonly name: string;
  readonly description: string;
  readonly category: string;
  readonly accepts: readonly string[];
  readonly platforms: readonly string[];
  readonly parameters: readonly ProcessorParam[];
}

export const PROCESSORS: readonly ProcessorDef[] = [
  {
    nodeType: "file-system",
    operation: "rename",
    name: "Rename Files",
    description: "Transform filenames using patterns, find/replace, and case rules",
    category: "file",
    accepts: [] as const,
    platforms: ["browser"] as const,
    parameters: [
      {
        name: "find",
        label: "Find",
        description: "Text or regex pattern to search for in the filename",
        type: "string" as const,
      },
      {
        name: "replace",
        label: "Replace",
        description: "Replacement text (used with Find)",
        type: "string" as const,
      },
      {
        name: "case",
        label: "Case",
        description: "Transform the filename to a specific case",
        type: "enum" as const,
        options: ["lower", "upper", "title"] as const,
      },
      {
        name: "prefix",
        label: "Prefix",
        description: "Text to prepend to the filename",
        type: "string" as const,
      },
      {
        name: "suffix",
        label: "Suffix",
        description: "Text to append before the file extension",
        type: "string" as const,
      },
      {
        name: "pattern",
        label: "Pattern",
        description:
          "Template for the output filename (supports {{name}}, {{ext}}, {{index}}, {{date}})",
        type: "string" as const,
      },
    ],
  },
  {
    nodeType: "image",
    operation: "compress",
    name: "Compress Images",
    description: "Reduce image file size while maintaining quality",
    category: "image",
    accepts: ["image/jpeg", "image/png", "image/webp"] as const,
    platforms: ["browser"] as const,
    parameters: [
      {
        name: "quality",
        label: "Quality",
        description: "Compression quality (1 = smallest file, 100 = best quality)",
        type: "number" as const,
        default: 80,
        constraints: { min: 1, max: 100, required: false },
      },
    ],
  },
  {
    nodeType: "image",
    operation: "convert",
    name: "Convert Image Format",
    description: "Convert images between JPEG, PNG, and WebP formats",
    category: "image",
    accepts: ["image/jpeg", "image/png", "image/webp"] as const,
    platforms: ["browser"] as const,
    parameters: [
      {
        name: "format",
        label: "Output Format",
        description: "The target image format to convert to",
        type: "enum" as const,
        options: ["jpeg", "png", "webp"] as const,
        constraints: { required: true },
      },
      {
        name: "quality",
        label: "Quality",
        description: "Output quality for lossy formats (1-100)",
        type: "number" as const,
        default: 80,
        constraints: { min: 1, max: 100, required: false },
      },
    ],
  },
  {
    nodeType: "image",
    operation: "resize",
    name: "Resize Images",
    description: "Change image dimensions while maintaining quality",
    category: "image",
    accepts: ["image/jpeg", "image/png", "image/webp"] as const,
    platforms: ["browser"] as const,
    parameters: [
      {
        name: "width",
        label: "Width",
        description: "Target width in pixels",
        type: "number" as const,
        constraints: { min: 1, required: false },
      },
      {
        name: "height",
        label: "Height",
        description: "Target height in pixels",
        type: "number" as const,
        constraints: { min: 1, required: false },
      },
      {
        name: "maintainAspect",
        label: "Maintain Aspect Ratio",
        description: "Keep the original width-to-height ratio when resizing",
        type: "boolean" as const,
        default: true,
      },
      {
        name: "quality",
        label: "Quality",
        description: "Output quality for lossy formats (1-100)",
        type: "number" as const,
        default: 80,
        constraints: { min: 1, max: 100, required: false },
      },
    ],
  },
  {
    nodeType: "spreadsheet",
    operation: "clean",
    name: "Clean CSV",
    description: "Remove empty rows, trim whitespace, and deduplicate CSV data",
    category: "spreadsheet",
    accepts: ["text/csv"] as const,
    platforms: ["browser"] as const,
    parameters: [
      {
        name: "trimWhitespace",
        label: "Trim Whitespace",
        description: "Remove leading and trailing whitespace from every cell",
        type: "boolean" as const,
        default: true,
      },
      {
        name: "removeEmptyRows",
        label: "Remove Empty Rows",
        description: "Skip rows where every cell is blank",
        type: "boolean" as const,
        default: true,
      },
      {
        name: "removeDuplicates",
        label: "Remove Duplicates",
        description: "Remove duplicate rows, keeping the first occurrence",
        type: "boolean" as const,
        default: true,
      },
    ],
  },
  {
    nodeType: "spreadsheet",
    operation: "rename",
    name: "Rename CSV Columns",
    description: "Rename column headers in a CSV file",
    category: "spreadsheet",
    accepts: ["text/csv"] as const,
    platforms: ["browser"] as const,
    parameters: [
      {
        name: "columns",
        label: "Column Mapping",
        description: 'Map of old column names to new names (e.g., {"Name": "full_name"})',
        type: "object" as const,
      },
    ],
  },
] as const;

// --- Lookup helpers ---

/** Map keyed by "nodeType:operation" for O(1) lookup. */
export const PROCESSOR_MAP = new Map<string, ProcessorDef>(
  PROCESSORS.map((p) => [`${p.nodeType}:${p.operation}`, p]),
);

/** Get the engine defaults for a specific processor. */
export function getProcessorDefaults(nodeType: string, operation: string): Record<string, unknown> {
  const proc = PROCESSOR_MAP.get(`${nodeType}:${operation}`);
  if (!proc) return {};
  const defaults: Record<string, unknown> = {};
  for (const param of proc.parameters) {
    if (param.default !== undefined) {
      defaults[param.name] = param.default;
    }
  }
  return defaults;
}

/** Get the engine constraints for a specific parameter. */
export function getParamConstraints(
  nodeType: string,
  operation: string,
  paramName: string,
): ProcessorParam["constraints"] | undefined {
  const proc = PROCESSOR_MAP.get(`${nodeType}:${operation}`);
  if (!proc) return undefined;
  const param = proc.parameters.find((p) => p.name === paramName);
  return param?.constraints;
}

/** Get the accepted MIME types for a specific processor. */
export function getProcessorAccepts(nodeType: string, operation: string): readonly string[] {
  const proc = PROCESSOR_MAP.get(`${nodeType}:${operation}`);
  return proc?.accepts ?? [];
}
