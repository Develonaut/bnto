"use client";

import { useCallback } from "react";
import { core } from "../core";

/**
 * Remove a locally-saved recipe (localStorage draft).
 *
 * Returns a mutate function that deletes the draft and
 * accepts an onSuccess callback for triggering a list refresh.
 */
export function useRemoveLocalRecipe() {
  const mutate = useCallback((recipeId: string, options?: { onSuccess?: () => void }) => {
    core.localRecipes.remove(recipeId);
    options?.onSuccess?.();
  }, []);

  return { mutate };
}
