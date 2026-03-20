/**
 * Spreadsheet clean field configs — augments engine-generated schema with UI hints.
 *
 * The Rust engine is the single source of truth for spreadsheet-clean parameters.
 * This file adds UI presentation metadata for the editor.
 *
 * To change parameters, edit the processor `metadata()` in
 * `engine/crates/bnto-csv/src/`, then run `task nodes:generate`.
 */

import type { NodeParamFields } from "./types";

export { spreadsheetCleanParamsSchema, spreadsheetCleanNodeSchema } from "../generated/schemas";
export type { SpreadsheetCleanParams } from "../generated/schemas";

/** UI presentation metadata for spreadsheet-clean node fields. */
export const spreadsheetCleanFields: NodeParamFields = {};
