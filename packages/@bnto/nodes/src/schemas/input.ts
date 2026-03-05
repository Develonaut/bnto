/**
 * Input node schema — declares how data enters a recipe.
 *
 * The input node is a declaration, not a processor. The execution
 * environment reads it to know what widget to present (file drop zone,
 * text area, URL input).
 */

import type { NodeSchema } from "./types";

/** Valid input modes — determines which UI widget the environment renders. */
export const INPUT_MODES = ["file-upload", "text", "url"] as const;

export const inputSchema: NodeSchema = {
  nodeType: "input",
  schemaVersion: 1,
  parameters: [
    {
      name: "mode",
      type: "enum",
      required: true,
      label: "Mode",
      description: "How data is provided to the recipe.",
      default: "file-upload",
      enumValues: INPUT_MODES,
    },
    {
      name: "accept",
      type: "array",
      required: false,
      label: "Accepted MIME Types",
      description: 'MIME types accepted (e.g., "image/jpeg", "image/png").',
      visibleWhen: { param: "mode", equals: "file-upload" },
    },
    {
      name: "extensions",
      type: "array",
      required: false,
      label: "File Extensions",
      description: 'File extensions accepted (e.g., ".jpg", ".png").',
      visibleWhen: { param: "mode", equals: "file-upload" },
    },
    {
      name: "label",
      type: "string",
      required: false,
      label: "Label",
      description: "Human-readable label for the input control.",
      placeholder: "JPEG, PNG, or WebP images",
      visibleWhen: { param: "mode", equals: "file-upload" },
    },
    {
      name: "multiple",
      type: "boolean",
      required: false,
      label: "Multiple",
      description: "Whether multiple files or items are accepted.",
      default: true,
      visibleWhen: { param: "mode", equals: "file-upload" },
    },
    {
      name: "maxFileSize",
      type: "number",
      required: false,
      label: "Max File Size",
      description: "Maximum file size in bytes. 0 = no limit.",
      default: 0,
      min: 0,
      visibleWhen: { param: "mode", equals: "file-upload" },
    },
    {
      name: "maxFiles",
      type: "number",
      required: false,
      label: "Max Files",
      description: "Maximum number of files. 0 = no limit.",
      default: 0,
      min: 0,
      visibleWhen: { param: "mode", equals: "file-upload" },
    },
    {
      name: "placeholder",
      type: "string",
      required: false,
      label: "Placeholder",
      description: "Placeholder text for text or URL input.",
      visibleWhen: [
        { param: "mode", equals: "text" },
        { param: "mode", equals: "url" },
      ],
    },
  ],
} as const;
