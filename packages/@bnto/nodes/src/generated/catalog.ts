/**
 * AUTO-GENERATED from engine/catalog.snapshot.json — DO NOT EDIT.
 *
 * Run `task nodes:generate` to regenerate after engine changes.
 * Source: engine/crates/bnto-wasm/src/catalog.rs
 *
 * This module is the TypeScript representation of the Rust engine's
 * self-describing processor metadata. It provides typed definitions,
 * defaults, and constraints for all engine-implemented node operations.
 */

// --- Types ---

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

// --- Processor definitions (from engine catalog v1.0.0) ---

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

/**
 * Get the engine defaults for a specific processor.
 *
 * Returns a record of parameter names to their default values,
 * including only parameters that have engine-defined defaults.
 * Recipe-specific overrides (e.g., format: "webp") are NOT included —
 * those are authoring choices, not engine defaults.
 */
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

/**
 * Get the engine constraints for a specific parameter.
 *
 * Returns { min, max, required } if the engine defines constraints,
 * or undefined if no constraints exist.
 */
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

/**
 * Get the accepted MIME types for a specific processor.
 */
export function getProcessorAccepts(nodeType: string, operation: string): readonly string[] {
  const proc = PROCESSOR_MAP.get(`${nodeType}:${operation}`);
  return proc?.accepts ?? [];
}
