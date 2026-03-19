import { describe, it, expect } from "vitest";
import { getAllRecipes, getRecipeBySlug, getRecipesByCategory } from "./recipes";

describe("recipes", () => {
  describe("getAllRecipes", () => {
    it("returns all 8 predefined recipes", () => {
      expect(getAllRecipes()).toHaveLength(8);
    });

    it("every recipe has required fields", () => {
      for (const recipe of getAllRecipes()) {
        expect(recipe.id).toBeTruthy();
        expect(recipe.slug).toBeTruthy();
        expect(recipe.name).toBeTruthy();
        expect(recipe.definition).toBeDefined();
        expect(recipe.category).toBeTruthy();
      }
    });
  });

  describe("getRecipeBySlug", () => {
    it("returns a recipe for a known slug", () => {
      const recipe = getRecipeBySlug("compress-images");
      expect(recipe).toBeDefined();
      expect(recipe!.slug).toBe("compress-images");
    });

    it("returns undefined for an unknown slug", () => {
      expect(getRecipeBySlug("nonexistent")).toBeUndefined();
    });
  });

  describe("getRecipesByCategory", () => {
    it("returns image recipes", () => {
      const imageRecipes = getRecipesByCategory("image");
      expect(imageRecipes.length).toBeGreaterThan(0);
      for (const r of imageRecipes) {
        expect(r.category).toBe("image");
      }
    });

    it("returns empty for unknown category", () => {
      expect(getRecipesByCategory("nonexistent")).toEqual([]);
    });
  });
});
