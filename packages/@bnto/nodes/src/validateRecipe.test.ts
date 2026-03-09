import { describe, expect, it } from "vitest";

import type { Definition } from "./definition";
import type { Recipe } from "./recipe";
import { CURRENT_FORMAT_VERSION } from "./formatVersion";
import { RECIPES } from "./recipes";
import { validateRecipe } from "./validateRecipe";

/** Creates a minimal valid recipe for testing. */
function validRecipe(overrides: Partial<Recipe> = {}): Recipe {
  return {
    slug: "test-recipe",
    name: "Test Recipe",
    description: "A test recipe.",
    category: "image",
    accept: { mimeTypes: ["image/png"], extensions: [".png"], label: "PNG images" },
    features: ["PNG"],
    seo: { title: "Test -- bnto", h1: "Test" },
    definition: {
      id: "test-recipe",
      type: "group",
      version: CURRENT_FORMAT_VERSION,
      name: "Test Recipe",
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
          parameters: {
            mode: "file-upload",
            accept: ["image/png"],
            extensions: [".png"],
            label: "PNG",
            multiple: true,
          },
          inputPorts: [],
          outputPorts: [{ id: "out-1", name: "files" }],
        },
        {
          id: "process",
          type: "image",
          version: CURRENT_FORMAT_VERSION,
          name: "Process",
          position: { x: 250, y: 0 },
          metadata: {},
          parameters: { operation: "compress" },
          inputPorts: [{ id: "in-1", name: "files" }],
          outputPorts: [{ id: "out-1", name: "files" }],
        },
        {
          id: "output",
          type: "output",
          version: CURRENT_FORMAT_VERSION,
          name: "Output",
          position: { x: 500, y: 0 },
          metadata: {},
          parameters: { mode: "download", label: "Output", zip: true, autoDownload: false },
          inputPorts: [{ id: "in-1", name: "files" }],
          outputPorts: [],
        },
      ],
      edges: [
        { id: "e1", source: "input", target: "process" },
        { id: "e2", source: "process", target: "output" },
      ],
    },
    ...overrides,
  };
}

/** Helper to replace the definition on a recipe. */
function withDef(recipe: Recipe, defOverrides: Partial<Definition>): Recipe {
  return { ...recipe, definition: { ...recipe.definition, ...defOverrides } };
}

describe("validateRecipe — I/O checks", () => {
  it("valid recipe passes", () => {
    const errors = validateRecipe(validRecipe());
    expect(errors).toHaveLength(0);
  });

  it("missing input → error", () => {
    const recipe = withDef(validRecipe(), {
      nodes: validRecipe().definition.nodes!.filter((n) => n.type !== "input"),
    });
    const errors = validateRecipe(recipe);
    expect(errors.some((e) => e.message.includes("input node"))).toBe(true);
  });

  it("missing output → error", () => {
    const recipe = withDef(validRecipe(), {
      nodes: validRecipe().definition.nodes!.filter((n) => n.type !== "output"),
    });
    const errors = validateRecipe(recipe);
    expect(errors.some((e) => e.message.includes("output node"))).toBe(true);
  });

  it("multiple inputs → error", () => {
    const inputNode = validRecipe().definition.nodes![0]!;
    const recipe = withDef(validRecipe(), {
      nodes: [...validRecipe().definition.nodes!, { ...inputNode, id: "input-2" }],
    });
    const errors = validateRecipe(recipe);
    expect(errors.some((e) => e.message.includes("2 input nodes"))).toBe(true);
  });

  it("multiple outputs → error", () => {
    const outputNode = validRecipe().definition.nodes![2]!;
    const recipe = withDef(validRecipe(), {
      nodes: [...validRecipe().definition.nodes!, { ...outputNode, id: "output-2" }],
    });
    const errors = validateRecipe(recipe);
    expect(errors.some((e) => e.message.includes("2 output nodes"))).toBe(true);
  });

  it("input without outputPorts → error", () => {
    const nodes = validRecipe().definition.nodes!.map((n) =>
      n.type === "input" ? { ...n, outputPorts: [] } : n,
    );
    const recipe = withDef(validRecipe(), { nodes });
    const errors = validateRecipe(recipe);
    expect(errors.some((e) => e.message.includes("outputPort"))).toBe(true);
  });

  it("output without inputPorts → error", () => {
    const nodes = validRecipe().definition.nodes!.map((n) =>
      n.type === "output" ? { ...n, inputPorts: [] } : n,
    );
    const recipe = withDef(validRecipe(), { nodes });
    const errors = validateRecipe(recipe);
    expect(errors.some((e) => e.message.includes("inputPort"))).toBe(true);
  });
});

describe("validateRecipe — structural checks", () => {
  it("root type not group → error", () => {
    const recipe = withDef(validRecipe(), { type: "image" });
    const errors = validateRecipe(recipe);
    expect(errors.some((e) => e.message.includes("must be type 'group'"))).toBe(true);
  });

  it("root missing nodes → error", () => {
    const recipe = withDef(validRecipe(), { nodes: [] });
    const errors = validateRecipe(recipe);
    expect(errors.some((e) => e.message.includes("must have a nodes array"))).toBe(true);
  });

  it("definition ID doesn't match slug → error", () => {
    const recipe = withDef(validRecipe(), { id: "wrong-id" });
    const errors = validateRecipe(recipe);
    expect(errors.some((e) => e.message.includes("must match recipe slug"))).toBe(true);
  });
});

describe("validateRecipe — edge connectivity", () => {
  it("disconnected node → error", () => {
    const recipe = validRecipe();
    const disconnected: Definition = {
      id: "disconnected",
      type: "image",
      version: CURRENT_FORMAT_VERSION,
      name: "Disconnected",
      position: { x: 300, y: 300 },
      metadata: {},
      parameters: { operation: "compress" },
      inputPorts: [],
      outputPorts: [],
    };
    const withDisconnected = withDef(recipe, {
      nodes: [...recipe.definition.nodes!, disconnected],
    });
    const errors = validateRecipe(withDisconnected);
    expect(errors.some((e) => e.message.includes("disconnected"))).toBe(true);
  });
});

describe("validateRecipe — all 12 recipes pass", () => {
  for (const recipe of RECIPES) {
    it(`${recipe.slug} passes validation`, () => {
      const errors = validateRecipe(recipe);
      expect(errors).toHaveLength(0);
    });
  }
});
