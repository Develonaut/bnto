/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for file-decode. */
export const fileDecodeProcessor: ProcessorDef = {
  nodeType: "file-decode",
  name: "Decode Files",
  description:
    "Decode each file's content from a text encoding (base64) into its bytes, so later nodes read the decoded content.",
  category: "file",
  accepts: [] as const,
  platforms: ["browser", "cli", "desktop"] as const,
  parameters: [
    {
      name: "encoding",
      label: "Encoding",
      description:
        "The text encoding each file is in. base64 is the standard alphabet (+ and /), padded or not; whitespace and line breaks are ignored. URL-safe base64 (- and _) is not accepted.",
      type: "enum" as const,
      options: ["base64"] as const,
      default: "base64",
    },
    {
      name: "filename",
      label: "Output Filename",
      description:
        "Name for each decoded file. {{name}} is the input's name without its last extension and {{ext}} that extension, e.g. {{name}}.json. Leave empty to keep the input's name.",
      type: "string" as const,
      placeholder: "{{name}}.json",
    },
  ],
  inputCardinality: "perFile" as const,
};
