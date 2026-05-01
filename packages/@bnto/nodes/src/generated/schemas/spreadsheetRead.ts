/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for spreadsheet-read node parameters. */
export const spreadsheetReadParamsSchema = z.object({
  hasHeaders: z.boolean().optional().default(true),
  delimiter: z
    .enum(["comma", "semicolon", "tab", "pipe"] as const)
    .optional()
    .default("comma"),
  maxRows: z.number().min(1).max(10000000).optional().default(100000),
});

/** Inferred TypeScript type for spreadsheet-read node parameters. */
export type SpreadsheetReadParams = z.infer<typeof spreadsheetReadParamsSchema>;

/** Full schema definition for the spreadsheet-read node type. */
export const spreadsheetReadNodeSchema: NodeSchema = {
  nodeType: "spreadsheet-read",
  schemaVersion: 1,
  schema: spreadsheetReadParamsSchema,
  params: {
    hasHeaders: {
      label: "Has Headers",
      description: "First row contains column headers",
    },
    delimiter: {
      label: "Delimiter",
      description: "Column separator character",
    },
    maxRows: {
      label: "Max Rows",
      description: "Maximum rows to process. Error if CSV exceeds this limit.",
    },
  },
};
