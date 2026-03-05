/**
 * Output node schema — declares how results are delivered.
 *
 * The output node is a declaration, not a processor. The execution
 * environment reads it to know how to present results (file downloads,
 * text display, inline preview).
 */

import { z } from "zod";
import type { NodeSchemaDefinition } from "./types";

/** Valid output modes — determines which UI widget the environment renders. */
export const OUTPUT_MODES = ["download", "display", "preview"] as const;

/** Zod schema for output node parameters. */
export const outputParamsSchema = z.object({
  mode: z.enum(OUTPUT_MODES).default("download"),
  filename: z.string().optional(),
  zip: z.boolean().optional().default(true),
  label: z.string().optional(),
  autoDownload: z.boolean().optional().default(false),
});

/** Inferred TypeScript type for output node parameters. */
export type OutputParams = z.infer<typeof outputParamsSchema>;

/** Full schema definition for the output node type. */
export const outputNodeSchema: NodeSchemaDefinition = {
  nodeType: "output",
  schemaVersion: 1,
  schema: outputParamsSchema,
  params: {
    mode: {
      label: "Mode",
      description: "How results are delivered to the user.",
    },
    filename: {
      label: "Filename Template",
      description: "Filename template for output files.",
      placeholder: "compressed-{{name}}",
      visibleWhen: { param: "mode", equals: "download" },
    },
    zip: {
      label: "ZIP Multiple",
      description: "Auto-zip when there are multiple output files.",
      visibleWhen: { param: "mode", equals: "download" },
    },
    label: {
      label: "Label",
      description: "Label for the download button or display section.",
      placeholder: "Compressed Images",
    },
    autoDownload: {
      label: "Auto-Download",
      description: "Automatically download results on completion.",
      visibleWhen: { param: "mode", equals: "download" },
    },
  },
};
