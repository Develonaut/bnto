/**
 * getNodeIcon tests — verify icon resolution for all node types and modes.
 */

import { describe, it, expect } from "vitest";
import { getNodeIcon } from "./getNodeIcon";
import { NODE_TYPE_NAMES, NODE_TYPE_INFO } from "./nodeTypes";

describe("getNodeIcon", () => {
  // --- Input node icons (mode-aware) ---

  it("returns 'file-up' for input file-upload mode", () => {
    expect(getNodeIcon("input", { mode: "file-upload" })).toBe("file-up");
  });

  it("returns 'text-cursor-input' for input text mode", () => {
    expect(getNodeIcon("input", { mode: "text" })).toBe("text-cursor-input");
  });

  it("returns 'link' for input url mode", () => {
    expect(getNodeIcon("input", { mode: "url" })).toBe("link");
  });

  it("falls back to 'file-up' for unknown input mode", () => {
    expect(getNodeIcon("input", { mode: "unknown-mode" })).toBe("file-up");
  });

  it("falls back to static icon when input has no params", () => {
    expect(getNodeIcon("input")).toBe("file-up");
  });

  it("falls back to static icon when input params has no mode", () => {
    expect(getNodeIcon("input", {})).toBe("file-up");
  });

  // --- Output node icons (mode-aware) ---

  it("returns 'download' for output write mode", () => {
    expect(getNodeIcon("output", { mode: "write" })).toBe("download");
  });

  it("returns 'replace' for output overwrite mode", () => {
    expect(getNodeIcon("output", { mode: "overwrite" })).toBe("replace");
  });

  it("returns 'message-square' for output message mode", () => {
    expect(getNodeIcon("output", { mode: "message" })).toBe("message-square");
  });

  it("returns 'circle-off' for output none mode", () => {
    expect(getNodeIcon("output", { mode: "none" })).toBe("circle-off");
  });

  it("falls back to 'download' for unknown output mode", () => {
    expect(getNodeIcon("output", { mode: "unknown-mode" })).toBe("download");
  });

  it("falls back to static icon when output has no params", () => {
    expect(getNodeIcon("output")).toBe("download");
  });

  it("falls back to static icon when output params has no mode", () => {
    expect(getNodeIcon("output", {})).toBe("download");
  });

  // --- Processing node icons (static from NODE_TYPE_INFO) ---

  it("returns static icon for all processing node types", () => {
    const processingTypes = NODE_TYPE_NAMES.filter((t) => t !== "input" && t !== "output");
    for (const typeName of processingTypes) {
      const icon = getNodeIcon(typeName);
      expect(icon).toBe(NODE_TYPE_INFO[typeName].icon);
      expect(icon).toBeTruthy();
    }
  });

  it("returns correct icons for per-operation image nodes", () => {
    expect(getNodeIcon("image-compress")).toBe("image");
    expect(getNodeIcon("image-resize")).toBe("image");
    expect(getNodeIcon("image-convert")).toBe("image");
  });

  it("returns correct icons for per-operation spreadsheet nodes", () => {
    expect(getNodeIcon("spreadsheet-clean")).toBe("sheet");
    expect(getNodeIcon("spreadsheet-rename")).toBe("sheet");
  });

  it("returns correct icons for other processing nodes", () => {
    expect(getNodeIcon("transform")).toBe("arrow-left-right");
    expect(getNodeIcon("edit-fields")).toBe("pen-line");
    expect(getNodeIcon("loop")).toBe("repeat");
    expect(getNodeIcon("parallel")).toBe("git-fork");
    expect(getNodeIcon("group")).toBe("box");
    expect(getNodeIcon("http-request")).toBe("globe");
    expect(getNodeIcon("file-rename")).toBe("folder-open");
    expect(getNodeIcon("shell-command")).toBe("terminal");
  });

  // --- Unknown node types fall back gracefully ---

  it("returns 'box' for an unknown node type", () => {
    expect(getNodeIcon("nonexistent" as any)).toBe("box");
  });

  it("returns 'box' for a stale node type like 'image'", () => {
    expect(getNodeIcon("image" as any)).toBe("box");
  });

  // --- All 16 node types return a non-empty string ---

  it("returns a non-empty string for all 16 node types", () => {
    for (const typeName of NODE_TYPE_NAMES) {
      const icon = getNodeIcon(typeName);
      expect(icon).toBeTruthy();
      expect(typeof icon).toBe("string");
    }
  });
});
