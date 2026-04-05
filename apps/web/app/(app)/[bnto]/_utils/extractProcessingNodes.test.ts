import { describe, expect, it } from "vitest";
import { getRecipeBySlug } from "@bnto/registry";

const compressImages = getRecipeBySlug("compress-images")!;
const optimizeImagesForWeb = getRecipeBySlug("optimize-images-for-web")!;
const cleanCsv = getRecipeBySlug("clean-csv")!;
import type { Definition } from "@bnto/core";
import { extractProcessingNodes } from "./extractProcessingNodes";

describe("extractProcessingNodes", () => {
  it("returns the single processing node from compress-images", () => {
    const nodes = extractProcessingNodes(compressImages.definition);
    expect(nodes).toHaveLength(1);
    expect(nodes[0].type).toBe("image-compress");
    expect(nodes[0].id).toBe("compress-image");
  });

  it("returns three processing nodes from optimize-images-for-web", () => {
    const nodes = extractProcessingNodes(optimizeImagesForWeb.definition);
    expect(nodes).toHaveLength(3);
    expect(nodes.map((n) => n.type)).toEqual(["image-resize", "image-convert", "image-compress"]);
  });

  it("returns the single processing node from clean-csv", () => {
    const nodes = extractProcessingNodes(cleanCsv.definition);
    expect(nodes).toHaveLength(1);
    expect(nodes[0].type).toBe("spreadsheet-clean");
  });

  it("skips input and output nodes", () => {
    const nodes = extractProcessingNodes(compressImages.definition);
    const types = nodes.map((n) => n.type);
    expect(types).not.toContain("input");
    expect(types).not.toContain("output");
  });

  it("recurses into nested containers", () => {
    const nested: Definition = {
      id: "root",
      type: "group",
      version: "1.0.0",
      name: "Root",
      position: { x: 0, y: 0 },
      metadata: {},
      parameters: {},
      inputPorts: [],
      outputPorts: [],
      nodes: [
        {
          id: "inner-group",
          type: "group",
          version: "1.0.0",
          name: "Inner",
          position: { x: 0, y: 0 },
          metadata: {},
          parameters: {},
          inputPorts: [],
          outputPorts: [],
          nodes: [
            {
              id: "deep-compress",
              type: "image-compress",
              version: "1.0.0",
              name: "Compress",
              position: { x: 0, y: 0 },
              metadata: {},
              parameters: { quality: 90 },
              inputPorts: [],
              outputPorts: [],
            },
          ],
        },
      ],
    };
    const nodes = extractProcessingNodes(nested);
    expect(nodes).toHaveLength(1);
    expect(nodes[0].id).toBe("deep-compress");
    expect(nodes[0].parameters).toEqual({ quality: 90 });
  });

  it("returns empty array for definition with no nodes", () => {
    const empty: Definition = {
      id: "empty",
      type: "group",
      version: "1.0.0",
      name: "Empty",
      position: { x: 0, y: 0 },
      metadata: {},
      parameters: {},
      inputPorts: [],
      outputPorts: [],
    };
    expect(extractProcessingNodes(empty)).toEqual([]);
  });

  it("preserves node parameters in the result", () => {
    const nodes = extractProcessingNodes(optimizeImagesForWeb.definition);
    const resize = nodes.find((n) => n.type === "image-resize");
    expect(resize?.parameters).toHaveProperty("width", 800);
  });
});
