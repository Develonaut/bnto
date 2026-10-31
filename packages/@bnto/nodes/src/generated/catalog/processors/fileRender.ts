/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for file-render. */
export const fileRenderProcessor: ProcessorDef = {
  nodeType: "file-render",
  name: "Render Text File",
  description:
    "Render one text file from a template: a header, one row per item from its metadata, then a footer (a Markdown table, a report).",
  category: "file",
  accepts: [] as const,
  platforms: ["browser", "cli", "desktop"] as const,
  parameters: [
    {
      name: "header",
      label: "Header",
      description:
        "Written once before the rows. Reads {{count}} (the number of items), run templates such as {{vars.X}} or {{node.<id>.<key>}}, and inside a loop the loop's {{item.KEY}}.",
      type: "string" as const,
    },
    {
      name: "row",
      label: "Row",
      description:
        "Written once per item. {{item.KEY}} is that item's metadata value KEY (inside a loop, over the loop's item, which {{loop.<key>}} reads as it was; a key neither has fails the run, naming the item); {{file}} is its file name.",
      type: "string" as const,
      constraints: { required: true },
    },
    {
      name: "footer",
      label: "Footer",
      description: "Written once after the rows. Reads what the header reads.",
      type: "string" as const,
    },
    {
      name: "separator",
      label: "Separator",
      description:
        "Put between rows. Header, rows and footer are each on their own line, and the file ends with a newline.",
      type: "string" as const,
      default: "\n",
    },
    {
      name: "filename",
      label: "File Name",
      description:
        "Name of the file written, e.g. report.md (.md is text/markdown, anything else text/plain). Reads run templates, and inside a loop the loop's {{item.*}}.",
      type: "string" as const,
      constraints: { required: true },
      placeholder: "report.md",
    },
    {
      name: "escape",
      label: "Escape Values",
      description:
        "How values put into a row are escaped; the template text never is. markdown puts a backslash before \\ ` * _ [ ] < > | and turns line breaks into <br>, so a value can't break a table cell.",
      type: "enum" as const,
      options: ["none", "markdown"] as const,
      default: "none",
    },
  ],
  inputCardinality: "batch" as const,
};
