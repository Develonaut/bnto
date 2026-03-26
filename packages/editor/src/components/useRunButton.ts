"use client";

import { useCallback, useRef } from "react";
import { useExecution } from "../hooks/useExecution";

/** Derive the button label from current phase and file state. */
function deriveLabel(phase: string, isDone: boolean, hasFiles: boolean): string {
  if (phase === "running") return "Running";
  if (isDone && hasFiles) return "Run again";
  return "Run";
}

/** Hook encapsulating RunButton state + file-input interaction. */
function useRunButton() {
  const { phase, canRun, inputFiles, fileAccept, run } = useExecution();
  const fileInputRef = useRef<HTMLInputElement>(null);
  const hasFiles = inputFiles.length > 0;
  const isDone = phase === "completed" || phase === "failed";
  const handleClick = useCallback(() => {
    if (hasFiles) { run(inputFiles); return; }
    fileInputRef.current?.click();
  }, [hasFiles, inputFiles, run]);
  const handleFileChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files;
    if (!files || files.length === 0) return;
    run(Array.from(files));
    e.target.value = "";
  }, [run]);
  return { phase, canRun, fileAccept, isDone, hasFiles, label: deriveLabel(phase, isDone, hasFiles), fileInputRef, handleClick, handleFileChange };
}

export { useRunButton };
