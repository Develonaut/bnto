/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for input node parameters. */
export const inputParamsSchema = z.object({
  mode: z
    .enum(["file-upload", "text", "url"] as const)
    .optional()
    .default("file-upload"),
  accept: z.array(z.string()).optional(),
  extensions: z.array(z.string()).optional(),
  label: z.string().optional(),
  multiple: z.boolean().optional().default(true),
  maxFileSize: z.number().min(0).optional().default(0),
  maxFiles: z.number().min(0).optional().default(0),
  placeholder: z.string().optional(),
});

/** Inferred TypeScript type for input node parameters. */
export type InputParams = z.infer<typeof inputParamsSchema>;

/** Full schema definition for the input node type. */
export const inputNodeSchema: NodeSchema = {
  nodeType: "input",
  schemaVersion: 1,
  schema: inputParamsSchema,
  params: {
    mode: {
      label: "Mode",
      description: "How data is provided to the recipe.",
    },
    accept: {
      label: "Accepted MIME Types",
      description:
        'MIME types accepted (e.g., "image/jpeg", "image/png"). Derived from extensions.',
    },
    extensions: {
      label: "File Extensions",
      description: 'File extensions accepted (e.g., ".jpg", ".png").',
    },
    label: {
      label: "Label",
      description: "Human-readable label for the input control.",
      placeholder: "JPEG, PNG, or WebP images",
    },
    multiple: {
      label: "Multiple",
      description: "Whether multiple files or items are accepted.",
    },
    maxFileSize: {
      label: "Max File Size",
      description: "Maximum file size in bytes. 0 = no limit.",
    },
    maxFiles: {
      label: "Max Files",
      description: "Maximum number of files. 0 = no limit.",
    },
    placeholder: {
      label: "Placeholder",
      description: "Placeholder text for text or URL input.",
    },
  },
};
