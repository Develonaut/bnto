/** Recipe lookup functions — stateless, reads from @bnto/nodes static exports. */

import { RECIPES } from "@bnto/nodes";
import type { Recipe } from "@bnto/nodes";

/** Returns all predefined recipes. */
export function getAllRecipes(): readonly Recipe[] {
  return RECIPES;
}

/** Returns the recipe matching a URL slug, or undefined if not found. */
export function getRecipeBySlug(slug: string): Recipe | undefined {
  return RECIPES.find((r) => r.slug === slug);
}

/** Returns all recipes in a given category. */
export function getRecipesByCategory(category: string): Recipe[] {
  return RECIPES.filter((r) => r.category === category);
}
