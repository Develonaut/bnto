/**
 * injectAddDividers — inject divider nodes between consecutive nodes.
 *
 * Pure function: takes positioned nodes and injects non-interactive
 * addDivider nodes in the gaps. Five locations:
 *
 * 1. **Top-level gaps**: Between consecutive top-level nodes (horizontal).
 * 2. **Empty container**: Single divider next to an expanded container with no children.
 * 3. **Before first child**: Between parent and first child on the flow axis.
 * 4. **Between children**: Between consecutive children along the flow axis.
 * 5. **After last child**: After the last child along the flow axis.
 *
 * Container flow direction alternates by depth (even = vertical, odd = horizontal).
 * Dividers use a stable ID pattern so RF doesn't unmount/remount them.
 */

import type { BentoNode } from "../../../adapters/types";
import { CELL, GAP_X, DIVIDER_THIN } from "../../../adapters/bentoSlots";
import { directionConfigFor, type DirectionConfig } from "./directionConfig";

/** Prefix for divider node IDs — filtered by change handlers. */
const ADD_DIVIDER_PREFIX = "__add_divider__";

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

/** Inject dividers for an expanded container's children. */
function injectContainerDividers(
  parent: BentoNode,
  children: BentoNode[] | undefined,
  out: BentoNode[],
) {
  const cfg = directionConfigFor(parent);

  if (!children?.length) {
    injectEmptyContainerDivider(parent, cfg, out);
    return;
  }

  children.sort(cfg.sort);
  injectBeforeFirstChild(parent, children[0]!, cfg, out);
  injectBetweenChildren(children, parent, cfg, out);
  injectAfterLastChild(children[children.length - 1]!, parent, cfg, out);
}

/** Single divider next to an empty container to add the first child. */
function injectEmptyContainerDivider(parent: BentoNode, cfg: DirectionConfig, out: BentoNode[]) {
  const p = cfg.primary(parent) + CELL + (cfg.gap - CELL - DIVIDER_THIN) / 2;
  out.push(createDivider({
    id: `${ADD_DIVIDER_PREFIX}empty__${parent.id}`,
    ...cfg.pos(p, parent),
    width: cfg.width, height: cfg.height,
    direction: cfg.direction,
    afterNodeId: null,
    intoContainerId: parent.id,
  }));
}

/** Divider between parent and first child. */
function injectBeforeFirstChild(parent: BentoNode, first: BentoNode, cfg: DirectionConfig, out: BentoNode[]) {
  const p = cfg.primary(parent) + CELL + (cfg.primary(first) - cfg.primary(parent) - CELL - DIVIDER_THIN) / 2;
  out.push(createDivider({
    id: `${ADD_DIVIDER_PREFIX}first__${parent.id}`,
    ...cfg.pos(p, parent),
    width: cfg.width, height: cfg.height,
    direction: cfg.direction,
    afterNodeId: null,
    intoContainerId: parent.id,
  }));
}

/** Dividers between consecutive children. */
function injectBetweenChildren(children: BentoNode[], parent: BentoNode, cfg: DirectionConfig, out: BentoNode[]) {
  for (let i = 0; i < children.length - 1; i++) {
    const prev = children[i]!;
    const next = children[i + 1]!;
    const p = cfg.primary(prev) + CELL + (cfg.primary(next) - cfg.primary(prev) - CELL - DIVIDER_THIN) / 2;
    out.push(createDivider({
      id: `${ADD_DIVIDER_PREFIX}${prev.id}`,
      ...cfg.pos(p, parent),
      width: cfg.width, height: cfg.height,
      direction: cfg.direction,
      afterNodeId: prev.id,
      intoContainerId: parent.id,
    }));
  }
}

/** Divider after the last child. */
function injectAfterLastChild(last: BentoNode, parent: BentoNode, cfg: DirectionConfig, out: BentoNode[]) {
  const p = cfg.primary(last) + CELL + (cfg.gap - CELL - DIVIDER_THIN) / 2;
  out.push(createDivider({
    id: `${ADD_DIVIDER_PREFIX}last__${parent.id}`,
    ...cfg.pos(p, parent),
    width: cfg.width, height: cfg.height,
    direction: cfg.direction,
    afterNodeId: last.id,
    intoContainerId: parent.id,
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
