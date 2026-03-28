/**
 * Recipe catalog completeness tests — ensures every recipe file
 * is registered in the RECIPES array and slugs are unique.
 */

import { describe, expect, it } from "vitest";
import { RECIPES } from "./recipesCatalog";
import * as recipeExports from "./recipes/index";

describe("recipe catalog completeness", () => {
  it("every recipe export is included in RECIPES", () => {
    const exportedRecipes = Object.values(recipeExports);
    for (const recipe of exportedRecipes) {
      const found = RECIPES.find((r) => r.slug === recipe.slug);
      expect(found, `Recipe "${recipe.slug}" is exported but not in RECIPES array`).toBeDefined();
    }
  });

  it("every RECIPES entry is exported from recipes/index", () => {
    const exportedSlugs = new Set(Object.values(recipeExports).map((r) => r.slug));
    for (const recipe of RECIPES) {
      expect(
        exportedSlugs.has(recipe.slug),
        `Recipe "${recipe.slug}" is in RECIPES but not exported from recipes/index`,
      ).toBe(true);
    }
  });

  it("every RECIPES entry has a unique slug", () => {
    const slugs = RECIPES.map((r) => r.slug);
    expect(new Set(slugs).size).toBe(slugs.length);
  });

  it("has at least one recipe", () => {
    expect(RECIPES.length).toBeGreaterThan(0);
  });
});
