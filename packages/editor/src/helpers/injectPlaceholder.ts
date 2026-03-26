import type { Node } from "@xyflow/react";
import { SLOTS } from "../adapters/bentoSlots";
import type { BentoNode } from "../adapters/types";

/** Stable placeholder node ID — never collides with UUIDs. */
const PLACEHOLDER_ID = "__placeholder__";

/**
 * Inject a placeholder node right before the output node (last in array).
 * All preceding nodes keep their positions; placeholder takes the slot
 * where output was, and output shifts one slot right.
 *
 * Returns the original array unchanged when fewer than 2 nodes or
 * no remaining slots.
 */
function injectPlaceholder(nodes: BentoNode[]): BentoNode[] {
  if (nodes.length < 2) return nodes;

  const placeholderSlot = nodes.length - 1;
  const outputSlot = nodes.length;

  const pSlot = SLOTS[placeholderSlot];
  const oSlot = SLOTS[outputSlot];
  const lastNode = nodes[nodes.length - 1];
  if (!pSlot || !oSlot || !lastNode) return nodes;

  const placeholder: Node = {
    id: PLACEHOLDER_ID,
    type: "placeholder",
    position: { x: pSlot.x, y: pSlot.y },
    selectable: false,
    draggable: false,
    data: {},
  };

  const before = nodes.slice(0, -1);
  const output = {
    ...lastNode,
    position: { x: oSlot.x, y: oSlot.y },
  };

  return [...before, placeholder as BentoNode, output];
}

export { injectPlaceholder, PLACEHOLDER_ID };
