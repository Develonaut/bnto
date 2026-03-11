/**
 * injectAddDividers — inject divider nodes between consecutive nodes.
 *
 * Pure function: takes positioned nodes and injects non-interactive
 * addDivider nodes in the gaps. Five locations:
 *
 * 1. **Top-level gaps**: Between consecutive top-level nodes (horizontal).
 * 2. **Empty container**: Single divider below an expanded container with no children.
 * 3. **Before first child**: Below parent, above the first child.
 * 4. **Between children**: Between consecutive children in a column.
 * 5. **After last child**: Below the last child.
 *
 * Dividers use a stable ID pattern so RF doesn't unmount/remount them.
 */

import type { BentoNode } from "../adapters/types";
import { CELL, GAP_X, ROW_OFFSET } from "../adapters/bentoSlots";

/** Prefix for divider node IDs — filtered by change handlers. */
const ADD_DIVIDER_PREFIX = "__add_divider__";

/** Divider thickness for the non-primary axis. */
const DIVIDER_THIN = 16;

function injectAddDividers(
  nodes: BentoNode[],
  expandedIds: Set<string>,
): BentoNode[] {
  const { topLevel, childrenByParent, hasPlaceholder } = partitionNodes(nodes);
  const dividers: BentoNode[] = [];

  if (!hasPlaceholder) {
    injectTopLevelDividers(topLevel, dividers);
  }

  for (const parentId of expandedIds) {
    const parent = nodes.find((n) => n.id === parentId);
    if (!parent) continue;
    injectContainerDividers(parent, childrenByParent.get(parentId), dividers);
  }

  return dividers.length > 0 ? [...nodes, ...dividers] : nodes;
}

/** Separate real nodes into top-level and children-by-parent. */
function partitionNodes(nodes: BentoNode[]) {
  const topLevel: BentoNode[] = [];
  const childrenByParent = new Map<string, BentoNode[]>();
  let hasPlaceholder = false;

  for (const node of nodes) {
    if (node.type === "containerGroup" || node.type === "addDivider") continue;
    if (node.type === "placeholder") { hasPlaceholder = true; continue; }
    const parentId = node.data.parentContainerId;
    if (parentId) {
      const siblings = childrenByParent.get(parentId) ?? [];
      siblings.push(node);
      childrenByParent.set(parentId, siblings);
    } else {
      topLevel.push(node);
    }
  }

  topLevel.sort((a, b) => a.position.x - b.position.x);
  return { topLevel, childrenByParent, hasPlaceholder };
}

/** Inject horizontal dividers between consecutive top-level nodes. */
function injectTopLevelDividers(topLevel: BentoNode[], out: BentoNode[]) {
  for (let i = 0; i < topLevel.length - 1; i++) {
    const left = topLevel[i]!;
    out.push(createDivider({
      id: `${ADD_DIVIDER_PREFIX}${left.id}`,
      x: left.position.x + CELL + (GAP_X - DIVIDER_THIN) / 2,
      y: left.position.y,
      width: DIVIDER_THIN,
      height: CELL,
      direction: "horizontal",
      afterNodeId: left.id,
      intoContainerId: null,
    }));
  }
}

/** Inject vertical dividers for an expanded container's children. */
function injectContainerDividers(
  parent: BentoNode,
  children: BentoNode[] | undefined,
  out: BentoNode[],
) {
  if (!children?.length) {
    injectEmptyContainerDivider(parent, out);
    return;
  }

  children.sort((a, b) => a.position.y - b.position.y);
  injectBeforeFirstChild(parent, children[0]!, out);
  injectBetweenChildren(children, parent.id, out);
  injectAfterLastChild(children[children.length - 1]!, parent.id, out);
}

/** Single divider below an empty container to add the first child. */
function injectEmptyContainerDivider(parent: BentoNode, out: BentoNode[]) {
  const y = parent.position.y + CELL + (ROW_OFFSET - CELL - DIVIDER_THIN) / 2;
  out.push(createDivider({
    id: `${ADD_DIVIDER_PREFIX}empty__${parent.id}`,
    x: parent.position.x, y,
    width: CELL, height: DIVIDER_THIN,
    direction: "vertical",
    afterNodeId: null,
    intoContainerId: parent.id,
  }));
}

/** Divider between parent and first child. */
function injectBeforeFirstChild(parent: BentoNode, first: BentoNode, out: BentoNode[]) {
  const y = parent.position.y + CELL + (first.position.y - parent.position.y - CELL - DIVIDER_THIN) / 2;
  out.push(createDivider({
    id: `${ADD_DIVIDER_PREFIX}first__${parent.id}`,
    x: parent.position.x, y,
    width: CELL, height: DIVIDER_THIN,
    direction: "vertical",
    afterNodeId: null,
    intoContainerId: parent.id,
  }));
}

/** Dividers between consecutive children. */
function injectBetweenChildren(children: BentoNode[], parentId: string, out: BentoNode[]) {
  for (let i = 0; i < children.length - 1; i++) {
    const above = children[i]!;
    const below = children[i + 1]!;
    const y = above.position.y + CELL + (below.position.y - above.position.y - CELL - DIVIDER_THIN) / 2;
    out.push(createDivider({
      id: `${ADD_DIVIDER_PREFIX}${above.id}`,
      x: above.position.x, y,
      width: CELL, height: DIVIDER_THIN,
      direction: "vertical",
      afterNodeId: above.id,
      intoContainerId: parentId,
    }));
  }
}

/** Divider below the last child. */
function injectAfterLastChild(last: BentoNode, parentId: string, out: BentoNode[]) {
  const y = last.position.y + CELL + (ROW_OFFSET - CELL - DIVIDER_THIN) / 2;
  out.push(createDivider({
    id: `${ADD_DIVIDER_PREFIX}last__${parentId}`,
    x: last.position.x, y,
    width: CELL, height: DIVIDER_THIN,
    direction: "vertical",
    afterNodeId: last.id,
    intoContainerId: parentId,
  }));
}

/** Divider data stored in node.data for the AddDividerNode renderer. */
interface DividerParams {
  id: string;
  x: number;
  y: number;
  width: number;
  height: number;
  direction: "horizontal" | "vertical";
  afterNodeId: string | null;
  intoContainerId: string | null;
}

function createDivider(params: DividerParams): BentoNode {
  return {
    id: params.id,
    type: "addDivider" as const,
    position: { x: params.x, y: params.y },
    selectable: false,
    draggable: false,
    focusable: false,
    data: {
      label: "",
      variant: "muted" as const,
      width: params.width,
      height: params.height,
      status: "idle" as const,
      dividerDirection: params.direction,
      dividerAfterNodeId: params.afterNodeId,
      dividerIntoContainerId: params.intoContainerId,
    },
  } as BentoNode;
}

export { injectAddDividers, ADD_DIVIDER_PREFIX };
