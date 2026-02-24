/**
 * Spreadsheet node schema — parameters for CSV/Excel operations.
 *
 * Go source: engine/pkg/node/library/spreadsheet/spreadsheet.go
 */

import type { NodeSchema } from "./types";

/** Valid spreadsheet operations. */
export const SPREADSHEET_OPERATIONS = ["read", "write"] as const;

/** Supported spreadsheet file formats. */
export const SPREADSHEET_FORMATS = ["csv", "excel"] as const;

export const spreadsheetSchema: NodeSchema = {
  nodeType: "spreadsheet",
  parameters: [
    {
      name: "operation",
      type: "enum",
      required: true,
      label: "Operation",
      description: "Whether to read from or write to a spreadsheet file.",
      enumValues: SPREADSHEET_OPERATIONS,
    },
    {
      name: "format",
      type: "enum",
      required: true,
      label: "Format",
      description: "File format — CSV or Excel.",
      enumValues: SPREADSHEET_FORMATS,
    },
    {
      name: "path",
      type: "string",
      required: true,
      label: "Path",
      description: "File path for the spreadsheet.",
      placeholder: "{{.INPUT_CSV}}",
    },
    {
      name: "rows",
      type: "array",
      required: false,
      label: "Rows",
      description:
        "Data rows for write operations. Array of objects with column headers as keys.",
      visibleWhen: { param: "operation", equals: "write" },
      requiredWhen: { param: "operation", equals: "write" },
    },
  ],
} as const;
