/** Returns all recipes in a given category. */

import { RECIPES } from "./recipesCatalog";
import type { Recipe } from "./recipe";

export function getRecipesByCategory(category: string): Recipe[] {
  return RECIPES.filter((r) => r.category === category);
}
