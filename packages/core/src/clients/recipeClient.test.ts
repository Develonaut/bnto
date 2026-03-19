import { describe, it, expect, beforeEach } from "vitest";
import { recipesStore } from "../stores/recipesStore";
import { registryStore } from "../stores/registryStore";
import { RECIPES, NODE_TYPE_INFO, CATEGORIES, PROCESSORS } from "@bnto/nodes";
import { createRecipeClient } from "./recipeClient";
import { createRegistryClient } from "./registryClient";

// Minimal stubs — createFromDefinition only uses upsert (store-backed), no services.
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

describe("recipeClient.createFromDefinition", () => {
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

  it("creates a personal recipe from a definition and returns the new ID", () => {
    const client = createRecipeClient(mockRecipeService, mockExecutionService);
    const registry = createRegistryClient();

    const compress = registry.getRecipes().find((r) => r.slug === "compress-images")!;
    const newId = client.createFromDefinition(compress.definition);

    expect(newId).toBeTruthy();

    const saved = client.get(newId)!;
    expect(saved.name).toBe(compress.definition.name);
    expect(saved.definition.nodes?.length).toBe(compress.definition.nodes?.length);
  });

  it("generates a new unique ID — not the original definition ID", () => {
    const client = createRecipeClient(mockRecipeService, mockExecutionService);
    const registry = createRegistryClient();
    const original = registry.getRecipes()[0]!;

    const newId = client.createFromDefinition(original.definition);
    expect(newId).not.toBe(original.definition.id);
  });

  it("does not modify the original definition in the registry", () => {
    const client = createRecipeClient(mockRecipeService, mockExecutionService);
    const registry = createRegistryClient();
    const original = registry.getRecipes()[0]!;
    const originalId = original.definition.id;

    client.createFromDefinition(original.definition);

    const stillOriginal = registry.getRecipes()[0]!;
    expect(stillOriginal.definition.id).toBe(originalId);
  });

  it("sets persistence fields on the new UserRecipe", () => {
    const client = createRecipeClient(mockRecipeService, mockExecutionService);
    const registry = createRegistryClient();
    const original = registry.getRecipes()[0]!;

    const newId = client.createFromDefinition(original.definition);
    const saved = client.get(newId)!;

    expect(saved.cloudId).toBeNull();
    expect(saved.savedAt).toBeGreaterThan(0);
    expect(saved.syncedAt).toBeNull();
  });

  it("stamps the new ID onto the cloned definition", () => {
    const client = createRecipeClient(mockRecipeService, mockExecutionService);
    const registry = createRegistryClient();
    const original = registry.getRecipes()[0]!;

    const newId = client.createFromDefinition(original.definition);
    const saved = client.get(newId)!;

    expect(saved.definition.id).toBe(newId);
    expect(saved.definition.id).not.toBe(original.definition.id);
  });

  it("derives display metadata from the definition", () => {
    const client = createRecipeClient(mockRecipeService, mockExecutionService);
    const registry = createRegistryClient();
    const compress = registry.getRecipes().find((r) => r.slug === "compress-images")!;

    const newId = client.createFromDefinition(compress.definition);
    const saved = client.get(newId)!;

    expect(saved.name).toBe(compress.definition.name);
    expect(saved.slug).toBeTruthy();
    expect(saved.accept).toBeDefined();
  });
});
