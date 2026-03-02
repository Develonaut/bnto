/**
 * Updates parameters on a specific node within a definition.
 *
 * Merges the provided params into the node's existing parameters,
 * then validates the entire definition. Returns the updated definition
 * and any validation errors.
 *
 * Searches the definition tree recursively to find the target node.
 * Returns a new Definition (immutable — never mutates the input).
 */

import type { Definition } from "./definition";
import type { DefinitionResult } from "./definitionResult";
import { validateDefinition } from "./validate";

/** Recursively find and update a node's parameters by ID. */
function updateParamsInTree(
  node: Definition,
  nodeId: string,
  params: Record<string, unknown>,
): Definition {
  // Found the target node — merge params
  if (node.id === nodeId) {
    return {
      ...node,
      parameters: { ...node.parameters, ...params },
    };
  }

  // Not this node — search children if container
  if (!node.nodes?.length) return node;

  const updatedChildren = node.nodes.map((child) =>
    updateParamsInTree(child, nodeId, params),
  );

  // Only create new object if something actually changed
  const changed = updatedChildren.some((c, i) => c !== node.nodes![i]);
  if (!changed) return node;

  return { ...node, nodes: updatedChildren };
}

/**
 * Updates parameters on a node, validates, and returns the result.
 *
 * Parameters are merged (not replaced) — existing params that aren't
 * in the update are preserved.
 */
export function updateNodeParams(
  definition: Definition,
  nodeId: string,
  params: Record<string, unknown>,
): DefinitionResult {
  const updated = updateParamsInTree(definition, nodeId, params);

  return {
    definition: updated,
    errors: validateDefinition(updated),
  };
}
