import { describe, it, expect, beforeEach } from "vitest";
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
    expect(state.recipes).toHaveLength(10);
    expect(Object.keys(state.nodeTypes)).toHaveLength(15);
    expect(state.categories).toHaveLength(8);
    expect(state.processors).toHaveLength(6);
  });

  it("initialize() is idempotent — skips if already initialized", () => {
    service.initialize();
    const firstRecipes = registryStore.getState().recipes;

    service.initialize();
    expect(registryStore.getState().recipes).toBe(firstRecipes);
  });
});
