import { describe, expect, it } from "vitest";
import { BNTO_REGISTRY } from "@/lib/bntoRegistry";
import { DEFAULT_CONFIGS, type BntoSlug } from "../types";

describe("BntoConfigMap defaults", () => {
  it("every BntoConfigMap slug has a default config", () => {
    const configSlugs = Object.keys(DEFAULT_CONFIGS) as BntoSlug[];
    for (const slug of configSlugs) {
      expect(DEFAULT_CONFIGS[slug]).toBeDefined();
    }
  });

  it("every config slug is a registered bnto", () => {
    const registrySlugs = new Set(BNTO_REGISTRY.map((e) => e.slug));
    for (const slug of Object.keys(DEFAULT_CONFIGS)) {
      expect(registrySlugs.has(slug)).toBe(true);
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

  it("watermark-images defaults to bottom-right at 25% size and 80% opacity", () => {
    expect(DEFAULT_CONFIGS["watermark-images"]).toEqual({
      watermark: "",
      position: "bottom-right",
      size: 25,
      opacity: 80,
      offsetX: 0,
      offsetY: 0,
      quality: 80,
    });
  });
});
