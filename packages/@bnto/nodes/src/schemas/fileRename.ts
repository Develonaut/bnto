/**
 * File rename field configs — augments engine-generated schema with UI hints.
 *
 * The Rust engine is the single source of truth for file-rename parameters.
 * This file adds UI presentation metadata (case options) for the editor.
 *
 * To change parameters, edit the processor `metadata()` in
 * `engine/crates/bnto-file/src/rename.rs`, then run `task nodes:generate`.
 */

import type { NodeParamFields } from "./types";

export { fileRenameParamsSchema, fileRenameNodeSchema } from "../generated/schemas";
export type { FileRenameParams } from "../generated/schemas";

/** UI presentation metadata for file-rename node fields. */
export const fileRenameFields: NodeParamFields = {
  case: {
    options: [
      { value: "lower", label: "lowercase" },
      { value: "upper", label: "UPPERCASE" },
      { value: "title", label: "Title Case" },
    ],
  },
};
