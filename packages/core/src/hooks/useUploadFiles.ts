"use client";

import { useCallback, useRef, useState } from "react";
import { core } from "../core";
import type { FileUploadProgress, UploadProgress, UploadSession } from "../types/upload";

const IDLE_PROGRESS: UploadProgress = {
  sessionId: null,
  files: [],
  status: "idle",
};

/**
 * Upload files to R2 via presigned URLs with real-time progress tracking.
 *
 * Returns upload state and an `upload` function that:
 * 1. Requests presigned URLs from the backend
 * 2. Uploads files directly to R2 in parallel
 * 3. Returns the UploadSession (with sessionId for execution)
 */
/** Derive overall upload status from per-file progress. */
function deriveUploadStatus(files: FileUploadProgress[]): UploadProgress["status"] {
  if (files.some((f) => f.status === "failed")) return "failed";
  if (files.every((f) => f.status === "completed")) return "completed";
  return "uploading";
}

export function useUploadFiles() {
  const [progress, setProgress] = useState<UploadProgress>(IDLE_PROGRESS);
  const abortRef = useRef(false);

  const handleProgress = useCallback((files: FileUploadProgress[]) => {
    setProgress((prev) => ({ ...prev, files, status: deriveUploadStatus(files) }));
  }, []);

  const upload = useCallback(
    async (files: File[]): Promise<UploadSession> => {
      abortRef.current = false;
      setProgress({ sessionId: null, files: [], status: "uploading" });
      const session = await core.uploads.uploadFiles(files, handleProgress);
      setProgress((prev) => ({ ...prev, sessionId: session.sessionId, status: "completed" }));
      return session;
    },
    [handleProgress],
  );

  const reset = useCallback(() => {
    abortRef.current = true;
    setProgress(IDLE_PROGRESS);
  }, []);

  return {
    progress,
    upload,
    reset,
    isUploading: progress.status === "uploading",
    isComplete: progress.status === "completed",
    isFailed: progress.status === "failed",
  };
}
