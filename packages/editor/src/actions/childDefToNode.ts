/**
 * childDefToNode — converts a child Definition into a BentoNode for the graph.
 *
 * Extracted from expandContainer to reduce complexity. Used when materializing
 * container children into the flat editor graph.
 */

import type { Definition, NodeTypeName } from "@bnto/core";
import {
  NODE_TYPE_INFO,
  getNodeIcon,
  getNodeSublabel,
  isIoNodeType,
  isContainerNodeType,
} from "@bnto/core";
import type { BentoNode } from "../adapters/types";
import { CELL, IO_CARD_SIZE } from "../adapters/bentoSlots";
import { CATEGORY_VARIANT } from "../adapters/categoryVariant";

/** Resolve the display label for a child definition node. */
function resolveChildLabel(child: Definition, nodeType: NodeTypeName, isIo: boolean): string {
  const info = NODE_TYPE_INFO[nodeType];
  if (isIo) return info?.label ?? child.type;
  const displayName = child.metadata?.customData?.displayName;
  return displayName || child.name || (info?.label ?? child.type);
}

/** Create a BentoNode from a child Definition for display in the graph. */
function childDefToNode(child: Definition, parentId: string, depth: number): BentoNode {
  const nodeType = child.type as NodeTypeName;
  const info = NODE_TYPE_INFO[nodeType];
  if (!info) {
    console.warn(`[expandContainer] Unknown node type "${child.type}" (id: ${child.id})`);
  }

  const variant = info ? (CATEGORY_VARIANT[info.category] ?? "muted") : "muted";
  const isIo = isIoNodeType(child.type);
  const isContainer = isContainerNodeType(child.type);
  const label = resolveChildLabel(child, nodeType, isIo);

  return {
    id: child.id,
    type: isIo ? ("io" as const) : ("compartment" as const),
    position: { x: 0, y: 0 },
    data: {
      label,
      sublabel: getNodeSublabel(nodeType, child.parameters, child.metadata),
      variant,
      width: isIo ? IO_CARD_SIZE : CELL,
      height: isIo ? IO_CARD_SIZE : CELL,
      status: "idle" as const,
      icon: getNodeIcon(nodeType, child.parameters),
      isIoNode: isIo,
      parentContainerId: parentId,
      depth,
      ...(isContainer ? { isContainer: true, isExpanded: false } : {}),
    },
  };
}

export { childDefToNode };
