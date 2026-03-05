/**
 * Push a snapshot onto an undo/redo stack, capping at MAX_UNDO_HISTORY.
 *
 * Returns a new array — never mutates the input stack.
 */

import type { EditorSnapshot } from "./types";
import { MAX_UNDO_HISTORY } from "./constants";

function pushToStack(stack: EditorSnapshot[], snapshot: EditorSnapshot): EditorSnapshot[] {
  const next = [...stack, snapshot];
  return next.length > MAX_UNDO_HISTORY ? next.slice(-MAX_UNDO_HISTORY) : next;
}

export { pushToStack };
