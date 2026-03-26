import { describe, it, expect } from "vitest";
import { RECIPES } from "./recipeLinks";

describe("RECIPES (buildRecipeCategories)", () => {
  it("returns an array of categories", () => {
    expect(Array.isArray(RECIPES)).toBe(true);
    expect(RECIPES.length).toBeGreaterThan(0);
  });

  it("every category has a title and non-empty links", () => {
    for (const cat of RECIPES) {
      expect(cat.title).toBeTruthy();
      expect(cat.links.length).toBeGreaterThan(0);
    }
  });

  it("every link has label, description, and url starting with /", () => {
    for (const cat of RECIPES) {
      for (const link of cat.links) {
        expect(link.label).toBeTruthy();
        expect(link.description).toBeTruthy();
        expect(link.url).toMatch(/^\//);
      }
    }
  });

  it("categories are ordered: Image, Data, File", () => {
    const titles = RECIPES.map((c) => c.title);
    expect(titles).toEqual(["Image", "Data", "File"]);
  });

  it("contains compress-images in the Image category", () => {
    const imageCategory = RECIPES.find((c) => c.title === "Image");
    expect(imageCategory).toBeDefined();
    const urls = imageCategory!.links.map((l) => l.url);
    expect(urls).toContain("/compress-images");
  });
});
