/**
 * Editor store types.
 *
 * The store owns nodes, edges, and configs (controlled mode).
 * RF receives these as props — the store is the single source of truth.
 */

import type { Edge, NodeChange, EdgeChange } from "@xyflow/react";
import type { ValidationError } from "@bnto/nodes";
import type { BentoNode, NodeConfig, NodeConfigs } from "../adapters/types";
import type { NodeTypeName } from "@bnto/nodes";

// ---------------------------------------------------------------------------
// Execution state — per-node status tracking
// ---------------------------------------------------------------------------

type NodeExecutionStatus = "idle" | "pending" | "active" | "completed" | "failed";

type ExecutionState = Record<string, NodeExecutionStatus>;

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
// State shape
// ---------------------------------------------------------------------------

interface EditorState {
  // --- Graph state (store owns, RF receives as props) ---
  nodes: BentoNode[];
  edges: Edge[];
  configs: NodeConfigs;

  // --- Metadata ---
  recipeMetadata: RecipeMetadata;
  isDirty: boolean;
  validationErrors: ValidationError[];
  executionState: ExecutionState;
  undoStack: EditorSnapshot[];
  redoStack: EditorSnapshot[];
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

  // --- Graph mutations ---
  setNodes: (nodes: BentoNode[]) => void;
  addNode: (type: NodeTypeName, position?: { x: number; y: number }) => string | null;
  removeNode: (id: string) => void;

  // --- Config mutations (no RF re-render) ---
  setConfigs: (configs: NodeConfigs) => void;
  setConfig: (nodeId: string, config: NodeConfig) => void;
  removeConfig: (nodeId: string) => void;
  updateConfigParams: (nodeId: string, params: Record<string, unknown>) => void;

  // --- History ---
  pushUndo: () => void;
  undo: () => void;
  redo: () => void;

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
  RecipeMetadata,
};
