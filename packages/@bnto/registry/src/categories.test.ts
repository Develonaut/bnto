import { describe, it, expect } from "vitest";
import { getAllCategories } from "./categories";

describe("categories", () => {
  it("returns all 8 categories", () => {
    expect(getAllCategories()).toHaveLength(8);
  });

  it("every category has required fields", () => {
    for (const cat of getAllCategories()) {
      expect(cat.name).toBeTruthy();
      expect(cat.label).toBeTruthy();
      expect(cat.description).toBeTruthy();
    }
  });
});
