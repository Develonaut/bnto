import { describe, expect, it } from "vitest";
import { getBrowserRecipes, getRecipeBySlug } from "@bnto/registry";
import type { Definition } from "@bnto/core";
import { buildPerNodeDefaults } from "./buildPerNodeDefaults";
import { missingRequiredParams } from "./missingRequiredParams";

const watermark = getRecipeBySlug("watermark-images")!.definition;

const OVERLAY = { nodeId: "overlay", param: "overlay", label: "Overlay Image" };

/** A one-step recipe whose `convert` step has `parameters`. */
function convertRecipe(parameters: Record<string, unknown>): Definition {
  return {
    id: "r",
    type: "group",
    version: "1.0.0",
    name: "R",
    position: { x: 0, y: 0 },
    metadata: {},
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    nodes: [
      {
        id: "convert",
        type: "image-convert",
        version: "1.0.0",
        name: "Convert",
        position: { x: 0, y: 0 },
        metadata: {},
        parameters,
        inputPorts: [],
        outputPorts: [],
      },
    ],
  };
}

describe("missingRequiredParams", () => {
  it("lists a required param the recipe leaves blank", () => {
    expect(missingRequiredParams(watermark, buildPerNodeDefaults(watermark))).toEqual([OVERLAY]);
  });

  it("is empty once the param has a value", () => {
    const config = buildPerNodeDefaults(watermark);
    config.overlay!.overlay = "data:image/png;base64,iVBORw0KGgo=";
    expect(missingRequiredParams(watermark, config)).toEqual([]);
  });

  it("counts null and whitespace-only text as no value", () => {
    for (const blank of [null, "   "]) {
      const config = buildPerNodeDefaults(watermark);
      config.overlay!.overlay = blank;
      expect(missingRequiredParams(watermark, config)).toEqual([OVERLAY]);
    }
  });

  it("counts a param left out as no value unless it has a default", () => {
    const config = buildPerNodeDefaults(watermark);
    delete config.overlay!.overlay;
    expect(missingRequiredParams(watermark, config)).toEqual([OVERLAY]);

    // image-convert's required `format` defaults to jpeg. An undefined value
    // is left out too: the JSON the engine gets drops it.
    expect(missingRequiredParams(convertRecipe({}), { convert: {} })).toEqual([]);
    const cleared = { convert: { format: undefined } };
    expect(missingRequiredParams(convertRecipe({}), cleared)).toEqual([]);
  });

  it("counts a blanked param as no value even when it has a default", () => {
    const missing = missingRequiredParams(convertRecipe({}), { convert: { format: "" } });
    expect(missing).toEqual([{ nodeId: "convert", param: "format", label: "Output Format" }]);
  });

  it("reads the recipe's own parameters for a node the config leaves out", () => {
    expect(missingRequiredParams(watermark, {})).toEqual([OVERLAY]);
    const filled = convertRecipe({ format: "png" });
    expect(missingRequiredParams(filled, {})).toEqual([]);
  });

  it("finds nothing missing in any other browser recipe as shipped", () => {
    const others = getBrowserRecipes().filter((r) => r.slug !== "watermark-images");
    for (const { slug, definition } of others) {
      const missing = missingRequiredParams(definition, buildPerNodeDefaults(definition));
      expect(missing, slug).toEqual([]);
    }
  });
});
