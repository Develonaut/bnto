/**
 * Recipe exports — composites (public, user-facing) and primitives (building blocks).
 */

// Composite recipes — the 6 predefined bntos that map to public URLs
export {
  cleanCsv,
  compressImages,
  convertImageFormat,
  renameCsvColumns,
  renameFiles,
  resizeImages,
} from "./composites";

// Primitive recipes — standalone building blocks used by composites
export {
  batchCompress,
  batchConvert,
  batchRename,
  batchResize,
  columnRenamer,
  csvCleaner,
} from "./primitives";
