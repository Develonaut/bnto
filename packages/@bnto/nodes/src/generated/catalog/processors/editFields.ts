/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for edit-fields. */
export const editFieldsProcessor: ProcessorDef = {
  nodeType: "edit-fields",
  name: "Edit Fields",
  description:
    "Set fields on each item from static values or templates over its other fields, such as {{item.first}} {{item.last}}; files pass through unchanged.",
  category: "data",
  accepts: [] as const,
  platforms: ["browser", "cli", "desktop"] as const,
  parameters: [
    {
      name: "values",
      label: "Values",
      description:
        "Field name to value. A value can read the item's other fields, e.g. {{item.first}} {{item.last}}; one that is exactly {{item.<key>}} keeps that field's type. Templates read the fields the item arrived with, and one it lacks fails the run.",
      type: "object" as const,
      constraints: { required: true },
    },
    {
      name: "keepOnlySet",
      label: "Keep Only Set Fields",
      description:
        "When on, each item keeps only the fields set in values; every other field it carried is dropped.",
      type: "boolean" as const,
      default: false,
    },
  ],
  inputCardinality: "batch" as const,
};
