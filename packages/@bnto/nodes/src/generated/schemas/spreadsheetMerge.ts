/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for spreadsheet-merge node parameters. */
export const spreadsheetMergeParamsSchema = z.object({
  headerHandling: z
    .enum(["first-file", "union"] as const)
    .optional()
    .default("first-file"),
  deduplicate: z.boolean().optional().default(false),
});

/** Inferred TypeScript type for spreadsheet-merge node parameters. */
export type SpreadsheetMergeParams = z.infer<typeof spreadsheetMergeParamsSchema>;

/** Full schema definition for the spreadsheet-merge node type. */
export const spreadsheetMergeNodeSchema: NodeSchema = {
  nodeType: "spreadsheet-merge",
  schemaVersion: 1,
  schema: spreadsheetMergeParamsSchema,
  params: {
    headerHandling: {
      label: "Header Handling",
      description: "How to reconcile headers across files",
    },
    deduplicate: {
      label: "Remove Duplicates",
      description: "Remove duplicate rows across all files",
    },
  },
};
