/**
 * Input node schema — declares how data enters a recipe.
 *
 * The input node is a declaration, not a processor. The execution
 * environment reads it to know what widget to present (file drop zone,
 * text area, URL input).
 */

import { z } from "zod";
import type { NodeParamFields, NodeSchema } from "./types";

/** Valid input modes — determines which UI widget the environment renders. */
export const INPUT_MODES = ["file-upload", "text", "url"] as const;

/** Zod schema for input node parameters. */
export const inputParamsSchema = z.object({
  mode: z.enum(INPUT_MODES).default("file-upload"),
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

/** UI presentation metadata for input node fields. */
export const inputFields: NodeParamFields = {
  mode: {
    options: [
      { value: "file-upload", label: "File Upload" },
      { value: "text", label: "Text" },
      { value: "url", label: "URL" },
    ],
  },
  accept: {
    visibleWhen: { param: "mode", equals: "file-upload" },
  },
  extensions: {
    visibleWhen: { param: "mode", equals: "file-upload" },
    options: [
      { value: ".jpg", label: ".jpg" },
      { value: ".jpeg", label: ".jpeg" },
      { value: ".png", label: ".png" },
      { value: ".webp", label: ".webp" },
      { value: ".gif", label: ".gif" },
      { value: ".svg", label: ".svg" },
      { value: ".csv", label: ".csv" },
      { value: ".json", label: ".json" },
      { value: ".txt", label: ".txt" },
      { value: ".pdf", label: ".pdf" },
      { value: ".xml", label: ".xml" },
      { value: ".zip", label: ".zip" },
      { value: ".mp3", label: ".mp3" },
      { value: ".mp4", label: ".mp4" },
      { value: ".wav", label: ".wav" },
    ],
  },
  label: {
    visibleWhen: { param: "mode", equals: "file-upload" },
  },
  multiple: {
    visibleWhen: { param: "mode", equals: "file-upload" },
  },
  maxFileSize: {
    visibleWhen: { param: "mode", equals: "file-upload" },
  },
  maxFiles: {
    visibleWhen: { param: "mode", equals: "file-upload" },
  },
  placeholder: {
    visibleWhen: [
      { param: "mode", equals: "text" },
      { param: "mode", equals: "url" },
    ],
  },
};
