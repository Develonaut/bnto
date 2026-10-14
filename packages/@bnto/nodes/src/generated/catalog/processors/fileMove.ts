/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for file-move. */
export const fileMoveProcessor: ProcessorDef = {
  nodeType: "file-move",
  name: "Move Files",
  description:
    "Move files into folders under a destination, by the path in each file's name; refuses the whole run if any move would replace a file.",
  category: "file",
  accepts: [] as const,
  platforms: ["cli", "desktop"] as const,
  parameters: [
    {
      name: "destination",
      label: "Destination",
      description:
        'Folder to move files under. Each file\'s name is its path inside it (e.g. Pistol/Slide/shot.wav); absolute paths and ".." are refused.',
      type: "string" as const,
      placeholder: "./sorted",
    },
    {
      name: "create_dirs",
      label: "Create Folders",
      description:
        "Create missing folders under the destination. Off refuses the run when a folder is missing.",
      type: "boolean" as const,
      default: true,
    },
  ],
  inputCardinality: "batch" as const,
};
