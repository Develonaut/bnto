"use client";

import { useCallback, useRef } from "react";
import { Button, PlayIcon, LoaderIcon } from "@bnto/ui";
import { useExecution } from "../hooks/useExecution";

/**
 * RunButton — run/rerun button with hidden file input for selecting files.
 *
 * Phase-dependent:
 * - idle (no files) → play icon → opens file picker → runs
 * - running → spinner (disabled)
 * - completed/failed → play icon → reruns with same files (no re-upload)
 *
 * Full reset (clear files + results) is handled by the toolbar reset button
 * near undo/redo — not duplicated here.
 */
function RunButton() {
  const { phase, canRun, inputFiles, run } = useExecution();
  const fileInputRef = useRef<HTMLInputElement>(null);

  const hasFiles = inputFiles.length > 0;
  const isDone = phase === "completed" || phase === "failed";

  const handleClick = useCallback(() => {
    if (isDone && hasFiles) {
      run(inputFiles);
      return;
    }
    fileInputRef.current?.click();
  }, [isDone, hasFiles, inputFiles, run]);

  const handleFileChange = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      const files = e.target.files;
      if (!files || files.length === 0) return;
      run(Array.from(files));
      e.target.value = "";
    },
    [run],
  );

  const icon =
    phase === "running" ? (
      <LoaderIcon className="size-4 motion-safe:animate-spin" />
    ) : (
      <PlayIcon className="size-4" />
    );

  const label = phase === "running" ? "Running" : isDone && hasFiles ? "Run again" : "Run";

  return (
    <>
      <input
        ref={fileInputRef}
        type="file"
        multiple
        className="hidden"
        onChange={handleFileChange}
        data-testid="run-file-input"
      />
      <Button
        size="icon"
        variant={phase === "failed" ? "destructive" : "primary"}
        elevation="sm"
        onClick={handleClick}
        disabled={!canRun && !isDone}
        aria-label={label}
        data-testid="run-button"
        data-phase={phase}
      >
        {icon}
      </Button>
    </>
  );
}

export { RunButton };
