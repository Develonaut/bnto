/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for report. */
export const reportProcessor: ProcessorDef = {
  nodeType: "report",
  name: "Report",
  description:
    "Report a templated line in the run's output: once, once per file, or once per entry of a list; files pass through unchanged.",
  category: "control",
  accepts: [] as const,
  platforms: ["browser", "cli", "desktop"] as const,
  parameters: [
    {
      name: "message",
      label: "Message",
      description:
        "The line to print, usually a template. {{file}} (the file's name) or {{item.<key>}} prints it once per file (inside a loop, over the loop's item, which {{loop.<key>}} reads as it was), {{entry.<key>}} (or {{entry}}) once per entry of each, where {{item.<key>}} is the loop's item.",
      type: "string" as const,
      constraints: { required: true },
    },
    {
      name: "each",
      label: "For Each",
      description:
        "A list to print the message once per entry of, in order, usually a {{node.<id>.<key>}} template, or inside a loop the loop's {{item.<key>}}.",
      type: "object" as const,
      placeholder: "{{node.<id>.<key>}}",
    },
  ],
  inputCardinality: "batch" as const,
};
