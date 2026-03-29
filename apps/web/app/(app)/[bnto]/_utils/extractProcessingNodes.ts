/** Walks a Definition tree and returns flat list of processing nodes (non-I/O, non-container). */

import type { Definition } from "@bnto/core";
import { isIoNodeType, isContainerNodeType } from "@bnto/core";

export interface ProcessingNode {
  id: string;
  type: string;
  name: string;
  parameters: Record<string, unknown>;
}

export function extractProcessingNodes(definition: Definition): ProcessingNode[] {
  if (!definition.nodes) return [];

  return definition.nodes
    .filter((node) => !isIoNodeType(node.type) && !isContainerNodeType(node.type))
    .map((node) => ({
      id: node.id,
      type: node.type,
      name: node.name,
      parameters: node.parameters,
    }));
}
