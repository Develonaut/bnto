import { describe, it, expect, vi, beforeEach } from "vitest";
import { recipesStore } from "../stores/recipesStore";
import { registryStore } from "../stores/registryStore";
import { RECIPES, NODE_TYPE_INFO, CATEGORIES, PROCESSORS } from "@bnto/registry";
import { createRecipeClient } from "./recipeClient";
import { createRegistryClient } from "./registryClient";

// Minimal stubs — createFromDefinition only uses upsert (store-backed), no services.
const mockRecipeService = {
  save: vi.fn().mockResolvedValue("mock-cloud-id"),
  remove: vi.fn().mockResolvedValue(undefined),
  invalidateList: vi.fn(),
  invalidateRecipe: vi.fn(),
  listQueryOptions: vi.fn(),
  getQueryOptions: vi.fn(),
} as unknown as Parameters<typeof createRecipeClient>[0];

const mockExecutionService = {
  start: async () => "mock-exec-id",
  invalidateExecutions: () => {},
} as unknown as Parameters<typeof createRecipeClient>[1];

const mockAuthClient = {
  isAuthenticated: () => false,
} as unknown as Parameters<typeof createRecipeClient>[2];

const authedClient = {
  isAuthenticated: () => true,
} as unknown as Parameters<typeof createRecipeClient>[2];

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
    const client = createRecipeClient(mockRecipeService, mockExecutionService, mockAuthClient);
    const registry = createRegistryClient();

    const compress = registry.getRecipes().find((r) => r.slug === "compress-images")!;
    const newId = client.createFromDefinition(compress.definition);

    expect(newId).toBeTruthy();

    const saved = client.get(newId)!;
    expect(saved.name).toBe(compress.definition.name);
    expect(saved.definition.nodes?.length).toBe(compress.definition.nodes?.length);
  });

  it("generates a new unique ID — not the original definition ID", () => {
    const client = createRecipeClient(mockRecipeService, mockExecutionService, mockAuthClient);
    const registry = createRegistryClient();
    const original = registry.getRecipes()[0]!;

    const newId = client.createFromDefinition(original.definition);
    expect(newId).not.toBe(original.definition.id);
  });

  it("does not modify the original definition in the registry", () => {
    const client = createRecipeClient(mockRecipeService, mockExecutionService, mockAuthClient);
    const registry = createRegistryClient();
    const original = registry.getRecipes()[0]!;
    const originalId = original.definition.id;

    client.createFromDefinition(original.definition);

    const stillOriginal = registry.getRecipes()[0]!;
    expect(stillOriginal.definition.id).toBe(originalId);
  });

  it("sets persistence fields on the new UserRecipe", () => {
    const client = createRecipeClient(mockRecipeService, mockExecutionService, mockAuthClient);
    const registry = createRegistryClient();
    const original = registry.getRecipes()[0]!;

    const newId = client.createFromDefinition(original.definition);
    const saved = client.get(newId)!;

    expect(saved.cloudId).toBeNull();
    expect(saved.savedAt).toBeGreaterThan(0);
    expect(saved.syncedAt).toBeNull();
  });

  it("stamps the new ID onto the cloned definition", () => {
    const client = createRecipeClient(mockRecipeService, mockExecutionService, mockAuthClient);
    const registry = createRegistryClient();
    const original = registry.getRecipes()[0]!;

    const newId = client.createFromDefinition(original.definition);
    const saved = client.get(newId)!;

    expect(saved.definition.id).toBe(newId);
    expect(saved.definition.id).not.toBe(original.definition.id);
  });

  it("derives display metadata from the definition", () => {
    const client = createRecipeClient(mockRecipeService, mockExecutionService, mockAuthClient);
    const registry = createRegistryClient();
    const compress = registry.getRecipes().find((r) => r.slug === "compress-images")!;

    const newId = client.createFromDefinition(compress.definition);
    const saved = client.get(newId)!;

    expect(saved.name).toBe(compress.definition.name);
    expect(saved.slug).toBeTruthy();
    expect(saved.accept).toBeDefined();
  });
});

// ── Auth Guard Tests ────────────────────────────────────────────────────────

describe("recipeClient — auth guards", () => {
  beforeEach(() => {
    vi.clearAllMocks();

    for (const id of Object.keys(recipesStore.getState().recipes)) {
      recipesStore.getState().remove(id);
    }

    registryStore.getState().reset();
    registryStore.getState().populate({
      recipes: RECIPES,
      nodeTypes: NODE_TYPE_INFO,
      categories: [...CATEGORIES],
      processors: [...PROCESSORS],
    });
  });

  function getDefinition() {
    const registry = createRegistryClient();
    return registry.getRecipes().find((r) => r.slug === "compress-images")!.definition;
  }

  describe("save", () => {
    it("skips cloud sync when unauthenticated", () => {
      const client = createRecipeClient(mockRecipeService, mockExecutionService, mockAuthClient);
      client.save(getDefinition(), { id: "r1", name: "Test", slug: "test" });

      expect(mockRecipeService.save).not.toHaveBeenCalled();
    });

    it("persists locally even when unauthenticated", () => {
      const client = createRecipeClient(mockRecipeService, mockExecutionService, mockAuthClient);
      client.save(getDefinition(), { id: "r1", name: "Test", slug: "test" });

      const saved = client.get("r1");
      expect(saved).toBeDefined();
      expect(saved!.name).toBe("Test");
    });

    it("attempts cloud sync when authenticated", () => {
      const client = createRecipeClient(mockRecipeService, mockExecutionService, authedClient);
      client.save(getDefinition(), { id: "r1", name: "Test", slug: "test" });

      expect(mockRecipeService.save).toHaveBeenCalled();
    });
  });

  describe("remove", () => {
    it("skips cloud removal when unauthenticated", () => {
      const client = createRecipeClient(mockRecipeService, mockExecutionService, mockAuthClient);
      const id = client.createFromDefinition(getDefinition());

      // Simulate a recipe with a cloudId
      recipesStore.getState().upsert({
        ...client.get(id)!,
        cloudId: "cloud-123",
      });

      client.remove(id);
      expect(mockRecipeService.remove).not.toHaveBeenCalled();
    });

    it("attempts cloud removal when authenticated and has cloudId", () => {
      const client = createRecipeClient(mockRecipeService, mockExecutionService, authedClient);
      const id = client.createFromDefinition(getDefinition());

      recipesStore.getState().upsert({
        ...client.get(id)!,
        cloudId: "cloud-123",
      });

      client.remove(id);
      expect(mockRecipeService.remove).toHaveBeenCalledWith("cloud-123");
    });

    it("removes locally regardless of auth state", () => {
      const client = createRecipeClient(mockRecipeService, mockExecutionService, mockAuthClient);
      const id = client.createFromDefinition(getDefinition());

      client.remove(id);
      expect(client.get(id)).toBeUndefined();
    });
  });

  describe("syncToCloud", () => {
    it("skips entirely when unauthenticated", async () => {
      const client = createRecipeClient(mockRecipeService, mockExecutionService, mockAuthClient);
      client.createFromDefinition(getDefinition());

      await client.syncToCloud();
      expect(mockRecipeService.save).not.toHaveBeenCalled();
    });

    it("syncs unsynced recipes when authenticated", async () => {
      const client = createRecipeClient(mockRecipeService, mockExecutionService, authedClient);
      client.createFromDefinition(getDefinition());

      await client.syncToCloud();
      expect(mockRecipeService.save).toHaveBeenCalled();
    });
  });
});
