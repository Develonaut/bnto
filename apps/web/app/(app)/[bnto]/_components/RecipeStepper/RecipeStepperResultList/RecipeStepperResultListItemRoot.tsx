"use client";

import type { BrowserFileResult } from "@bnto/core";
import { CompletedRow } from "./CompletedRow";
import { PendingRow } from "./PendingRow";

interface RecipeStepperResultListItemProps {
  file: File;
  result?: BrowserFileResult;
  isProcessing: boolean;
  isExecuting: boolean;
  onDelete: () => void;
  onDownload: (result: BrowserFileResult) => void;
}

/** Single file card — branches between completed and pending states. */
export function RecipeStepperResultListItem({
  file,
  result,
  isProcessing,
  isExecuting,
  onDelete,
  onDownload,
}: RecipeStepperResultListItemProps) {
  if (result) {
    return <CompletedRow result={result} onDownload={onDownload} />;
  }
  return (
    <PendingRow
      file={file}
      isProcessing={isProcessing}
      isExecuting={isExecuting}
      onDelete={onDelete}
    />
  );
}
