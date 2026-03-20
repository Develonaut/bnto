/**
 * Recipe exports — all predefined recipes.
 *
 * Every recipe is a complete `Recipe` with I/O nodes.
 */

export { cleanCsv } from "./cleanCsv";
export { compressAndRename } from "./compressAndRename";
export { compressImages } from "./compressImages";
export { convertImageFormat } from "./convertImageFormat";
export { generateThumbnails } from "./generateThumbnails";
export { optimizeImagesForWeb } from "./optimizeImagesForWeb";
export { renameCsvColumns } from "./renameCsvColumns";
export { renameFiles } from "./renameFiles";
export { resizeImages } from "./resizeImages";
export { standardizeCsv } from "./standardizeCsv";

// I/O node factory functions
export { defaultInputNode } from "./defaultInputNode";
export type { InputNodeOptions } from "./defaultInputNode";
export { defaultOutputNode } from "./defaultOutputNode";
export type { OutputNodeOptions } from "./defaultOutputNode";
