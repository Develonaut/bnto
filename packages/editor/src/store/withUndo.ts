/**
 * withUndo — wraps state changes with undo snapshot, dirty flag,
 * redo stack reset, and revalidation.
 *
 * Every editor action that modifies nodes or configs follows the
 * same pattern: capture snapshot → apply changes → clear redo →
 * mark dirty → revalidate. This helper eliminates that boilerplate.
 */

import type { EditorState } from "./types";
import type { BentoNode, NodeConfigs } from "../adapters/types";
import { captureSnapshot } from "./captureSnapshot";
import { pushToStack } from "./pushToStack";
import { revalidateState } from "./revalidateState";

function withUndo(state: EditorState, changes: Partial<EditorState>): Partial<EditorState> {
  const snapshot = captureSnapshot(state.nodes, state.configs);
  const nextNodes = (changes.nodes ?? state.nodes) as BentoNode[];
  const nextConfigs = (changes.configs ?? state.configs) as NodeConfigs;

  return {
    ...changes,
    isDirty: true,
    undoStack: pushToStack(state.undoStack, snapshot),
    redoStack: [],
    validationErrors: revalidateState(nextNodes, nextConfigs, state.recipeMetadata),
  };
}

export { withUndo };
