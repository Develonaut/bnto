import { describe, expect, it } from "vitest";

import { RECIPES, ALL_RECIPES, getRecipeBySlug } from "./recipes";
import { CURRENT_FORMAT_VERSION } from "./formatVersion";

const PUBLIC_SLUGS = [
  "compress-images",
  "resize-images",
  "convert-image-format",
  "rename-files",
  "clean-csv",
  "rename-csv-columns",
];

const BUILDING_BLOCK_SLUGS = [
  "batch-compress",
  "batch-resize",
  "batch-convert",
  "batch-rename",
  "csv-cleaner",
  "column-renamer",
];

describe("RECIPES (public)", () => {
  it("contains all 6 public Tier 1 recipes", () => {
    expect(RECIPES).toHaveLength(6);
  });

  it("contains every expected public slug", () => {
    const slugs = RECIPES.map((r) => r.slug);
    for (const expected of PUBLIC_SLUGS) {
      expect(slugs).toContain(expected);
    }
  });

  it("does not contain building-block recipes", () => {
    const slugs = RECIPES.map((r) => r.slug);
    for (const block of BUILDING_BLOCK_SLUGS) {
      expect(slugs).not.toContain(block);
    }
  });

  it("has unique slugs", () => {
    const slugs = RECIPES.map((r) => r.slug);
    expect(new Set(slugs).size).toBe(slugs.length);
  });
});

describe("ALL_RECIPES (full catalog)", () => {
  it("contains all 12 recipes", () => {
    expect(ALL_RECIPES).toHaveLength(12);
  });

  it("contains all public and building-block slugs", () => {
    const slugs = ALL_RECIPES.map((r) => r.slug);
    for (const expected of [...PUBLIC_SLUGS, ...BUILDING_BLOCK_SLUGS]) {
      expect(slugs).toContain(expected);
    }
  });

  it("has unique slugs", () => {
    const slugs = ALL_RECIPES.map((r) => r.slug);
    expect(new Set(slugs).size).toBe(slugs.length);
  });

  it("every recipe has required top-level fields", () => {
    for (const recipe of ALL_RECIPES) {
      expect(recipe.slug).toBeTruthy();
      expect(recipe.name).toBeTruthy();
      expect(recipe.description).toBeTruthy();
      expect(recipe.category).toBeTruthy();
      expect(recipe.features.length).toBeGreaterThan(0);
    }
  });

  it("every recipe has a valid accept spec", () => {
    for (const recipe of ALL_RECIPES) {
      expect(recipe.accept.mimeTypes.length).toBeGreaterThan(0);
      expect(recipe.accept.label).toBeTruthy();
    }
  });

  it("every recipe has valid SEO metadata", () => {
    for (const recipe of ALL_RECIPES) {
      expect(recipe.seo.title).toContain("-- bnto");
      expect(recipe.seo.h1).toBeTruthy();
    }
  });
});

describe("recipe definitions", () => {
  it("every definition is a group node", () => {
    for (const recipe of ALL_RECIPES) {
      expect(recipe.definition.type).toBe("group");
    }
  });

  it("every definition has an id matching the slug", () => {
    for (const recipe of ALL_RECIPES) {
      expect(recipe.definition.id).toBe(recipe.slug);
    }
  });

  it("every definition has version 1.0.0", () => {
    for (const recipe of ALL_RECIPES) {
      expect(recipe.definition.version).toBe(CURRENT_FORMAT_VERSION);
    }
  });

  it("every definition has child nodes", () => {
    for (const recipe of ALL_RECIPES) {
      expect(recipe.definition.nodes).toBeDefined();
      expect(recipe.definition.nodes!.length).toBeGreaterThan(0);
    }
  });

  it("every definition has edges connecting child nodes", () => {
    for (const recipe of ALL_RECIPES) {
      expect(recipe.definition.edges).toBeDefined();
      expect(recipe.definition.edges!.length).toBeGreaterThan(0);
    }
  });

  it("every child node has required fields", () => {
    for (const recipe of ALL_RECIPES) {
      for (const node of recipe.definition.nodes!) {
        expect(node.id).toBeTruthy();
        expect(node.type).toBeTruthy();
        expect(node.version).toBe(CURRENT_FORMAT_VERSION);
        expect(node.name).toBeTruthy();
        expect(node.position).toBeDefined();
        expect(node.parameters).toBeDefined();
      }
    }
  });

  it("every definition has an input node with id 'input'", () => {
    for (const recipe of ALL_RECIPES) {
      const inputNode = recipe.definition.nodes!.find((n) => n.type === "input");
      expect(inputNode).toBeDefined();
      expect(inputNode!.id).toBe("input");
      expect(inputNode!.parameters.mode).toBe("file-upload");
    }
  });

  it("every definition has an output node with id 'output'", () => {
    for (const recipe of ALL_RECIPES) {
      const outputNode = recipe.definition.nodes!.find((n) => n.type === "output");
      expect(outputNode).toBeDefined();
      expect(outputNode!.id).toBe("output");
      expect(outputNode!.parameters.mode).toBe("download");
    }
  });

  it("input node accept matches recipe-level accept mimeTypes", () => {
    for (const recipe of ALL_RECIPES) {
      const inputNode = recipe.definition.nodes!.find((n) => n.type === "input")!;
      expect(inputNode.parameters.accept).toEqual(recipe.accept.mimeTypes);
    }
  });

  it("edges reference valid child node ids", () => {
    for (const recipe of ALL_RECIPES) {
      const nodeIds = new Set(recipe.definition.nodes!.map((n) => n.id));
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

  it("finds every public recipe by its slug", () => {
    for (const expected of PUBLIC_SLUGS) {
      const recipe = getRecipeBySlug(expected);
      expect(recipe).toBeDefined();
      expect(recipe!.slug).toBe(expected);
    }
  });

  it("finds every building-block recipe by its slug", () => {
    for (const expected of BUILDING_BLOCK_SLUGS) {
      const recipe = getRecipeBySlug(expected);
      expect(recipe).toBeDefined();
      expect(recipe!.slug).toBe(expected);
    }
  });
});
