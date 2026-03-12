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
import { getEngineOperations } from "./schemas/deriveOperations";
import { CATEGORIES } from "./categories";
import { IMAGE_OPERATIONS } from "./schemas/image";
import { SPREADSHEET_OPERATIONS } from "./schemas/spreadsheet";
import { FILE_OPERATIONS } from "./schemas/fileSystem";

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
  it("image compression default matches engine", () => {
    const engineDefault = getProcessorDefaults("image", "compress").compression;
    const zodDefault = imageParamsSchema.shape.compression.parse(undefined);
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
// Operation lists match engine PROCESSORS exactly
// =============================================================================

describe("operation lists match engine", () => {
  it("IMAGE_OPERATIONS matches engine image processors", () => {
    const engineOps = getEngineOperations("image").sort();
    const schemaOps = [...IMAGE_OPERATIONS].sort();
    expect(schemaOps).toEqual(engineOps);
  });

  it("SPREADSHEET_OPERATIONS matches engine spreadsheet processors", () => {
    const engineOps = getEngineOperations("spreadsheet").sort();
    const schemaOps = [...SPREADSHEET_OPERATIONS].sort();
    expect(schemaOps).toEqual(engineOps);
  });

  it("FILE_OPERATIONS matches engine file-system processors", () => {
    const engineOps = getEngineOperations("file-system").sort();
    const schemaOps = [...FILE_OPERATIONS].sort();
    expect(schemaOps).toEqual(engineOps);
  });

  it("IMAGE_FORMATS matches engine image:convert format options", () => {
    const proc = PROCESSOR_MAP.get("image:convert")!;
    const formatParam = proc.parameters.find((p) => p.name === "format");
    const engineFormats = [...(formatParam?.options ?? [])].sort();
    const tsFormats = [...IMAGE_FORMATS].sort();
    expect(tsFormats).toEqual(engineFormats);
  });
});

// =============================================================================
// Categories sync — every category exists in NODE_TYPE_INFO
// =============================================================================

describe("categories sync", () => {
  it("every category in CATEGORIES exists in at least one NODE_TYPE_INFO entry", () => {
    const usedCategories = new Set(Object.values(NODE_TYPE_INFO).map((info) => info.category));
    for (const cat of CATEGORIES) {
      expect(usedCategories).toContain(cat.name);
    }
  });
});
