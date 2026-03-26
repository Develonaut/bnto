/** Returns the recipe matching a URL slug, or undefined if not found. */

import { RECIPES } from "./recipesCatalog";
import type { Recipe } from "./recipe";

export function getRecipeBySlug(slug: string): Recipe | undefined {
  return RECIPES.find((r) => r.slug === slug);
}
