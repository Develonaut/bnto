/**
 * Edit Fields node schema — parameters for setting field values.
 *
 * Go source: engine/pkg/node/library/editfields/editfields.go
 * Validator: engine/pkg/validator/validators.go → validateEditFields
 */

import { z } from "zod";
import type { NodeSchemaDefinition } from "./types";

/** Zod schema for edit-fields node parameters. */
export const editFieldsParamsSchema = z.object({
  values: z.record(z.unknown()),
  keepOnlySet: z.boolean().optional().default(false),
});

/** Inferred TypeScript type for edit-fields node parameters. */
export type EditFieldsParams = z.infer<typeof editFieldsParamsSchema>;

/** Full schema definition for the edit-fields node type. */
export const editFieldsNodeSchema: NodeSchemaDefinition = {
  nodeType: "edit-fields",
  schemaVersion: 1,
  schema: editFieldsParamsSchema,
  params: {
    values: {
      label: "Values",
      description:
        "Map of field names to values. Values can be static or Go template expressions (e.g., {{.record.name}}).",
    },
    keepOnlySet: {
      label: "Keep Only Set Fields",
      description: "When true, only output fields that are explicitly set in values.",
    },
  },
};
