/**
 * Output node schema — declares how results are delivered.
 *
 * The output node is a declaration, not a processor. The execution
 * environment reads it to know how to present results (file downloads,
 * text display, inline preview).
 */

import type { NodeSchema } from "./types";

/** Valid output modes — determines which UI widget the environment renders. */
export const OUTPUT_MODES = ["download", "display", "preview"] as const;

export const outputSchema: NodeSchema = {
  nodeType: "output",
  parameters: [
    {
      name: "mode",
      type: "enum",
      required: true,
      label: "Mode",
      description: "How results are delivered to the user.",
      default: "download",
      enumValues: OUTPUT_MODES,
    },
    {
      name: "filename",
      type: "string",
      required: false,
      label: "Filename Template",
      description: "Filename template for output files.",
      placeholder: "compressed-{{name}}",
      visibleWhen: { param: "mode", equals: "download" },
    },
    {
      name: "zip",
      type: "boolean",
      required: false,
      label: "ZIP Multiple",
      description: "Auto-zip when there are multiple output files.",
      default: true,
      visibleWhen: { param: "mode", equals: "download" },
    },
    {
      name: "label",
      type: "string",
      required: false,
      label: "Label",
      description: "Label for the download button or display section.",
      placeholder: "Compressed Images",
    },
    {
      name: "autoDownload",
      type: "boolean",
      required: false,
      label: "Auto-Download",
      description: "Automatically download results on completion.",
      default: false,
      visibleWhen: { param: "mode", equals: "download" },
    },
  ],
} as const;
