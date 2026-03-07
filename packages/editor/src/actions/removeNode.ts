/**
 * removeNode action — pure function that computes the next editor state
 * after removing a node.
 *
 * Enforces the I/O deletion guard (I/O nodes cannot be removed),
 * captures an undo snapshot, removes the node + config,
 * auto-selects the nearest neighbor, and revalidates.
 *
 * Returns null if the removal is blocked (I/O node).
 */

import type { EditorState } from "../store/types";
import { isIoNodeType } from "@bnto/nodes";
import { STRIDE } from "../adapters/bentoSlots";
import { withUndo } from "../store/withUndo";

export function removeNode(state: EditorState, id: string): Partial<EditorState> | null {
  // I/O nodes are structural — they cannot be deleted.
  const config = state.configs[id];
  if (config && isIoNodeType(config.nodeType)) return null;

  const removedIndex = state.nodes.findIndex((n) => n.id === id);
  const nextNodes = state.nodes.filter((n) => n.id !== id);
  const nextConfigs = { ...state.configs };
  delete nextConfigs[id];

  // Reflow positions — close the gap left by the removed node.
  // Nodes are in a horizontal strip; reposition each to its new index slot.
  for (let i = 0; i < nextNodes.length; i++) {
    const expectedX = i * STRIDE;
    if (nextNodes[i]!.position.x !== expectedX) {
      nextNodes[i] = { ...nextNodes[i]!, position: { ...nextNodes[i]!.position, x: expectedX } };
    }
  }

  // Auto-select the nearest remaining node after removal.
  if (nextNodes.length > 0) {
    const selectIndex = Math.min(removedIndex > 0 ? removedIndex - 1 : 0, nextNodes.length - 1);
    nextNodes[selectIndex] = { ...nextNodes[selectIndex]!, selected: true };
  }

  return withUndo(state, { nodes: nextNodes, configs: nextConfigs });
}
