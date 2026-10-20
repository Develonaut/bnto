/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for spreadsheet-export node parameters. */
export const spreadsheetExportParamsSchema = z.object({
  path: z.string(),
  nameColumn: z.string().optional().default("file"),
  overwrite: z.boolean().optional().default(false),
});

/** Inferred TypeScript type for spreadsheet-export node parameters. */
export type SpreadsheetExportParams = z.infer<typeof spreadsheetExportParamsSchema>;

/** Full schema definition for the spreadsheet-export node type. */
export const spreadsheetExportNodeSchema: NodeSchema = {
  nodeType: "spreadsheet-export",
  schemaVersion: 1,
  schema: spreadsheetExportParamsSchema,
  params: {
    path: {
      label: "CSV Path",
      description: "File to write, one row per file passing through; its folder must exist",
      placeholder: "items.csv",
    },
    nameColumn: {
      label: "Name Column",
      description: "Header of the first column, which holds each file's name",
    },
    overwrite: {
      label: "Overwrite",
      description:
        "Replace a CSV already at the path; off keeps it (it may be an earlier run's record) and fails the run",
    },
  },
};
