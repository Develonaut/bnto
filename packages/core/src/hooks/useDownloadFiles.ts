"use client";

import { useCallback, useState } from "react";
import { core } from "../core";
import type { OutputFileUrl } from "../types/download";

interface DownloadState {
  status: "idle" | "loading" | "ready" | "failed";
  urls: OutputFileUrl[];
  error?: string;
}

const IDLE_STATE: DownloadState = { status: "idle", urls: [] };

/**
 * Fetch presigned download URLs for an execution's output files.
 *
 * Returns state and actions:
 * - `fetchUrls(executionId)` — request download URLs from the backend
 * - `downloadFile(url)` — trigger a browser download for a single file
 * - `downloadAll()` — trigger downloads for all files sequentially
 */
export function useDownloadFiles() {
  const [state, setState] = useState<DownloadState>(IDLE_STATE);

  const fetchUrls = useCallback(async (executionId: string) => {
    setState({ status: "loading", urls: [] });
    try {
      const result = await core.downloads.getDownloadUrls(executionId);
      setState({ status: "ready", urls: result.urls });
      return result;
    } catch (error) {
      setState({
        status: "failed",
        urls: [],
        error: error instanceof Error ? error.message : "Download failed",
      });
      return null;
    }
  }, []);

  const downloadFile = useCallback((file: OutputFileUrl) => {
    core.downloads.downloadFile(file.url, file.name);
  }, []);

  const downloadAll = useCallback(() => {
    for (const file of state.urls) {
      core.downloads.downloadFile(file.url, file.name);
    }
  }, [state.urls]);

  const reset = useCallback(() => setState(IDLE_STATE), []);

  return {
    ...state,
    fetchUrls,
    downloadFile,
    downloadAll,
    reset,
    isLoading: state.status === "loading",
    isReady: state.status === "ready",
  };
}
