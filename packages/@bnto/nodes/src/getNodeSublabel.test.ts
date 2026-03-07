/**
 * getNodeSublabel tests — verify human-readable sublabel resolution.
 */

import { describe, it, expect } from "vitest";
import { getNodeSublabel } from "./getNodeSublabel";

describe("getNodeSublabel", () => {
  // --- Input modes ---
  it("returns 'File Upload' for input file-upload mode", () => {
    expect(getNodeSublabel("input", { mode: "file-upload" })).toBe("File Upload");
  });

  it("returns 'Text Input' for input text mode", () => {
    expect(getNodeSublabel("input", { mode: "text" })).toBe("Text Input");
  });

  it("returns 'URL' for input url mode", () => {
    expect(getNodeSublabel("input", { mode: "url" })).toBe("URL");
  });

  it("returns 'Input' when input has no mode", () => {
    expect(getNodeSublabel("input")).toBe("Input");
    expect(getNodeSublabel("input", {})).toBe("Input");
  });

  // --- Output modes ---
  it("returns 'Download' for output download mode", () => {
    expect(getNodeSublabel("output", { mode: "download" })).toBe("Download");
  });

  it("returns 'Display' for output display mode", () => {
    expect(getNodeSublabel("output", { mode: "display" })).toBe("Display");
  });

  it("returns 'Preview' for output preview mode", () => {
    expect(getNodeSublabel("output", { mode: "preview" })).toBe("Preview");
  });

  it("returns 'Output' when output has no mode", () => {
    expect(getNodeSublabel("output")).toBe("Output");
    expect(getNodeSublabel("output", {})).toBe("Output");
  });

  // --- Processing nodes use category label ---
  it("returns category label for processing nodes", () => {
    expect(getNodeSublabel("image")).toBe("Image");
    expect(getNodeSublabel("spreadsheet")).toBe("Spreadsheet");
    expect(getNodeSublabel("transform")).toBe("Data");
    expect(getNodeSublabel("shell-command")).toBe("System");
    expect(getNodeSublabel("http-request")).toBe("Network");
    expect(getNodeSublabel("file-system")).toBe("File");
  });

  // --- Control flow nodes use their own label, not "Control Flow" ---
  it("returns node label for control flow nodes", () => {
    expect(getNodeSublabel("loop")).toBe("Loop");
    expect(getNodeSublabel("group")).toBe("Group");
    expect(getNodeSublabel("parallel")).toBe("Parallel");
  });

  // --- Edit Fields uses category label "Data", not "Edit Fields" ---
  it("returns category label for non-control processing nodes", () => {
    expect(getNodeSublabel("edit-fields")).toBe("Data");
  });
});
