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

import type { BentoNode } from "../../types";
import { CELL, STRIDE, ROW_OFFSET, GAP_X, GROUP_INSET, getChildDirection } from "../../bentoSlots";

/** Prefix for containerGroup overlay node IDs. */
const GROUP_ID_PREFIX = "__group__";

function layoutNodes(
  nodes: BentoNode[],
  expandedIds: Set<string>,
): BentoNode[] {
  if (expandedIds.size === 0) return nodes;

  // Index children by parentContainerId for O(1) lookup
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

  const result: BentoNode[] = [...topLevel];

  // Process each expanded container in the result set
  // Use a queue to handle nested expansions without mutation during iteration
  const queue = [...result.filter((n) => expandedIds.has(n.id))];

  while (queue.length > 0) {
    const parent = queue.shift()!;
    const children = childrenByParent.get(parent.id);

    // Empty container — still show group overlay around the parent node
    if (!children?.length) {
      result.push(createGroupOverlay(parent, []));
      continue;
    }

    const px = parent.position.x;
    const py = parent.position.y;
    const childCount = children.length;
    const parentDepth = parent.data.depth ?? 0;
    const direction = getChildDirection(parentDepth);

    const positioned: BentoNode[] = [];
    for (let i = 0; i < childCount; i++) {
      const position =
        direction === "vertical"
          ? { x: px, y: py + ROW_OFFSET + i * ROW_OFFSET }
          : { x: px + STRIDE + i * STRIDE, y: py };
      const child: BentoNode = { ...children[i]!, position };
      positioned.push(child);
      result.push(child);

      // If this child is also expanded, add to queue
      if (expandedIds.has(child.id)) {
        queue.push(child);
      }
    }

    // Inject a containerGroup overlay node covering parent + children
    const groupNode = createGroupOverlay(parent, positioned);
    result.push(groupNode);
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
  const width = maxX - minX + CELL + (padding - inset) * 2;
  const height = maxY - minY + CELL + (padding - inset) * 2;

  return {
    id: `${GROUP_ID_PREFIX}${parent.id}`,
    type: "containerGroup" as const,
    position: { x: minX - padding + inset, y: minY - padding + inset },
    selectable: false,
    draggable: false,
    focusable: false,
    // Place behind real nodes so the overlay never blocks clicks
    zIndex: -1,
    data: {
      label: "",
      variant: "muted" as const,
      width,
      height,
      status: "idle" as const,
    },
  } as BentoNode;
}

export { layoutNodes, GROUP_ID_PREFIX };
