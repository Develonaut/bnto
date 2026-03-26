import { describe, it, expect } from "vitest";
import { toRecipe } from "./toRecipe";
import type { RawRecipeDoc } from "../types/raw";

const doc: RawRecipeDoc = {
  _id: "abc123",
  userId: "user456",
  name: "My Recipe",
  definition: {
    id: "root",
    type: "group",
    version: "1.0.0",
    name: "Root",
    position: { x: 0, y: 0 },
    metadata: {},
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    nodes: [],
  },
  version: 3,
  formatVersion: "1.0.0",
  isPublic: true,
  createdAt: 1000,
  updatedAt: 2000,
};

describe("toRecipe", () => {
  it("maps _id to string id", () => {
    expect(toRecipe(doc).id).toBe("abc123");
  });

  it("maps userId to string", () => {
    expect(toRecipe(doc).userId).toBe("user456");
  });

  it("passes through scalar fields", () => {
    const result = toRecipe(doc);
    expect(result.name).toBe("My Recipe");
    expect(result.version).toBe(3);
    expect(result.formatVersion).toBe("1.0.0");
    expect(result.isPublic).toBe(true);
    expect(result.createdAt).toBe(1000);
    expect(result.updatedAt).toBe(2000);
  });

  it("passes through definition", () => {
    expect(toRecipe(doc).definition).toBe(doc.definition);
  });

  it("handles missing formatVersion", () => {
    const { formatVersion: _, ...noFormat } = doc;
    expect(toRecipe(noFormat as RawRecipeDoc).formatVersion).toBeUndefined();
  });
});
