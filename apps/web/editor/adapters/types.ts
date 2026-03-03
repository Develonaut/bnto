/**
 * Adapter types — shared between Definition ↔ Bento adapters.
 */

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
 * This is distinct from CompartmentData in CompartmentNode.tsx, which
 * uses optional fields with defaults for direct UI consumption.
 */
interface CompartmentNodeData {
  label: string;
  sublabel?: string;
  variant: CompartmentVariant;
  width: number;
  height: number;
  status: "idle" | "pending" | "active" | "completed";
  /** Original node ID — links compartment back to the Definition node. */
  nodeId: string;
}

// ---------------------------------------------------------------------------
// ReactFlow-compatible node shape
// ---------------------------------------------------------------------------

interface BentoNode {
  id: string;
  type: "compartment";
  position: { x: number; y: number };
  data: CompartmentNodeData;
}

interface BentoLayout {
  nodes: BentoNode[];
}

export type {
  CompartmentVariant,
  CompartmentNodeData,
  BentoNode,
  BentoLayout,
};
