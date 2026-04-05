import { describe, it, expect } from "vitest";
import { getBrowserRecipes } from "./getBrowserRecipes";
import { getAllRecipes } from "./getAllRecipes";
import { isRecipeBrowserCapable } from "./isRecipeBrowserCapable";

describe("getBrowserRecipes", () => {
  it("returns a subset of getAllRecipes()", () => {
    const browser = getBrowserRecipes();
    const all = getAllRecipes();
    expect(browser.length).toBeLessThanOrEqual(all.length);
    expect(browser.length).toBeGreaterThan(0);
    for (const r of browser) {
      expect(all).toContain(r);
    }
  });

  it("excludes download-video (CLI-only)", () => {
    const slugs = getBrowserRecipes().map((r) => r.slug);
    expect(slugs).not.toContain("download-video");
  });

  it("every returned recipe passes isRecipeBrowserCapable", () => {
    for (const r of getBrowserRecipes()) {
      expect(isRecipeBrowserCapable(r)).toBe(true);
    }
  });
});
