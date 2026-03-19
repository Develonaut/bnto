import { describe, it, expect, beforeEach } from "vitest";
import { recipesStore } from "../stores/recipesStore";
import { registryStore } from "../stores/registryStore";
import { RECIPES, NODE_TYPE_INFO, CATEGORIES, PROCESSORS } from "@bnto/nodes";
import { createRecipeClient } from "./recipeClient";
import { createRegistryClient } from "./registryClient";

// Minimal stubs — createFromTemplate only uses upsert (store-backed), no services.
const mockRecipeService = {
  save: async () => "mock-cloud-id",
  remove: async () => {},
  invalidateList: () => {},
  invalidateRecipe: () => {},
  listQueryOptions: () => ({}),
  getQueryOptions: () => ({}),
} as unknown as Parameters<typeof createRecipeClient>[0];

const mockExecutionService = {
  start: async () => "mock-exec-id",
  invalidateExecutions: () => {},
} as unknown as Parameters<typeof createRecipeClient>[1];

describe("recipeClient.createFromTemplate", () => {
  beforeEach(() => {
    // Clear all recipes from store
    for (const id of Object.keys(recipesStore.getState().recipes)) {
      recipesStore.getState().remove(id);
    }

    // Populate registry with real data
    registryStore.getState().reset();
    registryStore.getState().populate({
      recipes: RECIPES,
      nodeTypes: NODE_TYPE_INFO,
      categories: [...CATEGORIES],
      processors: [...PROCESSORS],
    });
  });

  it("clones a predefined recipe into the personal store and returns the new ID", () => {
    const client = createRecipeClient(mockRecipeService, mockExecutionService);
    const registry = createRegistryClient();

    const compressRecipe = registry.getRecipes().find((r) => r.slug === "compress-images")!;
    const newId = client.createFromTemplate(compressRecipe);

    expect(newId).toBeTruthy();

    const saved = client.get(newId)!;
    expect(saved.name).toBe(compressRecipe.name);
    expect(saved.definition.nodes?.length).toBe(compressRecipe.definition.nodes?.length);
  });

  it("generates a new unique ID — not the original recipe ID", () => {
    const client = createRecipeClient(mockRecipeService, mockExecutionService);
    const registry = createRegistryClient();
    const original = registry.getRecipes()[0]!;

    const newId = client.createFromTemplate(original);
    expect(newId).not.toBe(original.id);
  });

  it("does not modify the original recipe in the registry", () => {
    const client = createRecipeClient(mockRecipeService, mockExecutionService);
    const registry = createRegistryClient();
    const original = registry.getRecipes()[0]!;
    const originalId = original.id;

    client.createFromTemplate(original);

    const stillOriginal = registry.getRecipes()[0]!;
    expect(stillOriginal.id).toBe(originalId);
  });

  it("sets persistence fields on the cloned UserRecipe", () => {
    const client = createRecipeClient(mockRecipeService, mockExecutionService);
    const registry = createRegistryClient();
    const original = registry.getRecipes()[0]!;

    const newId = client.createFromTemplate(original);
    const saved = client.get(newId)!;

    expect(saved.cloudId).toBeNull();
    expect(saved.savedAt).toBeGreaterThan(0);
    expect(saved.syncedAt).toBeNull();
  });

  it("clones the definition with a new ID", () => {
    const client = createRecipeClient(mockRecipeService, mockExecutionService);
    const registry = createRegistryClient();
    const original = registry.getRecipes()[0]!;

    const newId = client.createFromTemplate(original);
    const saved = client.get(newId)!;

    expect(saved.definition.id).toBe(newId);
    expect(saved.definition.id).not.toBe(original.definition.id);
  });
});
