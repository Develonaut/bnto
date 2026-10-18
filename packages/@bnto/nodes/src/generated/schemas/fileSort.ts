/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for file-sort node parameters. */
export const fileSortParamsSchema = z.object({
  by: z.string(),
  case: z
    .enum(["sensitive", "insensitive"] as const)
    .optional()
    .default("sensitive"),
});

/** Inferred TypeScript type for file-sort node parameters. */
export type FileSortParams = z.infer<typeof fileSortParamsSchema>;

/** Full schema definition for the file-sort node type. */
export const fileSortNodeSchema: NodeSchema = {
  nodeType: "file-sort",
  schemaVersion: 1,
  schema: fileSortParamsSchema,
  params: {
    by: {
      label: "Sort By",
      description:
        "Metadata fields to sort by, separated by commas; later fields break ties. A leading '-' sorts that field descending. Numbers sort numerically; items missing a field (or with it null or empty) go last.",
      placeholder: "source, -title",
    },
    case: {
      label: "Letter Case",
      description:
        "How text compares: as written, by Unicode code point (upper case before lower), or ignoring case.",
    },
  },
};
