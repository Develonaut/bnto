/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for edit-fields node parameters. */
export const editFieldsParamsSchema = z.object({
  values: z.record(z.unknown()),
  keepOnlySet: z.boolean().optional().default(false),
});

/** Inferred TypeScript type for edit-fields node parameters. */
export type EditFieldsParams = z.infer<typeof editFieldsParamsSchema>;

/** Full schema definition for the edit-fields node type. */
export const editFieldsNodeSchema: NodeSchema = {
  nodeType: "edit-fields",
  schemaVersion: 1,
  schema: editFieldsParamsSchema,
  params: {
    values: {
      label: "Values",
      description:
        "Field name to value. A value can read the item's other fields, e.g. {{item.first}} {{item.last}}, and {{file}} (the file's name); one that is exactly {{item.<key>}} keeps that field's type. Templates read the fields the item arrived with (inside a loop, over the loop's item, which {{loop.<key>}} reads as it was), and one neither has fails the run.",
    },
    keepOnlySet: {
      label: "Keep Only Set Fields",
      description:
        "When on, each item keeps only the fields set in values; every other field it carried is dropped.",
    },
  },
};
