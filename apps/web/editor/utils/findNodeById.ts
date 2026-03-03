/**
 * Recursive tree search — find a node by ID in a Definition tree.
 *
 * Searches the definition's child nodes recursively (definitions
 * can be nested — a definition's nodes are themselves definitions).
 */

import type { Definition } from "@bnto/nodes";

function findNodeById(definition: Definition, nodeId: string): Definition | null {
  for (const child of definition.nodes ?? []) {
    if (child.id === nodeId) return child;
    const found = findNodeById(child, nodeId);
    if (found) return found;
  }
  return null;
}

export { findNodeById };
