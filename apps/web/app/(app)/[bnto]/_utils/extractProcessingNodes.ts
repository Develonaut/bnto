/** Extract leaf processing nodes from a Definition tree — skips I/O and container nodes. */

import type { Definition } from "@bnto/core";
import { isIoNodeType, isContainerNodeType } from "@bnto/core";

export interface ProcessingNode {
  id: string;
  type: string;
  name: string;
  parameters: Record<string, unknown>;
}

export function extractProcessingNodes(definition: Definition): ProcessingNode[] {
  const results: ProcessingNode[] = [];
  collectLeafNodes(definition.nodes ?? [], results);
  return results;
}

function collectLeafNodes(nodes: Definition[], out: ProcessingNode[]): void {
  for (const node of nodes) {
    if (isIoNodeType(node.type)) continue;
    if (isContainerNodeType(node.type)) {
      collectLeafNodes(node.nodes ?? [], out);
      continue;
    }
    out.push({ id: node.id, type: node.type, name: node.name, parameters: node.parameters });
  }
}
