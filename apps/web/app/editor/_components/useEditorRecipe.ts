"use client";

import { useMemo } from "react";
import { useSearchParams } from "next/navigation";
import { core } from "@bnto/core";
import type { Definition } from "@bnto/core";

interface EditorRecipeResult {
  /** Resolved definition (undefined while loading or if not found). */
  definition: Definition | undefined;
  /** Convex cloud ID -- set only for cloud-origin recipes. */
  cloudId: string | undefined;
  /** True while a cloud recipe is being fetched. */
  isLoading: boolean;
  /** True when a cloud recipe was requested but not found. */
  notFound: boolean;
}

/** Result for a locally-found recipe. */
function localResult(recipe: {
  definition: Definition;
  cloudId?: string | null;
}): EditorRecipeResult {
  return {
    definition: recipe.definition,
    cloudId: recipe.cloudId ?? undefined,
    isLoading: false,
    notFound: false,
  };
}

/** Result for a cloud-fetched recipe. */
function cloudResult(
  recipeId: string,
  cloudRecipe: { definition?: unknown } | null | undefined,
  isLoading: boolean,
): EditorRecipeResult {
  return {
    definition: cloudRecipe?.definition as Definition | undefined,
    cloudId: recipeId,
    isLoading,
    notFound: !isLoading && !cloudRecipe?.definition,
  };
}

/** Result for a blank canvas (no recipe param). */
const BLANK_RESULT: EditorRecipeResult = {
  definition: undefined,
  cloudId: undefined,
  isLoading: false,
  notFound: false,
};

/**
 * Resolves the recipe definition for the editor from search params.
 *
 * Resolution:
 *   ?recipe=[id] -> local recipesStore, then Convex cloud
 *   No params    -> blank canvas (undefined definition)
 */
export function useEditorRecipe(): EditorRecipeResult {
  const searchParams = useSearchParams();
  const recipeId = searchParams.get("recipe") ?? undefined;

  const localRecipe = useMemo(
    () => (recipeId ? (core.recipes.get(recipeId) ?? null) : null),
    [recipeId],
  );

  const needsCloudFetch = !!recipeId && !localRecipe;
  const { data: cloudRecipe, isLoading: cloudLoading } = core.recipes.useRecipe(
    needsCloudFetch ? recipeId : "",
  );

  if (recipeId && localRecipe) return localResult(localRecipe);
  if (recipeId) return cloudResult(recipeId, cloudRecipe, cloudLoading);
  return BLANK_RESULT;
}
