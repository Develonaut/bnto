/**
 * RF nodes → Definition adapter.
 *
 * Reads ReactFlow nodes (which carry full domain data in their `data`
 * field) and reconstructs a Definition tree. Used at export boundaries
 * only — during editing, RF owns all state.
 *
 * The bento editor doesn't handle ports or edges (execution order is
 * implied by compartment position). Ports default to empty arrays.
 *
 * Pure function — no React, no DOM, fully testable.
 */

import type { Definition } from "@bnto/nodes";
import type { BentoNode } from "./types";

/**
 * Build a Definition from the current ReactFlow node state.
 *
 * @param rfNodes - Compartment nodes carrying domain data in `data`
 * @param rootDefinition - The parent definition (for root-level metadata)
 * @returns Full Definition with child nodes extracted from RF state
 */
function rfNodesToDefinition(
  rfNodes: BentoNode[],
  rootDefinition: Definition,
): Definition {
  const children: Definition[] = rfNodes.map((rfNode) => ({
    id: rfNode.id,
    type: rfNode.data.nodeType,
    version: "1.0.0",
    name: rfNode.data.name,
    position: rfNode.position,
    parameters: rfNode.data.parameters,
    metadata: {},
    inputPorts: [],
    outputPorts: [],
  }));

  return {
    ...rootDefinition,
    nodes: children,
  };
}

export { rfNodesToDefinition };
