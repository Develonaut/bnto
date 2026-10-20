/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for spreadsheet-lookup node parameters. */
export const spreadsheetLookupParamsSchema = z.object({
  table: z.string(),
  keyColumn: z.string().optional().default("name"),
  matchOn: z
    .enum(["path", "filename"] as const)
    .optional()
    .default("path"),
  unmatched: z
    .enum(["keep", "drop", "fail"] as const)
    .optional()
    .default("keep"),
});

/** Inferred TypeScript type for spreadsheet-lookup node parameters. */
export type SpreadsheetLookupParams = z.infer<typeof spreadsheetLookupParamsSchema>;

/** Full schema definition for the spreadsheet-lookup node type. */
export const spreadsheetLookupNodeSchema: NodeSchema = {
  nodeType: "spreadsheet-lookup",
  schemaVersion: 1,
  schema: spreadsheetLookupParamsSchema,
  params: {
    table: {
      label: "Table",
      description: "Path of the CSV (with a header row) to look each file up in",
      placeholder: "mapping.csv",
    },
    keyColumn: {
      label: "Key Column",
      description: "Table column matched against each file; a key may appear on one row only",
    },
    matchOn: {
      label: "Match On",
      description: "Which part of the file's name must equal the key exactly",
    },
    unmatched: {
      label: "Unmatched Files",
      description:
        "What happens to a file with no row: keep it (flagged unmatched: 1), drop it, or fail the run",
    },
  },
};
