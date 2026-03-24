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
  | "destructive"
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
  status: "idle" | "pending" | "active" | "completed" | "failed";
  /** Per-node execution progress (0–100). Undefined when idle. */
  progress?: number;
  /** Icon identifier string (from NodeTypeInfo.icon) — resolved to a component by the renderer. */
  icon?: string;
  /** True for input/output nodes — protects them from deletion and reordering. */
  isIoNode?: boolean;
  /** Which container this child belongs to. Absent for top-level nodes. */
  parentContainerId?: string;
  /** Whether this node is a container type (group, loop, parallel). */
  isContainer?: boolean;
  /** Whether this container's children are expanded on the canvas. */
  isExpanded?: boolean;
  /** Direction children flow when expanded: vertical (down) or horizontal (right). */
  childDirection?: "vertical" | "horizontal";
  /** Nesting depth: 0 = top-level, 1 = first nesting level, etc. */
  depth?: number;
  /** Divider direction: horizontal (top-level gaps) or vertical (child gaps). */
  dividerDirection?: "horizontal" | "vertical";
  /** The node ID this divider sits after (null = before first child). */
  dividerAfterNodeId?: string | null;
  /** The container ID to insert into (null = top-level insertion). */
  dividerIntoContainerId?: string | null;
  /** Hide the dashed line — edge dividers overlap with the group border. */
  dividerHideLine?: boolean;
};

// ---------------------------------------------------------------------------
// Domain config — lives in configs store, keyed by node ID
// ---------------------------------------------------------------------------

/** Domain state for a single node — stored outside RF in the configs map. */
type NodeConfig = {
  nodeType: string;
  name: string;
  /** User-friendly label from metadata.customData.displayName (optional). */
  displayName?: string;
  parameters: Record<string, unknown>;
};

/** Map of node ID → domain config. Keyed by RF node.id. */
type NodeConfigs = Record<string, NodeConfig>;

// ---------------------------------------------------------------------------
// ReactFlow-compatible node — extends RF's Node with typed data
// ---------------------------------------------------------------------------

type BentoNode = Node<
  CompartmentNodeData,
  "compartment" | "io" | "containerGroup" | "addDivider" | "placeholder"
>;

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
