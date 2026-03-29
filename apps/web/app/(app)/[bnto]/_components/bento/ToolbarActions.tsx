"use client";

import { Button, Row, DownloadIcon, RotateCcwIcon } from "@bnto/ui";
import type { ToolbarCardProps } from "./ToolbarCard";
import { RunButton } from "./RunButton";

/** Run/Reset/Download action buttons for the toolbar. */
export function ToolbarActions({
  phase,
  hasFiles,
  resultCount,
  onRun,
  onReset,
  onDownloadAll,
}: ToolbarCardProps) {
  const isDone = phase === "completed" || phase === "failed";

  return (
    <Row className="flex-wrap gap-2">
      {isDone ? (
        <Button size="sm" variant="outline" onClick={onReset}>
          <RotateCcwIcon className="size-3.5" />
          Reset
        </Button>
      ) : (
        <RunButton phase={phase} hasFiles={hasFiles} onRun={onRun} />
      )}
      {phase === "completed" && resultCount > 0 && (
        <Button size="sm" variant="outline" onClick={onDownloadAll}>
          <DownloadIcon className="size-3.5" />
          Download All
        </Button>
      )}
    </Row>
  );
}
