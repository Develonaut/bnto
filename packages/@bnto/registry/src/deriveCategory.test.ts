import { describe, expect, it } from "vitest";

import { deriveCategory, CURRENT_FORMAT_VERSION } from "@bnto/nodes";
import type { Definition } from "@bnto/nodes";
import { RECIPES } from "./recipesCatalog";

describe("deriveCategory", () => {
  it("derives 'image' for definitions with image processing nodes", () => {
    const recipe = RECIPES.find((r) => r.slug === "compress-images")!;
    expect(deriveCategory(recipe.definition)).toBe("image");
  });

  it("derives 'spreadsheet' for definitions with spreadsheet processing nodes", () => {
    const recipe = RECIPES.find((r) => r.slug === "clean-csv")!;
    expect(deriveCategory(recipe.definition)).toBe("spreadsheet");
  });

  it("derives 'file' for definitions with file-system processing nodes", () => {
    const recipe = RECIPES.find((r) => r.slug === "rename-files")!;
    expect(deriveCategory(recipe.definition)).toBe("file");
  });

  it("matches recipe.category for every predefined recipe", () => {
    for (const recipe of RECIPES) {
      expect(deriveCategory(recipe.definition)).toBe(recipe.category);
    }
  });

  it("returns 'custom' for definitions with no processing nodes", () => {
    const def: Definition = {
      id: "empty",
      type: "group",
      version: CURRENT_FORMAT_VERSION,
      name: "Empty",
      position: { x: 0, y: 0 },
      metadata: {},
      parameters: {},
      inputPorts: [],
      outputPorts: [],
      nodes: [
        {
          id: "input",
          type: "input",
          version: CURRENT_FORMAT_VERSION,
          name: "Input",
          position: { x: 0, y: 0 },
          metadata: {},
          parameters: { mode: "file-upload" },
          inputPorts: [],
          outputPorts: [],
        },
        {
          id: "output",
          type: "output",
          version: CURRENT_FORMAT_VERSION,
          name: "Output",
          position: { x: 0, y: 0 },
          metadata: {},
          parameters: { mode: "write" },
          inputPorts: [],
          outputPorts: [],
        },
      ],
      edges: [],
    };
    expect(deriveCategory(def)).toBe("custom");
  });

  it("returns 'custom' for definitions with no nodes", () => {
    const def: Definition = {
      id: "no-nodes",
      type: "group",
      version: CURRENT_FORMAT_VERSION,
      name: "No Nodes",
      position: { x: 0, y: 0 },
      metadata: {},
      parameters: {},
      inputPorts: [],
      outputPorts: [],
    };
    expect(deriveCategory(def)).toBe("custom");
  });

  it("uses first processing node category for mixed definitions", () => {
    const def: Definition = {
      id: "mixed",
      type: "group",
      version: CURRENT_FORMAT_VERSION,
      name: "Mixed",
      position: { x: 0, y: 0 },
      metadata: {},
      parameters: {},
      inputPorts: [],
      outputPorts: [],
      nodes: [
        {
          id: "input",
          type: "input",
          version: CURRENT_FORMAT_VERSION,
          name: "Input",
          position: { x: 0, y: 0 },
          metadata: {},
          parameters: { mode: "file-upload" },
          inputPorts: [],
          outputPorts: [],
        },
        {
          id: "img",
          type: "image-compress",
          version: CURRENT_FORMAT_VERSION,
          name: "Image Compress",
          position: { x: 0, y: 0 },
          metadata: {},
          parameters: {},
          inputPorts: [],
          outputPorts: [],
        },
      ],
      edges: [],
    };
    expect(deriveCategory(def)).toBe("image");
  });
});
