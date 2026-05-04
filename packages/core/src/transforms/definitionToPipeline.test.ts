import { describe, it, expect } from "vitest";
import { definitionToPipeline } from "./definitionToPipeline";
import type { Definition } from "@bnto/registry";
import { getRecipeBySlug } from "@bnto/registry";
import { SIMPLE_DEF, NESTED_DEF, MULTI_NODE_DEF } from "./__fixtures__/testDefinitions";

describe("definitionToPipeline", () => {
  it("converts a flat definition with correct node structure", () => {
    const pipeline = definitionToPipeline(SIMPLE_DEF);

    expect(pipeline.nodes).toHaveLength(3);
    expect(pipeline.nodes[0]).toEqual({
      id: "input",
      type: "input",
      params: { mode: "file-upload" },
    });
    expect(pipeline.nodes[1]).toEqual({
      id: "compress",
      type: "image-compress",
      params: { quality: 80 },
    });
  });

  it("strips editor metadata (position, ports, name, version)", () => {
    const pipeline = definitionToPipeline(SIMPLE_DEF);
    const node = pipeline.nodes[1]!;

    expect(node).not.toHaveProperty("position");
    expect(node).not.toHaveProperty("inputPorts");
    expect(node).not.toHaveProperty("outputPorts");
    expect(node).not.toHaveProperty("name");
    expect(node).not.toHaveProperty("version");
    expect(node).not.toHaveProperty("metadata");
  });

  it("merges per-node config into matching leaf processing node", () => {
    const pipeline = definitionToPipeline(SIMPLE_DEF, { compress: { quality: 50 } });

    expect(pipeline.nodes[1]!.params).toEqual({ quality: 50 });
  });

  it("does NOT merge config into I/O nodes", () => {
    const pipeline = definitionToPipeline(SIMPLE_DEF, { compress: { quality: 50 } });

    expect(pipeline.nodes[0]!.params).toEqual({ mode: "file-upload" });
    expect(pipeline.nodes[2]!.params).toEqual({ mode: "write" });
  });

  it("preserves nested children for container nodes", () => {
    const pipeline = definitionToPipeline(NESTED_DEF);

    const batchRename = pipeline.nodes[1]!;
    expect(batchRename.type).toBe("group");
    expect(batchRename.children).toHaveLength(1);

    const loop = batchRename.children![0]!;
    expect(loop.type).toBe("loop");
    expect(loop.children).toHaveLength(1);

    const renameFile = loop.children![0]!;
    expect(renameFile.type).toBe("file-rename");
    expect(renameFile.params).toEqual({ prefix: "default-" });
    expect(renameFile.children).toBeUndefined();
  });

  it("merges config into deeply nested leaf processors", () => {
    const pipeline = definitionToPipeline(NESTED_DEF, {
      "rename-file": { prefix: "custom-" },
    });

    const renameFile = pipeline.nodes[1]!.children![0]!.children![0]!;
    expect(renameFile.params).toEqual({ prefix: "custom-" });
  });

  it("does NOT merge config into container nodes", () => {
    const pipeline = definitionToPipeline(NESTED_DEF, {
      "rename-file": { prefix: "custom-" },
    });

    expect(pipeline.nodes[1]!.params).toEqual({});
    expect(pipeline.nodes[1]!.children![0]!.params).toEqual({ mode: "forEach" });
  });

  it("applies per-node config only to the matching node (no cross-contamination)", () => {
    const pipeline = definitionToPipeline(MULTI_NODE_DEF, {
      resize: { width: 800 },
    });

    expect(pipeline.nodes[1]!.params).toEqual({ width: 800 });
    expect(pipeline.nodes[2]!.params).toEqual({ quality: 80 });
  });

  it("applies config to multiple nodes independently", () => {
    const pipeline = definitionToPipeline(MULTI_NODE_DEF, {
      resize: { width: 640 },
      compress: { quality: 50 },
    });

    expect(pipeline.nodes[1]!.params).toEqual({ width: 640 });
    expect(pipeline.nodes[2]!.params).toEqual({ quality: 50 });
  });

  it("merges quality override into real compress-images recipe", () => {
    const recipe = getRecipeBySlug("compress-images");
    expect(recipe).toBeDefined();

    const compressNode = recipe!.definition.nodes!.find((n) => n.type === "image-compress");
    expect(compressNode).toBeDefined();

    const pipeline = definitionToPipeline(recipe!.definition, {
      [compressNode!.id]: { quality: 60 },
    });

    const pipelineNode = pipeline.nodes.find((n) => n.type === "image-compress");
    expect(pipelineNode).toBeDefined();
    expect(pipelineNode!.params).toHaveProperty("quality", 60);
  });

  it("uses engine defaults when no override provided for compress-images", () => {
    const recipe = getRecipeBySlug("compress-images");
    expect(recipe).toBeDefined();

    const pipeline = definitionToPipeline(recipe!.definition);

    const compressNode = pipeline.nodes.find((n) => n.type === "image-compress");
    expect(compressNode!.params).toHaveProperty("quality", 80);
  });

  it("handles definition with no children", () => {
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

    const pipeline = definitionToPipeline(emptyDef);
    expect(pipeline.nodes).toEqual([]);
  });
});
