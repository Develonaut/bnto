/**
 * Bento → Definition adapter.
 *
 * Converts BentoCanvas compartment nodes back into a Definition tree.
 * This is the reverse of definitionToBento — used when the visual editor
 * modifies node positions and the changes need to propagate back to the
 * headless Definition model.
 *
 * Pure function — no React, no DOM, fully testable.
 */

import type { Definition } from "@bnto/nodes";
import type { BentoNode } from "./definitionToBento";

/**
 * Update node positions in a Definition based on bento compartment positions.
 *
 * This does NOT reconstruct the full definition from scratch — it patches
 * position data from the visual layer back into the existing definition.
 * Node types, parameters, edges, and other data are preserved from the
 * original definition.
 *
 * @param definition - The original definition to update
 * @param bentoNodes - Compartment nodes with updated positions
 * @returns Updated definition with new positions applied
 */
function bentoToDefinition(
  definition: Definition,
  bentoNodes: BentoNode[],
): Definition {
  const positionMap = new Map(
    bentoNodes.map((node) => [node.data.nodeId, node.position]),
  );

  const updatedChildren = (definition.nodes ?? []).map((child) => {
    const newPosition = positionMap.get(child.id);
    if (!newPosition) return child;
    return { ...child, position: newPosition };
  });

  return {
    ...definition,
    nodes: updatedChildren,
  };
}

export { bentoToDefinition };
