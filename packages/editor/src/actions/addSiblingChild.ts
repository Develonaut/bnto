/**
 * addSiblingChild — adds a sibling node in the same container.
 *
 * Mode 2 of the addNode action. The afterNodeId is a child node,
 * so the new node inherits its parentContainerId and depth.
 */

import type { NodeTypeName } from "@bnto/core";
import type { EditorState } from "../store/types";
import type { BentoNode } from "../adapters/types";
import type { AddNodeResult } from "./types";
import type { CompartmentNodeResult } from "../adapters/createCompartmentNode";
import { withUndo } from "../store/withUndo";
import { addChildToContainer } from "../adapters/definitionTreeHelpers";
import { buildChildDefinition } from "./buildChildDefinition";

/** Compute the flat-array insert position for the new sibling. */
function findSiblingInsertIndex(deselected: BentoNode[], afterNodeId?: string | null): number {
  const afterIdx = afterNodeId ? deselected.findIndex((n) => n.id === afterNodeId) : -1;
  return afterIdx >= 0 ? afterIdx + 1 : deselected.length;
}

/** Build partial state updates for the new sibling node. */
function buildSiblingStateUpdates(
  state: EditorState,
  result: CompartmentNodeResult,
  parentContainerId: string,
  afterNodeId?: string | null,
  isContainer?: boolean,
) {
  const childDef = buildChildDefinition(
    result.node.id,
    result.config.nodeType as NodeTypeName,
    result.config,
  );
  const nextDefinition = state.definition
    ? addChildToContainer(state.definition, parentContainerId, childDef, afterNodeId ?? undefined)
    : state.definition;
  const nextExpandedIds = isContainer
    ? new Set([...state.expandedContainerIds, result.node.id])
    : undefined;
  return {
    ...(nextDefinition !== state.definition ? { definition: nextDefinition } : {}),
    ...(nextExpandedIds ? { expandedContainerIds: nextExpandedIds } : {}),
  };
}

function addSiblingChild(
  state: EditorState,
  result: CompartmentNodeResult,
  newNode: BentoNode,
  deselected: BentoNode[],
  parentContainerId: string,
  afterNodeId?: string | null,
  isContainer?: boolean,
): AddNodeResult {
  const insertAt = findSiblingInsertIndex(deselected, afterNodeId);
  const nextNodes = [...deselected.slice(0, insertAt), newNode, ...deselected.slice(insertAt)];
  const nextConfigs = { ...state.configs, [result.node.id]: result.config };
  const updates = buildSiblingStateUpdates(
    state,
    result,
    parentContainerId,
    afterNodeId,
    isContainer,
  );

  return {
    nextState: withUndo(state, { nodes: nextNodes, configs: nextConfigs, ...updates }),
    nodeId: result.node.id,
  };
}

export { addSiblingChild };
