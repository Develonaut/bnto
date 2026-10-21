/**
 * Guard tests for recipe page content registries.
 *
 * These tests ensure that category-level content (features, FAQ, mascots,
 * breadcrumb labels) stays in sync with the categories that exist in the
 * recipe registry. When a new category is added to the engine, these tests
 * fail and point the developer to exactly which registries need updating.
 */

import { describe, expect, it } from "vitest";
import { getAllRecipes, isRecipeBrowserCapable } from "@bnto/registry";
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
      const faq = getFaq(recipe.slug, recipe.category, true);
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

describe("CLI-only recipe pages", () => {
  const cliRecipes = getAllRecipes().filter((r) => !isRecipeBrowserCapable(r));
  // Claims that a page works in the browser; explaining why it doesn't is fine.
  const BROWSER_ONLY =
    /drop your files|into the browser|leave your browser|in your browser tab|download results|webassembly/i;

  it("there are CLI-only recipes to check", () => {
    expect(cliRecipes.map((r) => r.slug)).toContain("keep-awake");
  });

  it("how it works describes the CLI, not a browser upload", () => {
    for (const recipe of cliRecipes) {
      const text = getHowItWorks(recipe)
        .map((s) => `${s.title} ${s.description}`)
        .join(" ");
      expect(text, recipe.slug).not.toMatch(BROWSER_ONLY);
      expect(text, recipe.slug).toContain("cargo install bnto");
    }
  });

  it("how it works shows the exact run command", () => {
    const keepAwake = cliRecipes.find((r) => r.slug === "keep-awake")!;
    const text = getHowItWorks(keepAwake)
      .map((s) => s.description)
      .join(" ");
    expect(text).toContain("bnto run keep-awake.");
  });

  it("the FAQ makes no browser claims", () => {
    for (const recipe of cliRecipes) {
      const text = getFaq(recipe.slug, recipe.category, false)
        .map((q) => `${q.question} ${q.answer}`)
        .join(" ");
      expect(text, recipe.slug).not.toMatch(BROWSER_ONLY);
    }
  });

  it("browser recipes keep the browser FAQ", () => {
    const text = getFaq("compress-images", "image", true)
      .map((q) => q.answer)
      .join(" ");
    expect(text).toContain("never leave your browser");
  });
});

describe("category content claims", () => {
  it("never promises a consent prompt, which built-in recipes skip", () => {
    // Bundled recipes are trusted and run without asking (trust.rs).
    const text = JSON.stringify(CATEGORY_CONTENT);
    expect(text).not.toMatch(/asks for your approval|before .* runs for the first time/i);
  });

  it("names no single recipe in category-wide FAQs", () => {
    const slugsAndNames = getAllRecipes().flatMap((r) => [r.slug, r.name]);
    for (const [category, block] of Object.entries(CATEGORY_CONTENT)) {
      const faq = JSON.stringify(block.faq);
      for (const name of slugsAndNames) {
        expect(faq, `${category} FAQ names ${name}`).not.toContain(name);
      }
    }
  });
});
