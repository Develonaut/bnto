/**
 * Shared types for editor action functions.
 */

import type { EditorState } from "../store/types";

/** Result of an addNode action — the computed next state and the new node's ID. */
export interface AddNodeResult {
  nextState: Partial<EditorState>;
  nodeId: string;
}
