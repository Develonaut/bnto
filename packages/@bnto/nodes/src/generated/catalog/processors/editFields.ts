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
    "Edit fields on each item, setting them from static values or templates over its other fields and its file name, such as {{item.first}} {{item.last}} or {{file}}; files pass through unchanged.",
  category: "data",
  accepts: [] as const,
  platforms: ["browser", "cli", "desktop"] as const,
  parameters: [
    {
      name: "values",
      label: "Values",
      description:
        "Field name to value. A value can read the item's other fields, e.g. {{item.first}} {{item.last}}, and {{file}} (the file's name); one that is exactly {{item.<key>}} keeps that field's type. Templates read the fields the item arrived with (inside a loop, over the loop's item, which {{loop.<key>}} reads as it was), and one neither has fails the run.",
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
