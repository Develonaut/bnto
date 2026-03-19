/**
 * addSiblingChild — adds a sibling node in the same container.
 *
 * Mode 2 of the addNode action. The afterNodeId is a child node,
 * so the new node inherits its parentContainerId and depth.
 */

import type { NodeTypeName } from "@bnto/core";
import { isContainerNodeType } from "@bnto/core";
import type { EditorState } from "../store/types";
import type { BentoNode } from "../adapters/types";
import type { CompartmentNodeResult } from "../adapters/createCompartmentNode";
import { withUndo } from "../store/withUndo";
import { addChildToContainer } from "../adapters/definitionTreeHelpers";
import { buildChildDefinition } from "./buildChildDefinition";

interface AddNodeResult {
  nextState: Partial<EditorState>;
  nodeId: string;
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
  const afterIdx = afterNodeId ? deselected.findIndex((n) => n.id === afterNodeId) : -1;
  const insertAt = afterIdx >= 0 ? afterIdx + 1 : deselected.length;
  const nextNodes = [...deselected.slice(0, insertAt), newNode, ...deselected.slice(insertAt)];
  const nextConfigs = { ...state.configs, [result.node.id]: result.config };

  const childDef = buildChildDefinition(
    result.node.id,
    result.config.nodeType as NodeTypeName,
    result.config,
  );
  let nextDefinition = state.definition;
  if (nextDefinition) {
    nextDefinition = addChildToContainer(
      nextDefinition,
      parentContainerId,
      childDef,
      afterNodeId ?? undefined,
    );
  }

  const nextExpandedIds = isContainer
    ? new Set([...state.expandedContainerIds, result.node.id])
    : undefined;

  return {
    nextState: withUndo(state, {
      nodes: nextNodes,
      configs: nextConfigs,
      ...(nextDefinition !== state.definition ? { definition: nextDefinition } : {}),
      ...(nextExpandedIds ? { expandedContainerIds: nextExpandedIds } : {}),
    }),
    nodeId: result.node.id,
  };
}

export { addSiblingChild };
