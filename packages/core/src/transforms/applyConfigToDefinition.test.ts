import { describe, it, expect } from "vitest";
import { applyConfigToDefinition } from "./applyConfigToDefinition";
import type { Definition } from "@bnto/registry";
import { getRecipeBySlug } from "@bnto/registry";
import { SIMPLE_DEF, NESTED_DEF, MULTI_NODE_DEF } from "./__fixtures__/testDefinitions";

describe("applyConfigToDefinition", () => {
  it("merges per-node config into matching leaf processing node", () => {
    const result = applyConfigToDefinition(SIMPLE_DEF, { compress: { quality: 50 } });

    expect(result.nodes![1]!.parameters).toEqual({ quality: 50 });
  });

  it("does NOT merge config into I/O nodes", () => {
    const result = applyConfigToDefinition(SIMPLE_DEF, { compress: { quality: 50 } });

    expect(result.nodes![0]!.parameters).toEqual({ mode: "file-upload" });
    expect(result.nodes![2]!.parameters).toEqual({ mode: "download" });
  });

  it("preserves full Definition shape (ports, metadata, position)", () => {
    const result = applyConfigToDefinition(SIMPLE_DEF, { compress: { quality: 50 } });

    const compress = result.nodes![1]!;
    expect(compress.position).toEqual({ x: 200, y: 0 });
    expect(compress.inputPorts).toEqual([{ id: "in-1", name: "files" }]);
    expect(compress.name).toBe("Compress");
  });

  it("recurses into container children", () => {
    const result = applyConfigToDefinition(NESTED_DEF, {
      "rename-file": { prefix: "custom-" },
    });

    const renameFile = result.nodes![1]!.nodes![0]!.nodes![0]!;
    expect(renameFile.parameters).toEqual({ prefix: "custom-" });
  });

  it("does NOT merge config into container nodes themselves", () => {
    const result = applyConfigToDefinition(NESTED_DEF, {
      "rename-file": { prefix: "custom-" },
    });

    expect(result.nodes![1]!.parameters).toEqual({});
    expect(result.nodes![1]!.nodes![0]!.parameters).toEqual({ mode: "forEach" });
  });

  it("applies per-node config only to the matching node (no cross-contamination)", () => {
    const result = applyConfigToDefinition(MULTI_NODE_DEF, {
      resize: { width: 800 },
    });

    expect(result.nodes![1]!.parameters).toEqual({ width: 800 });
    expect(result.nodes![2]!.parameters).toEqual({ quality: 80 });
  });

  it("leaves unmatched processing nodes untouched", () => {
    const result = applyConfigToDefinition(SIMPLE_DEF, {
      "nonexistent-node": { foo: "bar" },
    });

    expect(result.nodes![1]!.parameters).toEqual({ quality: 80 });
  });

  it("returns original definition when config is empty", () => {
    const result = applyConfigToDefinition(SIMPLE_DEF, {});

    expect(result).toBe(SIMPLE_DEF);
  });

  it("returns original definition when nodes is empty", () => {
    const emptyDef: Definition = {
      id: "root",
      type: "group",
      version: "1.0.0",
      name: "Empty",
      position: { x: 0, y: 0 },
      metadata: {},
      parameters: {},
      inputPorts: [],
      outputPorts: [],
    };

    const result = applyConfigToDefinition(emptyDef, { compress: { quality: 50 } });
    expect(result).toBe(emptyDef);
  });

  it("works with real compress-images recipe", () => {
    const recipe = getRecipeBySlug("compress-images");
    expect(recipe).toBeDefined();

    const compressNode = recipe!.definition.nodes!.find((n) => n.type === "image-compress");
    expect(compressNode).toBeDefined();

    const result = applyConfigToDefinition(recipe!.definition, {
      [compressNode!.id]: { quality: 60 },
    });
    const resultNode = result.nodes!.find((n) => n.type === "image-compress");
    expect(resultNode).toBeDefined();
    expect(resultNode!.parameters).toHaveProperty("quality", 60);
  });
});
