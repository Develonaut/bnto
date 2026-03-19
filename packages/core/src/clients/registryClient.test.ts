import { describe, it, expect, beforeAll } from "vitest";
import { registryStore } from "../stores/registryStore";
import { createRegistryClient } from "./registryClient";
import { RECIPES, NODE_TYPE_INFO, CATEGORIES, PROCESSORS } from "@bnto/nodes";

describe("registryClient", () => {
  const client = createRegistryClient();

  beforeAll(() => {
    registryStore.getState().populate({
      recipes: RECIPES,
      nodeTypes: NODE_TYPE_INFO,
      categories: [...CATEGORIES],
      processors: [...PROCESSORS],
    });
  });

  describe("getRecipes", () => {
    it("returns all predefined recipes", () => {
      expect(client.getRecipes()).toHaveLength(8);
    });

    it("every recipe has required fields", () => {
      for (const recipe of client.getRecipes()) {
        expect(recipe.id).toBeTruthy();
        expect(recipe.slug).toBeTruthy();
        expect(recipe.name).toBeTruthy();
        expect(recipe.definition).toBeDefined();
      }
    });
  });

  describe("getRecipesByCategory", () => {
    it("returns image recipes", () => {
      const imageRecipes = client.getRecipesByCategory("image");
      expect(imageRecipes.length).toBeGreaterThan(0);
      for (const r of imageRecipes) {
        expect(r.category).toBe("image");
      }
    });

    it("returns empty for unknown category", () => {
      expect(client.getRecipesByCategory("nonexistent")).toEqual([]);
    });
  });

  describe("getNodeTypes", () => {
    it("returns all 12 node types", () => {
      expect(Object.keys(client.getNodeTypes())).toHaveLength(12);
    });

    it("every node type has required fields", () => {
      for (const info of Object.values(client.getNodeTypes())) {
        expect(info.name).toBeTruthy();
        expect(info.label).toBeTruthy();
        expect(info.category).toBeTruthy();
        expect(typeof info.isContainer).toBe("boolean");
        expect(typeof info.browserCapable).toBe("boolean");
      }
    });
  });

  describe("getBrowserNodeTypes", () => {
    it("excludes server-only node types", () => {
      const browserTypes = client.getBrowserNodeTypes();
      for (const info of browserTypes) {
        expect(info.browserCapable).toBe(true);
      }
    });

    it("has fewer items than all node types", () => {
      const all = Object.values(client.getNodeTypes());
      const browser = client.getBrowserNodeTypes();
      expect(browser.length).toBeLessThan(all.length);
    });
  });

  describe("getCategories", () => {
    it("returns all 8 categories", () => {
      expect(client.getCategories()).toHaveLength(8);
    });
  });

  describe("getProcessors", () => {
    it("returns all 6 processors", () => {
      expect(client.getProcessors()).toHaveLength(6);
    });
  });

  describe("isInitialized", () => {
    it("returns true after populate", () => {
      expect(client.isInitialized()).toBe(true);
    });
  });
});
