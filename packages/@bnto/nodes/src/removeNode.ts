/**
 * Removes a node from a definition and cleans up connected edges.
 *
 * Searches recursively through the definition tree to find and remove
 * the node. Also removes any edges that reference the removed node
 * at the same nesting level (cascade cleanup).
 *
 * Returns a new Definition (immutable — never mutates the input).
 */

import type { Definition } from "./definition";
import type { DefinitionResult } from "./definitionResult";
import { validateDefinition } from "./validate";

/** Recursively find and remove a node by ID, cleaning up edges at each level. */
function removeFromTree(node: Definition, nodeId: string): Definition {
  if (!node.nodes?.length) return node;

  const filtered = node.nodes.filter((n) => n.id !== nodeId);

  // Found and removed at this level — cascade-clean edges here
  if (filtered.length !== node.nodes.length) {
    const cleanedEdges = (node.edges ?? []).filter(
      (e) => e.source !== nodeId && e.target !== nodeId,
    );
    return { ...node, nodes: filtered, edges: cleanedEdges };
  }

  // Not found at this level — recurse into children
  const updatedChildren = node.nodes.map((child) =>
    removeFromTree(child, nodeId),
  );

  // Only create new object if something actually changed
  const changed = updatedChildren.some((c, i) => c !== node.nodes![i]);
  if (!changed) return node;

  return { ...node, nodes: updatedChildren };
}

/**
 * Removes a node by ID from the definition tree.
 * Also removes edges connected to that node at the same nesting level.
 */
export function removeNode(
  definition: Definition,
  nodeId: string,
): DefinitionResult {
  const updated = removeFromTree(definition, nodeId);

  return {
    definition: updated,
    errors: validateDefinition(updated),
  };
}
