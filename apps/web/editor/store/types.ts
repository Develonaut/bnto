/**
 * Editor store types.
 *
 * ReactFlow owns visual state (positions, selection, viewport).
 * The editor store owns domain structure (node types, params, validation).
 */

import type { Definition, Position, NodeTypeName, ValidationError } from "@bnto/nodes";

// ---------------------------------------------------------------------------
// Execution state — per-node status tracking
// ---------------------------------------------------------------------------

type NodeExecutionStatus = "idle" | "pending" | "active" | "completed" | "failed";

type ExecutionState = Record<string, NodeExecutionStatus>;

// ---------------------------------------------------------------------------
// Undo snapshot — captures both definition and RF positions
// ---------------------------------------------------------------------------

interface UndoSnapshot {
  definition: Definition;
  positions: Record<string, { x: number; y: number }>;
}

// ---------------------------------------------------------------------------
// Position getter — registered by the visual editor
// ---------------------------------------------------------------------------

/** Reads current RF node positions without importing React. */
type PositionGetter = () => Record<string, { x: number; y: number }>;

// ---------------------------------------------------------------------------
// State shape
// ---------------------------------------------------------------------------

interface EditorState {
  definition: Definition;
  isDirty: boolean;
  validationErrors: ValidationError[];
  executionState: ExecutionState;
  undoStack: UndoSnapshot[];
  redoStack: UndoSnapshot[];
}

// ---------------------------------------------------------------------------
// Actions
// ---------------------------------------------------------------------------

interface EditorActions {
  loadRecipe: (slug: string) => void;
  createBlank: () => void;
  addNode: (type: NodeTypeName, position?: Position) => string | null;
  removeNode: (id: string) => void;
  updateParams: (nodeId: string, params: Record<string, unknown>) => void;
  /** Returns snapshot so caller can restore RF positions. */
  undo: () => UndoSnapshot | null;
  /** Returns snapshot so caller can restore RF positions. */
  redo: () => UndoSnapshot | null;
  resetDirty: () => void;
  setExecutionState: (state: ExecutionState) => void;
  resetExecution: () => void;
  setDefinition: (definition: Definition) => void;
  setPositionGetter: (getter: PositionGetter) => void;
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
  UndoSnapshot,
  PositionGetter,
};
