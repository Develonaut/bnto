/**
 * Editor store types.
 *
 * ReactFlow is the single source of truth for node state.
 * The store owns metadata, history, validation, and execution.
 */

import type { ValidationError } from "@bnto/nodes";
import type { BentoNode } from "../adapters/types";

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
// Node getter — registered by the visual editor
// ---------------------------------------------------------------------------

/** Reads current RF nodes without importing React. */
type NodeGetter = () => BentoNode[];

// ---------------------------------------------------------------------------
// State shape
// ---------------------------------------------------------------------------

interface EditorState {
  recipeMetadata: RecipeMetadata;
  isDirty: boolean;
  validationErrors: ValidationError[];
  executionState: ExecutionState;
  undoStack: BentoNode[][];
  redoStack: BentoNode[][];
}

// ---------------------------------------------------------------------------
// Actions
// ---------------------------------------------------------------------------

interface EditorActions {
  loadRecipe: (slug: string) => void;
  createBlank: () => void;

  /** Snapshot current RF nodes to undo stack, clear redo. */
  pushUndo: () => void;
  /** Set isDirty = true. */
  markDirty: () => void;
  /** Derive Definition from RF nodes, run validateDefinition. */
  revalidate: () => void;

  /** Returns BentoNode[] snapshot so caller can setNodes(). Null if empty. */
  undo: () => BentoNode[] | null;
  /** Returns BentoNode[] snapshot so caller can setNodes(). Null if empty. */
  redo: () => BentoNode[] | null;

  resetDirty: () => void;
  setExecutionState: (state: ExecutionState) => void;
  resetExecution: () => void;
  setNodeGetter: (getter: NodeGetter) => void;
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
  NodeExecutionStatus,
  ExecutionState,
  RecipeMetadata,
  NodeGetter,
};
