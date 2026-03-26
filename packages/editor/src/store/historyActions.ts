/**
 * Pure history actions — undo, redo, pushUndo.
 *
 * Each reads state and returns partial state updates.
 * The store factory calls these and applies with `set()`.
 */

import type { EditorState } from "./types";
import { captureSnapshot } from "./captureSnapshot";
import { pushToStack } from "./pushToStack";
import { revalidateState } from "./revalidateState";

/** Capture a snapshot and push it onto the undo stack, clearing redo. */
function pushUndoAction(state: EditorState): Partial<EditorState> {
  const snapshot = captureSnapshot(
    state.nodes,
    state.configs,
    state.definition,
    state.expandedContainerIds,
  );
  return {
    undoStack: pushToStack(state.undoStack, snapshot),
    redoStack: [],
  };
}

/** Pop the most recent undo snapshot and apply it, pushing current state to redo. */
function undoAction(state: EditorState): Partial<EditorState> | null {
  const snapshot = state.undoStack[state.undoStack.length - 1];
  if (!snapshot) return null;
  const current = captureSnapshot(
    state.nodes,
    state.configs,
    state.definition,
    state.expandedContainerIds,
  );
  return {
    nodes: snapshot.nodes,
    configs: snapshot.configs,
    definition: snapshot.definition,
    expandedContainerIds: snapshot.expandedContainerIds,
    isDirty: true,
    undoStack: state.undoStack.slice(0, -1),
    redoStack: [...state.redoStack, current],
    validationErrors: revalidateState(snapshot.nodes, snapshot.configs, state.recipeMetadata),
  };
}

/** Pop the most recent redo snapshot and apply it, pushing current state to undo. */
function redoAction(state: EditorState): Partial<EditorState> | null {
  const snapshot = state.redoStack[state.redoStack.length - 1];
  if (!snapshot) return null;
  const current = captureSnapshot(
    state.nodes,
    state.configs,
    state.definition,
    state.expandedContainerIds,
  );
  return {
    nodes: snapshot.nodes,
    configs: snapshot.configs,
    definition: snapshot.definition,
    expandedContainerIds: snapshot.expandedContainerIds,
    isDirty: true,
    undoStack: [...state.undoStack, current],
    redoStack: state.redoStack.slice(0, -1),
    validationErrors: revalidateState(snapshot.nodes, snapshot.configs, state.recipeMetadata),
  };
}

export { pushUndoAction, undoAction, redoAction };
