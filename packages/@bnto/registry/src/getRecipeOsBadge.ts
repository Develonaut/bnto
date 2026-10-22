/**
 * The OS badge for a CLI recipe limited to some OSes ("macOS only"), or
 * null when there's nothing to say: a browser recipe runs anywhere a browser
 * does, and so does a CLI recipe with a branch for every OS.
 */

import { isRecipeBrowserCapable } from "./isRecipeBrowserCapable";
import { recipeOsLabels } from "./recipeOsLabels";
import type { Recipe } from "./recipe";

const OS_COUNT = 3;

export function getRecipeOsBadge(recipe: Recipe): string | null {
  const labels = recipeOsLabels(recipe.os);
  if (isRecipeBrowserCapable(recipe) || labels.length === 0 || labels.length >= OS_COUNT) {
    return null;
  }
  const last = labels.pop();
  return labels.length ? `${labels.join(", ")} and ${last} only` : `${last} only`;
}
