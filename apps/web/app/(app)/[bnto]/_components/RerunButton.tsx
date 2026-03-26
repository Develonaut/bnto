"use client";

import { cn, Button, RotateCcwIcon } from "@bnto/ui";
import type { RunPhase } from "./RunButton";

interface RerunButtonProps {
  phase: Extract<RunPhase, "completed" | "failed">;
  onRun: () => void;
  className?: string;
}

/** Retry/rerun button shown after execution completes or fails. */
export function RerunButton({ phase, onRun, className }: RerunButtonProps) {
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
