import { generatedHeader } from "../generatedHeader";
import type { CatalogEnvelope } from "../types";

// Everything between the category union and the processor's
// `inputCardinality` union, which are the only parts the catalog changes.
const STATIC_TYPES = `/** Metadata describing a node type's behavior and capabilities. */
export interface NodeTypeInfo {
  /** The node type name as used in \`.bnto.json\` definitions. */
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
  readonly parameters: readonly ProcessorParam[];`;

/** `"a" | "b"`, sorted and deduplicated. */
function stringUnion(values: string[]): string {
  return [...new Set(values)]
    .sort()
    .map((v) => `"${v}"`)
    .join(" | ");
}

/** `catalog/types.ts`: the node type name, category and processor types. */
export function catalogTypesFile(catalog: CatalogEnvelope): string {
  const categoryUnion = stringUnion(catalog.nodeTypes.map((t) => t.category));
  const sorted = [...catalog.nodeTypes].sort((a, b) => a.name.localeCompare(b.name));
  const nameUnion = sorted.map((t) => `  | ${JSON.stringify(t.name)}`).join("\n");
  // Derive inputCardinality union from actual snapshot values
  const cardinalityUnion = stringUnion(
    catalog.processors.map((p) => p.inputCardinality ?? "perFile"),
  );

  return `${generatedHeader(catalog.version)}

/** Union of all valid node type name strings. */
export type NodeTypeName =
${nameUnion};

/** Node category for grouping in the UI and documentation. */
export type NodeCategory = ${categoryUnion};

${STATIC_TYPES}
  readonly inputCardinality: ${cardinalityUnion};
}
`;
}
