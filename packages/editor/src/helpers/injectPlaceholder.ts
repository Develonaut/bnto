import type { Node } from "@xyflow/react";
import { SLOTS } from "../adapters/bentoSlots";
import type { BentoNode } from "../adapters/types";

/** Stable placeholder node ID — never collides with UUIDs. */
const PLACEHOLDER_ID = "__placeholder__";

/**
 * Inject a placeholder node between Input and Output when the canvas
 * only has I/O nodes. Input stays at slot 0, placeholder takes slot 1,
 * Output shifts to slot 2.
 *
 * Returns the original array unchanged when injection isn't needed.
 */
function injectPlaceholder(nodes: BentoNode[], onlyIoNodes: boolean): BentoNode[] {
  if (!onlyIoNodes || nodes.length < 2) return nodes;

  const placeholder: Node = {
    id: PLACEHOLDER_ID,
    type: "placeholder",
    position: { x: SLOTS[1]!.x, y: SLOTS[1]!.y },
    selectable: false,
    draggable: false,
    data: {},
  };

  const input = nodes[0]!;
  const output = {
    ...nodes[1]!,
    position: { x: SLOTS[2]!.x, y: SLOTS[2]!.y },
  };

  return [input, placeholder as BentoNode, output];
}

export { injectPlaceholder, PLACEHOLDER_ID };
