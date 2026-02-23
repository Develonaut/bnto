"use client";

import type { DownloadService } from "../services/downloadService";

/**
 * Download client — public API for downloading execution output files.
 *
 * Wraps the download service for fetching presigned URLs and
 * triggering browser downloads.
 */
export function createDownloadClient(downloads: DownloadService) {
  return {
    /** Get presigned download URLs for an execution's output files. */
    getDownloadUrls: (executionId: string) =>
      downloads.getDownloadUrls(executionId),

    /** Trigger a browser download for a single file. */
    downloadFile: (url: string, fileName: string) =>
      downloads.downloadFile(url, fileName),
  } as const;
}

export type DownloadClient = ReturnType<typeof createDownloadClient>;
