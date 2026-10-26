import { toCamelCase } from "../toCamelCase";
import type { RawNodeType } from "../types";

/** `catalog/nodeTypes/<type>.ts`: one node type's info. */
export function nodeTypeFile(t: RawNodeType, header: string): string {
  return `${header}

import type { NodeTypeInfo } from "../types";

/** Node type info for ${t.name}. */
export const ${toCamelCase(t.name)}NodeType: NodeTypeInfo = {
  name: ${JSON.stringify(t.name)},
  label: ${JSON.stringify(t.label)},
  description: ${JSON.stringify(t.description)},
  category: ${JSON.stringify(t.category)},
  isContainer: ${t.isContainer},
  platforms: ${JSON.stringify(t.platforms)} as const,
  icon: ${JSON.stringify(t.icon)},
};
`;
}
