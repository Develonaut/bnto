/**
 * Removes a node from a definition and cleans up connected edges.
 *
 * Searches recursively through the definition tree to find and remove
 * the node. Also removes any edges that reference the removed node
 * (cascade cleanup).
 *
 * Returns a new Definition (immutable — never mutates the input).
 */

import type { Definition } from "./definition";
import type { DefinitionResult } from "./definitionResult";
import { validateDefinition } from "./validate";

/**
 * Removes a child node by ID from the definition's nodes array.
 * Also removes edges connected to that node (cascade).
 */
export function removeNode(
  definition: Definition,
  nodeId: string,
): DefinitionResult {
  const currentNodes = definition.nodes ?? [];
  const filtered = currentNodes.filter((n) => n.id !== nodeId);

  // Nothing removed — node wasn't found at this level
  if (filtered.length === currentNodes.length) {
    return {
      definition,
      errors: validateDefinition(definition),
    };
  }

  // Remove edges that reference the removed node
  const currentEdges = definition.edges ?? [];
  const cleanedEdges = currentEdges.filter(
    (e) => e.source !== nodeId && e.target !== nodeId,
  );

  const updated: Definition = {
    ...definition,
    nodes: filtered,
    edges: cleanedEdges,
  };

  return {
    definition: updated,
    errors: validateDefinition(updated),
  };
}
