/**
 * loadRecipe action — pure function that computes the next editor state
 * after loading a predefined recipe by slug.
 *
 * Looks up the recipe in the registry, converts to bento nodes/configs,
 * extracts metadata, and resets history + execution state.
 *
 * Returns null if the slug is not found.
 */

import { getRecipeBySlug, validateDefinition } from "@bnto/nodes";
import type { EditorState } from "../store/types";
import { definitionToBento } from "../adapters/definitionToBento";
import { metadataFromDefinition } from "../store/resolveInitialState";

export function loadRecipe(slug: string): Partial<EditorState> | null {
  const recipe = getRecipeBySlug(slug);
  if (!recipe) return null;
  const { nodes, configs } = definitionToBento(recipe.definition);
  return {
    nodes,
    configs,
    recipeMetadata: metadataFromDefinition(recipe.definition),
    isDirty: false,
    validationErrors: validateDefinition(recipe.definition),
    executionState: {},
    undoStack: [],
    redoStack: [],
  };
}
