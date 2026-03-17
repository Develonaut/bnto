"use client";

import { useCallback, useEffect, useState } from "react";
import { core } from "../core";
import type { RecipeListItem } from "../types";

/**
 * List all locally-saved recipes (localStorage drafts).
 *
 * Returns the same RecipeListItem[] shape as cloud recipes.
 * Provides a `refresh` callback for re-reading after mutations.
 */
export function useLocalRecipes() {
  const [recipes, setRecipes] = useState<RecipeListItem[]>([]);

  useEffect(() => {
    setRecipes(core.localRecipes.list());
  }, []);

  const refresh = useCallback(() => {
    setRecipes(core.localRecipes.list());
  }, []);

  return { data: recipes, refresh };
}
