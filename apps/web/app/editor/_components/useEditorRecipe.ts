"use client";

import { useMemo } from "react";
import { useSearchParams } from "next/navigation";
import { core } from "@bnto/core";
import type { Definition } from "@bnto/core";

interface EditorRecipeResult {
  /** Resolved definition (undefined while loading or if not found). */
  definition: Definition | undefined;
  /** Convex cloud ID — set only for cloud-origin recipes. */
  cloudId: string | undefined;
  /** True while a cloud recipe is being fetched. */
  isLoading: boolean;
  /** True when a cloud recipe was requested but not found. */
  notFound: boolean;
}

/**
 * Resolves the recipe definition for the editor from search params.
 *
 * Resolution:
 *   ?recipe=[id] → local recipesStore, then Convex cloud
 *   No params    → blank canvas (undefined definition)
 */
export function useEditorRecipe(): EditorRecipeResult {
  const searchParams = useSearchParams();
  const recipeId = searchParams.get("recipe") ?? undefined;

  // Synchronous store read — Zustand persist middleware hydrates from
  // localStorage synchronously on first access.
  const localRecipe = useMemo(
    () => (recipeId ? (core.recipes.get(recipeId) ?? null) : null),
    [recipeId],
  );

  // Cloud fetch — skips when no recipeId or found locally (adapter uses "skip" for "").
  const needsCloudFetch = !!recipeId && !localRecipe;
  const { data: cloudRecipe, isLoading: cloudLoading } = core.recipes.useRecipe(
    needsCloudFetch ? recipeId : "",
  );

  // Local recipe found — use it directly (preserve cloudId for cloud sync)
  if (recipeId && localRecipe) {
    return {
      definition: localRecipe.definition,
      cloudId: localRecipe.cloudId ?? undefined,
      isLoading: false,
      notFound: false,
    };
  }

  // Cloud recipe requested
  if (recipeId) {
    return {
      definition: cloudRecipe?.definition as Definition | undefined,
      cloudId: recipeId,
      isLoading: cloudLoading,
      notFound: !cloudLoading && !cloudRecipe?.definition,
    };
  }

  // Blank canvas
  return {
    definition: undefined,
    cloudId: undefined,
    isLoading: false,
    notFound: false,
  };
}
