/**
 * Edit Fields node schema — parameters for setting field values.
 *
 * Go source: engine/pkg/node/library/editfields/editfields.go
 * Validator: engine/pkg/validator/validators.go → validateEditFields
 */

import type { NodeSchema } from "./types";

export const editFieldsSchema: NodeSchema = {
  nodeType: "edit-fields",
  schemaVersion: 1,
  parameters: [
    {
      name: "values",
      type: "object",
      required: true,
      label: "Values",
      description:
        "Map of field names to values. Values can be static or Go template expressions (e.g., {{.record.name}}).",
    },
    {
      name: "keepOnlySet",
      type: "boolean",
      required: false,
      label: "Keep Only Set Fields",
      description: "When true, only output fields that are explicitly set in values.",
      default: false,
    },
  ],
} as const;
