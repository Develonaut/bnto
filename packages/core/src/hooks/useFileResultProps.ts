import { useMemo, useCallback } from "react";
import type { BrowserFileResult } from "../types/browser";
import { core } from "../core";
import { parseFilename } from "../utils/parseFilename";

interface FileResultDisplayProps {
  filename: string;
  extension: string | null;
  outputSize: string;
  originalSize: string | undefined;
  savings: string | undefined;
  download: () => void;
}

/**
 * Derives display props from a BrowserFileResult for use with ResultFileCard.
 *
 * Handles filename, extension extraction, size formatting, savings calculation,
 * and the download callback — so consumers don't repeat this logic.
 */
function useFileResultProps(result: BrowserFileResult): FileResultDisplayProps {
  const download = useCallback(() => {
    core.executions.downloadResult(result);
  }, [result]);

  return useMemo(() => {
    const origBytes = result.metadata.originalSize as number | undefined;
    const outputBytes = result.blob.size;

    // Always show originalSize when the engine provides it.
    // Savings are shown as negative percentage when the file got smaller.
    const hasOriginal = origBytes != null && origBytes !== outputBytes;
    const savingsPercent = hasOriginal
      ? Math.round((1 - outputBytes / origBytes) * 100)
      : undefined;

    return {
      filename: result.filename,
      extension: parseFilename(result.filename).extension,
      outputSize: formatFileSize(outputBytes),
      originalSize: hasOriginal ? formatFileSize(origBytes) : undefined,
      savings: savingsPercent != null && savingsPercent > 0 ? `-${savingsPercent}%` : undefined,
      download,
    };
  }, [result, download]);
}

/**
 * Format bytes to human-readable string.
 *
 * Intentional duplication of @bnto/ui's formatFileSize — core and ui are
 * independent leaf packages that cannot depend on each other. Keep in sync
 * with packages/ui/src/utils/formatFileSize.ts.
 */
function formatFileSize(bytes: number): string {
  if (bytes === 0) return "0 B";
  const k = 1024;
  const units = ["B", "KB", "MB", "GB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  const value = bytes / Math.pow(k, i);
  return `${parseFloat(value.toFixed(1))} ${units[i]}`;
}

export { useFileResultProps };
export type { FileResultDisplayProps };
