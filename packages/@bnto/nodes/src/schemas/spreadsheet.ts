/**
 * Spreadsheet node schema — re-exports from engine-generated schemas.
 *
 * The Rust engine is the single source of truth for spreadsheet node parameters.
 * Zod schemas, UI metadata (visibleWhen, labels, descriptions), and
 * constraints are all generated from the engine catalog snapshot.
 *
 * To change spreadsheet parameters, edit the processor `metadata()` in
 * `engine/crates/bnto-csv/src/`, then run `task nodes:generate`.
 */

export {
  SPREADSHEET_OPERATIONS,
  spreadsheetParamsSchema,
  spreadsheetNodeSchema,
} from "../generated/schemas";

export type { SpreadsheetParams } from "../generated/schemas";
