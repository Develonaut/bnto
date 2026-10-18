/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for file-join. */
export const fileJoinProcessor: ProcessorDef = {
  nodeType: "file-join",
  name: "Join Files",
  description:
    "Join the files flowing in into one file, in the order they arrive, with an optional separator between them.",
  category: "file",
  accepts: [] as const,
  platforms: ["browser", "cli", "desktop"] as const,
  parameters: [
    {
      name: "filename",
      label: "File Name",
      description:
        "Name of the joined file, e.g. README.md; its MIME type follows the extension. Reads run templates such as {{vars.X}}.",
      type: "string" as const,
      constraints: { required: true },
      placeholder: "joined.txt",
    },
    {
      name: "separator",
      label: "Separator",
      description:
        "Put between files, as written (a newline, a blank line, ---). Not before the first or after the last. Empty joins them directly.",
      type: "string" as const,
      default: "",
    },
  ],
  inputCardinality: "batch" as const,
};
