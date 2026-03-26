"use client";

import { Button, FileUploadClear, Row, DownloadIcon, TrashIcon } from "@bnto/ui";
import { RunButton } from "./RunButton";
import type { RunPhase } from "./RunButton";

interface ToolbarActionsProps {
  activePhase: 2 | 3;
  resolvedPhase: RunPhase;
  fileCount: number;
  onRun: () => void;
  onDownloadAll: () => void;
  className?: string;
}

/** Shared action buttons for the recipe toolbar (mobile + desktop). */
export function ToolbarActions({
  activePhase,
  resolvedPhase,
  fileCount,
  onRun,
  onDownloadAll,
  className,
}: ToolbarActionsProps) {
  return (
    <Row gap="xs" className={className}>
      {activePhase === 3 && (
        <DownloadButton disabled={resolvedPhase !== "completed"} onClick={onDownloadAll} />
      )}
      {activePhase === 2 && <ClearFilesButton />}
      <RunButton phase={resolvedPhase} hasFiles={fileCount > 0} onRun={onRun} />
    </Row>
  );
}

function DownloadButton({ disabled, onClick }: { disabled: boolean; onClick: () => void }) {
  return (
    <Button
      variant="outline"
      size="icon"
      elevation="sm"
      disabled={disabled}
      onClick={onClick}
      aria-label="Download all"
      data-testid="download-all-button"
    >
      <DownloadIcon className="size-4" />
    </Button>
  );
}

function ClearFilesButton() {
  return (
    <FileUploadClear asChild>
      <Button variant="outline" size="icon" aria-label="Clear all files">
        <TrashIcon className="size-4" />
      </Button>
    </FileUploadClear>
  );
}
