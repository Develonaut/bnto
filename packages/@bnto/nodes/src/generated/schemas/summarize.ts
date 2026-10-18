/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for summarize node parameters. */
export const summarizeParamsSchema = z.object({
  groupBy: z.string().optional(),
  sum: z.string().optional(),
});

/** Inferred TypeScript type for summarize node parameters. */
export type SummarizeParams = z.infer<typeof summarizeParamsSchema>;

/** Full schema definition for the summarize node type. */
export const summarizeNodeSchema: NodeSchema = {
  nodeType: "summarize",
  schemaVersion: 1,
  schema: summarizeParamsSchema,
  params: {
    groupBy: {
      label: "Group By",
      description:
        "Metadata fields to group items by, separated by commas; items whose values print the same share a group. Groups keep the order their first item arrived in. Items missing a field (or with it null or empty) group together, with that field null. Leave empty to summarize every item as one group.",
      placeholder: "type, year",
    },
    sum: {
      label: "Sum",
      description:
        "Numeric metadata fields to total per group, separated by commas; each total is written as sum_<field>. Numbers written as text count; blank values are skipped; any other value fails the run, naming the item.",
      placeholder: "file_size",
    },
  },
};
