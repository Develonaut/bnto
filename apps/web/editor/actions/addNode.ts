/**
 * addNode action — pure function that computes the next editor state
 * after adding a new node.
 *
 * Enforces the I/O singleton constraint (one input, one output per recipe),
 * creates the compartment node + config, captures an undo snapshot,
 * auto-selects the new node, and revalidates.
 *
 * Returns null if the add is blocked (unknown type or duplicate I/O).
 */

import type { NodeTypeName } from "@bnto/nodes";
import type { EditorState } from "../store/types";
import { createCompartmentNode } from "../adapters/createCompartmentNode";
import { captureSnapshot } from "../store/captureSnapshot";
import { pushToStack } from "../store/pushToStack";
import { revalidateState } from "../store/revalidateState";
import { isIoNodeType } from "../helpers/isIoNodeType";

interface AddNodeResult {
  nextState: Partial<EditorState>;
  nodeId: string;
}

export function addNode(
  state: EditorState,
  type: NodeTypeName,
  position?: { x: number; y: number },
): AddNodeResult | null {
  // Only one input and one output node allowed per recipe.
  if (isIoNodeType(type)) {
    const alreadyExists = Object.values(state.configs).some(
      (c) => c.nodeType === type,
    );
    if (alreadyExists) return null;
  }

  const slotIndex = state.nodes.length;
  const result = createCompartmentNode(type, slotIndex, position);
  if (!result) return null;

  const snapshot = captureSnapshot(state.nodes, state.configs);

  // Auto-select the new node, deselect all others
  const deselected = state.nodes.map((n) =>
    n.selected ? { ...n, selected: false } : n,
  );
  const newNode = { ...result.node, selected: true };
  const nextNodes = [...deselected, newNode];
  const nextConfigs = { ...state.configs, [result.node.id]: result.config };

  return {
    nextState: {
      nodes: nextNodes,
      configs: nextConfigs,
      isDirty: true,
      undoStack: pushToStack(state.undoStack, snapshot),
      redoStack: [],
      validationErrors: revalidateState(
        nextNodes,
        nextConfigs,
        state.recipeMetadata,
      ),
    },
    nodeId: result.node.id,
  };
}
