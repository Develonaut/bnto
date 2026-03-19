/**
 * Spreadsheet node schema — re-exports from engine-generated schemas
 * with operation hidden (pre-set from palette).
 *
 * The Rust engine is the single source of truth for spreadsheet node parameters.
 * Zod schemas, UI metadata (visibleWhen, labels, descriptions), and
 * constraints are all generated from the engine catalog snapshot.
 *
 * To change spreadsheet parameters, edit the processor `metadata()` in
 * `engine/crates/bnto-csv/src/`, then run `task nodes:generate`.
 */

import {
  SPREADSHEET_OPERATIONS,
  spreadsheetParamsSchema,
  spreadsheetNodeSchema as generated,
} from "../generated/schemas";
import type { FieldConfigMap } from "./types";

export { SPREADSHEET_OPERATIONS, spreadsheetParamsSchema };
export type { SpreadsheetParams } from "../generated/schemas";

/** Spreadsheet schema — uses engine-generated params directly. */
export const spreadsheetNodeSchema = generated;

/** UI presentation metadata for spreadsheet node fields. */
export const spreadsheetFields: FieldConfigMap = {
  operation: {
    hidden: true,
    options: [
      { value: "clean", label: "Clean" },
      { value: "rename", label: "Rename Columns" },
    ],
  },
};
