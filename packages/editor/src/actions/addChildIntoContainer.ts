/**
 * addChildIntoContainer — adds a new node as a child inside a container.
 *
 * Mode 1 of the addNode action. Creates the node, marks the container
 * as expanded, updates the definition tree, and captures undo state.
 */

import type { NodeTypeName } from "@bnto/nodes";
import { isContainerNodeType } from "@bnto/nodes";
import type { EditorState } from "../store/types";
import type { BentoNode } from "../adapters/types";
import { createCompartmentNode } from "../adapters/createCompartmentNode";
import { withUndo } from "../store/withUndo";
import { addChildToContainer } from "../adapters/definitionTreeHelpers";
import { buildChildDefinition } from "./buildChildDefinition";

interface AddNodeResult {
  nextState: Partial<EditorState>;
  nodeId: string;
}

function addChildIntoContainer(
  state: EditorState,
  type: NodeTypeName,
  containerId: string,
  afterNodeId?: string | null,
): AddNodeResult | null {
  const container = state.nodes.find((n) => n.id === containerId);
  if (!container) return null;

  const slotIndex = state.nodes.length;
  const result = createCompartmentNode(type, slotIndex);
  if (!result) return null;

  const parentDepth = container.data.depth ?? 0;

  // Deselect all + ensure container is marked expanded
  const deselected = state.nodes.map((n) => {
    if (n.selected) n = { ...n, selected: false };
    if (n.id === containerId && !n.data.isExpanded) {
      n = { ...n, data: { ...n.data, isExpanded: true } };
    }
    return n;
  });

  const childIsContainer = isContainerNodeType(type);
  const newNode: BentoNode = {
    ...result.node,
    selected: true,
    position: { x: 0, y: 0 }, // layoutNodes repositions
    data: {
      ...result.node.data,
      parentContainerId: containerId,
      depth: parentDepth + 1,
      ...(childIsContainer ? { isContainer: true, isExpanded: true } : {}),
    },
  };

  // Insert after a specific child, or right after the container (first child)
  const insertAt = afterNodeId
    ? (deselected.findIndex((n) => n.id === afterNodeId) + 1) || deselected.length
    : deselected.findIndex((n) => n.id === containerId) + 1;

  const nextNodes = [...deselected.slice(0, insertAt), newNode, ...deselected.slice(insertAt)];
  const nextConfigs = { ...state.configs, [result.node.id]: result.config };

  const childDef = buildChildDefinition(result.node.id, type, result.config);
  let nextDefinition = state.definition;
  if (nextDefinition) {
    nextDefinition = addChildToContainer(nextDefinition, containerId, childDef, afterNodeId ?? undefined);
  }

  const nextExpandedIds = new Set(state.expandedContainerIds);
  nextExpandedIds.add(containerId);
  if (childIsContainer) nextExpandedIds.add(result.node.id);

  return {
    nextState: withUndo(state, {
      nodes: nextNodes,
      configs: nextConfigs,
      expandedContainerIds: nextExpandedIds,
      ...(nextDefinition !== state.definition ? { definition: nextDefinition } : {}),
    }),
    nodeId: result.node.id,
  };
}

export { addChildIntoContainer };
export type { AddNodeResult };
