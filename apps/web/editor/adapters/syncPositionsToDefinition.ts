/**
 * Sync RF positions → Definition adapter.
 *
 * Patches ReactFlow node positions back into a Definition tree.
 * Used at save/export boundaries — NOT on every render. Positions
 * are owned by ReactFlow during editing; this captures them when
 * the definition needs to be serialized.
 *
 * Pure function — no React, no DOM, fully testable.
 */

import type { Definition } from "@bnto/nodes";
import type { BentoNode } from "./types";

function syncPositionsToDefinition(
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

export { syncPositionsToDefinition };
