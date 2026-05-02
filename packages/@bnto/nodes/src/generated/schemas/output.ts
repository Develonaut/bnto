/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for output node parameters. */
export const outputParamsSchema = z.object({
  mode: z
    .enum(["download", "display", "preview"] as const)
    .optional()
    .default("download"),
  directory: z.string().optional().default(""),
  filename: z.string().optional(),
  zip: z.boolean().optional().default(true),
  label: z.string().optional(),
  autoDownload: z.boolean().optional().default(true),
});

/** Inferred TypeScript type for output node parameters. */
export type OutputParams = z.infer<typeof outputParamsSchema>;

/** Full schema definition for the output node type. */
export const outputNodeSchema: NodeSchema = {
  nodeType: "output",
  schemaVersion: 1,
  schema: outputParamsSchema,
  params: {
    mode: {
      label: "Mode",
      description: "How results are delivered to the user.",
    },
    directory: {
      label: "Output Directory",
      description: "Output directory path. Supports {{ctx.date}}, {{ctx.timestamp}} templates.",
      placeholder: "{{ctx.date}}-bulk-download",
    },
    filename: {
      label: "Filename Template",
      description: "Filename template for output files.",
      placeholder: "compressed-{{name}}",
    },
    zip: {
      label: "ZIP Multiple",
      description: "Auto-zip when there are multiple output files.",
    },
    label: {
      label: "Label",
      description: "Label for the download button or display section.",
      placeholder: "Compressed Images",
    },
    autoDownload: {
      label: "Auto-Download",
      description: "Automatically download results on completion.",
    },
  },
};
