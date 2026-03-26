"use client";

import { Button, PlayIcon, LoaderIcon } from "@bnto/ui";
import { useRunButton } from "./useRunButton";

/**
 * RunButton — run/rerun button with hidden file input for selecting files.
 *
 * Phase-dependent:
 * - idle (no files staged) -> play icon -> opens file picker -> runs
 * - idle (files staged via RunTab) -> play icon -> runs staged files
 * - running -> spinner (disabled)
 * - completed/failed -> play icon -> reruns with same files
 *
 * Full reset (clear files + results) is handled by the toolbar reset button
 * near undo/redo -- not duplicated here.
 */
function RunButton() {
  const { phase, canRun, fileAccept, isDone, label, fileInputRef, handleClick, handleFileChange } =
    useRunButton();

  return (
    <>
      <RunFileInput ref={fileInputRef} accept={fileAccept} onChange={handleFileChange} />
      <Button
        size="icon"
        variant={phase === "failed" ? "destructive" : "primary"}
        elevation="sm"
        onClick={handleClick}
        disabled={!canRun && !isDone}
        aria-label={label}
        aria-busy={phase === "running"}
        data-testid="run-button"
        data-phase={phase}
      >
        {phase === "running" ? (
          <LoaderIcon className="size-4 motion-safe:animate-spin" />
        ) : (
          <PlayIcon className="size-4" />
        )}
      </Button>
    </>
  );
}

import { forwardRef } from "react";

interface RunFileInputProps {
  accept: string | undefined;
  onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
}

const RunFileInput = forwardRef<HTMLInputElement, RunFileInputProps>(function RunFileInput(
  { accept, onChange },
  ref,
) {
  return (
    <input
      ref={ref}
      type="file"
      multiple
      accept={accept}
      aria-label="Select files to process"
      className="hidden"
      onChange={onChange}
      data-testid="run-file-input"
    />
  );
});

export { RunButton };
