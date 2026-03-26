"use client";

import { useEffect, useMemo } from "react";
import { useStore } from "zustand";
import { core } from "../core";
import { recipesStore } from "../stores/recipesStore";
import { recipeToListItem } from "../transforms/recipeToListItem";
import { useIsAuthenticated } from "./useIsAuthenticated";

/**
 * List all recipes — reactive, store-backed.
 *
 * Returns RecipeListItem[] sorted newest-first. Re-renders when the
 * recipesStore changes (upsert, remove, hydrate). No manual refresh needed.
 *
 * When `sync` is true (default), pushes unsynced local recipes to cloud
 * on mount if the user is authenticated. Disable with `{ sync: false }`
 * when the hook is used in a context where syncing is undesirable
 * (e.g. read-only previews, background counts).
 */
export function useRecipes(options?: { sync?: boolean }) {
  const { sync = true } = options ?? {};
  const isAuthenticated = useIsAuthenticated();
  const recipes = useStore(recipesStore, (s) => s.recipes);

  useEffect(() => {
    if (sync && isAuthenticated) {
      core.recipes.syncToCloud().catch(() => {});
    }
  }, [sync, isAuthenticated]);

  const data = useMemo(
    () =>
      Object.values(recipes)
        .map(recipeToListItem)
        .sort((a, b) => b.updatedAt - a.updatedAt),
    [recipes],
  );
  return { data, isLoading: false };
}
