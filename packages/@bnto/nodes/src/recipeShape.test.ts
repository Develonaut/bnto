import { describe, expect, it } from "vitest";

import { ALL_RECIPES, getRecipeBySlug } from "./recipes";

describe("recipeShape — uniform structure", () => {
  it("has exactly 12 recipes", () => {
    expect(ALL_RECIPES).toHaveLength(12);
  });

  it("all slugs are unique", () => {
    const slugs = ALL_RECIPES.map((r) => r.slug);
    expect(new Set(slugs).size).toBe(slugs.length);
  });

  it("getRecipeBySlug finds all 12 recipes", () => {
    for (const recipe of ALL_RECIPES) {
      expect(getRecipeBySlug(recipe.slug)).toBe(recipe);
    }
  });

  for (const recipe of ALL_RECIPES) {
    describe(`${recipe.slug}`, () => {
      it("has required metadata fields", () => {
        expect(recipe.slug).toBeTruthy();
        expect(recipe.name).toBeTruthy();
        expect(recipe.description).toBeTruthy();
        expect(recipe.category).toBeTruthy();
        expect(recipe.features.length).toBeGreaterThan(0);
      });

      it("has accept spec", () => {
        expect(recipe.accept.mimeTypes.length).toBeGreaterThan(0);
        expect(recipe.accept.label).toBeTruthy();
      });

      it("has seo spec", () => {
        expect(recipe.seo.title).toBeTruthy();
        expect(recipe.seo.h1).toBeTruthy();
      });

      it("has a valid definition", () => {
        expect(recipe.definition).toBeDefined();
        expect(recipe.definition.type).toBe("group");
        expect(recipe.definition.nodes!.length).toBeGreaterThanOrEqual(2);
      });

      it("has input and output nodes", () => {
        const nodes = recipe.definition.nodes!;
        expect(nodes.some((n) => n.type === "input")).toBe(true);
        expect(nodes.some((n) => n.type === "output")).toBe(true);
      });
    });
  }
});
