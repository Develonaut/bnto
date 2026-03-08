/**
 * Primitive recipes — standalone single-operation building blocks.
 *
 * Each primitive is a complete recipe (a group node with children)
 * that performs one focused task. Composites import and compose
 * these into larger workflows. Primitives have no dependencies
 * on other recipes.
 */

export { batchCompress } from "./batchCompress";
export { batchConvert } from "./batchConvert";
export { batchRename } from "./batchRename";
export { batchResize } from "./batchResize";
export { columnRenamer } from "./columnRenamer";
export { csvCleaner } from "./csvCleaner";
