/**
 * useNodePalette tests — verify multi-operation split and palette grouping.
 *
 * Tests the pure `computePalette` function directly. The hook is a thin
 * useMemo wrapper, so testing the pure function covers all logic.
 */

import { describe, it, expect } from "vitest";
import { NODE_TYPE_INFO, CATEGORIES, PROCESSORS } from "@bnto/nodes";
import { computePalette } from "./useNodePalette";
import type { PaletteItem } from "./useNodePalette";

function allItems(browserOnly = false): PaletteItem[] {
  return computePalette(NODE_TYPE_INFO, CATEGORIES, PROCESSORS, browserOnly).groups.flatMap(
    (g) => g.items,
  );
}

describe("computePalette", () => {
  it("splits image into 3 palette items (compress, convert, resize)", () => {
    const imageItems = allItems().filter((i) => i.type === "image");
    expect(imageItems).toHaveLength(3);
    const ops = imageItems.map((i) => i.defaultParams?.operation);
    expect(ops).toContain("compress");
    expect(ops).toContain("convert");
    expect(ops).toContain("resize");
  });

  it("splits spreadsheet into 2 palette items (clean, rename)", () => {
    const items = allItems().filter((i) => i.type === "spreadsheet");
    expect(items).toHaveLength(2);
    const ops = items.map((i) => i.defaultParams?.operation);
    expect(ops).toContain("clean");
    expect(ops).toContain("rename");
  });

  it("pre-sets file-system operation to rename", () => {
    const items = allItems().filter((i) => i.type === "file-system");
    expect(items).toHaveLength(1);
    expect(items[0]!.defaultParams).toEqual({ operation: "rename" });
  });

  it("uses processor name as label for split items", () => {
    const compress = allItems().find(
      (i) => i.type === "image" && i.defaultParams?.operation === "compress",
    );
    expect(compress!.label).toBe("Compress Images");

    const convert = allItems().find(
      (i) => i.type === "image" && i.defaultParams?.operation === "convert",
    );
    expect(convert!.label).toBe("Convert Image Format");

    const renameFiles = allItems().find(
      (i) => i.type === "file-system" && i.defaultParams?.operation === "rename",
    );
    expect(renameFiles!.label).toBe("Rename Files");
  });

  it("uses processor description for split items", () => {
    const compress = allItems().find(
      (i) => i.type === "image" && i.defaultParams?.operation === "compress",
    );
    expect(compress!.description).toBe("Reduce image file size while maintaining quality");
  });

  it("leaves non-processor nodes unchanged", () => {
    const groups = allItems().filter((i) => i.type === "group");
    expect(groups).toHaveLength(1);
    expect(groups[0]!.defaultParams).toBeUndefined();
    expect(groups[0]!.label).toBe("Group");
  });

  it("excludes I/O nodes from the palette", () => {
    const items = allItems();
    const ioItems = items.filter((i) => i.type === "input" || i.type === "output");
    expect(ioItems).toHaveLength(0);
  });

  it("preserves category and browserCapable from NodeTypeInfo", () => {
    const compress = allItems().find(
      (i) => i.type === "image" && i.defaultParams?.operation === "compress",
    );
    expect(compress!.category).toBe("image");
    expect(compress!.browserCapable).toBe(true);
  });

  it("groups items by category in CATEGORIES display order", () => {
    const { groups } = computePalette(NODE_TYPE_INFO, CATEGORIES, PROCESSORS, false);
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
