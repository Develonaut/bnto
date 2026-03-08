/**
 * Composite recipes — the 6 Tier 1 predefined bntos.
 *
 * Each composite is a complete Input -> Group -> Output pipeline that
 * composes one or more primitive recipes into a user-facing workflow.
 * These map to public URLs at `/{slug}`.
 */

export { cleanCsv } from "./cleanCsv";
export { compressImages } from "./compressImages";
export { convertImageFormat } from "./convertImageFormat";
export { renameCsvColumns } from "./renameCsvColumns";
export { renameFiles } from "./renameFiles";
export { resizeImages } from "./resizeImages";
