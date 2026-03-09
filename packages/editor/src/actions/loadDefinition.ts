/**
 * loadDefinition action — pure function that computes the next editor state
 * after loading a raw Definition (e.g., a sub-recipe primitive).
 *
 * Mirrors loadRecipe but accepts a Definition directly instead of
 * looking it up by slug. Used by the DevTab to load sub-recipes.
 */

import { validateDefinition } from "@bnto/nodes";
import type { Definition } from "@bnto/nodes";
import type { EditorState } from "../store/types";
import { definitionToBento } from "../adapters/definitionToBento";
import { metadataFromDefinition } from "../store/resolveInitialState";

export function loadDefinition(def: Definition): Partial<EditorState> {
  const { nodes, configs } = definitionToBento(def);
  return {
    nodes,
    configs,
    definition: def,
    slug: null,
    recipeMetadata: metadataFromDefinition(def),
    isDirty: false,
    validationErrors: validateDefinition(def),
    executionState: {},
    undoStack: [],
    redoStack: [],
  };
}
