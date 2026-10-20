/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for spreadsheet-lookup. */
export const spreadsheetLookupProcessor: ProcessorDef = {
  nodeType: "spreadsheet-lookup",
  name: "Look Up Rows",
  description: "Look up the CSV row whose key matches each file's name and attach it to that file",
  category: "spreadsheet",
  accepts: [] as const,
  platforms: ["cli", "desktop"] as const,
  parameters: [
    {
      name: "table",
      label: "Table",
      description: "Path of the CSV (with a header row) to look each file up in",
      type: "string" as const,
      constraints: { required: true },
      placeholder: "mapping.csv",
    },
    {
      name: "key_column",
      label: "Key Column",
      description: "Table column matched against each file; a key may appear on one row only",
      type: "string" as const,
      default: "name",
    },
    {
      name: "match_on",
      label: "Match On",
      description: "Which part of the file's name must equal the key exactly",
      type: "enum" as const,
      options: ["path", "filename"] as const,
      default: "path",
    },
    {
      name: "unmatched",
      label: "Unmatched Files",
      description:
        "What happens to a file with no row: keep it (flagged unmatched: 1), drop it, or fail the run",
      type: "enum" as const,
      options: ["keep", "drop", "fail"] as const,
      default: "keep",
    },
  ],
  inputCardinality: "perFile" as const,
};
