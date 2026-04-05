/** Returns only recipes that can run entirely in the browser (WASM). */

import { RECIPES } from "./recipesCatalog";
import { isRecipeBrowserCapable } from "./isRecipeBrowserCapable";
import type { Recipe } from "./recipe";

export function getBrowserRecipes(): readonly Recipe[] {
  return RECIPES.filter(isRecipeBrowserCapable);
}
