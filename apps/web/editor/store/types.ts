/**
 * Editor store types — state shape, actions, and execution status.
 *
 * Extracted from createEditorStore.ts for file size compliance.
 */

import type { Definition, Position, NodeTypeName, ValidationError } from "@bnto/nodes";

// ---------------------------------------------------------------------------
// Execution state — per-node status tracking
// ---------------------------------------------------------------------------

type NodeExecutionStatus = "idle" | "pending" | "active" | "completed" | "failed";

type ExecutionState = Record<string, NodeExecutionStatus>;

// ---------------------------------------------------------------------------
// State shape
// ---------------------------------------------------------------------------

interface EditorState {
  /** Current recipe definition tree. */
  definition: Definition;

  /** ID of the currently selected node (null = nothing selected). */
  selectedNodeId: string | null;

  /** Whether the definition has been modified since last save/load. */
  isDirty: boolean;

  /** Validation errors from the most recent operation. */
  validationErrors: ValidationError[];

  /** Per-node execution status map (node ID → status). */
  executionState: ExecutionState;

  /** History stack for undo (previous definitions). */
  undoStack: Definition[];

  /** History stack for redo (undone definitions). */
  redoStack: Definition[];
}

// ---------------------------------------------------------------------------
// Actions
// ---------------------------------------------------------------------------

interface EditorActions {
  /** Load a predefined recipe by slug. Resets history. */
  loadRecipe: (slug: string) => void;

  /** Start with a blank canvas. Resets history. */
  createBlank: () => void;

  /** Add a node of the given type. Position auto-assigned if omitted. */
  addNode: (type: NodeTypeName, position?: Position) => void;

  /** Remove the node with the given ID. Clears selection if it was selected. */
  removeNode: (id: string) => void;

  /** Select a node by ID, or null to deselect. */
  selectNode: (id: string | null) => void;

  /** Update parameters on a node. Validates against schema. */
  updateParams: (nodeId: string, params: Record<string, unknown>) => void;

  /** Move a node to a new position. */
  moveNode: (nodeId: string, position: Position) => void;

  /** Undo the last definition change. */
  undo: () => void;

  /** Redo the last undone change. */
  redo: () => void;

  /** Mark the definition as clean (e.g. after save/export). */
  resetDirty: () => void;

  /** Update per-node execution status. */
  setExecutionState: (state: ExecutionState) => void;

  /** Reset execution state to all idle. */
  resetExecution: () => void;

  /** Replace the definition directly (for code editor sync). */
  setDefinition: (definition: Definition) => void;
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
};
