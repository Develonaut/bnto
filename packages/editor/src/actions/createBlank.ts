/**
 * createBlank action — pure function that computes the next editor state
 * for a blank canvas with default I/O nodes.
 *
 * Creates a blank definition, converts to bento nodes/configs,
 * and resets all transient state (history, execution, dirty flag).
 */

import { createBlankDefinition } from "@bnto/nodes";
import type { EditorState } from "../store/types";
import { definitionToGraph } from "../adapters/definitionToGraph";
import { metadataFromBlank } from "../store/resolveInitialState";

export function createBlank(): Partial<EditorState> {
  const blankDef = createBlankDefinition();
  const blank = definitionToGraph(blankDef);
  return {
    nodes: blank.nodes,
    configs: blank.configs,
    definition: blankDef,
    recipeMetadata: metadataFromBlank(),
    isDirty: false,
    validationErrors: [],
    executionState: {},
    nodeProgress: {},
    executionPhase: "idle",
    executionResults: [],
    executionErrors: [],
    executionLogs: [],
    executionFileProgress: null,
    executionInputFiles: [],
    undoStack: [],
    redoStack: [],
    expandedContainerIds: new Set(),
  };
}
