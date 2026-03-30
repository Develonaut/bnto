"use client";

import { Row } from "@bnto/ui";
import type { RunPhase } from "./RunButton";
import { ToolbarActions } from "./ToolbarActions";

interface ToolbarBackSectionProps {
  activePhase: 2 | 3;
  resolvedPhase: RunPhase;
  fileCount: number;
  onRun: () => void;
  onDownloadAll: () => void;
}

/** Left section of the recipe toolbar — file count + actions. */
export function ToolbarBackSection({
  activePhase,
  resolvedPhase,
  fileCount,
  onRun,
  onDownloadAll,
}: ToolbarBackSectionProps) {
  return (
    <Row gap="xs">
      <p className="shrink-0 text-sm font-medium text-foreground" data-testid="file-count">
        {fileCount} {fileCount === 1 ? "file" : "files"} selected
      </p>
      <ToolbarActions
        activePhase={activePhase}
        resolvedPhase={resolvedPhase}
        fileCount={fileCount}
        onRun={onRun}
        onDownloadAll={onDownloadAll}
        className="ml-auto shrink-0"
      />
    </Row>
  );
}
