/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for spreadsheet-rename node parameters. */
export const spreadsheetRenameParamsSchema = z.object({
  columns: z.record(z.unknown()).optional(),
});

/** Inferred TypeScript type for spreadsheet-rename node parameters. */
export type SpreadsheetRenameParams = z.infer<typeof spreadsheetRenameParamsSchema>;

/** Full schema definition for the spreadsheet-rename node type. */
export const spreadsheetRenameNodeSchema: NodeSchema = {
  nodeType: "spreadsheet-rename",
  schemaVersion: 1,
  schema: spreadsheetRenameParamsSchema,
  params: {
    columns: {
      label: "Column Mapping",
      description: 'Map of old column names to new names (e.g., {"Name": "full_name"})',
    },
  },
};
