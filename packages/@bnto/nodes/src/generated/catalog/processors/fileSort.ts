/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for file-sort. */
export const fileSortProcessor: ProcessorDef = {
  nodeType: "file-sort",
  name: "Sort Files",
  description:
    "Sort items by one or more metadata fields, each ascending or descending (`source, -title`). Stable; numbers sort numerically, missing values last. Data and metadata are untouched.",
  category: "file",
  accepts: [] as const,
  platforms: ["browser", "cli", "desktop"] as const,
  parameters: [
    {
      name: "by",
      label: "Sort By",
      description:
        "Metadata fields to sort by, separated by commas; later fields break ties. A leading '-' sorts that field descending. Numbers sort numerically; items missing a field (or with it null or empty) go last.",
      type: "string" as const,
      constraints: { required: true },
      placeholder: "source, -title",
    },
    {
      name: "case",
      label: "Letter Case",
      description:
        "How text compares: as written, by Unicode code point (upper case before lower), or ignoring case.",
      type: "enum" as const,
      options: ["sensitive", "insensitive"] as const,
      default: "sensitive",
    },
  ],
  inputCardinality: "batch" as const,
};
