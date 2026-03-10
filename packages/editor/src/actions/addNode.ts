/**
 * addNode action — pure function that computes the next editor state
 * after adding a new node.
 *
 * Enforces the I/O singleton constraint (one input, one output per recipe),
 * creates the compartment node + config, captures an undo snapshot,
 * auto-selects the new node, and revalidates.
 *
 * When `afterNodeId` is provided, the new node is inserted immediately
 * after that node (and all subsequent nodes shift right by one stride).
 * Otherwise the node is inserted before the output node.
 *
 * Returns null if the add is blocked (unknown type or duplicate I/O).
 */

import type { NodeTypeName } from "@bnto/nodes";
import { isIoNodeType } from "@bnto/nodes";
import type { EditorState } from "../store/types";
import { createCompartmentNode } from "../adapters/createCompartmentNode";
import { STRIDE } from "../adapters/bentoSlots";
import { withUndo } from "../store/withUndo";

interface AddNodeResult {
  nextState: Partial<EditorState>;
  nodeId: string;
}

export function addNode(
  state: EditorState,
  type: NodeTypeName,
  afterNodeId?: string | null,
): AddNodeResult | null {
  // Only one input and one output node allowed per recipe.
  if (isIoNodeType(type)) {
    const alreadyExists = Object.values(state.configs).some((c) => c.nodeType === type);
    if (alreadyExists) return null;
  }

  const slotIndex = state.nodes.length;
  const result = createCompartmentNode(type, slotIndex);
  if (!result) return null;

  // Auto-select the new node, deselect all others
  const deselected = state.nodes.map((n) => (n.selected ? { ...n, selected: false } : n));

  // Determine insertion index
  let insertIndex: number;
  if (afterNodeId) {
    const afterIndex = deselected.findIndex((n) => n.id === afterNodeId);
    insertIndex = afterIndex >= 0 ? afterIndex + 1 : deselected.length;
  } else {
    // Default: insert before the output node so output stays last.
    const outputIndex = deselected.findIndex((n) => state.configs[n.id]?.nodeType === "output");
    insertIndex = outputIndex >= 0 ? outputIndex : deselected.length;
  }

  // Place the new node at the insertion position and shift everything after right.
  const target = deselected[insertIndex];
  const newNode = {
    ...result.node,
    selected: true,
    position: target
      ? { ...target.position }
      : { x: slotIndex * STRIDE, y: 0 },
  };

  const shifted = deselected.slice(insertIndex).map((n) => ({
    ...n,
    position: { x: n.position.x + STRIDE, y: n.position.y },
  }));

  const nextNodes = [
    ...deselected.slice(0, insertIndex),
    newNode,
    ...shifted,
  ];

  const nextConfigs = { ...state.configs, [result.node.id]: result.config };

  return {
    nextState: withUndo(state, { nodes: nextNodes, configs: nextConfigs }),
    nodeId: result.node.id,
  };
}
