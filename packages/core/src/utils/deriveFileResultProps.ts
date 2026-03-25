import type { BrowserFileResult } from "../types/browser";
import { parseFilename } from "./parseFilename";

interface FileResultDisplay {
  filename: string;
  extension: string | null;
  outputSize: string;
  originalSize: string | undefined;
  savings: string | undefined;
}

/**
 * Pure function that derives display props from a BrowserFileResult.
 *
 * Extracts filename, extension, sizes, and savings percentage — everything
 * FileList consumers need except the download callback (which is consumer-specific).
 */
function deriveFileResultProps(result: BrowserFileResult): FileResultDisplay {
  const origBytes = result.metadata.originalSize as number | undefined;
  const outputBytes = result.blob.size;

  const hasOriginal = origBytes != null && origBytes !== outputBytes;
  const savingsPercent = hasOriginal ? Math.round((1 - outputBytes / origBytes) * 100) : undefined;

  return {
    filename: result.filename,
    extension: parseFilename(result.filename).extension,
    outputSize: formatFileSize(outputBytes),
    originalSize: hasOriginal ? formatFileSize(origBytes) : undefined,
    savings: savingsPercent != null && savingsPercent > 0 ? `-${savingsPercent}%` : undefined,
  };
}

/**
 * Format bytes to human-readable string.
 *
 * Intentional duplication of @bnto/ui's formatFileSize — core and ui are
 * independent leaf packages that cannot depend on each other.
 */
function formatFileSize(bytes: number): string {
  if (bytes === 0) return "0 B";
  const k = 1024;
  const units = ["B", "KB", "MB", "GB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  const value = bytes / Math.pow(k, i);
  return `${parseFloat(value.toFixed(1))} ${units[i]}`;
}

export { deriveFileResultProps };
export type { FileResultDisplay };
