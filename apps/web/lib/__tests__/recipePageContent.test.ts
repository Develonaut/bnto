/**
 * Guard tests for recipe page content registries.
 *
 * These tests ensure that category-level content (features, FAQ, mascots,
 * breadcrumb labels) stays in sync with the categories that exist in the
 * recipe registry. When a new category is added to the engine, these tests
 * fail and point the developer to exactly which registries need updating.
 */

import { describe, expect, it } from "vitest";
import { getAllRecipes } from "@bnto/registry";
import { CATEGORY_CONTENT } from "../categoryContent";
import {
  CATEGORY_BREADCRUMB_LABELS,
  getFeatures,
  getFaq,
  getHowItWorks,
} from "../recipePageContent";

/** All unique categories that have at least one recipe. */
const activeCategories = [...new Set(getAllRecipes().map((r) => r.category))];

describe("category content coverage", () => {
  it("every active recipe category has a CATEGORY_CONTENT block", () => {
    const covered = Object.keys(CATEGORY_CONTENT);
    for (const category of activeCategories) {
      expect(
        covered,
        `missing CATEGORY_CONTENT block for category: "${category}" — add it to categoryContent.ts`,
      ).toContain(category);
    }
  });

  it("every active recipe category has a breadcrumb label", () => {
    const covered = Object.keys(CATEGORY_BREADCRUMB_LABELS);
    for (const category of activeCategories) {
      expect(
        covered,
        `missing CATEGORY_BREADCRUMB_LABELS entry for category: "${category}" — add it to recipePageContent.ts`,
      ).toContain(category);
    }
  });

  it("getFeatures returns non-empty array for every active category", () => {
    for (const category of activeCategories) {
      const features = getFeatures(category);
      expect(
        features.length,
        `getFeatures("${category}") returned empty — add features to categoryContent.ts`,
      ).toBeGreaterThan(0);
    }
  });

  it("getFaq returns at least shared items for every recipe", () => {
    for (const recipe of getAllRecipes()) {
      const faq = getFaq(recipe.slug, recipe.category);
      expect(
        faq.length,
        `getFaq("${recipe.slug}") returned empty — shared FAQ should always be present`,
      ).toBeGreaterThan(0);
    }
  });

  it("getHowItWorks returns 3 steps for every recipe", () => {
    for (const recipe of getAllRecipes()) {
      const steps = getHowItWorks(recipe);
      expect(steps).toHaveLength(3);
    }
  });
});
