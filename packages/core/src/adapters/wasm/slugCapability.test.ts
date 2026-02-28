import { describe, it, expect } from "vitest";
import {
  isBrowserCapable,
  getBrowserNodeType,
  getBrowserCapableSlugs,
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
    it.each(ALL_TIER_1_SLUGS)("returns the node type for %s", (slug) => {
      expect(getBrowserNodeType(slug)).toBe(slug);
    });

    it("returns undefined for unknown slugs", () => {
      expect(getBrowserNodeType("unknown")).toBeUndefined();
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
