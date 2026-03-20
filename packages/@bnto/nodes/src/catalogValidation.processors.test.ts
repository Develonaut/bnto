/**
 * Per-processor parameter validation tests — Engine catalog processors.
 *
 * Validates that each processor in the catalog has the expected parameters,
 * types, defaults, and constraints.
 */

import { describe, expect, it } from "vitest";
import { PROCESSOR_MAP } from "./generated/catalog";

// =============================================================================
// image-compress
// =============================================================================

describe("image-compress", () => {
  const proc = PROCESSOR_MAP.get("image-compress")!;

  it("accepts image/jpeg, image/png, image/webp", () => {
    expect(proc.accepts).toEqual(expect.arrayContaining(["image/jpeg", "image/png", "image/webp"]));
  });

  it("has quality param: number, default 80, min 1, max 100", () => {
    const quality = proc.parameters.find((p) => p.name === "quality")!;
    expect(quality.type).toBe("number");
    expect(quality.default).toBe(80);
    expect(quality.constraints?.min).toBe(1);
    expect(quality.constraints?.max).toBe(100);
  });
});

// =============================================================================
// image-resize
// =============================================================================

describe("image-resize", () => {
  const proc = PROCESSOR_MAP.get("image-resize")!;

  it("has width: number, min 1", () => {
    const width = proc.parameters.find((p) => p.name === "width")!;
    expect(width.type).toBe("number");
    expect(width.constraints?.min).toBe(1);
  });

  it("has height: number, min 1", () => {
    const height = proc.parameters.find((p) => p.name === "height")!;
    expect(height.type).toBe("number");
    expect(height.constraints?.min).toBe(1);
  });

  it("has maintainAspect: boolean, default true", () => {
    const ma = proc.parameters.find((p) => p.name === "maintainAspect")!;
    expect(ma.type).toBe("boolean");
    expect(ma.default).toBe(true);
  });
});

// =============================================================================
// image-convert
// =============================================================================

describe("image-convert", () => {
  const proc = PROCESSOR_MAP.get("image-convert")!;

  it("has format: enum with jpeg/png/webp, required, default jpeg", () => {
    const format = proc.parameters.find((p) => p.name === "format")!;
    expect(format.type).toBe("enum");
    expect(format.options).toEqual(expect.arrayContaining(["jpeg", "png", "webp"]));
    expect(format.constraints?.required).toBe(true);
    expect(format.default).toBe("jpeg");
  });

  it("has quality: number, default 80", () => {
    const quality = proc.parameters.find((p) => p.name === "quality")!;
    expect(quality.type).toBe("number");
    expect(quality.default).toBe(80);
  });
});

// =============================================================================
// spreadsheet-clean
// =============================================================================

describe("spreadsheet-clean", () => {
  const proc = PROCESSOR_MAP.get("spreadsheet-clean")!;

  it("accepts text/csv", () => {
    expect(proc.accepts).toContain("text/csv");
  });

  it("has all three clean params with correct defaults", () => {
    const params = proc.parameters;
    const tw = params.find((p) => p.name === "trimWhitespace")!;
    const rer = params.find((p) => p.name === "removeEmptyRows")!;
    const rd = params.find((p) => p.name === "removeDuplicates")!;

    expect(tw.type).toBe("boolean");
    expect(tw.default).toBe(true);
    expect(rer.type).toBe("boolean");
    expect(rer.default).toBe(true);
    expect(rd.type).toBe("boolean");
    expect(rd.default).toBe(true);
  });
});

// =============================================================================
// spreadsheet-rename
// =============================================================================

describe("spreadsheet-rename", () => {
  const proc = PROCESSOR_MAP.get("spreadsheet-rename")!;

  it("has columns param: object", () => {
    const columns = proc.parameters.find((p) => p.name === "columns")!;
    expect(columns.type).toBe("object");
  });
});

// =============================================================================
// file-rename
// =============================================================================

describe("file-rename", () => {
  const proc = PROCESSOR_MAP.get("file-rename")!;

  it("accepts any file type (empty accepts)", () => {
    expect(proc.accepts).toEqual([]);
  });

  it("has all rename params", () => {
    const names = proc.parameters.map((p) => p.name);
    expect(names).toEqual(["find", "replace", "case", "prefix", "suffix", "pattern"]);
  });

  it("case param is enum with lower/upper/title", () => {
    const caseP = proc.parameters.find((p) => p.name === "case")!;
    expect(caseP.type).toBe("enum");
    expect(caseP.options).toEqual(["lower", "upper", "title"]);
  });
});
