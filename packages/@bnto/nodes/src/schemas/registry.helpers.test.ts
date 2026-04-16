/**
 * Tests for schema registry helper functions:
 * getNodeSchema, getRequiredParams, getConditionallyRequired,
 * getVisibleParams, getParamFieldInfo.
 */

import { describe, expect, it } from "vitest";

import {
  NODE_SCHEMAS,
  getNodeSchema,
  getRequiredParams,
  getConditionallyRequired,
  getVisibleParams,
  getParamFieldInfo,
  GROUP_MODES,
} from "./index";

// ---------- getNodeSchema ----------

describe("getNodeSchema", () => {
  it("returns schema for valid per-operation type", () => {
    const schema = getNodeSchema("image-compress");
    expect(schema).toBeDefined();
    expect(schema!.nodeType).toBe("image-compress");
  });

  it("returns schema for image-resize", () => {
    const schema = getNodeSchema("image-resize");
    expect(schema).toBeDefined();
    expect(schema!.nodeType).toBe("image-resize");
  });

  it("returns schema for image-convert", () => {
    const schema = getNodeSchema("image-convert");
    expect(schema).toBeDefined();
    expect(schema!.nodeType).toBe("image-convert");
  });

  it("returns undefined for unknown type", () => {
    expect(getNodeSchema("unknown")).toBeUndefined();
  });

  it("returns undefined for types without schemas", () => {
    expect(getNodeSchema("http-request")).toBeUndefined();
    expect(getNodeSchema("shell-command")).toBeUndefined();
  });

  it("returns undefined for old multi-operation type names", () => {
    expect(getNodeSchema("image")).toBeUndefined();
    expect(getNodeSchema("file-system")).toBeUndefined();
    expect(getNodeSchema("spreadsheet")).toBeUndefined();
  });
});

// ---------- getRequiredParams ----------

describe("getRequiredParams", () => {
  it("returns empty array for unknown type", () => {
    expect(getRequiredParams("unknown")).toEqual([]);
  });

  it("returns empty for group (all optional/defaulted)", () => {
    const required = getRequiredParams("group");
    expect(required).toHaveLength(0);
  });
});

// ---------- getConditionallyRequired ----------

describe("getConditionallyRequired", () => {
  it("items is not conditionally required for forEach (engine iterates files directly)", () => {
    const params = getConditionallyRequired("loop", "mode", "forEach");
    expect(params).not.toContain("items");
  });

  it("finds count when loop mode is times", () => {
    const params = getConditionallyRequired("loop", "mode", "times");
    expect(params).toContain("count");
  });

  it("returns empty for non-matching value", () => {
    const params = getConditionallyRequired("loop", "mode", "nonexistent");
    expect(params).toHaveLength(0);
  });

  it("returns empty for unknown type", () => {
    expect(getConditionallyRequired("unknown", "mode", "forEach")).toEqual([]);
  });
});

// ---------- getVisibleParams ----------

describe("getVisibleParams", () => {
  it("returns all params for image-resize (no visibleWhen on per-operation types)", () => {
    const names = getVisibleParams("image-resize", "unused", "unused");
    expect(names).toContain("width");
    expect(names).toContain("height");
    expect(names).toContain("maintainAspect");
    expect(names).toContain("quality");
  });

  it("returns quality for image-compress", () => {
    const names = getVisibleParams("image-compress", "unused", "unused");
    expect(names).toContain("quality");
  });

  it("returns format and quality for image-convert", () => {
    const names = getVisibleParams("image-convert", "unused", "unused");
    expect(names).toContain("format");
    expect(names).toContain("quality");
  });

  it("evaluates visibleWhen on input node (file-upload mode)", () => {
    const names = getVisibleParams("input", "mode", "file-upload");
    expect(names).toContain("accept");
    expect(names).toContain("extensions");
    expect(names).toContain("label");
    expect(names).toContain("multiple");
    expect(names).not.toContain("placeholder");
  });

  it("evaluates visibleWhen on input node (text mode)", () => {
    const names = getVisibleParams("input", "mode", "text");
    expect(names).toContain("placeholder");
    expect(names).not.toContain("accept");
    expect(names).not.toContain("extensions");
  });

  it("evaluates visibleWhen on output node (download mode)", () => {
    const names = getVisibleParams("output", "mode", "download");
    expect(names).toContain("filename");
    expect(names).toContain("zip");
    expect(names).toContain("autoDownload");
  });

  it("excludes download-only params for output display mode", () => {
    const names = getVisibleParams("output", "mode", "display");
    expect(names).not.toContain("filename");
    expect(names).not.toContain("zip");
    expect(names).not.toContain("autoDownload");
  });

  it("returns empty for unknown type", () => {
    expect(getVisibleParams("unknown", "op", "val")).toEqual([]);
  });

  // --- parameters-map overload (used by editor config panel) ---

  it("parameters-map: returns all params for image-resize", () => {
    const names = getVisibleParams("image-resize", { quality: 80 });
    expect(names).toContain("width");
    expect(names).toContain("height");
    expect(names).toContain("maintainAspect");
    expect(names).toContain("quality");
  });

  it("parameters-map: evaluates input visibleWhen against current values", () => {
    const fileUpload = getVisibleParams("input", { mode: "file-upload" });
    expect(fileUpload).toContain("accept");
    expect(fileUpload).toContain("extensions");
    expect(fileUpload).not.toContain("placeholder");

    const text = getVisibleParams("input", { mode: "text" });
    expect(text).toContain("placeholder");
    expect(text).not.toContain("accept");
  });

  it("parameters-map: returns empty for unknown type", () => {
    expect(getVisibleParams("unknown", { op: "val" })).toEqual([]);
  });
});

// ---------- getParamFieldInfo ----------

describe("getParamFieldInfo", () => {
  it("detects enum type for image-convert format", () => {
    const info = getParamFieldInfo("image-convert", "format");
    expect(info).toBeDefined();
    expect(info!.type).toBe("enum");
    expect(info!.enumValues).toEqual(["jpeg", "png", "webp"]);
  });

  it("detects number type with min/max for image-compress quality", () => {
    const info = getParamFieldInfo("image-compress", "quality");
    expect(info).toBeDefined();
    expect(info!.type).toBe("number");
    expect(info!.min).toBe(1);
    expect(info!.max).toBe(100);
  });

  it("detects boolean type for image-resize maintainAspect", () => {
    const info = getParamFieldInfo("image-resize", "maintainAspect");
    expect(info).toBeDefined();
    expect(info!.type).toBe("boolean");
  });

  it("detects string type for file-rename find", () => {
    const info = getParamFieldInfo("file-rename", "find");
    expect(info).toBeDefined();
    expect(info!.type).toBe("string");
  });

  it("detects enum for group mode", () => {
    const info = getParamFieldInfo("group", "mode");
    expect(info).toBeDefined();
    expect(info!.type).toBe("enum");
    expect(info!.enumValues).toEqual([...GROUP_MODES]);
  });

  it("returns undefined for unknown type", () => {
    expect(getParamFieldInfo("unknown", "field")).toBeUndefined();
  });

  it("returns undefined for unknown param", () => {
    expect(getParamFieldInfo("image-compress", "nonexistent")).toBeUndefined();
  });
});
