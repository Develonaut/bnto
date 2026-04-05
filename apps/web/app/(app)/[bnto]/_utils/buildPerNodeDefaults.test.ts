import { describe, expect, it } from "vitest";
import { getRecipeBySlug } from "@bnto/registry";

const compressImages = getRecipeBySlug("compress-images")!;
const optimizeImagesForWeb = getRecipeBySlug("optimize-images-for-web")!;
import type { Definition } from "@bnto/core";
import { buildPerNodeDefaults } from "./buildPerNodeDefaults";

describe("buildPerNodeDefaults", () => {
  it("returns per-node defaults keyed by node ID", () => {
    const defaults = buildPerNodeDefaults(compressImages.definition);
    expect(Object.keys(defaults)).toEqual(["compress-image"]);
    expect(defaults["compress-image"]).toHaveProperty("quality");
  });

  it("returns defaults for all processing nodes in multi-node recipe", () => {
    const defaults = buildPerNodeDefaults(optimizeImagesForWeb.definition);
    expect(Object.keys(defaults)).toEqual(["resize", "convert", "compress"]);
    expect(defaults["resize"]).toHaveProperty("width", 800);
    expect(defaults["convert"]).toHaveProperty("format", "webp");
    expect(defaults["compress"]).toHaveProperty("quality");
  });

  it("returns empty object for definition with no processing nodes", () => {
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
    expect(buildPerNodeDefaults(empty)).toEqual({});
  });

  it("shallow copies parameters (no shared references)", () => {
    const defaults = buildPerNodeDefaults(compressImages.definition);
    const original = compressImages.definition.nodes![1].parameters;
    expect(defaults["compress-image"]).toEqual(original);
    expect(defaults["compress-image"]).not.toBe(original);
  });
});
