/**
 * createBlank action — pure function that computes the next editor state
 * for a blank canvas with default I/O nodes.
 *
 * Creates a blank definition, converts to bento nodes/configs,
 * and resets all transient state (history, execution, dirty flag).
 */

import { createBlankDefinition } from "@bnto/nodes";
import type { EditorState } from "../store/types";
import { definitionToBento } from "../adapters/definitionToBento";
import { metadataFromBlank } from "../store/resolveInitialState";

export function createBlank(): Partial<EditorState> {
  const blank = definitionToBento(createBlankDefinition());
  return {
    nodes: blank.nodes,
    configs: blank.configs,
    recipeMetadata: metadataFromBlank(),
    isDirty: false,
    validationErrors: [],
    executionState: {},
    undoStack: [],
    redoStack: [],
  };
}
