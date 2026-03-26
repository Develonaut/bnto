/** Returns all predefined recipes. */

import { RECIPES } from "./recipesCatalog";
import type { Recipe } from "./recipe";

export function getAllRecipes(): readonly Recipe[] {
  return RECIPES;
}
