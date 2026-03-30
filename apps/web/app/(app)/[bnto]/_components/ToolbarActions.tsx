"use client";

import { Button, Row, DownloadIcon } from "@bnto/ui";
import type { RunPhase } from "./RunButton";

interface ToolbarActionsProps {
  activePhase: 2 | 3;
  resolvedPhase: RunPhase;
  fileCount: number;
  onRun: () => void;
  onDownloadAll: () => void;
  className?: string;
}

/** Action buttons for the recipe toolbar. */
export function ToolbarActions({
  activePhase,
  resolvedPhase,
  onDownloadAll,
  className,
}: ToolbarActionsProps) {
  if (activePhase !== 3) return null;

  return (
    <Row gap="xs" className={className}>
      <Button
        variant="outline"
        size="icon"
        elevation="sm"
        disabled={resolvedPhase !== "completed"}
        onClick={onDownloadAll}
        aria-label="Download all"
        data-testid="download-all-button"
      >
        <DownloadIcon className="size-4" />
      </Button>
    </Row>
  );
}
