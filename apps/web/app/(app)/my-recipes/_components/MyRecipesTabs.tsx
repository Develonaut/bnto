"use client";

import { RecipeGrid } from "./RecipeGrid";

/**
 * Client boundary for the My Recipes page.
 *
 * Renders the saved recipes grid directly — no tabs, no stats.
 * The parent page.tsx is a server component that renders the
 * heading and description statically.
 */
export function MyRecipesTabs() {
  return <RecipeGrid />;
}
