"use client";

import { Button, PlayIcon, LoaderIcon } from "@bnto/ui";
import type { BentoFlowPhase } from "../../_hooks/useBentoRecipeFlow";

interface RunButtonProps {
  phase: BentoFlowPhase;
  hasFiles: boolean;
  onRun: () => void;
}

/** Run/Processing button — shows spinner when running. */
export function RunButton({ phase, hasFiles, onRun }: RunButtonProps) {
  const isRunning = phase === "running";
  const icon = isRunning ? (
    <LoaderIcon className="size-3.5 motion-safe:animate-spin" />
  ) : (
    <PlayIcon className="size-3.5" />
  );

  return (
    <Button
      size="sm"
      onClick={onRun}
      disabled={!hasFiles || isRunning}
      data-testid="bento-run-button"
    >
      {icon}
      {isRunning ? "Processing" : "Run"}
    </Button>
  );
}
