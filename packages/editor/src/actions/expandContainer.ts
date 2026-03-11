/**
 * expandContainer action — materializes a container's children into the graph.
 *
 * Reads children from `state.definition` for the given container nodeId,
 * creates BentoNodes + NodeConfigs for each child (matching definitionToGraph
 * patterns), and adds them to `state.nodes` + `state.configs`.
 *
 * Returns null if the container has no definition, is already expanded,
 * or exceeds max nesting depth.
 */

import type { Definition, NodeTypeName } from "@bnto/nodes";
import { NODE_TYPE_INFO, getNodeIcon, getNodeSublabel, isIoNodeType, isContainerNodeType } from "@bnto/nodes";
import type { EditorState } from "../store/types";
import type { BentoNode, NodeConfigs } from "../adapters/types";
import { CELL, IO_CARD_SIZE, MAX_CONTAINER_DEPTH } from "../adapters/bentoSlots";
import { CATEGORY_VARIANT } from "../adapters/categoryVariant";
import { findDefinitionById } from "../adapters/findDefinitionById";
import { withUndo } from "../store/withUndo";

/** Create a BentoNode from a child Definition for display in the graph. */
function childDefToNode(
  child: Definition,
  parentId: string,
  depth: number,
): BentoNode {
  const nodeType = child.type as NodeTypeName;
  const info = NODE_TYPE_INFO[nodeType];
  const variant = info ? (CATEGORY_VARIANT[info.category] ?? "muted") : "muted";
  const isIo = isIoNodeType(child.type);
  const isContainer = isContainerNodeType(child.type);
  const displayName = child.metadata?.customData?.displayName;
  const label = isIo
    ? (info?.label ?? child.type)
    : displayName || child.name || (info?.label ?? child.type);
  const icon = getNodeIcon(nodeType, child.parameters);
  const sublabel = getNodeSublabel(nodeType, child.parameters, child.metadata);

  return {
    id: child.id,
    type: isIo ? ("io" as const) : ("compartment" as const),
    // Position will be computed by layoutNodes — use (0,0) as placeholder
    position: { x: 0, y: 0 },
    data: {
      label,
      sublabel,
      variant,
      width: isIo ? IO_CARD_SIZE : CELL,
      height: isIo ? IO_CARD_SIZE : CELL,
      status: "idle" as const,
      icon,
      isIoNode: isIo,
      parentContainerId: parentId,
      depth,
      ...(isContainer ? { isContainer: true, isExpanded: false } : {}),
    },
  };
}

export function expandContainer(
  state: EditorState,
  nodeId: string,
): Partial<EditorState> | null {
  if (!state.definition) return null;
  if (state.expandedContainerIds.has(nodeId)) return null;

  // Check depth limit
  const parentNode = state.nodes.find((n) => n.id === nodeId);
  if (!parentNode) return null;
  const parentDepth = parentNode.data.depth ?? 0;
  if (parentDepth >= MAX_CONTAINER_DEPTH) return null;

  // Find the container in the definition tree
  const containerDef = findDefinitionById(state.definition, nodeId);
  if (!containerDef) return null;

  const children = containerDef.nodes ?? [];
  const childDepth = parentDepth + 1;

  // Materialize children into graph
  const childNodes: BentoNode[] = children.map((child) =>
    childDefToNode(child, nodeId, childDepth),
  );

  const childConfigs: NodeConfigs = {};
  for (const child of children) {
    const displayName = child.metadata?.customData?.displayName;
    childConfigs[child.id] = {
      nodeType: child.type,
      name: child.name,
      ...(displayName ? { displayName } : {}),
      parameters: child.parameters,
    };
  }

  // Update parent node to show expanded state
  const nextNodes = state.nodes.map((n) =>
    n.id === nodeId
      ? { ...n, data: { ...n.data, isExpanded: true } }
      : n,
  );

  const nextExpandedIds = new Set(state.expandedContainerIds);
  nextExpandedIds.add(nodeId);

  return withUndo(state, {
    nodes: [...nextNodes, ...childNodes],
    configs: { ...state.configs, ...childConfigs },
    expandedContainerIds: nextExpandedIds,
  });
}
