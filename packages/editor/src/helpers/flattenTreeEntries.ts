/**
 * flattenTreeEntries — linearizes the hierarchical node tree for keyboard traversal.
 *
 * Walks the tree depth-first, respecting container expansion state.
 * Collapsed containers appear as a single item; expanded containers
 * include their children in traversal order.
 *
 * Returns an ordered array of node IDs that matches the visual order
 * the user sees in the NodeTree.
 */

import type { NodeListEntry } from "./buildNodeListTree";

function flattenTreeEntries(entries: NodeListEntry[], expandedContainerIds: Set<string>): string[] {
  const ids: string[] = [];

  function walk(entry: NodeListEntry) {
    ids.push(entry.node.id);
    if (entry.isContainer && expandedContainerIds.has(entry.node.id)) {
      for (const child of entry.children) {
        walk(child);
      }
    }
  }

  for (const entry of entries) {
    walk(entry);
  }

  return ids;
}

export { flattenTreeEntries };
