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

  it("returns 'download' for output download mode", () => {
    expect(getNodeIcon("output", { mode: "download" })).toBe("download");
  });

  it("returns 'monitor' for output display mode", () => {
    expect(getNodeIcon("output", { mode: "display" })).toBe("monitor");
  });

  it("returns 'eye' for output preview mode", () => {
    expect(getNodeIcon("output", { mode: "preview" })).toBe("eye");
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

  it("returns correct updated icons for processing nodes", () => {
    expect(getNodeIcon("image")).toBe("image");
    expect(getNodeIcon("spreadsheet")).toBe("sheet");
    expect(getNodeIcon("transform")).toBe("arrow-left-right");
    expect(getNodeIcon("edit-fields")).toBe("pen-line");
    expect(getNodeIcon("loop")).toBe("repeat");
    expect(getNodeIcon("parallel")).toBe("git-fork");
    expect(getNodeIcon("group")).toBe("box");
    expect(getNodeIcon("http-request")).toBe("globe");
    expect(getNodeIcon("file-system")).toBe("folder-open");
    expect(getNodeIcon("shell-command")).toBe("terminal");
  });

  // --- All 12 node types return a non-empty string ---

  it("returns a non-empty string for all 12 node types", () => {
    for (const typeName of NODE_TYPE_NAMES) {
      const icon = getNodeIcon(typeName);
      expect(icon).toBeTruthy();
      expect(typeof icon).toBe("string");
    }
  });
});
