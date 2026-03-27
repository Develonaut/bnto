import { describe, it, expect } from "vitest";
import { applyConfigToDefinition } from "./applyConfigToDefinition";
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
      type: "image-compress",
      version: "1.0.0",
      name: "Compress",
      position: { x: 200, y: 0 },
      metadata: {},
      parameters: { quality: 80 },
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
              type: "file-rename",
              version: "1.0.0",
              name: "Rename File",
              position: { x: 0, y: 0 },
              metadata: {},
              parameters: { prefix: "default-" },
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

describe("applyConfigToDefinition", () => {
  it("merges config into leaf processing nodes", () => {
    const result = applyConfigToDefinition(SIMPLE_DEF, { quality: 50 });

    expect(result.nodes![1]!.parameters).toEqual({ quality: 50 });
  });

  it("does NOT merge config into I/O nodes", () => {
    const result = applyConfigToDefinition(SIMPLE_DEF, { quality: 50 });

    expect(result.nodes![0]!.parameters).toEqual({ mode: "file-upload" });
    expect(result.nodes![2]!.parameters).toEqual({ mode: "download" });
  });

  it("preserves full Definition shape (ports, metadata, position)", () => {
    const result = applyConfigToDefinition(SIMPLE_DEF, { quality: 50 });

    const compress = result.nodes![1]!;
    expect(compress.position).toEqual({ x: 200, y: 0 });
    expect(compress.inputPorts).toEqual([{ id: "in-1", name: "files" }]);
    expect(compress.name).toBe("Compress");
  });

  it("recurses into container children", () => {
    const result = applyConfigToDefinition(NESTED_DEF, { prefix: "custom-" });

    const renameFile = result.nodes![1]!.nodes![0]!.nodes![0]!;
    expect(renameFile.parameters).toEqual({ prefix: "custom-" });
  });

  it("does NOT merge config into container nodes themselves", () => {
    const result = applyConfigToDefinition(NESTED_DEF, { prefix: "custom-" });

    expect(result.nodes![1]!.parameters).toEqual({});
    expect(result.nodes![1]!.nodes![0]!.parameters).toEqual({ mode: "forEach" });
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

    const result = applyConfigToDefinition(emptyDef, { quality: 50 });
    expect(result).toBe(emptyDef);
  });

  it("works with real compress-images recipe", () => {
    const recipe = getRecipeBySlug("compress-images");
    expect(recipe).toBeDefined();

    const result = applyConfigToDefinition(recipe!.definition, { quality: 60 });
    const compressNode = result.nodes!.find((n) => n.type === "image-compress");
    expect(compressNode).toBeDefined();
    expect(compressNode!.parameters).toHaveProperty("quality", 60);
  });
});
