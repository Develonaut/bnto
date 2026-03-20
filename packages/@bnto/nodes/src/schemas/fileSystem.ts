/**
 * File System node schema — re-exports from engine-generated schemas
 * with operation hidden (pre-set from palette).
 *
 * The Rust engine is the single source of truth for file-system node parameters.
 * Zod schemas, UI metadata (visibleWhen, labels, descriptions), and
 * constraints are all generated from the engine catalog snapshot.
 *
 * To change file-system parameters, edit the processor `metadata()` in
 * `engine/crates/bnto-file/src/rename.rs`, then run `task nodes:generate`.
 */

import {
  FILE_SYSTEM_OPERATIONS,
  fileSystemParamsSchema,
  fileSystemNodeSchema as generated,
} from "../generated/schemas";
import type { FieldConfigMap } from "./types";

export { FILE_SYSTEM_OPERATIONS as FILE_OPERATIONS, fileSystemParamsSchema };
export type { FileSystemParams } from "../generated/schemas";

/** File system schema — uses engine-generated params directly. */
export const fileSystemNodeSchema = generated;

/** UI presentation metadata for file system node fields. */
export const fileSystemFields: FieldConfigMap = {
  operation: {
    hidden: true,
    options: [{ value: "rename", label: "Rename" }],
  },
};
