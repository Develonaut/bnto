/**
 * Single Definition node → ReactFlow BentoNode adapter.
 *
 * Converts one Definition child node into one BentoNode for ReactFlow.
 * Used by useDefinitionSync when addNode inserts a new child — avoids
 * full re-conversion of the entire definition.
 *
 * Pure function — no React, no DOM, fully testable.
 */

import type { NodeTypeName } from "@bnto/nodes";
import { NODE_TYPE_INFO } from "@bnto/nodes";
import type { Definition } from "@bnto/nodes";
import type { BentoNode } from "./types";
import { SLOTS } from "./bentoSlots";
import { CATEGORY_VARIANT } from "./categoryVariant";

function definitionNodeToRfNode(
  node: Definition,
  slotIndex: number,
): BentoNode | null {
  const slot = SLOTS[slotIndex];
  if (!slot) return null;

  const info = NODE_TYPE_INFO[node.type as NodeTypeName];
  const variant = info
    ? CATEGORY_VARIANT[info.category] ?? "muted"
    : "muted";

  return {
    id: node.id,
    type: "compartment" as const,
    position: node.position ?? { x: slot.x, y: slot.y },
    data: {
      // Visual fields
      label: node.name || (info?.label ?? node.type),
      sublabel: info?.category ?? "",
      variant,
      width: slot.w,
      height: slot.h,
      status: "idle" as const,
      // Domain fields — RF as source of truth
      nodeType: node.type,
      name: node.name,
      parameters: node.parameters,
      // Deprecated — same as RF node.id
      nodeId: node.id,
    },
  };
}

export { definitionNodeToRfNode };
