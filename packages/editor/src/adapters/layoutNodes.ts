/**
 * layoutNodes — repositions nodes based on container expansion state.
 *
 * Top-level nodes stay in a horizontal strip at y=0. When a container
 * is expanded, its children flow in an alternating direction based on
 * nesting depth: even depth = vertical (down), odd depth = horizontal
 * (right). Also injects containerGroup overlay nodes (dashed boundary).
 *
 * Pure function — no React, no DOM, fully testable.
 */

import type { BentoNode } from "./types";
import { CELL, STRIDE, ROW_OFFSET, GAP_X, GROUP_INSET, getChildDirection } from "./bentoSlots";

/** Prefix for containerGroup overlay node IDs. */
const GROUP_ID_PREFIX = "__group__";

/** Index children by parentContainerId and separate top-level nodes. */
function partitionNodes(nodes: BentoNode[]) {
  const childrenByParent = new Map<string, BentoNode[]>();
  const topLevel: BentoNode[] = [];
  for (const node of nodes) {
    const parentId = node.data.parentContainerId;
    if (parentId) {
      const siblings = childrenByParent.get(parentId) ?? [];
      siblings.push(node);
      childrenByParent.set(parentId, siblings);
    } else {
      topLevel.push(node);
    }
  }
  return { childrenByParent, topLevel };
}

/** Position children relative to parent and push to result. */
function positionChildren(
  parent: BentoNode,
  children: BentoNode[],
  expandedIds: Set<string>,
  result: BentoNode[],
  queue: BentoNode[],
): void {
  const direction = getChildDirection(parent.data.depth ?? 0);
  const { x: px, y: py } = parent.position;

  for (let i = 0; i < children.length; i++) {
    const original = children[i];
    if (!original) continue;
    const position =
      direction === "vertical"
        ? { x: px, y: py + ROW_OFFSET + i * ROW_OFFSET }
        : { x: px + STRIDE + i * STRIDE, y: py };
    const child: BentoNode = { ...original, position };
    result.push(child);
    if (expandedIds.has(child.id)) queue.push(child);
  }
}

function layoutNodes(nodes: BentoNode[], expandedIds: Set<string>): BentoNode[] {
  if (expandedIds.size === 0) return nodes;

  const { childrenByParent, topLevel } = partitionNodes(nodes);
  const result: BentoNode[] = [...topLevel];
  const queue = [...result.filter((n) => expandedIds.has(n.id))];

  while (queue.length > 0) {
    const parent = queue.shift();
    if (!parent) continue;
    const children = childrenByParent.get(parent.id);

    if (!children?.length) {
      result.push(createGroupOverlay(parent, []));
      continue;
    }

    const positioned: BentoNode[] = [];
    positionChildren(parent, children, expandedIds, positioned, queue);
    for (const child of positioned) result.push(child);
    result.push(createGroupOverlay(parent, positioned));
  }

  return result;
}

/** Create a containerGroup overlay node sized to cover parent + children. */
function createGroupOverlay(parent: BentoNode, children: BentoNode[]): BentoNode {
  const allX = [parent.position.x, ...children.map((c) => c.position.x)];
  const allY = [parent.position.y, ...children.map((c) => c.position.y)];
  const minX = Math.min(...allX);
  const maxX = Math.max(...allX);
  const minY = Math.min(...allY);
  const maxY = Math.max(...allY);

  const padding = GAP_X;
  const inset = GROUP_INSET;

  return {
    id: `${GROUP_ID_PREFIX}${parent.id}`,
    type: "containerGroup" as const,
    position: { x: minX - padding + inset, y: minY - padding + inset },
    selectable: false,
    draggable: false,
    focusable: false,
    zIndex: -1,
    data: {
      label: "",
      variant: "muted" as const,
      width: maxX - minX + CELL + (padding - inset) * 2,
      height: maxY - minY + CELL + (padding - inset) * 2,
      status: "idle" as const,
    },
  } as BentoNode;
}

export { layoutNodes, GROUP_ID_PREFIX };
