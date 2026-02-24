import { describe, expect, it } from "vitest";

import { RECIPES, getRecipeBySlug } from "./recipes";

const EXPECTED_SLUGS = [
  "compress-images",
  "resize-images",
  "convert-image-format",
  "rename-files",
  "clean-csv",
  "rename-csv-columns",
];

describe("RECIPES", () => {
  it("contains all 6 Tier 1 recipes", () => {
    expect(RECIPES).toHaveLength(6);
  });

  it("contains every expected slug", () => {
    const slugs = RECIPES.map((r) => r.slug);
    for (const expected of EXPECTED_SLUGS) {
      expect(slugs).toContain(expected);
    }
  });

  it("has unique slugs", () => {
    const slugs = RECIPES.map((r) => r.slug);
    expect(new Set(slugs).size).toBe(slugs.length);
  });

  it("every recipe has required top-level fields", () => {
    for (const recipe of RECIPES) {
      expect(recipe.slug).toBeTruthy();
      expect(recipe.name).toBeTruthy();
      expect(recipe.description).toBeTruthy();
      expect(recipe.category).toBeTruthy();
      expect(recipe.features.length).toBeGreaterThan(0);
    }
  });

  it("every recipe has a valid accept spec", () => {
    for (const recipe of RECIPES) {
      expect(recipe.accept.mimeTypes.length).toBeGreaterThan(0);
      expect(recipe.accept.label).toBeTruthy();
    }
  });

  it("every recipe has valid SEO metadata", () => {
    for (const recipe of RECIPES) {
      expect(recipe.seo.title).toContain("-- bnto");
      expect(recipe.seo.h1).toBeTruthy();
    }
  });
});

describe("recipe definitions", () => {
  it("every definition is a group node", () => {
    for (const recipe of RECIPES) {
      expect(recipe.definition.type).toBe("group");
    }
  });

  it("every definition has an id matching the slug", () => {
    for (const recipe of RECIPES) {
      expect(recipe.definition.id).toBe(recipe.slug);
    }
  });

  it("every definition has version 1.0.0", () => {
    for (const recipe of RECIPES) {
      expect(recipe.definition.version).toBe("1.0.0");
    }
  });

  it("every definition has child nodes", () => {
    for (const recipe of RECIPES) {
      expect(recipe.definition.nodes).toBeDefined();
      expect(recipe.definition.nodes!.length).toBeGreaterThan(0);
    }
  });

  it("every definition has edges connecting child nodes", () => {
    for (const recipe of RECIPES) {
      expect(recipe.definition.edges).toBeDefined();
      expect(recipe.definition.edges!.length).toBeGreaterThan(0);
    }
  });

  it("every child node has required fields", () => {
    for (const recipe of RECIPES) {
      for (const node of recipe.definition.nodes!) {
        expect(node.id).toBeTruthy();
        expect(node.type).toBeTruthy();
        expect(node.version).toBe("1.0.0");
        expect(node.name).toBeTruthy();
        expect(node.position).toBeDefined();
        expect(node.parameters).toBeDefined();
      }
    }
  });

  it("edges reference valid child node ids", () => {
    for (const recipe of RECIPES) {
      const nodeIds = new Set(
        recipe.definition.nodes!.map((n) => n.id),
      );
      for (const edge of recipe.definition.edges!) {
        expect(nodeIds.has(edge.source)).toBe(true);
        expect(nodeIds.has(edge.target)).toBe(true);
      }
    }
  });
});

describe("getRecipeBySlug", () => {
  it("returns the correct recipe for a valid slug", () => {
    const recipe = getRecipeBySlug("compress-images");
    expect(recipe).toBeDefined();
    expect(recipe!.name).toBe("Compress Images");
  });

  it("returns undefined for an unknown slug", () => {
    expect(getRecipeBySlug("nonexistent")).toBeUndefined();
  });

  it("finds every recipe by its slug", () => {
    for (const expected of EXPECTED_SLUGS) {
      const recipe = getRecipeBySlug(expected);
      expect(recipe).toBeDefined();
      expect(recipe!.slug).toBe(expected);
    }
  });
});
