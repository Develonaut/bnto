"use client";

import { useCallback } from "react";
import { core } from "../core";

/**
 * Remove a recipe from the local store.
 *
 * Returns a mutate function with the same API shape as the old React Query
 * mutation to minimize consumer changes.
 */
export function useRemoveRecipe() {
  const mutate = useCallback((id: string, options?: { onSuccess?: () => void }) => {
    core.recipes.remove(id);
    options?.onSuccess?.();
  }, []);

  return { mutate, isPending: false };
}
