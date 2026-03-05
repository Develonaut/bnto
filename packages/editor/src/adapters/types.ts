/**
 * Adapter types — shared between Definition ↔ Bento adapters.
 *
 * node.data is visual-only (rendering CompartmentNode).
 * Domain data lives in configs[nodeId] (NodeConfig).
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
  | "warning"
  | "info";

// ---------------------------------------------------------------------------
// Compartment data — visual fields only (thin node.data)
// ---------------------------------------------------------------------------

/**
 * Visual-only data for the CompartmentNode renderer.
 *
 * Domain fields (nodeType, name, parameters) live in the configs
 * store — NOT in RF node.data. This prevents parameter changes from
 * triggering RF's change pipeline and re-rendering CompartmentNode.
 *
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
  /** Icon identifier string (from NodeTypeInfo.icon) — resolved to a component by the renderer. */
  icon?: string;
};

// ---------------------------------------------------------------------------
// Domain config — lives in configs store, keyed by node ID
// ---------------------------------------------------------------------------

/** Domain state for a single node — stored outside RF in the configs map. */
type NodeConfig = {
  nodeType: string;
  name: string;
  parameters: Record<string, unknown>;
};

/** Map of node ID → domain config. Keyed by RF node.id. */
type NodeConfigs = Record<string, NodeConfig>;

// ---------------------------------------------------------------------------
// ReactFlow-compatible node — extends RF's Node with typed data
// ---------------------------------------------------------------------------

type BentoNode = Node<CompartmentNodeData, "compartment">;

type BentoLayout = {
  nodes: BentoNode[];
  configs: NodeConfigs;
};

export type {
  CompartmentVariant,
  CompartmentNodeData,
  NodeConfig,
  NodeConfigs,
  BentoNode,
  BentoLayout,
};
