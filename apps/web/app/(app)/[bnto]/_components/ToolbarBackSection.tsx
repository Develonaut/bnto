"use client";

import { Button, Row, ArrowLeftIcon } from "@bnto/ui";
import type { RunPhase } from "./RunButton";
import { ToolbarActions } from "./ToolbarActions";

interface ToolbarBackSectionProps {
  activePhase: 2 | 3;
  resolvedPhase: RunPhase;
  isProcessing: boolean;
  fileCount: number;
  onBack: () => void;
  onRun: () => void;
  onDownloadAll: () => void;
}

/** Left section of the recipe toolbar — back button + file count + mobile actions. */
export function ToolbarBackSection({
  activePhase,
  resolvedPhase,
  isProcessing,
  fileCount,
  onBack,
  onRun,
  onDownloadAll,
}: ToolbarBackSectionProps) {
  const backLabel = activePhase === 3 ? "Back to configure" : "Back to file selection";
  return (
    <Row gap="xs">
      <Button
        variant="ghost"
        size="icon"
        elevation="sm"
        disabled={isProcessing}
        onClick={onBack}
        aria-label={backLabel}
        data-testid="back-button"
      >
        <ArrowLeftIcon className="size-4" />
      </Button>
      <p className="shrink-0 text-sm font-medium text-foreground" data-testid="file-count">
        {fileCount} {fileCount === 1 ? "file" : "files"} selected
      </p>
      <ToolbarActions
        activePhase={activePhase}
        resolvedPhase={resolvedPhase}
        fileCount={fileCount}
        onRun={onRun}
        onDownloadAll={onDownloadAll}
        className="ml-auto shrink-0 md:hidden"
      />
    </Row>
  );
}
