/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

/** Union of all valid node type name strings. */
export type NodeTypeName =
  | "edit-fields"
  | "file-rename"
  | "group"
  | "http-request"
  | "image-compress"
  | "image-convert"
  | "image-overlay"
  | "image-resize"
  | "image-strip-exif"
  | "input"
  | "loop"
  | "output"
  | "parallel"
  | "shell-command"
  | "spreadsheet-clean"
  | "spreadsheet-convert"
  | "spreadsheet-merge"
  | "spreadsheet-rename"
  | "transform";

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
  /** Execution platforms this node type supports (e.g. "browser", "server", "cli"). */
  platforms: readonly string[];
  /** Lucide icon name for visual consumers. */
  icon: string;
}

export type ParamType = "number" | "string" | "boolean" | "enum" | "object" | "file";

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
  readonly placeholder?: string;
  /** Accepted MIME types for file-type parameters. */
  readonly accept?: readonly string[];
  /** Whether this param is eligible for surfacing in container config panels. Defaults to true. */
  readonly surfaceable?: boolean;
}

export interface ProcessorDef {
  readonly nodeType: string;
  readonly name: string;
  readonly description: string;
  readonly category: string;
  readonly accepts: readonly string[];
  readonly platforms: readonly string[];
  readonly parameters: readonly ProcessorParam[];
  readonly inputCardinality: "perFile" | "batch";
}
