"use client";

import type {
  UploadService,
  OnUploadProgress,
} from "../services/uploadService";
import type { UploadFileInput, UploadSession } from "../types/upload";

/**
 * Upload client — public API for file upload operations.
 *
 * Wraps the upload service for generating presigned URLs and
 * uploading files to R2 with progress tracking.
 */
export function createUploadClient(uploads: UploadService) {
  return {
    /** Generate presigned upload URLs without uploading. */
    generateUrls: (files: UploadFileInput[]) => uploads.generateUrls(files),

    /** Upload files to R2 and return the session. */
    uploadFiles: (files: File[], onProgress?: OnUploadProgress) =>
      uploads.uploadFiles(files, onProgress),
  } as const;
}

export type UploadClient = ReturnType<typeof createUploadClient>;
