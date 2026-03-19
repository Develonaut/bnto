import { describe, it, expect } from "vitest";
import { definitionToPipeline } from "./definitionToPipeline";
import type { Definition } from "@bnto/registry";
import { getRecipeBySlug } from "@bnto/registry";

const SIMPLE_DEF: Definition = {
  id: "root",
  type: "group",
  version: "1.0.0",
  name: "Test",
  position: { x: 0, y: 0 },
  metadata: {},
  parameters: {},
  inputPorts: [],
  outputPorts: [],
  nodes: [
    {
      id: "input",
      type: "input",
      version: "1.0.0",
      name: "Input",
      position: { x: 0, y: 0 },
      metadata: {},
      parameters: { mode: "file-upload" },
      inputPorts: [],
      outputPorts: [{ id: "out-1", name: "files" }],
    },
    {
      id: "compress",
      type: "image",
      version: "1.0.0",
      name: "Compress",
      position: { x: 200, y: 0 },
      metadata: {},
      parameters: { operation: "compress", quality: 80 },
      inputPorts: [{ id: "in-1", name: "files" }],
      outputPorts: [{ id: "out-1", name: "files" }],
    },
    {
      id: "output",
      type: "output",
      version: "1.0.0",
      name: "Output",
      position: { x: 400, y: 0 },
      metadata: {},
      parameters: { mode: "download" },
      inputPorts: [{ id: "in-1", name: "files" }],
      outputPorts: [],
    },
  ],
};

const NESTED_DEF: Definition = {
  id: "root",
  type: "group",
  version: "1.0.0",
  name: "Rename Files",
  position: { x: 0, y: 0 },
  metadata: {},
  parameters: {},
  inputPorts: [],
  outputPorts: [],
  nodes: [
    {
      id: "input",
      type: "input",
      version: "1.0.0",
      name: "Input",
      position: { x: 0, y: 0 },
      metadata: {},
      parameters: {},
      inputPorts: [],
      outputPorts: [],
    },
    {
      id: "batch-rename",
      type: "group",
      version: "1.0.0",
      name: "Batch Rename",
      position: { x: 200, y: 0 },
      metadata: {},
      parameters: {},
      inputPorts: [],
      outputPorts: [],
      nodes: [
        {
          id: "rename-loop",
          type: "loop",
          version: "1.0.0",
          name: "Loop",
          position: { x: 0, y: 0 },
          metadata: {},
          parameters: { mode: "forEach" },
          inputPorts: [],
          outputPorts: [],
          nodes: [
            {
              id: "rename-file",
              type: "file-system",
              version: "1.0.0",
              name: "Rename File",
              position: { x: 0, y: 0 },
              metadata: {},
              parameters: { operation: "rename", prefix: "default-" },
              inputPorts: [],
              outputPorts: [],
            },
          ],
        },
      ],
    },
    {
      id: "output",
      type: "output",
      version: "1.0.0",
      name: "Output",
      position: { x: 400, y: 0 },
      metadata: {},
      parameters: {},
      inputPorts: [],
      outputPorts: [],
    },
  ],
};

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
      type: "image",
      params: { operation: "compress", quality: 80 },
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

  it("merges config overrides into leaf processing nodes", () => {
    const pipeline = definitionToPipeline(SIMPLE_DEF, { quality: 50 });

    expect(pipeline.nodes[1]!.params).toEqual({
      operation: "compress",
      quality: 50,
    });
  });

  it("does NOT merge config into I/O nodes", () => {
    const pipeline = definitionToPipeline(SIMPLE_DEF, { quality: 50 });

    expect(pipeline.nodes[0]!.params).toEqual({ mode: "file-upload" });
    expect(pipeline.nodes[2]!.params).toEqual({ mode: "download" });
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
    expect(renameFile.type).toBe("file-system");
    expect(renameFile.params).toEqual({ operation: "rename", prefix: "default-" });
    expect(renameFile.children).toBeUndefined();
  });

  it("merges config into deeply nested leaf processors", () => {
    const pipeline = definitionToPipeline(NESTED_DEF, { prefix: "custom-" });

    const renameFile = pipeline.nodes[1]!.children![0]!.children![0]!;
    expect(renameFile.params).toEqual({
      operation: "rename",
      prefix: "custom-",
    });
  });

  it("does NOT merge config into container nodes", () => {
    const pipeline = definitionToPipeline(NESTED_DEF, { prefix: "custom-" });

    expect(pipeline.nodes[1]!.params).toEqual({});
    expect(pipeline.nodes[1]!.children![0]!.params).toEqual({ mode: "forEach" });
  });

  it("merges quality override into real compress-images recipe", () => {
    const recipe = getRecipeBySlug("compress-images");
    expect(recipe).toBeDefined();

    const pipeline = definitionToPipeline(recipe!.definition, { quality: 60 });

    // The compress-image node is inside a loop container
    const loop = pipeline.nodes.find((n) => n.type === "loop");
    expect(loop).toBeDefined();
    expect(loop!.children).toBeDefined();

    const compressNode = loop!.children!.find((n) => n.type === "image");
    expect(compressNode).toBeDefined();
    expect(compressNode!.params).toHaveProperty("quality", 60);
    expect(compressNode!.params).toHaveProperty("operation", "compress");
  });

  it("uses engine defaults when no override provided for compress-images", () => {
    const recipe = getRecipeBySlug("compress-images");
    expect(recipe).toBeDefined();

    const pipeline = definitionToPipeline(recipe!.definition);

    const loop = pipeline.nodes.find((n) => n.type === "loop");
    const compressNode = loop!.children!.find((n) => n.type === "image");
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
