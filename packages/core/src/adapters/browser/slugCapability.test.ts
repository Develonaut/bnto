import { describe, it, expect } from "vitest";
import {
  isBrowserCapable,
  getBrowserNodeType,
  getBrowserCapableSlugs,
} from "./slugCapability";

describe("slugCapability", () => {
  describe("isBrowserCapable", () => {
    it("returns true for compress-images", () => {
      expect(isBrowserCapable("compress-images")).toBe(true);
    });

    it("returns false for slugs without browser implementations", () => {
      expect(isBrowserCapable("resize-images")).toBe(false);
      expect(isBrowserCapable("rename-files")).toBe(false);
      expect(isBrowserCapable("clean-csv")).toBe(false);
    });

    it("returns false for unknown slugs", () => {
      expect(isBrowserCapable("unknown-slug")).toBe(false);
      expect(isBrowserCapable("")).toBe(false);
    });
  });

  describe("getBrowserNodeType", () => {
    it("returns the node type for compress-images", () => {
      expect(getBrowserNodeType("compress-images")).toBe("compress-images");
    });

    it("returns undefined for slugs without browser implementations", () => {
      expect(getBrowserNodeType("resize-images")).toBeUndefined();
      expect(getBrowserNodeType("unknown")).toBeUndefined();
    });
  });

  describe("getBrowserCapableSlugs", () => {
    it("returns an array of capable slugs", () => {
      const slugs = getBrowserCapableSlugs();
      expect(slugs).toContain("compress-images");
      expect(slugs.length).toBeGreaterThan(0);
    });
  });
});
