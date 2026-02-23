"use client";

import { generateDownloadUrls } from "../adapters/convex/downloadAdapter";
import type { DownloadUrlsResult } from "../types/download";

/**
 * Download service — single-domain logic for output file downloads.
 *
 * Fetches presigned GET URLs from the backend and triggers browser downloads.
 */
export function createDownloadService() {
  return {
    /** Get presigned download URLs for all output files of an execution. */
    getDownloadUrls: (executionId: string): Promise<DownloadUrlsResult> =>
      generateDownloadUrls(executionId),

    /**
     * Trigger a browser download for a single file via its presigned URL.
     * Creates a temporary anchor element to initiate the download.
     */
    downloadFile: (url: string, fileName: string) => {
      const anchor = document.createElement("a");
      anchor.href = url;
      anchor.download = fileName;
      anchor.style.display = "none";
      document.body.appendChild(anchor);
      anchor.click();
      document.body.removeChild(anchor);
    },
  } as const;
}

export type DownloadService = ReturnType<typeof createDownloadService>;
