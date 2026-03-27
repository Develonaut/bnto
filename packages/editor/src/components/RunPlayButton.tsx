"use client";

import { Button, LoaderIcon, PlayIcon, RotateCcwIcon } from "@bnto/ui";
import { useExecution } from "../hooks/useExecution";

interface RunPlayButtonProps {
  onClick: () => void;
}

/**
 * RunPlayButton — shared run/rerun button used by toolbar and run panel.
 *
 * Reads `canRun` and `phase` from the execution store so both consumers
 * share identical disabled logic. Accepts only an `onClick` — the caller
 * decides what "run" means (toolbar opens a file picker first; the run
 * panel runs staged files directly).
 */
function RunPlayButton({ onClick }: RunPlayButtonProps) {
  const { phase, canRun } = useExecution();
  const isDone = phase === "completed" || phase === "failed";
  const isProcessing = phase === "running";

  if (isDone) {
    return (
      <Button
        variant={phase === "failed" ? "outline" : "primary"}
        size="icon"
        elevation="sm"
        onClick={onClick}
        aria-label={phase === "failed" ? "Try again" : "Rerun"}
        data-testid="run-button"
        data-phase={phase}
      >
        <RotateCcwIcon className="size-4" />
      </Button>
    );
  }

  return (
    <Button
      size="icon"
      elevation="sm"
      variant="primary"
      onClick={onClick}
      disabled={!canRun || isProcessing}
      aria-label={isProcessing ? "Running" : "Run"}
      aria-busy={isProcessing}
      data-testid="run-button"
      data-phase={phase}
    >
      {isProcessing ? (
        <LoaderIcon className="size-4 motion-safe:animate-spin" />
      ) : (
        <PlayIcon className="size-4" />
      )}
    </Button>
  );
}

export { RunPlayButton };
