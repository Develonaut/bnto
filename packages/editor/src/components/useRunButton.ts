"use client";

import { useCallback, useRef } from "react";
import { useExecution } from "../hooks/useExecution";

/** Hook encapsulating RunButton state + file-input interaction. */
function useRunButton() {
  const { phase, canRun, inputFiles, fileAccept, run } = useExecution();
  const fileInputRef = useRef<HTMLInputElement>(null);

  const hasFiles = inputFiles.length > 0;
  const isDone = phase === "completed" || phase === "failed";

  const handleClick = useCallback(() => {
    if (hasFiles) {
      run(inputFiles);
      return;
    }
    fileInputRef.current?.click();
  }, [hasFiles, inputFiles, run]);

  const handleFileChange = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      const files = e.target.files;
      if (!files || files.length === 0) return;
      run(Array.from(files));
      e.target.value = "";
    },
    [run],
  );

  const label = phase === "running" ? "Running" : isDone && hasFiles ? "Run again" : "Run";

  return {
    phase,
    canRun,
    fileAccept,
    isDone,
    hasFiles,
    label,
    fileInputRef,
    handleClick,
    handleFileChange,
  };
}

export { useRunButton };
