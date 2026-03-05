/**
 * Tests for input and output node schemas — visibility rules,
 * parameter structure, and conditional behavior.
 */

import { describe, expect, it } from "vitest";

import { getVisibleParams, getNodeSchema, getRequiredParams } from "./index";
import { inputNodeSchema, INPUT_MODES } from "./input";
import { outputNodeSchema, OUTPUT_MODES } from "./output";

// ---------- Input schema structure ----------

describe("inputNodeSchema", () => {
  it("has nodeType 'input'", () => {
    expect(inputNodeSchema.nodeType).toBe("input");
  });

  it("mode defaults to file-upload via Zod", () => {
    const result = inputNodeSchema.schema.safeParse({});
    expect(result.success).toBe(true);
    if (result.success) {
      expect(result.data.mode).toBe("file-upload");
    }
  });

  it("has file-upload-specific params with visibleWhen", () => {
    const fileUploadParams = [
      "accept",
      "extensions",
      "label",
      "multiple",
      "maxFileSize",
      "maxFiles",
    ];
    for (const name of fileUploadParams) {
      expect(inputNodeSchema.params[name]).toBeDefined();
      expect(inputNodeSchema.params[name].visibleWhen).toBeDefined();
    }
  });

  it("placeholder is visible for text and url modes (OR condition)", () => {
    const meta = inputNodeSchema.params.placeholder;
    expect(meta).toBeDefined();
    expect(Array.isArray(meta.visibleWhen)).toBe(true);
  });
});

// ---------- Output schema structure ----------

describe("outputNodeSchema", () => {
  it("has nodeType 'output'", () => {
    expect(outputNodeSchema.nodeType).toBe("output");
  });

  it("mode defaults to download via Zod", () => {
    const result = outputNodeSchema.schema.safeParse({});
    expect(result.success).toBe(true);
    if (result.success) {
      expect(result.data.mode).toBe("download");
    }
  });

  it("has download-specific params with visibleWhen", () => {
    const downloadParams = ["filename", "zip", "autoDownload"];
    for (const name of downloadParams) {
      expect(outputNodeSchema.params[name]).toBeDefined();
      expect(outputNodeSchema.params[name].visibleWhen).toBeDefined();
    }
  });

  it("label is always visible (no visibleWhen)", () => {
    expect(outputNodeSchema.params.label).toBeDefined();
    expect(outputNodeSchema.params.label.visibleWhen).toBeUndefined();
  });
});

// ---------- Visibility rules via registry helpers ----------

describe("input visibility rules", () => {
  it("shows file-upload params in file-upload mode", () => {
    const names = getVisibleParams("input", "mode", "file-upload");
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
    const names = getVisibleParams("input", "mode", "text");
    expect(names).toContain("mode");
    expect(names).toContain("placeholder");
    expect(names).not.toContain("accept");
    expect(names).not.toContain("multiple");
  });

  it("shows placeholder in url mode", () => {
    const names = getVisibleParams("input", "mode", "url");
    expect(names).toContain("mode");
    expect(names).toContain("placeholder");
    expect(names).not.toContain("accept");
  });
});

describe("output visibility rules", () => {
  it("shows download params in download mode", () => {
    const names = getVisibleParams("output", "mode", "download");
    expect(names).toContain("mode");
    expect(names).toContain("filename");
    expect(names).toContain("zip");
    expect(names).toContain("autoDownload");
    expect(names).toContain("label"); // always visible
  });

  it("hides download-specific params in display mode", () => {
    const names = getVisibleParams("output", "mode", "display");
    expect(names).toContain("mode");
    expect(names).toContain("label"); // always visible
    expect(names).not.toContain("filename");
    expect(names).not.toContain("zip");
    expect(names).not.toContain("autoDownload");
  });

  it("hides download-specific params in preview mode", () => {
    const names = getVisibleParams("output", "mode", "preview");
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
    expect(Object.keys(schema!.params).length).toBeGreaterThan(0);
  });

  it("getNodeSchema returns output schema", () => {
    const schema = getNodeSchema("output");
    expect(schema).toBeDefined();
    expect(schema!.nodeType).toBe("output");
    expect(Object.keys(schema!.params).length).toBeGreaterThan(0);
  });

  it("input has no strictly required params (mode has default)", () => {
    const required = getRequiredParams("input");
    expect(required).toHaveLength(0);
  });

  it("output has no strictly required params (mode has default)", () => {
    const required = getRequiredParams("output");
    expect(required).toHaveLength(0);
  });
});
