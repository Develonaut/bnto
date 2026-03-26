/**
 * buildNewNode — creates a selected BentoNode with parent context.
 *
 * Shared by addNode for modes 2 (sibling child) and 3 (top-level).
 */

import type { BentoNode } from "../adapters/types";

interface BuildNewNodeOptions {
  parentContainerId?: string;
  depth: number;
  isContainer: boolean;
  fallbackPosition: { x: number; y: number };
}

function buildNewNode(base: BentoNode, options: BuildNewNodeOptions): BentoNode {
  const { parentContainerId, depth, isContainer, fallbackPosition } = options;
  return {
    ...base,
    selected: true,
    position: fallbackPosition,
    data: {
      ...base.data,
      ...(parentContainerId ? { parentContainerId, depth } : { depth: 0 }),
      ...(isContainer ? { isContainer: true, isExpanded: true } : {}),
    },
  };
}

export { buildNewNode };
