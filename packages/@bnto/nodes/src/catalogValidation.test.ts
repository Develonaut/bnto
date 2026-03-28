/**
 * Cross-boundary validation tests — Engine catalog vs @bnto/nodes definitions.
 *
 * These tests validate that the generated TypeScript module
 * (`src/generated/catalog.ts`) stays in sync with the engine's
 * `catalog.snapshot.json` and that the Zod schemas use engine-sourced
 * defaults and constraints.
 */

import { describe, expect, it } from "vitest";
import { NODE_TYPE_INFO, NODE_TYPE_NAMES, ITERATION_MODES } from "./generated/catalog";
import { imageCompressParamsSchema } from "./schemas/imageCompress";
import { imageResizeParamsSchema } from "./schemas/imageResize";
import { IMAGE_FORMATS, imageConvertFields } from "./schemas/imageConvert";
import { spreadsheetCleanParamsSchema } from "./schemas/spreadsheetClean";
import { fileRenameParamsSchema } from "./schemas/fileRename";
import { CURRENT_FORMAT_VERSION } from "./formatVersion";
import { CATALOG_FORMAT_VERSION } from "./generated/formatVersion";
import { PROCESSORS, PROCESSOR_MAP, getProcessorDefaults } from "./generated/catalog";
import { CATEGORIES } from "./categories";
import { NODE_SCHEMAS, NODE_PARAM_FIELDS } from "./schemas/registry";
import { DEFINITION_JSON_SCHEMA } from "./generated/definitionSchema";
import { inputFields } from "./schemas/input";

// Import raw JSON to validate generated module matches it exactly
// eslint-disable-next-line @typescript-eslint/no-require-imports
import catalog from "../../../engine/catalog.snapshot.json";

// =============================================================================
// Generated module matches raw JSON
// =============================================================================

describe("generated catalog matches raw JSON", () => {
  it("has the same number of processors", () => {
    expect(PROCESSORS.length).toBe((catalog as { processors: unknown[] }).processors.length);
  });

  it("every raw processor exists in PROCESSOR_MAP", () => {
    for (const raw of (catalog as { processors: Array<{ nodeType: string }> }).processors) {
      expect(PROCESSOR_MAP.has(raw.nodeType)).toBe(true);
    }
  });

  it("version matches CURRENT_FORMAT_VERSION", () => {
    expect((catalog as { version: string }).version).toBe(CURRENT_FORMAT_VERSION);
  });

  it("CATALOG_FORMAT_VERSION matches catalog version", () => {
    expect(CATALOG_FORMAT_VERSION).toBe((catalog as { version: string }).version);
  });

  it("CURRENT_FORMAT_VERSION derives from CATALOG_FORMAT_VERSION", () => {
    expect(CURRENT_FORMAT_VERSION).toBe(CATALOG_FORMAT_VERSION);
  });
});

// =============================================================================
// Structural Tests
// =============================================================================

describe("catalog structure", () => {
  it("has all registered processors", () => {
    expect(PROCESSORS).toHaveLength((catalog as { processors: unknown[] }).processors.length);
  });

  it("every catalog nodeType exists in NODE_TYPE_INFO", () => {
    const tsNodeTypes = new Set(Object.keys(NODE_TYPE_INFO));
    for (const proc of PROCESSORS) {
      expect(tsNodeTypes).toContain(proc.nodeType);
    }
  });

  it("all expected per-operation node types are present", () => {
    expect(PROCESSOR_MAP.has("image-compress")).toBe(true);
    expect(PROCESSOR_MAP.has("image-resize")).toBe(true);
    expect(PROCESSOR_MAP.has("image-convert")).toBe(true);
    expect(PROCESSOR_MAP.has("image-strip-exif")).toBe(true);
    expect(PROCESSOR_MAP.has("spreadsheet-clean")).toBe(true);
    expect(PROCESSOR_MAP.has("spreadsheet-rename")).toBe(true);
    expect(PROCESSOR_MAP.has("spreadsheet-convert")).toBe(true);
    expect(PROCESSOR_MAP.has("file-rename")).toBe(true);
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
  it("image-compress quality default matches engine", () => {
    const engineDefault = getProcessorDefaults("image-compress").quality;
    const zodDefault = imageCompressParamsSchema.shape.quality.parse(undefined);
    expect(zodDefault).toBe(engineDefault);
  });

  it("image-resize maintainAspect default matches engine", () => {
    const engineDefault = getProcessorDefaults("image-resize").maintainAspect;
    const zodDefault = imageResizeParamsSchema.shape.maintainAspect.parse(undefined);
    expect(zodDefault).toBe(engineDefault);
  });

  it("spreadsheet-clean defaults match engine", () => {
    const engineDefaults = getProcessorDefaults("spreadsheet-clean");
    const zodTrimDefault = spreadsheetCleanParamsSchema.shape.trimWhitespace.parse(undefined);
    const zodEmptyDefault = spreadsheetCleanParamsSchema.shape.removeEmptyRows.parse(undefined);
    const zodDedupDefault = spreadsheetCleanParamsSchema.shape.removeDuplicates.parse(undefined);
    expect(zodTrimDefault).toBe(engineDefaults.trimWhitespace);
    expect(zodEmptyDefault).toBe(engineDefaults.removeEmptyRows);
    expect(zodDedupDefault).toBe(engineDefaults.removeDuplicates);
  });

  it("file-rename case options match engine", () => {
    const proc = PROCESSOR_MAP.get("file-rename")!;
    const caseParam = proc.parameters.find((p) => p.name === "case");
    // The Zod schema should accept all engine-defined case options
    for (const opt of caseParam?.options ?? []) {
      expect(() => fileRenameParamsSchema.shape.case.parse(opt)).not.toThrow();
    }
  });

  it("image-convert format options match IMAGE_FORMATS", () => {
    const proc = PROCESSOR_MAP.get("image-convert")!;
    const formatParam = proc.parameters.find((p) => p.name === "format");
    const engineFormats = [...(formatParam?.options ?? [])].sort();
    const tsFormats = [...IMAGE_FORMATS].sort();
    expect(engineFormats).toEqual(tsFormats);
  });
});

// =============================================================================
// Schema registry completeness — every node type has a schema entry
// =============================================================================

describe("schema registry completeness", () => {
  // http-request and shell-command have no processors yet
  const TYPES_WITHOUT_SCHEMAS = new Set(["http-request", "shell-command"]);

  it("every node type in NODE_TYPE_NAMES has a NODE_SCHEMAS entry (except unimplemented types)", () => {
    for (const name of NODE_TYPE_NAMES) {
      if (TYPES_WITHOUT_SCHEMAS.has(name)) continue;
      expect(NODE_SCHEMAS[name], `Missing NODE_SCHEMAS entry for "${name}"`).toBeDefined();
    }
  });

  it("every NODE_SCHEMAS entry has a matching NODE_PARAM_FIELDS entry", () => {
    for (const name of Object.keys(NODE_SCHEMAS)) {
      expect(
        NODE_PARAM_FIELDS[name],
        `Missing NODE_PARAM_FIELDS entry for "${name}"`,
      ).toBeDefined();
    }
  });
});

// =============================================================================
// Image convert options — derived from IMAGE_FORMATS
// =============================================================================

describe("image convert field options sync", () => {
  it("image-convert field options match IMAGE_FORMATS", () => {
    const fieldOptions = imageConvertFields.format!.options!.map((o: { value: string }) => o.value);
    expect(fieldOptions.sort()).toEqual([...IMAGE_FORMATS].sort());
  });
});

// =============================================================================
// Iteration modes — match definition JSON Schema
// =============================================================================

describe("iteration modes sync", () => {
  it("ITERATION_MODES matches definition schema iteration enum", () => {
    const schema = DEFINITION_JSON_SCHEMA as Record<string, unknown>;
    const defs = schema.$defs as Record<string, unknown>;
    const ps = defs.PipelineSettings as { properties: { iteration: { enum: string[] } } };
    const iterEnum = ps.properties.iteration.enum;
    expect([...ITERATION_MODES]).toEqual(iterEnum);
  });
});

// =============================================================================
// File extension advisory — input extensions cover processor-accepted types
// =============================================================================

describe("input extensions coverage (advisory)", () => {
  it("input extensions options cover common processor-accepted file types", () => {
    const extensionOptions =
      inputFields.extensions?.options?.map((o: { value: string }) => o.value) ?? [];
    // MIME type → expected extension mapping for known processors
    const mimeToExt: Record<string, string> = {
      "image/jpeg": ".jpg",
      "image/png": ".png",
      "image/webp": ".webp",
      "text/csv": ".csv",
    };
    const allAccepts = PROCESSORS.flatMap((p) => [...p.accepts]);
    const uniqueAccepts = [...new Set(allAccepts)];
    for (const mime of uniqueAccepts) {
      const ext = mimeToExt[mime];
      if (ext) {
        expect(
          extensionOptions,
          `Extension "${ext}" (for ${mime}) should be in input field options`,
        ).toContain(ext);
      }
    }
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
