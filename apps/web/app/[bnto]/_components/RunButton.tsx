"use client";

import { LoaderIcon, PlayIcon, RotateCcwIcon } from "@/components/ui/icons";
import { Button } from "@/components/ui/button";

export type RunPhase =
  | "idle"
  | "uploading"
  | "running"
  | "completed"
  | "failed";

interface RunButtonProps {
  phase: RunPhase;
  hasFiles: boolean;
  onRun: () => void;
  onReset: () => void;
}

/**
 * Primary CTA that triggers the upload → execution flow.
 *
 * Displays contextual label and icon based on the current execution phase.
 * Disabled when no files are selected or an operation is in progress.
 */
export function RunButton({ phase, hasFiles, onRun, onReset }: RunButtonProps) {
  if (phase === "completed" || phase === "failed") {
    return (
      <Button
        size="lg"
        variant={phase === "failed" ? "outline" : "primary"}
        onClick={onReset}
        className="w-full"
        data-testid="run-button"
        data-phase={phase}
      >
        <RotateCcwIcon className="size-4" />
        {phase === "failed" ? "Try Again" : "Run Again"}
      </Button>
    );
  }

  const isProcessing = phase === "uploading" || phase === "running";

  return (
    <Button
      size="lg"
      onClick={onRun}
      disabled={!hasFiles || isProcessing}
      className="w-full"
      data-testid="run-button"
      data-phase={phase}
    >
      {isProcessing ? (
        <LoaderIcon className="size-4 motion-safe:animate-spin" />
      ) : (
        <PlayIcon className="size-4" />
      )}
      {getLabel(phase, hasFiles)}
    </Button>
  );
}

function getLabel(phase: RunPhase, hasFiles: boolean): string {
  switch (phase) {
    case "uploading":
      return "Uploading\u2026";
    case "running":
      return "Processing\u2026";
    default:
      return hasFiles ? "Run" : "Select files to run";
  }
}
