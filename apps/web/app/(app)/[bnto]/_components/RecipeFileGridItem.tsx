"use client";

import type { BrowserExecution, BrowserFileResult } from "@bnto/core";
import { FileCard } from "./FileCard";

interface RecipeFileGridItemProps {
  file: File;
  index: number;
  activePhase: 1 | 2 | 3;
  isBrowserPath: boolean;
  browserExec: BrowserExecution;
  onDelete: () => void;
  onDownload: (result: BrowserFileResult) => void;
}

/** Single file card in the recipe file grid with execution-aware state. */
export function RecipeFileGridItem({
  file,
  index,
  activePhase,
  isBrowserPath,
  browserExec,
  onDelete,
  onDownload,
}: RecipeFileGridItemProps) {
  const result = activePhase === 3 && isBrowserPath ? browserExec.results[index] : undefined;
  const isFileProcessing =
    activePhase === 3 &&
    isBrowserPath &&
    browserExec.status === "processing" &&
    browserExec.fileProgress?.fileIndex === index;

  return (
    <FileCard
      file={file}
      result={result}
      isProcessing={isFileProcessing}
      isExecuting={activePhase === 3}
      onDelete={onDelete}
      onDownload={onDownload}
    />
  );
}
