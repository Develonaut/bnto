import { describe, it, expect } from "vitest";
import { mergeCloudRecipes } from "./mergeCloudRecipes";
import type { UserRecipe } from "../types/recipe";

function makeRecipe(overrides: Partial<UserRecipe> = {}): UserRecipe {
  return {
    id: "local-1",
    slug: "test",
    name: "Test",
    description: "test",
    category: "custom",
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
    accept: { mimeTypes: [], extensions: [], label: "Any" },
    features: [],
    cloudId: null,
    savedAt: 1000,
    syncedAt: null,
    ...overrides,
  };
}

describe("mergeCloudRecipes", () => {
  it("adds new cloud recipes to empty local map", () => {
    const local: Record<string, UserRecipe> = {};
    const incoming = [makeRecipe({ id: "r1", cloudId: "cloud-1" })];
    mergeCloudRecipes(local, incoming);
    expect(local["r1"]).toBeDefined();
    expect(local["r1"].cloudId).toBe("cloud-1");
  });

  it("skips recipes already linked by cloudId", () => {
    const local: Record<string, UserRecipe> = {
      "local-1": makeRecipe({ id: "local-1", cloudId: "cloud-1", savedAt: 1000 }),
    };
    const incoming = [makeRecipe({ id: "new-id", cloudId: "cloud-1", savedAt: 2000 })];
    mergeCloudRecipes(local, incoming);
    // Should NOT add the duplicate
    expect(local["new-id"]).toBeUndefined();
    // Original unchanged
    expect(local["local-1"].savedAt).toBe(1000);
  });

  it("merges cloudId onto local-only recipe matched by id", () => {
    const local: Record<string, UserRecipe> = {
      r1: makeRecipe({ id: "r1", cloudId: null }),
    };
    const incoming = [makeRecipe({ id: "r1", cloudId: "cloud-1", syncedAt: 5000 })];
    mergeCloudRecipes(local, incoming);
    expect(local["r1"].cloudId).toBe("cloud-1");
    expect(local["r1"].syncedAt).toBe(5000);
  });

  it("does not overwrite existing recipe that already has a cloudId", () => {
    const local: Record<string, UserRecipe> = {
      r1: makeRecipe({ id: "r1", cloudId: "cloud-A" }),
    };
    const incoming = [makeRecipe({ id: "r1", cloudId: "cloud-B" })];
    mergeCloudRecipes(local, incoming);
    // cloudId should remain unchanged (cloudId-based dedup skips it)
    expect(local["r1"].cloudId).toBe("cloud-A");
  });
});
