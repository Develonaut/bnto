"use client";

import { generateUploadUrls } from "../adapters/convex/uploadAdapter";
import type {
  UploadFileInput,
  UploadSession,
  FileUploadProgress,
} from "../types/upload";

/** Callback invoked whenever a file's upload progress changes. */
export type OnUploadProgress = (files: FileUploadProgress[]) => void;

/**
 * Upload a single file to an R2 presigned URL.
 * Uses XMLHttpRequest for progress events — fetch doesn't support upload progress.
 */
function uploadFileToR2(
  file: File,
  url: string,
  onProgress: (loaded: number, total: number) => void,
): Promise<void> {
  return new Promise((resolve, reject) => {
    const xhr = new XMLHttpRequest();
    xhr.open("PUT", url, true);
    xhr.setRequestHeader("Content-Type", file.type);

    xhr.upload.addEventListener("progress", (event) => {
      if (event.lengthComputable) {
        onProgress(event.loaded, event.total);
      }
    });

    xhr.addEventListener("load", () => {
      if (xhr.status >= 200 && xhr.status < 300) {
        resolve();
      } else {
        reject(new Error(`Upload failed with status ${xhr.status}`));
      }
    });

    xhr.addEventListener("error", () => reject(new Error("Upload failed")));
    xhr.addEventListener("abort", () => reject(new Error("Upload cancelled")));

    xhr.send(file);
  });
}

export function createUploadService() {
  return {
    /** Request presigned URLs from the backend. */
    generateUrls: (files: UploadFileInput[]) => generateUploadUrls(files),

    /**
     * Upload files to R2 via presigned URLs with progress tracking.
     *
     * 1. Calls the backend to generate presigned URLs (returns a sessionId).
     * 2. Uploads each file in parallel with per-file progress.
     * 3. Calls onProgress after each progress event.
     * 4. Returns the sessionId on success.
     */
    uploadFiles: async (
      files: File[],
      onProgress?: OnUploadProgress,
    ): Promise<UploadSession> => {
      const inputs: UploadFileInput[] = files.map((f) => ({
        name: f.name,
        contentType: f.type || "application/octet-stream",
        sizeBytes: f.size,
      }));

      const session = await generateUploadUrls(inputs);

      const progress: FileUploadProgress[] = files.map((f) => ({
        fileName: f.name,
        loaded: 0,
        total: f.size,
        status: "pending" as const,
      }));

      onProgress?.(progress);

      const uploadPromises = session.urls.map(async (presigned, index) => {
        const file = files[index];
        progress[index] = { ...progress[index], status: "uploading" };
        onProgress?.([...progress]);

        try {
          await uploadFileToR2(file, presigned.url, (loaded, total) => {
            progress[index] = { ...progress[index], loaded, total };
            onProgress?.([...progress]);
          });

          progress[index] = {
            ...progress[index],
            loaded: file.size,
            status: "completed",
          };
          onProgress?.([...progress]);
        } catch (error) {
          progress[index] = {
            ...progress[index],
            status: "failed",
            error: error instanceof Error ? error.message : "Upload failed",
          };
          onProgress?.([...progress]);
          throw error;
        }
      });

      await Promise.all(uploadPromises);

      return session;
    },
  } as const;
}

export type UploadService = ReturnType<typeof createUploadService>;
