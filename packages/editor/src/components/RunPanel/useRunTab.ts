"use client";

import { useMemo } from "react";
import { toDropzoneAccept } from "@bnto/ui";
import { useEditor } from "../../context";
import { useRunHandlers, useRunDownloads } from "./useRunHandlers";

function useRunTab() {
  const editor = useEditor();
  const { inputFiles, fileAccept, phase, results, fileProgress, errors } =
    editor.execution.useExecution();

  const { setFiles, removeFile, handleBack } = useRunHandlers(inputFiles, phase);
  const { handleClear, handleDownloadAll, handleDownload } = useRunDownloads();

  const acceptLabel = fileAccept && fileAccept !== "*/*" ? fileAccept : "all files";
  const dropzoneAccept = useMemo(
    () => (fileAccept ? toDropzoneAccept(fileAccept) : undefined),
    [fileAccept],
  );
  const showDropzone = phase === "idle" && inputFiles.length === 0;

  return {
    inputFiles,
    phase,
    results,
    fileProgress,
    errors,
    setFiles,
    removeFile,
    handleBack,
    handleClear,
    handleDownloadAll,
    handleDownload,
    acceptLabel,
    dropzoneAccept,
    showDropzone,
  };
}

export { useRunTab };
