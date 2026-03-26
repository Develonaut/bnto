import { describe, it, expect, vi } from "vitest";
import { cloudRecipeToUserRecipe } from "./cloudRecipeToUserRecipe";
import type { RawRecipeDoc } from "../types/raw";

const doc: RawRecipeDoc = {
  _id: "abc123",
  userId: "user456",
  name: "My Recipe",
  definition: {
    id: "root",
    type: "group",
    version: "1.0.0",
    name: "My Recipe",
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

describe("cloudRecipeToUserRecipe", () => {
  it("sets cloudId from doc._id", () => {
    expect(cloudRecipeToUserRecipe(doc).cloudId).toBe("abc123");
  });

  it("sets savedAt from doc.updatedAt", () => {
    expect(cloudRecipeToUserRecipe(doc).savedAt).toBe(2000);
  });

  it("sets syncedAt to current time", () => {
    vi.useFakeTimers({ now: 9999 });
    expect(cloudRecipeToUserRecipe(doc).syncedAt).toBe(9999);
    vi.useRealTimers();
  });

  it("generates a new local id (UUID)", () => {
    const result = cloudRecipeToUserRecipe(doc);
    expect(result.id).not.toBe("abc123");
    expect(result.id).toMatch(/^[0-9a-f-]{36}$/);
  });

  it("preserves the definition", () => {
    expect(cloudRecipeToUserRecipe(doc).definition).toBe(doc.definition);
  });

  it("preserves the name", () => {
    expect(cloudRecipeToUserRecipe(doc).name).toBe("My Recipe");
  });
});
