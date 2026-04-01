/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for spreadsheet-clean node parameters. */
export const spreadsheetCleanParamsSchema = z.object({
  trimWhitespace: z.boolean().optional().default(true),
  removeEmptyRows: z.boolean().optional().default(true),
  removeDuplicates: z.boolean().optional().default(true),
});

/** Inferred TypeScript type for spreadsheet-clean node parameters. */
export type SpreadsheetCleanParams = z.infer<typeof spreadsheetCleanParamsSchema>;

/** Full schema definition for the spreadsheet-clean node type. */
export const spreadsheetCleanNodeSchema: NodeSchema = {
  nodeType: "spreadsheet-clean",
  schemaVersion: 1,
  schema: spreadsheetCleanParamsSchema,
  params: {
    trimWhitespace: {
      label: "Trim Whitespace",
      description: "Remove leading and trailing whitespace from every cell",
    },
    removeEmptyRows: {
      label: "Remove Empty Rows",
      description: "Skip rows where every cell is blank",
    },
    removeDuplicates: {
      label: "Remove Duplicates",
      description: "Remove duplicate rows, keeping the first occurrence",
    },
  },
};
