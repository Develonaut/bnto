"use client";

import { useCallback, useEffect, useState } from "react";
import { core } from "../core";
import type { RecipeListItem } from "../types";

/**
 * List all device-local draft recipes.
 *
 * Returns the same RecipeListItem[] shape as cloud recipes.
 * Provides a `refresh` callback for re-reading after mutations.
 */
export function useDraftRecipes() {
  const [recipes, setRecipes] = useState<RecipeListItem[]>([]);

  useEffect(() => {
    setRecipes(core.recipes.listDrafts());
  }, []);

  const refresh = useCallback(() => {
    setRecipes(core.recipes.listDrafts());
  }, []);

  return { data: recipes, refresh };
}
