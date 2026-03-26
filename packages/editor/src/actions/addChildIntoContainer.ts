/**
 * addChildIntoContainer — adds a new node as a child inside a container.
 *
 * Mode 1 of the addNode action. Creates the node, marks the container
 * as expanded, updates the definition tree, and captures undo state.
 */

import type { NodeTypeName } from "@bnto/core";
import { isContainerNodeType } from "@bnto/core";
import type { EditorState } from "../store/types";
import type { BentoNode } from "../adapters/types";
import type { AddNodeResult } from "./types";
import { createCompartmentNode } from "../adapters/createCompartmentNode";
import { withUndo } from "../store/withUndo";
import { addChildToContainer } from "../adapters/definitionTreeHelpers";
import { buildChildDefinition } from "./buildChildDefinition";

/** Deselect all nodes and ensure the container is marked expanded. */
function prepareNodes(nodes: BentoNode[], containerId: string): BentoNode[] {
  return nodes.map((n) => {
    if (n.selected) n = { ...n, selected: false };
    if (n.id === containerId && !n.data.isExpanded) {
      n = { ...n, data: { ...n.data, isExpanded: true } };
    }
    return n;
  });
}

/** Build the child BentoNode with parent context. */
function buildChildNode(
  base: BentoNode,
  containerId: string,
  parentDepth: number,
  type: NodeTypeName,
): BentoNode {
  const childIsContainer = isContainerNodeType(type);
  return {
    ...base,
    selected: true,
    position: { x: 0, y: 0 },
    data: {
      ...base.data,
      parentContainerId: containerId,
      depth: parentDepth + 1,
      ...(childIsContainer ? { isContainer: true, isExpanded: true } : {}),
    },
  };
}

/** Resolve the insert position within the deselected nodes array. */
function resolveChildInsertAt(
  deselected: BentoNode[],
  containerId: string,
  afterNodeId?: string | null,
): number {
  if (afterNodeId)
    return deselected.findIndex((n) => n.id === afterNodeId) + 1 || deselected.length;
  return deselected.findIndex((n) => n.id === containerId) + 1;
}

/** Compute the expanded container IDs set after adding a child. */
function nextExpandedSet(
  state: EditorState,
  containerId: string,
  type: NodeTypeName,
  nodeId: string,
): Set<string> {
  const next = new Set(state.expandedContainerIds);
  next.add(containerId);
  if (isContainerNodeType(type)) next.add(nodeId);
  return next;
}

function addChildIntoContainer(
  state: EditorState,
  type: NodeTypeName,
  containerId: string,
  afterNodeId?: string | null,
  defaultParams?: Record<string, unknown>,
): AddNodeResult | null {
  const container = state.nodes.find((n) => n.id === containerId);
  if (!container) return null;

  const result = createCompartmentNode(type, state.nodes.length, undefined, defaultParams);
  if (!result) return null;

  const deselected = prepareNodes(state.nodes, containerId);
  const newNode = buildChildNode(result.node, containerId, container.data.depth ?? 0, type);
  const insertAt = resolveChildInsertAt(deselected, containerId, afterNodeId);

  const nextNodes = [...deselected.slice(0, insertAt), newNode, ...deselected.slice(insertAt)];
  const childDef = buildChildDefinition(result.node.id, type, result.config);
  const nextDef = state.definition
    ? addChildToContainer(state.definition, containerId, childDef, afterNodeId ?? undefined)
    : state.definition;

  return {
    nextState: withUndo(state, {
      nodes: nextNodes,
      configs: { ...state.configs, [result.node.id]: result.config },
      expandedContainerIds: nextExpandedSet(state, containerId, type, result.node.id),
      ...(nextDef !== state.definition ? { definition: nextDef } : {}),
    }),
    nodeId: result.node.id,
  };
}

export { addChildIntoContainer };
