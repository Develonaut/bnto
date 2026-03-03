/**
 * Definition → Bento adapter.
 *
 * Converts a Definition's child nodes into CompartmentNode data for
 * the BentoCanvas. Pure function — no React, no DOM, fully testable.
 *
 * The bento box visual editor doesn't show edges. Execution order is
 * implied by compartment position (top-left → bottom-right).
 */

import type { Definition } from "@bnto/nodes";
import type { NodeTypeName } from "@bnto/nodes";
import { NODE_TYPE_INFO } from "@bnto/nodes";

// ---------------------------------------------------------------------------
// Compartment types (matches CompartmentNode.tsx data shape)
// ---------------------------------------------------------------------------

type CompartmentVariant =
  | "primary"
  | "secondary"
  | "accent"
  | "muted"
  | "success"
  | "warning";

interface CompartmentData {
  label: string;
  sublabel?: string;
  variant: CompartmentVariant;
  width: number;
  height: number;
  status: "idle" | "pending" | "active" | "completed";
  /** Original node ID — links compartment back to the Definition node. */
  nodeId: string;
}

interface BentoNode {
  id: string;
  type: "compartment";
  position: { x: number; y: number };
  data: CompartmentData;
}

interface BentoLayout {
  nodes: BentoNode[];
}

// ---------------------------------------------------------------------------
// Node type → visual mapping
// ---------------------------------------------------------------------------

const CATEGORY_VARIANT: Record<string, CompartmentVariant> = {
  image: "primary",
  spreadsheet: "secondary",
  file: "accent",
  data: "muted",
  network: "warning",
  control: "success",
  system: "warning",
};

// ---------------------------------------------------------------------------
// Bento slot layout (same as BentoBoxShowcase)
// ---------------------------------------------------------------------------

const CELL = 120;
const GAP = 20;
const S = CELL + GAP;

const SLOTS: { x: number; y: number; w: number; h: number }[] = [
  // Row 0 — alternating small + wide
  { x: 0, y: 0, w: 120, h: CELL },
  { x: 140, y: 0, w: 200, h: CELL },
  { x: 360, y: 0, w: 120, h: CELL },
  { x: 500, y: 0, w: 160, h: CELL },
  // Row 1 — two wide panels
  { x: 0, y: S, w: 340, h: CELL },
  { x: 360, y: S, w: 300, h: CELL },
  // Row 2 — four cells
  { x: 0, y: S * 2, w: 160, h: CELL },
  { x: 180, y: S * 2, w: 160, h: CELL },
  { x: 360, y: S * 2, w: 140, h: CELL },
  { x: 520, y: S * 2, w: 140, h: CELL },
];

// ---------------------------------------------------------------------------
// Adapter function
// ---------------------------------------------------------------------------

/**
 * Convert a Definition's child nodes into BentoCanvas compartment nodes.
 *
 * Maps each child node to a visual compartment with:
 * - Variant color based on node category
 * - Label from node name (or NODE_TYPE_INFO label)
 * - Position from the bento slot layout
 * - Dimensions from the slot
 *
 * Nodes beyond the available slots (10) are skipped.
 */
function definitionToBento(definition: Definition): BentoLayout {
  const children = definition.nodes ?? [];

  const nodes: BentoNode[] = children
    .slice(0, SLOTS.length)
    .map((node, index) => {
      const slot = SLOTS[index]!;
      const info = NODE_TYPE_INFO[node.type as NodeTypeName];
      const variant = info
        ? CATEGORY_VARIANT[info.category] ?? "muted"
        : "muted";

      return {
        id: node.id,
        type: "compartment" as const,
        position: { x: slot.x, y: slot.y },
        data: {
          label: node.name || (info?.label ?? node.type),
          sublabel: info?.category ?? "",
          variant,
          width: slot.w,
          height: slot.h,
          status: "idle" as const,
          nodeId: node.id,
        },
      };
    });

  return { nodes };
}

export { definitionToBento, SLOTS, CELL, GAP };
export type { BentoNode, BentoLayout, CompartmentData, CompartmentVariant };
