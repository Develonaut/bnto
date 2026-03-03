/**
 * Adapter types — shared between Definition ↔ Bento adapters.
 */

import type { Node } from "@xyflow/react";

// ---------------------------------------------------------------------------
// Visual variant colors (maps to CompartmentNode CSS classes)
// ---------------------------------------------------------------------------

type CompartmentVariant =
  | "primary"
  | "secondary"
  | "accent"
  | "muted"
  | "success"
  | "warning";

// ---------------------------------------------------------------------------
// Compartment data for the visual editor (adapter output)
// ---------------------------------------------------------------------------

/**
 * Full compartment data produced by the definitionToBento adapter.
 *
 * All fields are required because the adapter always computes them.
 * Uses `type` (not `interface`) so it satisfies RF's
 * `Record<string, unknown>` constraint on Node data.
 */
type CompartmentNodeData = {
  label: string;
  sublabel?: string;
  variant: CompartmentVariant;
  width: number;
  height: number;
  status: "idle" | "pending" | "active" | "completed";
  /** Original node ID — links compartment back to the Definition node. */
  nodeId: string;
};

// ---------------------------------------------------------------------------
// ReactFlow-compatible node — extends RF's Node with typed data
// ---------------------------------------------------------------------------

type BentoNode = Node<CompartmentNodeData, "compartment">;

type BentoLayout = {
  nodes: BentoNode[];
};

export type {
  CompartmentVariant,
  CompartmentNodeData,
  BentoNode,
  BentoLayout,
};
