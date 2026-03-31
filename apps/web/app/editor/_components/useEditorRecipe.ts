"use client";

import { useMemo } from "react";
import { useSearchParams } from "next/navigation";
import { core } from "@bnto/core";
import type { Definition } from "@bnto/core";

interface EditorRecipeResult {
  /** Resolved definition (undefined while loading or if not found). */
  definition: Definition | undefined;
  /** True while resolving. Always false for store-backed lookups. */
  isLoading: boolean;
  /** True when a recipe ID was provided but not found in the store. */
  notFound: boolean;
}

/** Result for a blank canvas (no recipe param). */
const BLANK_RESULT: EditorRecipeResult = {
  definition: undefined,
  isLoading: false,
  notFound: false,
};

/**
 * Resolves the recipe definition for the editor from search params.
 *
 * Resolution:
 *   ?recipe=[id] -> local recipesStore (sessionStorage)
 *   No params    -> blank canvas (undefined definition)
 */
export function useEditorRecipe(): EditorRecipeResult {
  const searchParams = useSearchParams();
  const recipeId = searchParams.get("recipe") ?? undefined;

  const recipe = useMemo(
    () => (recipeId ? (core.recipes.get(recipeId) ?? null) : null),
    [recipeId],
  );

  if (!recipeId) return BLANK_RESULT;

  if (recipe) {
    return {
      definition: recipe.definition,
      isLoading: false,
      notFound: false,
    };
  }

  return {
    definition: undefined,
    isLoading: false,
    notFound: true,
  };
}
