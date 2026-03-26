"use client";

import { useCallback } from "react";
import type { BrowserFileResult } from "@bnto/core";
import { useEditor } from "../../context";

function useRunHandlers(inputFiles: File[], phase: string) {
  const editor = useEditor();

  const setFiles = useCallback((files: File[]) => editor.execution.setInputFiles(files), [editor]);

  const removeFile = useCallback(
    (index: number) => setFiles(inputFiles.filter((_, i) => i !== index)),
    [inputFiles, setFiles],
  );

  const handleBack = useCallback(() => {
    if (phase === "idle") {
      setFiles([]);
      return;
    }
    const files = inputFiles;
    editor.execution.resetRun();
    editor.execution.setInputFiles(files);
  }, [phase, inputFiles, editor, setFiles]);

  return { setFiles, removeFile, handleBack };
}

function useRunDownloads() {
  const editor = useEditor();

  const handleClear = useCallback(() => editor.execution.setInputFiles([]), [editor]);

  const handleDownloadAll = useCallback(() => editor.execution.downloadAllResults(), [editor]);

  const handleDownload = useCallback(
    (r: BrowserFileResult) => editor.execution.downloadResult(r),
    [editor],
  );

  return { handleClear, handleDownloadAll, handleDownload };
}

export { useRunHandlers, useRunDownloads };
