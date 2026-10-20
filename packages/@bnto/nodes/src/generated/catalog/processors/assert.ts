/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for assert. */
export const assertProcessor: ProcessorDef = {
  nodeType: "assert",
  name: "Assert",
  description:
    "Fail the run when a comparison doesn't hold (equals, not equals, after, before); otherwise pass files through unchanged.",
  category: "control",
  accepts: [] as const,
  platforms: ["browser", "cli", "desktop"] as const,
  parameters: [
    {
      name: "value",
      label: "Value",
      description:
        "What to check, usually a template such as {{node.<id>.<key>}}. With {{item.<key>}} or {{file}} (the file's name) it is checked once per incoming file, and no files means nothing to check.",
      type: "string" as const,
      constraints: { required: true },
    },
    {
      name: "op",
      label: "Comparison",
      description: "How value is compared to expected. after and before compare RFC 3339 times.",
      type: "enum" as const,
      options: ["equals", "not_equals", "after", "before"] as const,
      default: "equals",
      constraints: { required: true },
    },
    {
      name: "expected",
      label: "Expected",
      description: "What the value is compared to.",
      type: "string" as const,
    },
    {
      name: "message",
      label: "Message",
      description:
        "Put before the reasons when the check fails, so the failure says what was checked.",
      type: "string" as const,
    },
  ],
  inputCardinality: "batch" as const,
};
