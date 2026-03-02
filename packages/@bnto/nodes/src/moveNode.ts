/**
 * Updates a node's position within a definition.
 *
 * Searches the definition tree recursively to find the target node
 * and updates its position. Returns a new Definition (immutable).
 */

import type { Definition, Position } from "./definition";
import type { DefinitionResult } from "./definitionResult";
import { validateDefinition } from "./validate";

/** Recursively find and update a node's position by ID. */
function moveInTree(
  node: Definition,
  nodeId: string,
  position: Position,
): Definition {
  if (node.id === nodeId) {
    return { ...node, position };
  }

  if (!node.nodes?.length) return node;

  const updatedChildren = node.nodes.map((child) =>
    moveInTree(child, nodeId, position),
  );

  const changed = updatedChildren.some((c, i) => c !== node.nodes![i]);
  if (!changed) return node;

  return { ...node, nodes: updatedChildren };
}

/** Moves a node to a new position. Returns the updated definition. */
export function moveNode(
  definition: Definition,
  nodeId: string,
  position: Position,
): DefinitionResult {
  const updated = moveInTree(definition, nodeId, position);

  return {
    definition: updated,
    errors: validateDefinition(updated),
  };
}
