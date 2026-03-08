/**
 * Editor store types.
 *
 * The store holds two complementary views of the recipe:
 * - **Graph** (nodes, edges, configs) — flat RF working state for visual editing
 * - **Definition** — the nested tree the engine takes, with container children
 *
 * There is no overlap. Top-level params live in the graph (configs).
 * Nested container children live in the definition. On export,
 * rfNodesToDefinition() merges both into a complete Definition.
 */

import type { Edge, NodeChange, EdgeChange } from "@xyflow/react";
import type { Definition, ValidationError } from "@bnto/nodes";
import type { PipelineEvent } from "@bnto/core";
import type { BentoNode, NodeConfig, NodeConfigs } from "../adapters/types";

// ---------------------------------------------------------------------------
// Execution state — per-node status tracking
// ---------------------------------------------------------------------------

type NodeExecutionStatus = "idle" | "pending" | "active" | "completed" | "failed";

type ExecutionState = Record<string, NodeExecutionStatus>;

/** A single log entry — a pipeline event with a capture timestamp. */
interface RunLogEntry {
  timestamp: number;
  event: PipelineEvent;
}

// ---------------------------------------------------------------------------
// Recipe metadata — root definition fields without child nodes
// ---------------------------------------------------------------------------

interface RecipeMetadata {
  id: string;
  name: string;
  type: string;
  version: string;
}

// ---------------------------------------------------------------------------
// Undo/redo snapshot — captures both nodes and configs atomically
// ---------------------------------------------------------------------------

interface EditorSnapshot {
  nodes: BentoNode[];
  configs: NodeConfigs;
}

// ---------------------------------------------------------------------------
// Panel visibility — centralized panel state
// ---------------------------------------------------------------------------

/** All panel identifiers. Add new panels here — one place to extend. */
type PanelId = "layers" | "config" | "palette" | "run";

/** Map of panel IDs to their open/closed state. */
type PanelState = Record<PanelId, boolean>;

/** Side slot — panels on the same side are mutually exclusive. */
type PanelSide = "left" | "right";

/** Which side each panel lives on. Panels sharing a side are exclusive. */
const PANEL_SIDES: Partial<Record<PanelId, PanelSide>> = {
  layers: "left",
  palette: "left",
  config: "right",
  run: "right",
};

// ---------------------------------------------------------------------------
// State shape
// ---------------------------------------------------------------------------

interface EditorState {
  // --- Graph: flat RF working state (store owns, RF renders as controlled props) ---
  nodes: BentoNode[];
  edges: Edge[];
  configs: NodeConfigs;

  // --- Definition: nested tree the engine takes (container children live here) ---
  definition: Definition | null;

  // --- Metadata ---
  slug: string | null;
  recipeMetadata: RecipeMetadata;
  isDirty: boolean;
  validationErrors: ValidationError[];
  executionState: ExecutionState;
  undoStack: EditorSnapshot[];
  redoStack: EditorSnapshot[];

  // --- Selection ---
  selectedNodeId: string | null;

  // --- Panel visibility ---
  panels: PanelState;
}

// ---------------------------------------------------------------------------
// Actions
// ---------------------------------------------------------------------------

interface EditorActions {
  // --- Entry points ---
  loadRecipe: (slug: string) => void;
  createBlank: () => void;

  // --- RF controlled-mode change handlers ---
  onNodesChange: (changes: NodeChange<BentoNode>[]) => void;
  onEdgesChange: (changes: EdgeChange[]) => void;

  // --- Graph setters ---
  setNodes: (nodes: BentoNode[]) => void;
  selectNode: (id: string | null) => void;

  // --- Config setters ---
  setConfigs: (configs: NodeConfigs) => void;
  setConfig: (nodeId: string, config: NodeConfig) => void;
  removeConfig: (nodeId: string) => void;

  // --- History ---
  pushUndo: () => void;
  undo: () => void;
  redo: () => void;

  // --- Selection ---
  setSelectedNodeId: (id: string | null) => void;

  // --- Panel visibility ---
  openPanel: (id: PanelId) => void;
  closePanel: (id: PanelId) => void;
  togglePanel: (id: PanelId) => void;

  // --- Utility ---
  markDirty: () => void;
  revalidate: () => void;
  resetDirty: () => void;
  setExecutionState: (state: ExecutionState) => void;
  resetExecution: () => void;
  setRecipeMetadata: (metadata: RecipeMetadata) => void;
  resetHistory: () => void;
}

// ---------------------------------------------------------------------------
// Full store type
// ---------------------------------------------------------------------------

type EditorStore = EditorState & EditorActions;

export type {
  EditorStore,
  EditorState,
  EditorActions,
  EditorSnapshot,
  NodeExecutionStatus,
  ExecutionState,
  RunLogEntry,
  RecipeMetadata,
  PanelId,
  PanelState,
  PanelSide,
};
export { PANEL_SIDES };
