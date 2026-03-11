/**
 * addNode action — pure function that computes the next editor state
 * after adding a new node.
 *
 * Enforces the I/O singleton constraint (one input, one output per recipe),
 * creates the compartment node + config, captures an undo snapshot,
 * auto-selects the new node, and revalidates.
 *
 * Three insertion modes (delegated to separate files):
 * 1. `intoContainerId` — adds the new node as a child inside a container.
 * 2. `afterNodeId` resolves to a child — adds a sibling child in the same
 *    container with matching parentContainerId and depth.
 * 3. `afterNodeId` resolves to a top-level node (or null) — inserts into
 *    the top-level pipeline. Default: before the output node.
 *
 * Returns null if the add is blocked (unknown type or duplicate I/O).
 */

import type { NodeTypeName } from "@bnto/nodes";
import { isIoNodeType, isContainerNodeType } from "@bnto/nodes";
import type { EditorState } from "../store/types";
import type { BentoNode } from "../adapters/types";
import { createCompartmentNode } from "../adapters/createCompartmentNode";
import { STRIDE } from "../adapters/bentoSlots";
import { addChildIntoContainer } from "./addChildIntoContainer";
import { addSiblingChild } from "./addSiblingChild";
import { addTopLevel } from "./addTopLevel";

interface AddNodeResult {
  nextState: Partial<EditorState>;
  nodeId: string;
}

export function addNode(
  state: EditorState,
  type: NodeTypeName,
  afterNodeId?: string | null,
  intoContainerId?: string | null,
): AddNodeResult | null {
  // Only one input and one output node allowed per recipe.
  if (isIoNodeType(type)) {
    const alreadyExists = Object.values(state.configs).some((c) => c.nodeType === type);
    if (alreadyExists) return null;
  }

  // --- Mode 1: Add as child inside a container ---
  if (intoContainerId) {
    return addChildIntoContainer(state, type, intoContainerId, afterNodeId);
  }

  // Check if afterNodeId is a child node (inherits its container context)
  const afterNode = afterNodeId ? state.nodes.find((n) => n.id === afterNodeId) : null;
  const parentContainerId = afterNode?.data.parentContainerId;
  const depth = afterNode?.data.depth ?? 0;

  const slotIndex = state.nodes.length;
  const result = createCompartmentNode(type, slotIndex);
  if (!result) return null;

  // Auto-select the new node, deselect all others
  const deselected = state.nodes.map((n) => (n.selected ? { ...n, selected: false } : n));

  // Build the new node with parent context if it's a child
  const sameLevelNodes = parentContainerId
    ? deselected.filter((n) => n.data.parentContainerId === parentContainerId)
    : deselected.filter((n) => !n.data.parentContainerId);

  let insertIndex: number;
  if (afterNodeId) {
    const afterIndex = sameLevelNodes.findIndex((n) => n.id === afterNodeId);
    insertIndex = afterIndex >= 0 ? afterIndex + 1 : sameLevelNodes.length;
  } else {
    const outputIndex = sameLevelNodes.findIndex(
      (n) => state.configs[n.id]?.nodeType === "output",
    );
    insertIndex = outputIndex >= 0 ? outputIndex : sameLevelNodes.length;
  }

  const target = sameLevelNodes[insertIndex];
  const isContainer = isContainerNodeType(type);
  const newNode: BentoNode = {
    ...result.node,
    selected: true,
    position: target ? { ...target.position } : { x: slotIndex * STRIDE, y: 0 },
    data: {
      ...result.node.data,
      ...(parentContainerId ? { parentContainerId, depth } : { depth: 0 }),
      ...(isContainer ? { isContainer: true, isExpanded: true } : {}),
    },
  };

  if (!parentContainerId) {
    return addTopLevel(state, result, newNode, deselected, afterNodeId, isContainer);
  }

  return addSiblingChild(state, result, newNode, deselected, parentContainerId, afterNodeId, isContainer);
}
