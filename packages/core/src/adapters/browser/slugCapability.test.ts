import { describe, it, expect } from "vitest";
import {
  isBrowserCapable,
  getBrowserNodeType,
  getBrowserCapableSlugs,
  getNodeOperation,
} from "./slugCapability";

const ALL_TIER_1_SLUGS = [
  "compress-images",
  "resize-images",
  "convert-image-format",
  "clean-csv",
  "rename-csv-columns",
  "rename-files",
];

describe("slugCapability", () => {
  describe("isBrowserCapable", () => {
    it.each(ALL_TIER_1_SLUGS)("returns true for %s", (slug) => {
      expect(isBrowserCapable(slug)).toBe(true);
    });

    it("returns false for unknown slugs", () => {
      expect(isBrowserCapable("unknown-slug")).toBe(false);
      expect(isBrowserCapable("")).toBe(false);
    });
  });

  describe("getBrowserNodeType", () => {
    it.each(ALL_TIER_1_SLUGS)("returns a node type for %s", (slug) => {
      expect(getBrowserNodeType(slug)).toBeDefined();
    });

    it("returns undefined for unknown slugs", () => {
      expect(getBrowserNodeType("unknown")).toBeUndefined();
    });
  });

  describe("getNodeOperation", () => {
    it("returns nodeType and operation for image slugs", () => {
      expect(getNodeOperation("compress-images")).toEqual({
        nodeType: "image",
        operation: "compress",
      });
      expect(getNodeOperation("resize-images")).toEqual({ nodeType: "image", operation: "resize" });
      expect(getNodeOperation("convert-image-format")).toEqual({
        nodeType: "image",
        operation: "convert",
      });
    });

    it("returns nodeType and operation for csv slugs", () => {
      expect(getNodeOperation("clean-csv")).toEqual({
        nodeType: "spreadsheet",
        operation: "clean",
      });
      expect(getNodeOperation("rename-csv-columns")).toEqual({
        nodeType: "spreadsheet",
        operation: "rename",
      });
    });

    it("returns nodeType and operation for file slugs", () => {
      expect(getNodeOperation("rename-files")).toEqual({
        nodeType: "file-system",
        operation: "rename",
      });
    });

    it("returns undefined for unknown slugs", () => {
      expect(getNodeOperation("unknown")).toBeUndefined();
    });
  });

  describe("getBrowserCapableSlugs", () => {
    it("returns all 6 Tier 1 slugs", () => {
      const slugs = getBrowserCapableSlugs();
      expect(slugs).toHaveLength(6);
      for (const slug of ALL_TIER_1_SLUGS) {
        expect(slugs).toContain(slug);
      }
    });
  });
});
