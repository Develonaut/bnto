/**
 * File System node schema — re-exports from engine-generated schemas.
 *
 * The Rust engine is the single source of truth for file-system node parameters.
 * Zod schemas, UI metadata (visibleWhen, labels, descriptions), and
 * constraints are all generated from the engine catalog snapshot.
 *
 * To change file-system parameters, edit the processor `metadata()` in
 * `engine/crates/bnto-file/src/rename.rs`, then run `task nodes:generate`.
 */

export {
  FILE_SYSTEM_OPERATIONS as FILE_OPERATIONS,
  fileSystemParamsSchema,
  fileSystemNodeSchema,
} from "../generated/schemas";

export type { FileSystemParams } from "../generated/schemas";
