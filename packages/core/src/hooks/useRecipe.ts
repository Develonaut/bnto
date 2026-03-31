"use client";

import { useMemo } from "react";
import { useStore } from "zustand";
import { recipesStore } from "../stores/recipesStore";

/**
 * Get a single recipe by ID — reactive, store-backed.
 *
 * Returns `{ data, isLoading }` matching the old React Query shape.
 * Pass an empty string to skip the lookup.
 */
export function useRecipe(id: string) {
  const recipes = useStore(recipesStore, (s) => s.recipes);
  const data = useMemo(() => (id ? (recipes[id] ?? null) : null), [id, recipes]);
  return { data, isLoading: false };
}
