/**
 * useNodePalette tests — verify 1:1 node type to palette item mapping.
 *
 * Tests the pure `computePalette` function directly. The hook is a thin
 * useMemo wrapper, so testing the pure function covers all logic.
 */

import { describe, it, expect } from "vitest";
import { NODE_TYPE_INFO, CATEGORIES } from "@bnto/core";
import { computePalette } from "./useNodePalette";

function allItems(browserOnly = false) {
  return computePalette(NODE_TYPE_INFO, CATEGORIES, browserOnly).groups.flatMap((g) => g.items);
}

describe("computePalette", () => {
  it("creates one palette item per image node type", () => {
    const imageItems = allItems().filter((i) => i.category === "image");
    expect(imageItems).toHaveLength(4);
    const types = imageItems.map((i) => i.type);
    expect(types).toContain("image-compress");
    expect(types).toContain("image-convert");
    expect(types).toContain("image-resize");
    expect(types).toContain("image-strip-exif");
  });

  it("creates one palette item per spreadsheet node type", () => {
    const items = allItems().filter((i) => i.category === "spreadsheet");
    expect(items).toHaveLength(3);
    const types = items.map((i) => i.type);
    expect(types).toContain("spreadsheet-clean");
    expect(types).toContain("spreadsheet-rename");
    expect(types).toContain("spreadsheet-convert");
  });

  it("creates a palette item for file-rename", () => {
    const items = allItems().filter((i) => i.type === "file-rename");
    expect(items).toHaveLength(1);
    expect(items[0]!.label).toBe("Rename Files");
  });

  it("uses NODE_TYPE_INFO label for each item", () => {
    const compress = allItems().find((i) => i.type === "image-compress");
    expect(compress!.label).toBe("Compress Images");

    const convert = allItems().find((i) => i.type === "image-convert");
    expect(convert!.label).toBe("Convert Image Format");

    const rename = allItems().find((i) => i.type === "file-rename");
    expect(rename!.label).toBe("Rename Files");
  });

  it("uses NODE_TYPE_INFO description for each item", () => {
    const compress = allItems().find((i) => i.type === "image-compress");
    expect(compress!.description).toBe("Reduce image file size while maintaining quality.");
  });

  it("leaves container nodes unchanged", () => {
    const groups = allItems().filter((i) => i.type === "group");
    expect(groups).toHaveLength(1);
    expect(groups[0]!.label).toBe("Group");
  });

  it("excludes I/O nodes from the palette", () => {
    const items = allItems();
    const ioItems = items.filter((i) => i.type === "input" || i.type === "output");
    expect(ioItems).toHaveLength(0);
  });

  it("preserves category and browserCapable from NodeTypeInfo", () => {
    const compress = allItems().find((i) => i.type === "image-compress");
    expect(compress!.category).toBe("image");
    expect(compress!.browserCapable).toBe(true);
  });

  it("groups items by category in CATEGORIES display order", () => {
    const { groups } = computePalette(NODE_TYPE_INFO, CATEGORIES, false);
    const categoryNames = groups.map((g) => g.category.name);
    // CATEGORIES order: io (excluded), image, spreadsheet, file, ...
    expect(categoryNames.indexOf("image")).toBeLessThan(categoryNames.indexOf("file"));
  });

  it("browserOnly filters to browser-capable items only", () => {
    const items = allItems(true);
    expect(items.every((i) => i.browserCapable)).toBe(true);
    expect(items.find((i) => i.type === "http-request")).toBeUndefined();
  });
});
