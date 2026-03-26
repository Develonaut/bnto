import { describe, it, expect } from "vitest";
import { toRecipeListItem } from "./toRecipeListItem";
import type { RawRecipeListProjection } from "../types/raw";

const projection: RawRecipeListProjection = {
  _id: "rec789",
  name: "Compress Images",
  nodeCount: 3,
  nodeTypes: ["image-compress", "image-resize"],
  updatedAt: 5000,
};

describe("toRecipeListItem", () => {
  it("maps _id to string id", () => {
    expect(toRecipeListItem(projection).id).toBe("rec789");
  });

  it("passes through name and nodeCount", () => {
    const result = toRecipeListItem(projection);
    expect(result.name).toBe("Compress Images");
    expect(result.nodeCount).toBe(3);
  });

  it("passes through nodeTypes when present", () => {
    expect(toRecipeListItem(projection).nodeTypes).toEqual(["image-compress", "image-resize"]);
  });

  it("defaults nodeTypes to empty array when undefined", () => {
    const noTypes: RawRecipeListProjection = {
      _id: "rec000",
      name: "No Types",
      nodeCount: 1,
      updatedAt: 1000,
    };
    expect(toRecipeListItem(noTypes).nodeTypes).toEqual([]);
  });

  it("passes through updatedAt", () => {
    expect(toRecipeListItem(projection).updatedAt).toBe(5000);
  });
});
