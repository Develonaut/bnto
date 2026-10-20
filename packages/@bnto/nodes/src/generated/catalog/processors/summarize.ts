/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for summarize. */
export const summarizeProcessor: ProcessorDef = {
  nodeType: "summarize",
  name: "Summarize",
  description:
    "Summarize items by metadata fields, one item per group with its count and the totals of numeric fields (sum_<field>). Groups keep first-seen order; the items summarized don't pass through.",
  category: "data",
  accepts: [] as const,
  platforms: ["browser", "cli", "desktop"] as const,
  parameters: [
    {
      name: "groupBy",
      label: "Group By",
      description:
        "Metadata fields to group items by, separated by commas; items whose values print the same share a group. Groups keep the order their first item arrived in. Items missing a field (or with it null or empty) group together, with that field null. Leave empty to summarize every item as one group.",
      type: "string" as const,
      placeholder: "type, year",
    },
    {
      name: "sum",
      label: "Sum",
      description:
        "Numeric metadata fields to total per group, separated by commas; each total is written as sum_<field>. Numbers written as text count; blank values are skipped; any other value fails the run, naming the item.",
      type: "string" as const,
      placeholder: "file_size",
    },
  ],
  inputCardinality: "batch" as const,
};
