/**
 * Tests for input and output node schemas — visibility rules,
 * parameter structure, and conditional behavior.
 */

import { describe, expect, it } from "vitest";

import { getVisibleParams, getNodeSchema, getRequiredParams } from "./index";
import { inputSchema, INPUT_MODES } from "./input";
import { outputSchema, OUTPUT_MODES } from "./output";

// ---------- Input schema structure ----------

describe("inputSchema", () => {
  it("has nodeType 'input'", () => {
    expect(inputSchema.nodeType).toBe("input");
  });

  it("has 'mode' as a required enum parameter", () => {
    const mode = inputSchema.parameters.find((p) => p.name === "mode");
    expect(mode).toBeDefined();
    expect(mode!.type).toBe("enum");
    expect(mode!.required).toBe(true);
    expect(mode!.enumValues).toEqual(INPUT_MODES);
    expect(mode!.default).toBe("file-upload");
  });

  it("has file-upload-specific params with visibleWhen", () => {
    const fileUploadParams = ["accept", "extensions", "label", "multiple", "maxFileSize", "maxFiles"];
    for (const name of fileUploadParams) {
      const param = inputSchema.parameters.find((p) => p.name === name);
      expect(param).toBeDefined();
      expect(param!.visibleWhen).toBeDefined();
    }
  });

  it("placeholder is visible for text and url modes", () => {
    const placeholder = inputSchema.parameters.find((p) => p.name === "placeholder");
    expect(placeholder).toBeDefined();
    expect(Array.isArray(placeholder!.visibleWhen)).toBe(true);
  });
});

// ---------- Output schema structure ----------

describe("outputSchema", () => {
  it("has nodeType 'output'", () => {
    expect(outputSchema.nodeType).toBe("output");
  });

  it("has 'mode' as a required enum parameter", () => {
    const mode = outputSchema.parameters.find((p) => p.name === "mode");
    expect(mode).toBeDefined();
    expect(mode!.type).toBe("enum");
    expect(mode!.required).toBe(true);
    expect(mode!.enumValues).toEqual(OUTPUT_MODES);
    expect(mode!.default).toBe("download");
  });

  it("has download-specific params with visibleWhen", () => {
    const downloadParams = ["filename", "zip", "autoDownload"];
    for (const name of downloadParams) {
      const param = outputSchema.parameters.find((p) => p.name === name);
      expect(param).toBeDefined();
      expect(param!.visibleWhen).toBeDefined();
    }
  });

  it("label is always visible (no visibleWhen)", () => {
    const label = outputSchema.parameters.find((p) => p.name === "label");
    expect(label).toBeDefined();
    expect(label!.visibleWhen).toBeUndefined();
  });
});

// ---------- Visibility rules via registry helpers ----------

describe("input visibility rules", () => {
  it("shows file-upload params in file-upload mode", () => {
    const params = getVisibleParams("input", "mode", "file-upload");
    const names = params.map((p) => p.name);
    expect(names).toContain("mode");
    expect(names).toContain("accept");
    expect(names).toContain("extensions");
    expect(names).toContain("label");
    expect(names).toContain("multiple");
    expect(names).toContain("maxFileSize");
    expect(names).toContain("maxFiles");
    expect(names).not.toContain("placeholder");
  });

  it("shows placeholder in text mode", () => {
    const params = getVisibleParams("input", "mode", "text");
    const names = params.map((p) => p.name);
    expect(names).toContain("mode");
    expect(names).toContain("placeholder");
    expect(names).not.toContain("accept");
    expect(names).not.toContain("multiple");
  });

  it("shows placeholder in url mode", () => {
    const params = getVisibleParams("input", "mode", "url");
    const names = params.map((p) => p.name);
    expect(names).toContain("mode");
    expect(names).toContain("placeholder");
    expect(names).not.toContain("accept");
  });
});

describe("output visibility rules", () => {
  it("shows download params in download mode", () => {
    const params = getVisibleParams("output", "mode", "download");
    const names = params.map((p) => p.name);
    expect(names).toContain("mode");
    expect(names).toContain("filename");
    expect(names).toContain("zip");
    expect(names).toContain("autoDownload");
    expect(names).toContain("label"); // always visible
  });

  it("hides download-specific params in display mode", () => {
    const params = getVisibleParams("output", "mode", "display");
    const names = params.map((p) => p.name);
    expect(names).toContain("mode");
    expect(names).toContain("label"); // always visible
    expect(names).not.toContain("filename");
    expect(names).not.toContain("zip");
    expect(names).not.toContain("autoDownload");
  });

  it("hides download-specific params in preview mode", () => {
    const params = getVisibleParams("output", "mode", "preview");
    const names = params.map((p) => p.name);
    expect(names).toContain("mode");
    expect(names).toContain("label");
    expect(names).not.toContain("zip");
  });
});

// ---------- Registry integration ----------

describe("I/O schemas in registry", () => {
  it("getNodeSchema returns input schema", () => {
    const schema = getNodeSchema("input");
    expect(schema).toBeDefined();
    expect(schema!.nodeType).toBe("input");
    expect(schema!.parameters.length).toBeGreaterThan(0);
  });

  it("getNodeSchema returns output schema", () => {
    const schema = getNodeSchema("output");
    expect(schema).toBeDefined();
    expect(schema!.nodeType).toBe("output");
    expect(schema!.parameters.length).toBeGreaterThan(0);
  });

  it("input has exactly 1 required param (mode)", () => {
    const required = getRequiredParams("input");
    expect(required).toHaveLength(1);
    expect(required[0].name).toBe("mode");
  });

  it("output has exactly 1 required param (mode)", () => {
    const required = getRequiredParams("output");
    expect(required).toHaveLength(1);
    expect(required[0].name).toBe("mode");
  });
});
