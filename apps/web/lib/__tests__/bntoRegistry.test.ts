import { describe, expect, it } from "vitest";
import { getAllRecipes } from "@bnto/registry";
import { BNTO_REGISTRY, RESERVED_PATHS, getBntoBySlug, isValidBntoSlug } from "../bntoRegistry";

const allRecipes = getAllRecipes();

describe("BNTO_REGISTRY", () => {
  it("has all Tier 1 bntos", () => {
    const slugs = BNTO_REGISTRY.map((b) => b.slug);
    expect(slugs).toContain("compress-images");
    expect(slugs).toContain("resize-images");
    expect(slugs).toContain("convert-image-format");
    expect(slugs).toContain("rename-files");
    expect(slugs).toContain("clean-csv");
    expect(slugs).toContain("rename-csv-columns");
  });

  it("all slugs are lowercase-hyphen format", () => {
    for (const entry of BNTO_REGISTRY) {
      expect(entry.slug).toMatch(/^[a-z][a-z0-9-]+[a-z0-9]$/);
    }
  });

  it("no slug collides with reserved paths", () => {
    const reserved = new Set<string>(RESERVED_PATHS);
    for (const entry of BNTO_REGISTRY) {
      expect(reserved.has(entry.slug)).toBe(false);
    }
  });

  it("all entries have required fields", () => {
    for (const entry of BNTO_REGISTRY) {
      expect(entry.title).toBeTruthy();
      expect(entry.description).toBeTruthy();
      expect(entry.h1).toBeTruthy();
      expect(entry.fixture).toBeTruthy();
      expect(entry.features.length).toBeGreaterThan(0);
    }
  });

  it("all titles end with -- bnto", () => {
    for (const entry of BNTO_REGISTRY) {
      expect(entry.title).toMatch(/-- bnto$/);
    }
  });

  it("has no duplicate slugs", () => {
    const slugs = BNTO_REGISTRY.map((b) => b.slug);
    expect(new Set(slugs).size).toBe(slugs.length);
  });
});

describe("isValidBntoSlug", () => {
  it("returns true for registered slugs", () => {
    expect(isValidBntoSlug("compress-images")).toBe(true);
    expect(isValidBntoSlug("clean-csv")).toBe(true);
  });

  it("returns false for unregistered slugs", () => {
    expect(isValidBntoSlug("nonexistent")).toBe(false);
    expect(isValidBntoSlug("")).toBe(false);
    expect(isValidBntoSlug("settings")).toBe(false);
  });
});

describe("getBntoBySlug", () => {
  it("returns entry for valid slug", () => {
    const entry = getBntoBySlug("compress-images");
    expect(entry).toBeDefined();
    expect(entry!.h1).toBe("Compress Images Online Free");
    expect(entry!.title).toBe("Compress Images Online Free -- bnto");
  });

  it("returns undefined for invalid slug", () => {
    expect(getBntoBySlug("nonexistent")).toBeUndefined();
  });
});

describe("single source of truth — @bnto/registry propagation", () => {
  it("BNTO_REGISTRY count matches getAllRecipes()", () => {
    expect(BNTO_REGISTRY.length).toBe(allRecipes.length);
  });

  it("every registry recipe has a matching BntoEntry with correct metadata", () => {
    for (const recipe of allRecipes) {
      const entry = getBntoBySlug(recipe.slug);
      expect(entry, `missing BntoEntry for slug: ${recipe.slug}`).toBeDefined();
      expect(entry!.title).toContain(recipe.name);
      expect(entry!.description).toBe(recipe.description);
      expect(entry!.features).toEqual(recipe.features);
    }
  });

  it("BNTO_REGISTRY order matches getAllRecipes() order", () => {
    const registrySlugs = BNTO_REGISTRY.map((e) => e.slug);
    const recipeSlugs = allRecipes.map((r) => r.slug);
    expect(registrySlugs).toEqual(recipeSlugs);
  });
});
