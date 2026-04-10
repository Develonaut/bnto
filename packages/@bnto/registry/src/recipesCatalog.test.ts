/**
 * Recipe catalog completeness tests — ensures all engine recipes
 * are represented in the RECIPES array with valid structure.
 */

import { describe, expect, it } from "vitest";
import {
  ITERATION_MODES,
  CURRENT_FORMAT_VERSION,
  validateDefinition,
  GENERATED_RECIPES,
} from "@bnto/nodes";
import { RECIPES } from "./recipesCatalog";

describe("recipe catalog completeness", () => {
  it("every engine recipe is included in RECIPES", () => {
    for (const generated of GENERATED_RECIPES) {
      const found = RECIPES.find((r) => r.slug === generated.slug);
      expect(found, `Engine recipe "${generated.slug}" is not in RECIPES array`).toBeDefined();
    }
  });

  it("every RECIPES entry maps to an engine recipe", () => {
    const engineSlugs = new Set(GENERATED_RECIPES.map((r) => r.slug));
    for (const recipe of RECIPES) {
      expect(
        engineSlugs.has(recipe.slug),
        `Recipe "${recipe.slug}" is in RECIPES but not in engine`,
      ).toBe(true);
    }
  });

  it("every RECIPES entry has a unique slug", () => {
    const slugs = RECIPES.map((r) => r.slug);
    expect(new Set(slugs).size).toBe(slugs.length);
  });

  it("has exactly 17 recipes", () => {
    expect(RECIPES.length).toBe(17);
  });
});

// =============================================================================
// Recipe definition structural compliance — catches engine schema drift
// =============================================================================

describe("recipe definition structure", () => {
  it("every recipe definition has settings.iteration set to a valid ITERATION_MODES value", () => {
    const validModes: readonly string[] = ITERATION_MODES;
    for (const recipe of RECIPES) {
      expect(
        recipe.definition.settings?.iteration,
        `Recipe "${recipe.slug}" is missing settings.iteration`,
      ).toBeDefined();
      expect(
        validModes,
        `Recipe "${recipe.slug}" has iteration "${recipe.definition.settings?.iteration}" which is not in ITERATION_MODES`,
      ).toContain(recipe.definition.settings!.iteration);
    }
  });

  it("every recipe definition uses CURRENT_FORMAT_VERSION", () => {
    for (const recipe of RECIPES) {
      expect(
        recipe.definition.version,
        `Recipe "${recipe.slug}" uses version "${recipe.definition.version}" instead of "${CURRENT_FORMAT_VERSION}"`,
      ).toBe(CURRENT_FORMAT_VERSION);
    }
  });

  it("every recipe definition passes structural validation", () => {
    for (const recipe of RECIPES) {
      const errors = validateDefinition(recipe.definition);
      expect(
        errors,
        `Recipe "${recipe.slug}" has validation errors: ${errors.map((e) => e.message).join(", ")}`,
      ).toHaveLength(0);
    }
  });
});
