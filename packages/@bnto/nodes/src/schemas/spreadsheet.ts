/**
 * Spreadsheet node schema — parameters for CSV operations.
 *
 * Engine-sourced operations: "clean" and "rename" (from catalog).
 * Legacy Go-era operations (read/write) kept for backward compat
 * but marked as non-engine.
 */

import { z } from "zod";
import type { NodeSchemaDefinition } from "./types";
import { getProcessorDefaults } from "../generated/catalog";

/** Valid spreadsheet operations (engine + legacy). */
export const SPREADSHEET_OPERATIONS = ["clean", "rename", "read", "write"] as const;

/** Supported spreadsheet file formats. */
export const SPREADSHEET_FORMATS = ["csv", "excel"] as const;

// --- Engine-sourced defaults ---

const cleanDefaults = getProcessorDefaults("spreadsheet", "clean");

/** Zod schema for spreadsheet node parameters. */
export const spreadsheetParamsSchema = z.object({
  operation: z.enum(SPREADSHEET_OPERATIONS),
  // Engine-implemented clean params (defaults from engine)
  trimWhitespace: z
    .boolean()
    .optional()
    .default(cleanDefaults.trimWhitespace as boolean),
  removeEmptyRows: z
    .boolean()
    .optional()
    .default(cleanDefaults.removeEmptyRows as boolean),
  removeDuplicates: z
    .boolean()
    .optional()
    .default(cleanDefaults.removeDuplicates as boolean),
  // Engine-implemented rename params
  columns: z.record(z.string()).optional(),
  // Legacy Go-era params (not in engine)
  format: z.enum(SPREADSHEET_FORMATS).optional(),
  path: z.string().optional(),
  rows: z.array(z.record(z.unknown())).optional(),
});

/** Inferred TypeScript type for spreadsheet node parameters. */
export type SpreadsheetParams = z.infer<typeof spreadsheetParamsSchema>;

/** Full schema definition for the spreadsheet node type. */
export const spreadsheetNodeSchema: NodeSchemaDefinition = {
  nodeType: "spreadsheet",
  schemaVersion: 1,
  schema: spreadsheetParamsSchema,
  params: {
    operation: {
      label: "Operation",
      description: "The spreadsheet operation to perform.",
    },
    trimWhitespace: {
      label: "Trim Whitespace",
      description: "Remove leading and trailing whitespace from every cell.",
      visibleWhen: { param: "operation", equals: "clean" },
    },
    removeEmptyRows: {
      label: "Remove Empty Rows",
      description: "Skip rows where every cell is blank.",
      visibleWhen: { param: "operation", equals: "clean" },
    },
    removeDuplicates: {
      label: "Remove Duplicates",
      description: "Remove duplicate rows, keeping the first occurrence.",
      visibleWhen: { param: "operation", equals: "clean" },
    },
    columns: {
      label: "Column Mapping",
      description: "Map of old column names to new names.",
      visibleWhen: { param: "operation", equals: "rename" },
    },
    format: {
      label: "Format",
      description: "File format — CSV or Excel.",
      visibleWhen: [
        { param: "operation", equals: "read" },
        { param: "operation", equals: "write" },
      ],
    },
    path: {
      label: "Path",
      description: "File path for the spreadsheet.",
      placeholder: "{{.INPUT_CSV}}",
      visibleWhen: [
        { param: "operation", equals: "read" },
        { param: "operation", equals: "write" },
      ],
    },
    rows: {
      label: "Rows",
      description: "Data rows for write operations.",
      visibleWhen: { param: "operation", equals: "write" },
      requiredWhen: { param: "operation", equals: "write" },
    },
  },
};
