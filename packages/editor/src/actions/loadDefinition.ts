/**
 * loadDefinition action — pure function that computes the next editor state
 * after loading a Definition.
 *
 * This is the primary entry point for initializing the editor with a recipe.
 * The editor accepts definitions directly — slug lookup belongs at the app layer.
 *
 * Containers are always expanded — children are materialized on load.
 */

import { validateDefinition, isContainerNodeType } from "@bnto/nodes";
import type { Definition } from "@bnto/nodes";
import type { EditorState } from "../store/types";
import { definitionToGraph } from "../adapters/definitionToGraph";
import { metadataFromDefinition } from "../store/resolveInitialState";
import { expandContainer } from "./expandContainer";

/** Collect IDs of all container nodes in a definition tree. */
function collectContainerIds(def: Definition): string[] {
  const ids: string[] = [];
  for (const node of def.nodes ?? []) {
    if (isContainerNodeType(node.type)) {
      ids.push(node.id);
    }
    // Recurse into children for nested containers
    ids.push(...collectContainerIds(node));
  }
  return ids;
}

export function loadDefinition(def: Definition): Partial<EditorState> {
  const { nodes, configs } = definitionToGraph(def);
  let state: Partial<EditorState> = {
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
    expandedContainerIds: new Set(),
  };

  // Auto-expand all containers so children are always visible
  const containerIds = collectContainerIds(def);
  for (const id of containerIds) {
    const result = expandContainer(state as EditorState, id);
    if (result) {
      state = { ...state, ...result, undoStack: [], redoStack: [], isDirty: false };
    }
  }

  return state;
}
