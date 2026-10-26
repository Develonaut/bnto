import { generatedHeader } from "../generatedHeader";
import { toCamelCase } from "../toCamelCase";
import type { CatalogEnvelope, RawNodeType } from "../types";

const NAMES_ARRAY = `/** All node type names as a readonly array. */
export const NODE_TYPE_NAMES: readonly NodeTypeName[] = Object.values(NODE_TYPES) as NodeTypeName[];`;

function nodeTypesConst(sorted: RawNodeType[]): string {
  const entries = sorted
    .map((t) => `  ${toCamelCase(t.name)}: ${JSON.stringify(t.name)}`)
    .join(",\n");
  return `/**
 * All registered node type names.
 * Maps camelCase keys to kebab-case type name strings.
 */
export const NODE_TYPES = {
${entries},
} as const;`;
}

function nodeTypeInfoConst(sorted: RawNodeType[]): string {
  const entries = sorted
    .map((t) => `  ${JSON.stringify(t.name)}: ${toCamelCase(t.name)}NodeType`)
    .join(",\n");
  return `/**
 * Metadata for all registered node types.
 * Maps node type name → info.
 */
export const NODE_TYPE_INFO: Record<NodeTypeName, NodeTypeInfo> = {
${entries},
} as const satisfies Record<NodeTypeName, NodeTypeInfo>;`;
}

/** `catalog/nodeTypes/index.ts`: every node type's name and info. */
export function nodeTypesBarrel(catalog: CatalogEnvelope): string {
  const sorted = [...catalog.nodeTypes].sort((a, b) => a.name.localeCompare(b.name));
  const imports = sorted
    .map((t) => `import { ${toCamelCase(t.name)}NodeType } from "./${toCamelCase(t.name)}";`)
    .join("\n");

  return `${generatedHeader(catalog.version)}

import type { NodeTypeName, NodeTypeInfo } from "../types";

${imports}

${nodeTypesConst(sorted)}

${NAMES_ARRAY}

${nodeTypeInfoConst(sorted)}
`;
}
