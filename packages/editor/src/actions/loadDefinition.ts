/**
 * loadDefinition action — pure function that computes the next editor state
 * after loading a Definition.
 *
 * This is the primary entry point for initializing the editor with a recipe.
 * The editor accepts definitions directly — slug lookup belongs at the app layer.
 */

import { validateDefinition } from "@bnto/nodes";
import type { Definition } from "@bnto/nodes";
import type { EditorState } from "../store/types";
import { definitionToGraph } from "../adapters/definitionToGraph";
import { metadataFromDefinition } from "../store/resolveInitialState";

export function loadDefinition(def: Definition): Partial<EditorState> {
  const { nodes, configs } = definitionToGraph(def);
  return {
    nodes,
    configs,
    definition: def,
    recipeMetadata: metadataFromDefinition(def),
    isDirty: false,
    validationErrors: validateDefinition(def),
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
  };
}
