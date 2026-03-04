/**
 * Direct node factory — (NodeTypeName, slotIndex) → BentoNode.
 *
 * Skips the Definition intermediary entirely. The mutation hooks
 * call this to create a new RF node in one step instead of the
 * old two-step addNode(definition) + definitionNodeToRfNode().
 *
 * Pure function — no React, no DOM, fully testable.
 */

import type { NodeTypeName } from "@bnto/nodes";
import { NODE_TYPE_INFO, NODE_SCHEMAS } from "@bnto/nodes";
import type { BentoNode } from "./types";
import { SLOTS } from "./bentoSlots";
import { CATEGORY_VARIANT } from "./categoryVariant";

/** Builds default parameters from the schema for a node type. */
function buildDefaultParams(nodeType: NodeTypeName): Record<string, unknown> {
  const schema = NODE_SCHEMAS[nodeType];
  if (!schema) return {};
  const params: Record<string, unknown> = {};
  for (const param of schema.parameters) {
    if (param.default !== undefined) {
      params[param.name] = param.default;
    }
  }
  return params;
}

/**
 * Create a BentoNode directly from a node type and slot index.
 *
 * Returns null if the slot index is out of range (canvas full).
 */
function createCompartmentNode(
  type: NodeTypeName,
  slotIndex: number,
  position?: { x: number; y: number },
): BentoNode | null {
  const slot = SLOTS[slotIndex];
  if (!slot) return null;

  const info = NODE_TYPE_INFO[type];
  const variant = info ? CATEGORY_VARIANT[info.category] ?? "muted" : "muted";
  const label = info?.label ?? type;
  const id = crypto.randomUUID();

  return {
    id,
    type: "compartment" as const,
    position: position ?? { x: slot.x, y: slot.y },
    data: {
      label,
      sublabel: info?.category ?? "",
      variant,
      width: slot.w,
      height: slot.h,
      status: "idle" as const,
      nodeType: type,
      name: label,
      parameters: buildDefaultParams(type),
    },
  };
}

export { createCompartmentNode };
