"use client";

import { useCallback } from "react";
import { core } from "../core";

/**
 * Remove a device-local draft recipe.
 *
 * Returns a mutate function that deletes the draft and
 * accepts an onSuccess callback for triggering a list refresh.
 */
export function useRemoveDraftRecipe() {
  const mutate = useCallback((recipeId: string, options?: { onSuccess?: () => void }) => {
    core.recipes.removeDraft(recipeId);
    options?.onSuccess?.();
  }, []);

  return { mutate };
}
