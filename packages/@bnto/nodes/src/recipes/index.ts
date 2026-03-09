/**
 * Recipe exports — all 12 predefined recipes.
 *
 * Every recipe is a complete `Recipe` with I/O nodes. No distinction
 * between "primitives" and "composites" — all recipes are uniform.
 */

// Building-block recipes (formerly "primitives")
export { batchCompress } from "./batchCompress";
export { batchConvert } from "./batchConvert";
export { batchRename } from "./batchRename";
export { batchResize } from "./batchResize";
export { columnRenamer } from "./columnRenamer";
export { csvCleaner } from "./csvCleaner";

// Composite recipes — the 6 Tier 1 bntos that map to public URLs
export { cleanCsv } from "./cleanCsv";
export { compressImages } from "./compressImages";
export { convertImageFormat } from "./convertImageFormat";
export { renameCsvColumns } from "./renameCsvColumns";
export { renameFiles } from "./renameFiles";
export { resizeImages } from "./resizeImages";
