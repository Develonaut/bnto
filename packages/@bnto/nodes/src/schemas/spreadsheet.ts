/**
 * Spreadsheet node schema — parameters for CSV/Excel operations.
 *
 * Go source: engine/pkg/node/library/spreadsheet/spreadsheet.go
 */

import { z } from "zod";
import type { NodeSchemaDefinition } from "./types";

/** Valid spreadsheet operations. */
export const SPREADSHEET_OPERATIONS = ["read", "write"] as const;

/** Supported spreadsheet file formats. */
export const SPREADSHEET_FORMATS = ["csv", "excel"] as const;

/** Zod schema for spreadsheet node parameters. */
export const spreadsheetParamsSchema = z.object({
  operation: z.enum(SPREADSHEET_OPERATIONS),
  format: z.enum(SPREADSHEET_FORMATS),
  path: z.string(),
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
      description: "Whether to read from or write to a spreadsheet file.",
    },
    format: {
      label: "Format",
      description: "File format — CSV or Excel.",
    },
    path: {
      label: "Path",
      description: "File path for the spreadsheet.",
      placeholder: "{{.INPUT_CSV}}",
    },
    rows: {
      label: "Rows",
      description: "Data rows for write operations. Array of objects with column headers as keys.",
      visibleWhen: { param: "operation", equals: "write" },
      requiredWhen: { param: "operation", equals: "write" },
    },
  },
};
