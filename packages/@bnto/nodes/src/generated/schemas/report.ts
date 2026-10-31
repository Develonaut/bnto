/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for report node parameters. */
export const reportParamsSchema = z.object({
  message: z.string(),
  each: z.record(z.unknown()).optional(),
});

/** Inferred TypeScript type for report node parameters. */
export type ReportParams = z.infer<typeof reportParamsSchema>;

/** Full schema definition for the report node type. */
export const reportNodeSchema: NodeSchema = {
  nodeType: "report",
  schemaVersion: 1,
  schema: reportParamsSchema,
  params: {
    message: {
      label: "Message",
      description:
        "The line to print, usually a template. {{file}} (the file's name) or {{item.<key>}} prints it once per file (inside a loop, over the loop's item, which {{loop.<key>}} reads as it was), {{entry.<key>}} (or {{entry}}) once per entry of each, where {{item.<key>}} is the loop's item.",
    },
    each: {
      label: "For Each",
      description:
        "A list to print the message once per entry of, in order, usually a {{node.<id>.<key>}} template, or inside a loop the loop's {{item.<key>}}.",
      placeholder: "{{node.<id>.<key>}}",
    },
  },
};
