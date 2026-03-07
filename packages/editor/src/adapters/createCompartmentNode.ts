/**
 * Direct node factory — (NodeTypeName, slotIndex) → BentoNode + NodeConfig.
 *
 * Returns both the visual RF node (thin data) and the domain config
 * as separate objects. The store adds the node to its nodes array and
 * the config to its configs map.
 *
 * Pure function — no React, no DOM, fully testable.
 */

import type { NodeTypeName } from "@bnto/nodes";
import {
  NODE_TYPE_INFO,
  NODE_SCHEMA_DEFS,
  getNodeIcon,
  getNodeSublabel,
  isIoNodeType,
} from "@bnto/nodes";
import type { BentoNode, NodeConfig } from "./types";
import { SLOTS, IO_CARD_SIZE } from "./bentoSlots";
import { CATEGORY_VARIANT } from "./categoryVariant";

/** Builds default parameters from the Zod schema for a node type. */
function buildDefaultParams(nodeType: NodeTypeName): Record<string, unknown> {
  const schemaDef = NODE_SCHEMA_DEFS[nodeType];
  if (!schemaDef) return {};
  // Parse an empty object through the schema to get Zod defaults
  const result = schemaDef.schema.safeParse({});
  if (result.success) return { ...result.data };
  // If parsing fails (required fields missing), extract defaults manually
  const shape = schemaDef.schema.shape as Record<string, unknown>;
  const params: Record<string, unknown> = {};
  for (const [name, field] of Object.entries(shape)) {
    const def = (field as { _def?: { typeName?: string; defaultValue?: () => unknown } })?._def;
    if (def?.typeName === "ZodDefault" && typeof def.defaultValue === "function") {
      params[name] = def.defaultValue();
    }
  }
  return params;
}

interface CompartmentNodeResult {
  node: BentoNode;
  config: NodeConfig;
}

/**
 * Create a BentoNode + NodeConfig from a node type and slot index.
 *
 * Returns null if the slot index is out of range (canvas full).
 */
function createCompartmentNode(
  type: NodeTypeName,
  slotIndex: number,
  position?: { x: number; y: number },
): CompartmentNodeResult | null {
  const slot = SLOTS[slotIndex];
  if (!slot) return null;

  const info = NODE_TYPE_INFO[type];
  const variant = info ? (CATEGORY_VARIANT[info.category] ?? "muted") : "muted";
  const label = info?.label ?? type;
  const id = crypto.randomUUID();
  const parameters = buildDefaultParams(type);

  const isIo = isIoNodeType(type);
  // Icon via getNodeIcon — single source of truth from @bnto/nodes.
  const icon = getNodeIcon(type, parameters);
  const sublabel = getNodeSublabel(type, parameters);
  // All nodes use uniform CELL×CELL slots — no y-offset math.
  // I/O visual size is handled by the renderer.
  const defaultPos = { x: slot.x, y: slot.y };

  const node: BentoNode = {
    id,
    type: isIo ? ("io" as const) : ("compartment" as const),
    position: position ?? defaultPos,
    data: {
      label,
      sublabel,
      variant,
      width: isIo ? IO_CARD_SIZE : slot.w,
      height: isIo ? IO_CARD_SIZE : slot.h,
      status: "idle" as const,
      icon,
      isIoNode: isIo,
    },
  };

  const config: NodeConfig = {
    nodeType: type,
    name: label,
    parameters,
  };

  return { node, config };
}

export { createCompartmentNode };
export type { CompartmentNodeResult };
