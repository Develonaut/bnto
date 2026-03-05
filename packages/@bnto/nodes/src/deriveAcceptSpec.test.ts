import { describe, expect, it } from "vitest";

import { deriveAcceptSpec } from "./deriveAcceptSpec";
import { createBlankDefinition } from "./createBlankDefinition";
import { RECIPES } from "./recipes";
import type { Definition } from "./definition";

describe("deriveAcceptSpec", () => {
  it("derives accept spec from blank definition", () => {
    const def = createBlankDefinition();
    const spec = deriveAcceptSpec(def);
    expect(spec).toBeDefined();
    expect(spec!.mimeTypes).toEqual(["*/*"]);
    expect(spec!.extensions).toEqual([]);
    expect(spec!.label).toBe("Any files");
  });

  it("derives correct accept spec from compress-images recipe", () => {
    const recipe = RECIPES.find((r) => r.slug === "compress-images")!;
    const spec = deriveAcceptSpec(recipe.definition);
    expect(spec).toBeDefined();
    expect(spec!.mimeTypes).toContain("image/jpeg");
    expect(spec!.mimeTypes).toContain("image/png");
    expect(spec!.mimeTypes).toContain("image/webp");
  });

  it("derives correct accept spec from clean-csv recipe", () => {
    const recipe = RECIPES.find((r) => r.slug === "clean-csv")!;
    const spec = deriveAcceptSpec(recipe.definition);
    expect(spec).toBeDefined();
    expect(spec!.mimeTypes).toEqual(["text/csv"]);
    expect(spec!.extensions).toEqual([".csv"]);
    expect(spec!.label).toBe("CSV files");
  });

  it("derives correct accept spec from rename-files recipe", () => {
    const recipe = RECIPES.find((r) => r.slug === "rename-files")!;
    const spec = deriveAcceptSpec(recipe.definition);
    expect(spec).toBeDefined();
    expect(spec!.mimeTypes).toEqual(["*/*"]);
    expect(spec!.label).toBe("any files");
  });

  it("matches recipe.accept for every predefined recipe", () => {
    for (const recipe of RECIPES) {
      const derived = deriveAcceptSpec(recipe.definition);
      expect(derived).toBeDefined();
      // The derived spec should match the recipe-level accept spec
      expect(derived!.mimeTypes).toEqual(recipe.accept.mimeTypes);
      expect(derived!.label).toBe(recipe.accept.label);
    }
  });

  it("returns undefined when no input node exists", () => {
    const def: Definition = {
      id: "no-input",
      type: "group",
      version: "1.0.0",
      name: "No Input",
      position: { x: 0, y: 0 },
      metadata: {},
      parameters: {},
      inputPorts: [],
      outputPorts: [],
      nodes: [],
      edges: [],
    };
    expect(deriveAcceptSpec(def)).toBeUndefined();
  });

  it("defaults label to 'Any files' when input has no label", () => {
    const def: Definition = {
      id: "no-label",
      type: "group",
      version: "1.0.0",
      name: "No Label",
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
          outputPorts: [],
        },
      ],
      edges: [],
    };
    const spec = deriveAcceptSpec(def);
    expect(spec).toBeDefined();
    expect(spec!.mimeTypes).toEqual([]);
    expect(spec!.extensions).toEqual([]);
    expect(spec!.label).toBe("Any files");
  });
});
