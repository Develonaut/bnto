/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for spreadsheet-convert node parameters. */
export const spreadsheetConvertParamsSchema = z.object({
  delimiter: z
    .enum(["comma", "semicolon", "tab", "pipe"] as const)
    .optional()
    .default("comma"),
  pretty: z.boolean().optional().default(false),
});

/** Inferred TypeScript type for spreadsheet-convert node parameters. */
export type SpreadsheetConvertParams = z.infer<typeof spreadsheetConvertParamsSchema>;

/** Full schema definition for the spreadsheet-convert node type. */
export const spreadsheetConvertNodeSchema: NodeSchema = {
  nodeType: "spreadsheet-convert",
  schemaVersion: 1,
  schema: spreadsheetConvertParamsSchema,
  params: {
    delimiter: {
      label: "Delimiter",
      description: "Column separator character",
    },
    pretty: {
      label: "Pretty Print",
      description: "Format output JSON with indentation",
    },
  },
};
