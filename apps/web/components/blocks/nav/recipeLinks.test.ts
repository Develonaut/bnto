import { describe, it, expect } from "vitest";
import { RECIPES, FEATURED_RECIPES } from "./recipeLinks";

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

  it("categories are ordered: Image, Data, File, Vector", () => {
    const titles = RECIPES.map((c) => c.title);
    expect(titles).toEqual(["Image", "Data", "File", "Vector"]);
  });

  it("contains compress-images in the Image category", () => {
    const imageCategory = RECIPES.find((c) => c.title === "Image");
    expect(imageCategory).toBeDefined();
    const urls = imageCategory!.links.map((l) => l.url);
    expect(urls).toContain("/compress-images");
  });

  it("does not contain non-browser recipes like download-video", () => {
    const allUrls = RECIPES.flatMap((c) => c.links.map((l) => l.url));
    expect(allUrls).not.toContain("/download-video");
  });
});

describe("FEATURED_RECIPES", () => {
  it("is a strict subset of RECIPES", () => {
    const allUrls = new Set(RECIPES.flatMap((c) => c.links.map((l) => l.url)));
    const featuredUrls = FEATURED_RECIPES.flatMap((c) => c.links.map((l) => l.url));
    for (const url of featuredUrls) {
      expect(allUrls.has(url)).toBe(true);
    }
  });

  it("contains exactly 12 featured recipes", () => {
    const count = FEATURED_RECIPES.reduce((sum, cat) => sum + cat.links.length, 0);
    expect(count).toBe(12);
  });

  it("preserves category order: Image, Data, File, Vector", () => {
    const titles = FEATURED_RECIPES.map((c) => c.title);
    expect(titles).toEqual(["Image", "Data", "File", "Vector"]);
  });

  it("features the right recipes per category", () => {
    const byCategory = Object.fromEntries(
      FEATURED_RECIPES.map((c) => [c.title, c.links.map((l) => l.url)]),
    );
    expect(byCategory["Image"]).toEqual(
      expect.arrayContaining([
        "/compress-images",
        "/resize-images",
        "/convert-image-format",
        "/optimize-images-for-web",
        "/watermark-images",
      ]),
    );
    expect(byCategory["Data"]).toEqual(
      expect.arrayContaining(["/clean-csv", "/csv-to-json", "/merge-csv"]),
    );
    expect(byCategory["File"]).toEqual(expect.arrayContaining(["/rename-files"]));
  });
});
