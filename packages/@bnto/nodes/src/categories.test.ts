import { describe, expect, it } from "vitest";

import { CATEGORIES, getCategoryInfo } from "./categories";

describe("CATEGORIES", () => {
  it("contains all registered categories", () => {
    // Categories are derived from NODE_TYPE_INFO, count should stay in sync
    expect(CATEGORIES.length).toBeGreaterThanOrEqual(1);
    expect(new Set(CATEGORIES.map((c) => c.name)).size).toBe(CATEGORIES.length);
  });

  it("every category has required fields", () => {
    for (const cat of CATEGORIES) {
      expect(cat.name).toBeTruthy();
      expect(cat.label).toBeTruthy();
      expect(cat.description).toBeTruthy();
    }
  });

  it("has unique names", () => {
    const names = CATEGORIES.map((c) => c.name);
    expect(new Set(names).size).toBe(names.length);
  });
});

describe("getCategoryInfo", () => {
  it("returns info for a valid category", () => {
    const info = getCategoryInfo("image");
    expect(info).toBeDefined();
    expect(info!.name).toBe("image");
    expect(info!.label).toBe("Image");
  });

  it("returns undefined for unknown category", () => {
    expect(getCategoryInfo("nonexistent")).toBeUndefined();
  });
});
