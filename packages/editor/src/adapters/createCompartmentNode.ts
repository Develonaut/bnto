/**
 * Direct node factory — (NodeTypeName, slotIndex) → BentoNode + NodeConfig.
 *
 * Returns both the visual RF node (thin data) and the domain config
 * as separate objects. The store adds the node to its nodes array and
 * the config to its configs map.
 *
 * Pure function — no React, no DOM, fully testable.
 */

import type { NodeTypeName } from "@bnto/core";
import {
  NODE_TYPE_INFO,
  NODE_SCHEMAS,
  getNodeIcon,
  getNodeSublabel,
  isIoNodeType,
  isContainerNodeType,
} from "@bnto/core";
import type { BentoNode, CompartmentVariant, NodeConfig } from "./types";
import { SLOTS, IO_CARD_SIZE } from "./bentoSlots";
import { CATEGORY_VARIANT } from "./categoryVariant";

/** Builds default parameters from the Zod schema for a node type. */
function buildDefaultParams(nodeType: NodeTypeName): Record<string, unknown> {
  const schemaDef = NODE_SCHEMAS[nodeType];
  if (!schemaDef) return {};
  const result = schemaDef.schema.safeParse({});
  if (result.success) return { ...result.data };
  return extractZodDefaults(schemaDef.schema.shape as Record<string, unknown>);
}

/** Extract defaults from Zod schema shape when safeParse fails. */
function extractZodDefaults(shape: Record<string, unknown>): Record<string, unknown> {
  const params: Record<string, unknown> = {};
  for (const [name, field] of Object.entries(shape)) {
    const def = (field as { _def?: { typeName?: string; defaultValue?: () => unknown } })?._def;
    if (def?.typeName === "ZodDefault" && typeof def.defaultValue === "function") {
      params[name] = def.defaultValue();
    }
  }
  return params;
}

/** Human-readable labels for loop modes. */
const LOOP_MODE_LABELS: Record<string, string> = {
  forEach: "For Each",
  times: "Times",
  while: "While",
};

/** Derive a short card label from the node's mode parameter or type info. */
function deriveLabel(
  type: NodeTypeName,
  parameters: Record<string, unknown>,
  fallback: string,
): string {
  if (isContainerNodeType(type)) {
    const mode = parameters.mode as string | undefined;
    const modeLabel = mode ? LOOP_MODE_LABELS[mode] : undefined;
    if (modeLabel) return modeLabel;
  }
  return fallback;
}

interface CompartmentNodeResult {
  node: BentoNode;
  config: NodeConfig;
}

/**
 * Create a BentoNode + NodeConfig from a node type and slot index.
 * Returns null if the slot index is out of range (canvas full).
 */
function createCompartmentNode(
  type: NodeTypeName,
  slotIndex: number,
  position?: { x: number; y: number },
  defaultParams?: Record<string, unknown>,
): CompartmentNodeResult | null {
  const slot = SLOTS[slotIndex];
  if (!slot) return null;

  const info = NODE_TYPE_INFO[type];
  const variant = info ? (CATEGORY_VARIANT[info.category] ?? "muted") : "muted";
  const id = crypto.randomUUID();
  const parameters = { ...buildDefaultParams(type), ...defaultParams };
  const label = deriveLabel(type, parameters, info?.label ?? type);
  const isIo = isIoNodeType(type);

  return buildResult(id, type, label, isIo, variant, parameters, slot, position);
}

/** Assemble the BentoNode + NodeConfig result object. */
function buildResult(
  id: string,
  type: NodeTypeName,
  label: string,
  isIo: boolean,
  variant: CompartmentVariant,
  parameters: Record<string, unknown>,
  slot: (typeof SLOTS)[number],
  position?: { x: number; y: number },
): CompartmentNodeResult {
  const node: BentoNode = {
    id,
    type: isIo ? ("io" as const) : ("compartment" as const),
    position: position ?? { x: slot.x, y: slot.y },
    data: {
      label,
      sublabel: getNodeSublabel(type, parameters),
      variant,
      width: isIo ? IO_CARD_SIZE : slot.w,
      height: isIo ? IO_CARD_SIZE : slot.h,
      status: "idle" as const,
      icon: getNodeIcon(type, parameters),
      isIoNode: isIo,
    },
  };
  return { node, config: { nodeType: type, name: label, parameters } };
}

export { createCompartmentNode };
export type { CompartmentNodeResult };
