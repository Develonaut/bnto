"use client";

import { useMemo } from "react";
import { useStore } from "zustand";
import { recipesStore } from "../stores/recipesStore";
import { storedRecipeToListItem } from "../transforms/localRecipe";

/**
 * List all recipes — reactive, store-backed.
 *
 * Returns RecipeListItem[] sorted newest-first. Re-renders when the
 * recipesStore changes (upsert, remove, hydrate). No manual refresh needed.
 */
export function useRecipes() {
  const recipes = useStore(recipesStore, (s) => s.recipes);
  const data = useMemo(
    () =>
      Object.values(recipes)
        .map(storedRecipeToListItem)
        .sort((a, b) => b.updatedAt - a.updatedAt),
    [recipes],
  );
  return { data, isLoading: false };
}
