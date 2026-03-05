"use client";

import { cn } from "@/lib/cn";
import { Button, LoaderIcon, PlayIcon, RotateCcwIcon } from "@bnto/ui";

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
  className?: string;
}

/**
 * Primary CTA that triggers the upload → execution flow.
 *
 * Icon-only button — contextual icon based on the current execution phase.
 * Disabled when no files are selected or an operation is in progress.
 */
export function RunButton({ phase, hasFiles, onRun, className }: RunButtonProps) {
  if (phase === "completed" || phase === "failed") {
    return (
      <Button
        variant={phase === "failed" ? "outline" : "primary"}
        size="icon"
        onClick={onRun}
        className={cn(className)}
        data-testid="run-button"
        data-phase={phase}
        aria-label={phase === "failed" ? "Try again" : "Rerun"}
      >
        <RotateCcwIcon className="size-4" />
      </Button>
    );
  }

  const isProcessing = phase === "uploading" || phase === "running";

  return (
    <Button
      size="icon"
      onClick={onRun}
      disabled={!hasFiles || isProcessing}
      className={cn(className)}
      data-testid="run-button"
      data-phase={phase}
      aria-label={getLabel(phase, hasFiles)}
    >
      {isProcessing ? (
        <LoaderIcon className="size-4 motion-safe:animate-spin" />
      ) : (
        <PlayIcon className="size-4" />
      )}
    </Button>
  );
}

function getLabel(phase: RunPhase, hasFiles: boolean): string {
  switch (phase) {
    case "uploading":
      return "Uploading";
    case "running":
      return "Processing";
    default:
      return hasFiles ? "Run" : "Select files to run";
  }
}
