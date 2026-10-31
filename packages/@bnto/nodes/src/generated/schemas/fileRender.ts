/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for file-render node parameters. */
export const fileRenderParamsSchema = z.object({
  header: z.string().optional(),
  row: z.string(),
  footer: z.string().optional(),
  separator: z.string().optional().default("\n"),
  filename: z.string(),
  escape: z
    .enum(["none", "markdown"] as const)
    .optional()
    .default("none"),
});

/** Inferred TypeScript type for file-render node parameters. */
export type FileRenderParams = z.infer<typeof fileRenderParamsSchema>;

/** Full schema definition for the file-render node type. */
export const fileRenderNodeSchema: NodeSchema = {
  nodeType: "file-render",
  schemaVersion: 1,
  schema: fileRenderParamsSchema,
  params: {
    header: {
      label: "Header",
      description:
        "Written once before the rows. Reads {{count}} (the number of items), run templates such as {{vars.X}} or {{node.<id>.<key>}}, and inside a loop the loop's {{item.KEY}}.",
    },
    row: {
      label: "Row",
      description:
        "Written once per item. {{item.KEY}} is that item's metadata value KEY (inside a loop, over the loop's item, which {{loop.<key>}} reads as it was; a key neither has fails the run, naming the item); {{file}} is its file name.",
    },
    footer: {
      label: "Footer",
      description: "Written once after the rows. Reads what the header reads.",
    },
    separator: {
      label: "Separator",
      description:
        "Put between rows. Header, rows and footer are each on their own line, and the file ends with a newline.",
    },
    filename: {
      label: "File Name",
      description:
        "Name of the file written, e.g. report.md (.md is text/markdown, anything else text/plain). Reads run templates, and inside a loop the loop's {{item.*}}.",
      placeholder: "report.md",
    },
    escape: {
      label: "Escape Values",
      description:
        "How values put into a row are escaped; the template text never is. markdown puts a backslash before \\ ` * _ [ ] < > | and turns line breaks into <br>, so a value can't break a table cell.",
    },
  },
};
