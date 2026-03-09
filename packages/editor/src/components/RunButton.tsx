"use client";

import { useCallback, useRef } from "react";
import { Button, PlayIcon, LoaderIcon, RotateCcwIcon } from "@bnto/ui";
import { useEditorExecution } from "../hooks/useEditorExecution";

/**
 * RunButton — run/reset button with hidden file input for selecting files.
 *
 * Phase-dependent: idle → play icon (triggers file picker), running → spinner,
 * completed/failed → reset icon (clears execution state).
 */
function RunButton() {
  const { phase, canRun, run, reset } = useEditorExecution();
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleClick = useCallback(() => {
    if (phase === "completed" || phase === "failed") {
      reset();
      return;
    }
    fileInputRef.current?.click();
  }, [phase, reset]);

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
    ) : phase === "completed" || phase === "failed" ? (
      <RotateCcwIcon className="size-4" />
    ) : (
      <PlayIcon className="size-4" />
    );

  const label =
    phase === "running"
      ? "Running"
      : phase === "completed" || phase === "failed"
        ? "Reset run"
        : "Run";

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
        disabled={!canRun && phase !== "completed" && phase !== "failed"}
        aria-label={label}
        data-testid="run-button"
      >
        {icon}
      </Button>
    </>
  );
}

export { RunButton };
