"use client";

import { useCallback, useMemo } from "react";
import { toDropzoneAccept } from "@bnto/ui";
import { useEditor } from "../../context";
import { useRunHandlers, useRunDownloads } from "./useRunHandlers";

function deriveStep(phase: string, fileCount: number) {
  if (phase === "idle") return fileCount === 0 ? "dropzone" : "staging";
  return phase;
}

function deriveAccept(fileAccept: string | undefined) {
  const label = fileAccept && fileAccept !== "*/*" ? fileAccept : "all files";
  const dropzone = fileAccept ? toDropzoneAccept(fileAccept) : undefined;
  return { acceptLabel: label, dropzoneAccept: dropzone };
}

function useRunTab() {
  const editor = useEditor();
  const { inputFiles, fileAccept, phase, results, fileProgress, errors } =
    editor.execution.useExecution();

  const { setFiles, removeFile, handleBack } = useRunHandlers(inputFiles, phase);
  const { handleClear, handleDownloadAll, handleDownload } = useRunDownloads();
  const handleRun = useCallback(
    () => editor.execution.runExecution(inputFiles),
    [editor, inputFiles],
  );
  const { acceptLabel, dropzoneAccept } = useMemo(() => deriveAccept(fileAccept), [fileAccept]);

  return {
    inputFiles,
    phase,
    results,
    fileProgress,
    errors,
    setFiles,
    removeFile,
    handleBack,
    handleRun,
    handleClear,
    handleDownloadAll,
    handleDownload,
    acceptLabel,
    dropzoneAccept,
    step: deriveStep(phase, inputFiles.length),
  };
}

export { useRunTab };
