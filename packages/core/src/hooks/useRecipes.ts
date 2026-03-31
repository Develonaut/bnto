"use client";

import { useMemo } from "react";
import { useStore } from "zustand";
import { recipesStore } from "../stores/recipesStore";
import type { UserRecipe } from "../types/recipe";

/**
 * List all recipes — reactive, store-backed.
 *
 * Returns UserRecipe[] sorted newest-first. Re-renders when the
 * recipesStore changes (upsert, remove).
 */
export function useRecipes() {
  const recipes = useStore(recipesStore, (s) => s.recipes);
  const data = useMemo(
    () => Object.values(recipes).sort((a: UserRecipe, b: UserRecipe) => b.savedAt - a.savedAt),
    [recipes],
  );
  return { data, isLoading: false };
}
