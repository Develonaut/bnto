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
 * Contains both visual fields (for rendering CompartmentNode) and
 * domain fields (RF as single source of truth for node state).
 *
 * Uses `type` (not `interface`) so it satisfies RF's
 * `Record<string, unknown>` constraint on Node data.
 */
type CompartmentNodeData = {
  // --- Visual fields (for rendering) ---
  label: string;
  sublabel?: string;
  variant: CompartmentVariant;
  width: number;
  height: number;
  status: "idle" | "pending" | "active" | "completed";

  // --- Domain fields (RF as source of truth) ---
  /** Node type name (e.g., "image", "spreadsheet", "loop"). */
  nodeType: string;
  /** Human-readable node name from the Definition. */
  name: string;
  /** Node-specific configuration parameters. */
  parameters: Record<string, unknown>;

  /**
   * Original node ID — links compartment back to the Definition node.
   * @deprecated Redundant with RF node `id`. Kept for compat, remove in PR 7.
   */
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
