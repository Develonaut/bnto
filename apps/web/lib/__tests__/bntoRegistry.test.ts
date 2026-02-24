import { describe, expect, it } from "vitest";
import {
  BNTO_REGISTRY,
  RESERVED_PATHS,
  getBntoBySlug,
  isValidBntoSlug,
} from "../bntoRegistry";

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
  });

  it("returns undefined for invalid slug", () => {
    expect(getBntoBySlug("nonexistent")).toBeUndefined();
  });
});
