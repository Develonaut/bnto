import { describe, expect, it } from "vitest";
import { BNTO_REGISTRY } from "#lib/bntoRegistry";
import { DEFAULT_CONFIGS, type BntoSlug } from "../types";

describe("BntoConfigMap defaults", () => {
  it("provides a default config for every registered bnto slug", () => {
    for (const entry of BNTO_REGISTRY) {
      const config = DEFAULT_CONFIGS[entry.slug as BntoSlug];
      expect(config).toBeDefined();
    }
  });

  it("compress-images defaults to quality 80", () => {
    expect(DEFAULT_CONFIGS["compress-images"]).toEqual({ quality: 80 });
  });

  it("resize-images defaults to width 800 with aspect ratio", () => {
    expect(DEFAULT_CONFIGS["resize-images"]).toEqual({
      width: 800,
      maintainAspectRatio: true,
    });
  });

  it("convert-image-format defaults to webp at quality 80", () => {
    expect(DEFAULT_CONFIGS["convert-image-format"]).toEqual({
      format: "webp",
      quality: 80,
    });
  });

  it("rename-files defaults to a pattern with name placeholder", () => {
    expect(DEFAULT_CONFIGS["rename-files"]).toEqual({
      pattern: "renamed-{{name}}",
    });
  });

  it("clean-csv defaults to trim + remove empty rows, no dedup", () => {
    expect(DEFAULT_CONFIGS["clean-csv"]).toEqual({
      trimWhitespace: true,
      removeEmptyRows: true,
      removeDuplicates: false,
    });
  });

  it("rename-csv-columns has an empty config", () => {
    expect(DEFAULT_CONFIGS["rename-csv-columns"]).toEqual({});
  });
});
