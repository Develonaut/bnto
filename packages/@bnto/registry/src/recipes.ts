/** Recipe lookup functions — reads from local predefined recipes. */

import { RECIPES } from "./recipesCatalog";
import type { Recipe } from "./recipe";

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
