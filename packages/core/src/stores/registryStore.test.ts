import { describe, it, expect, beforeEach } from "vitest";
import { registryStore } from "./registryStore";
import { RECIPES, NODE_TYPE_INFO, CATEGORIES, PROCESSORS } from "@bnto/registry";

describe("registryStore", () => {
  beforeEach(() => {
    registryStore.getState().reset();
  });

  it("starts uninitialized with empty data", () => {
    const state = registryStore.getState();
    expect(state.initialized).toBe(false);
    expect(state.recipes).toEqual([]);
    expect(state.nodeTypes).toEqual({});
    expect(state.categories).toEqual([]);
    expect(state.processors).toEqual([]);
  });

  it("populate() sets all data and marks initialized", () => {
    registryStore.getState().populate({
      recipes: RECIPES,
      nodeTypes: NODE_TYPE_INFO,
      categories: [...CATEGORIES],
      processors: [...PROCESSORS],
    });

    const state = registryStore.getState();
    expect(state.initialized).toBe(true);
    expect(state.recipes).toHaveLength(10);
    expect(Object.keys(state.nodeTypes)).toHaveLength(15);
    expect(state.categories).toHaveLength(8);
    expect(state.processors).toHaveLength(6);
  });

  it("populate() is idempotent — calling twice keeps data intact", () => {
    const data = {
      recipes: RECIPES,
      nodeTypes: NODE_TYPE_INFO,
      categories: [...CATEGORIES],
      processors: [...PROCESSORS],
    };

    registryStore.getState().populate(data);
    registryStore.getState().populate(data);

    const state = registryStore.getState();
    expect(state.initialized).toBe(true);
    expect(state.recipes).toHaveLength(10);
  });

  it("reset() clears all data and marks uninitialized", () => {
    registryStore.getState().populate({
      recipes: RECIPES,
      nodeTypes: NODE_TYPE_INFO,
      categories: [...CATEGORIES],
      processors: [...PROCESSORS],
    });

    registryStore.getState().reset();

    const state = registryStore.getState();
    expect(state.initialized).toBe(false);
    expect(state.recipes).toEqual([]);
  });
});
