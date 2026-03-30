"use client";

import type { RecipeDefn } from "../_stores/recipeStepperContext";
import { useRecipeDefinition } from "./useRecipeDefinition";

/** Build the static recipe definition from the slug hook. */
export function useRecipeDefinitionFromSlug(slug: string): RecipeDefn {
  const hook = useRecipeDefinition(slug);
  return {
    definition: hook.definition,
    acceptLabel: hook.acceptLabel,
    dropzoneAccept: hook.dropzoneAccept,
  };
}
