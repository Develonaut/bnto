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

import type { NodeTypeName } from "@bnto/core";
import { isIoNodeType, isContainerNodeType } from "@bnto/core";
import type { EditorState } from "../store/types";
import type { AddNodeResult } from "./types";
import { createCompartmentNode } from "../adapters/createCompartmentNode";
import { STRIDE } from "../adapters/bentoSlots";
import { addChildIntoContainer } from "./addChildIntoContainer";
import { addSiblingChild } from "./addSiblingChild";
import { addTopLevel } from "./addTopLevel";
import { buildNewNode } from "./buildNewNode";

/** Check if an I/O node of this type already exists. */
function isDuplicateIoNode(state: EditorState, type: NodeTypeName): boolean {
  if (!isIoNodeType(type)) return false;
  return Object.values(state.configs).some((c) => c.nodeType === type);
}

/** Determine insert index within same-level peers. */
function resolveInsertIndex(
  state: EditorState,
  sameLevelNodes: { id: string }[],
  afterNodeId?: string | null,
): number {
  if (afterNodeId) {
    const afterIndex = sameLevelNodes.findIndex((n) => n.id === afterNodeId);
    return afterIndex >= 0 ? afterIndex + 1 : sameLevelNodes.length;
  }
  const outputIndex = sameLevelNodes.findIndex((n) => state.configs[n.id]?.nodeType === "output");
  return outputIndex >= 0 ? outputIndex : sameLevelNodes.length;
}

/** Prepare common state for a non-container insertion. */
function prepareInsertion(
  state: EditorState,
  type: NodeTypeName,
  afterNodeId?: string | null,
  defaultParams?: Record<string, unknown>,
) {
  const afterNode = afterNodeId ? state.nodes.find((n) => n.id === afterNodeId) : null;
  const parentContainerId = afterNode?.data.parentContainerId;
  const result = createCompartmentNode(type, state.nodes.length, undefined, defaultParams);
  if (!result) return null;

  const deselected = state.nodes.map((n) => (n.selected ? { ...n, selected: false } : n));
  const sameLevelNodes = parentContainerId
    ? deselected.filter((n) => n.data.parentContainerId === parentContainerId)
    : deselected.filter((n) => !n.data.parentContainerId);

  const insertIndex = resolveInsertIndex(state, sameLevelNodes, afterNodeId);
  const target = sameLevelNodes[insertIndex];
  const isContainer = isContainerNodeType(type);

  const newNode = buildNewNode(result.node, {
    parentContainerId,
    depth: afterNode?.data.depth ?? 0,
    isContainer,
    fallbackPosition: target ? { ...target.position } : { x: state.nodes.length * STRIDE, y: 0 },
  });

  return { result, deselected, parentContainerId, newNode, isContainer };
}

/** Dispatch to top-level or sibling-child insertion based on parent. */
function dispatchInsertion(
  state: EditorState,
  prep: NonNullable<ReturnType<typeof prepareInsertion>>,
  afterNodeId?: string | null,
): AddNodeResult {
  if (!prep.parentContainerId) {
    return addTopLevel(
      state,
      prep.result,
      prep.newNode,
      prep.deselected,
      afterNodeId,
      prep.isContainer,
    );
  }
  return addSiblingChild(
    state,
    prep.result,
    prep.newNode,
    prep.deselected,
    prep.parentContainerId,
    afterNodeId,
    prep.isContainer,
  );
}

export function addNode(
  state: EditorState,
  type: NodeTypeName,
  afterNodeId?: string | null,
  intoContainerId?: string | null,
  defaultParams?: Record<string, unknown>,
): AddNodeResult | null {
  if (isDuplicateIoNode(state, type)) return null;
  if (intoContainerId)
    return addChildIntoContainer(state, type, intoContainerId, afterNodeId, defaultParams);

  const prep = prepareInsertion(state, type, afterNodeId, defaultParams);
  if (!prep) return null;
  return dispatchInsertion(state, prep, afterNodeId);
}
