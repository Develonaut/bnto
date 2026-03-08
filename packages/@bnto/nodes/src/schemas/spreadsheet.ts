/**
 * Spreadsheet node schema — parameters for CSV operations.
 *
 * Operations are derived from the engine catalog — only engine-backed
 * operations are valid. Currently: ["clean", "rename"].
 */

import { z } from "zod";
import type { NodeSchemaDefinition } from "./types";
import { getProcessorDefaults } from "../generated/catalog";
import { getEngineOperations } from "./deriveOperations";

/** Valid spreadsheet operations — derived from engine PROCESSORS. */
export const SPREADSHEET_OPERATIONS = getEngineOperations("spreadsheet");

// --- Engine-sourced defaults ---

const cleanDefaults = getProcessorDefaults("spreadsheet", "clean");

/** Zod schema for spreadsheet node parameters. */
export const spreadsheetParamsSchema = z.object({
  operation: z.enum(SPREADSHEET_OPERATIONS as [string, ...string[]]),
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
  },
};
