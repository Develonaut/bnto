import { describe, it, expect, beforeEach } from "vitest";
import { RECIPES, NODE_TYPE_INFO, CATEGORIES, PROCESSORS } from "@bnto/registry";
import { registryStore } from "../stores/registryStore";
import { createRegistryService } from "./registryService";

describe("registryService", () => {
  const service = createRegistryService();

  beforeEach(() => {
    registryStore.getState().reset();
  });

  it("initialize() populates the registry with @bnto/nodes data", () => {
    service.initialize();

    const state = registryStore.getState();
    expect(state.initialized).toBe(true);
    expect(state.recipes).toHaveLength(RECIPES.length);
    expect(Object.keys(state.nodeTypes)).toHaveLength(Object.keys(NODE_TYPE_INFO).length);
    expect(state.categories).toHaveLength(CATEGORIES.length);
    expect(state.processors).toHaveLength(PROCESSORS.length);
  });

  it("initialize() is idempotent — skips if already initialized", () => {
    service.initialize();
    const firstRecipes = registryStore.getState().recipes;

    service.initialize();
    expect(registryStore.getState().recipes).toBe(firstRecipes);
  });
});
