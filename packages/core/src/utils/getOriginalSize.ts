import type { ProcessorMetadata } from "../types/processorMetadata";

/**
 * The size to compare a result against, or undefined when no node reported
 * sizes (renames, CSV conversions — no comparison is shown).
 *
 * Uses the uploaded file's size (`inputSize`, recorded when the file enters
 * the pipeline) over a node's `originalSize`, which is that node's own input:
 * in resize → convert, convert's originalSize is the resized image.
 */
function getOriginalSize(metadata: ProcessorMetadata): number | undefined {
  if (metadata.originalSize == null) return undefined;
  return typeof metadata.inputSize === "number" ? metadata.inputSize : metadata.originalSize;
}

export { getOriginalSize };
