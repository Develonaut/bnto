/**
 * Definition service tests — verifies definition operations through service.
 */

import { describe, it, expect, beforeEach } from "vitest";
import { getRecipeBySlug } from "@bnto/nodes";
import { createEditorStore } from "../store/createEditorStore";
import { createDefinitionService } from "./definitionService";
import { createNodeService } from "./nodeService";
import type { DefinitionService } from "../editorTypes";

function compressImagesDefinition() {
  const recipe = getRecipeBySlug("compress-images");
  if (!recipe) throw new Error("compress-images recipe not found");
  return recipe.definition;
}

describe("createDefinitionService", () => {
  let service: DefinitionService;
  let storeApi: ReturnType<typeof createEditorStore>;

  beforeEach(() => {
    storeApi = createEditorStore();
    service = createDefinitionService(storeApi);
  });

  it("loadDefinition replaces the current state", () => {
    const def = compressImagesDefinition();
    service.loadDefinition(def);
    expect(storeApi.getState().recipeMetadata.name).toBe("Compress Images");
  });

  it("createBlank resets to default I/O nodes", () => {
    const def = compressImagesDefinition();
    service.loadDefinition(def);
    service.createBlank();
    expect(storeApi.getState().recipeMetadata.type).toBe("group");
    expect(storeApi.getState().nodes.length).toBe(2);
  });

  it("updateParams merges params into node config", () => {
    const nodeSvc = createNodeService(storeApi);
    const id = nodeSvc.addNode("image")!;
    const ok = service.updateParams(id, { quality: 90 });
    expect(ok).toBe(true);
    expect(storeApi.getState().configs[id]?.parameters.quality).toBe(90);
  });

  it("updateParams returns false for nonexistent node", () => {
    expect(service.updateParams("fake", { quality: 90 })).toBe(false);
  });

  it("markDirty and resetDirty toggle dirty flag", () => {
    expect(storeApi.getState().isDirty).toBe(false);
    service.markDirty();
    expect(storeApi.getState().isDirty).toBe(true);
    service.resetDirty();
    expect(storeApi.getState().isDirty).toBe(false);
  });

  it("setRecipeMetadata updates metadata", () => {
    service.setRecipeMetadata({ id: "custom", name: "Custom", type: "group", version: "2.0" });
    expect(storeApi.getState().recipeMetadata.name).toBe("Custom");
  });

  it("exportAsDefinition returns a valid Definition", () => {
    const nodeSvc = createNodeService(storeApi);
    nodeSvc.addNode("image");
    const def = service.exportAsDefinition();
    expect(def.nodes).toBeDefined();
    expect(def.nodes!.length).toBeGreaterThan(0);
  });
});
