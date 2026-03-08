/**
 * Cross-boundary validation tests — Engine catalog vs @bnto/nodes definitions.
 *
 * These tests validate that the generated TypeScript module
 * (`src/generated/catalog.ts`) stays in sync with the engine's
 * `catalog.snapshot.json` and that the Zod schemas use engine-sourced
 * defaults and constraints.
 */

import { describe, expect, it } from "vitest";
import { NODE_TYPE_INFO } from "./generated/catalog";
import { imageParamsSchema, IMAGE_FORMATS } from "./schemas/image";
import { spreadsheetParamsSchema } from "./schemas/spreadsheet";
import { fileSystemParamsSchema } from "./schemas/fileSystem";
import { CURRENT_FORMAT_VERSION } from "./formatVersion";
import { PROCESSORS, PROCESSOR_MAP, getProcessorDefaults } from "./generated/catalog";

// Import raw JSON to validate generated module matches it exactly
// eslint-disable-next-line @typescript-eslint/no-require-imports
import catalog from "../../../engine/catalog.snapshot.json";

/** Helper: find a processor in raw JSON */
function findRaw(nodeType: string, operation: string) {
  return (catalog as { processors: Array<Record<string, unknown>> }).processors.find(
    (p) => p.nodeType === nodeType && p.operation === operation,
  );
}

// =============================================================================
// Generated module matches raw JSON
// =============================================================================

describe("generated catalog matches raw JSON", () => {
  it("has the same number of processors", () => {
    expect(PROCESSORS.length).toBe((catalog as { processors: unknown[] }).processors.length);
  });

  it("every raw processor exists in PROCESSOR_MAP", () => {
    for (const raw of (catalog as { processors: Array<{ nodeType: string; operation: string }> })
      .processors) {
      const key = `${raw.nodeType}:${raw.operation}`;
      expect(PROCESSOR_MAP.has(key)).toBe(true);
    }
  });

  it("version matches CURRENT_FORMAT_VERSION", () => {
    expect((catalog as { version: string }).version).toBe(CURRENT_FORMAT_VERSION);
  });
});

// =============================================================================
// Structural Tests
// =============================================================================

describe("catalog structure", () => {
  it("has exactly 6 processors", () => {
    expect(PROCESSORS).toHaveLength(6);
  });

  it("every catalog nodeType exists in NODE_TYPE_INFO", () => {
    const tsNodeTypes = new Set(Object.keys(NODE_TYPE_INFO));
    for (const proc of PROCESSORS) {
      expect(tsNodeTypes).toContain(proc.nodeType);
    }
  });

  it("all expected compound keys are present", () => {
    expect(PROCESSOR_MAP.has("image:compress")).toBe(true);
    expect(PROCESSOR_MAP.has("image:resize")).toBe(true);
    expect(PROCESSOR_MAP.has("image:convert")).toBe(true);
    expect(PROCESSOR_MAP.has("spreadsheet:clean")).toBe(true);
    expect(PROCESSOR_MAP.has("spreadsheet:rename")).toBe(true);
    expect(PROCESSOR_MAP.has("file-system:rename")).toBe(true);
  });

  it("every processor includes browser in platforms", () => {
    for (const proc of PROCESSORS) {
      expect(proc.platforms).toContain("browser");
    }
  });
});

// =============================================================================
// Engine defaults flow through to Zod schemas
// =============================================================================

describe("engine defaults flow through to schemas", () => {
  it("image quality default matches engine", () => {
    const engineDefault = getProcessorDefaults("image", "compress").quality;
    const zodDefault = imageParamsSchema.shape.quality.parse(undefined);
    expect(zodDefault).toBe(engineDefault);
  });

  it("image maintainAspect default matches engine", () => {
    const engineDefault = getProcessorDefaults("image", "resize").maintainAspect;
    const zodDefault = imageParamsSchema.shape.maintainAspect.parse(undefined);
    expect(zodDefault).toBe(engineDefault);
  });

  it("spreadsheet clean defaults match engine", () => {
    const engineDefaults = getProcessorDefaults("spreadsheet", "clean");
    const zodTrimDefault = spreadsheetParamsSchema.shape.trimWhitespace.parse(undefined);
    const zodEmptyDefault = spreadsheetParamsSchema.shape.removeEmptyRows.parse(undefined);
    const zodDedupDefault = spreadsheetParamsSchema.shape.removeDuplicates.parse(undefined);
    expect(zodTrimDefault).toBe(engineDefaults.trimWhitespace);
    expect(zodEmptyDefault).toBe(engineDefaults.removeEmptyRows);
    expect(zodDedupDefault).toBe(engineDefaults.removeDuplicates);
  });

  it("file-system case options match engine", () => {
    const proc = PROCESSOR_MAP.get("file-system:rename")!;
    const caseParam = proc.parameters.find((p) => p.name === "case");
    // The Zod schema should accept all engine-defined case options
    for (const opt of caseParam?.options ?? []) {
      expect(() => fileSystemParamsSchema.shape.case.parse(opt)).not.toThrow();
    }
  });

  it("image format options match IMAGE_FORMATS", () => {
    const proc = PROCESSOR_MAP.get("image:convert")!;
    const formatParam = proc.parameters.find((p) => p.name === "format");
    const engineFormats = [...(formatParam?.options ?? [])].sort();
    const tsFormats = [...IMAGE_FORMATS].sort();
    expect(engineFormats).toEqual(tsFormats);
  });
});

// =============================================================================
// Per-processor parameter validation
// =============================================================================

describe("image:compress", () => {
  const proc = PROCESSOR_MAP.get("image:compress")!;

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

describe("image:resize", () => {
  const proc = PROCESSOR_MAP.get("image:resize")!;

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

describe("image:convert", () => {
  const proc = PROCESSOR_MAP.get("image:convert")!;

  it("has format: enum with jpeg/png/webp, required", () => {
    const format = proc.parameters.find((p) => p.name === "format")!;
    expect(format.type).toBe("enum");
    expect(format.options).toEqual(expect.arrayContaining(["jpeg", "png", "webp"]));
    expect(format.constraints?.required).toBe(true);
  });

  it("has quality: number, default 80", () => {
    const quality = proc.parameters.find((p) => p.name === "quality")!;
    expect(quality.type).toBe("number");
    expect(quality.default).toBe(80);
  });
});

describe("spreadsheet:clean", () => {
  const proc = PROCESSOR_MAP.get("spreadsheet:clean")!;

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

describe("spreadsheet:rename", () => {
  const proc = PROCESSOR_MAP.get("spreadsheet:rename")!;

  it("has columns param: object", () => {
    const columns = proc.parameters.find((p) => p.name === "columns")!;
    expect(columns.type).toBe("object");
  });
});

describe("file-system:rename", () => {
  const proc = PROCESSOR_MAP.get("file-system:rename")!;

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
