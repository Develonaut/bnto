/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for spreadsheet-read node parameters. */
export const spreadsheetReadParamsSchema = z.object({
  format: z
    .enum(["auto", "csv", "json"] as const)
    .optional()
    .default("auto"),
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
    format: {
      label: "Format",
      description: "How the file is read: auto picks JSON for a .json file and CSV otherwise",
    },
    hasHeaders: {
      label: "Has Headers",
      description: "First row contains column headers (CSV only)",
    },
    delimiter: {
      label: "Delimiter",
      description: "Column separator character (CSV only)",
    },
    maxRows: {
      label: "Max Rows",
      description:
        "Maximum rows (CSV rows or JSON array elements) to process. Error if the file has more.",
    },
  },
};
