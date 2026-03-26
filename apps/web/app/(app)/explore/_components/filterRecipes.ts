/**
 * Client-side recipe filtering by category and search text.
 */

import { getAllRecipes } from "@bnto/registry";
import type { Recipe } from "@bnto/core";

const ALL_RECIPES = getAllRecipes();

export function filterRecipes(query: string, category: string): readonly Recipe[] {
  let recipes = [...ALL_RECIPES] as Recipe[];
  if (category && category !== "all") {
    recipes = recipes.filter((r) => r.category === category);
  }
  if (query) {
    const q = query.toLowerCase();
    recipes = recipes.filter(
      (r) =>
        r.name.toLowerCase().includes(q) ||
        r.description.toLowerCase().includes(q) ||
        r.features.some((f) => f.toLowerCase().includes(q)),
    );
  }
  return recipes;
}
