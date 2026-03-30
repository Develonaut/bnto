"use client";

import type { BrowserExecution, BrowserFileResult } from "@bnto/core";
import { FileCard } from "./FileCard";

interface RecipeFileGridItemProps {
  file: File;
  index: number;
  activeStep: 1 | 2 | 3;
  isBrowserPath: boolean;
  browserExec: BrowserExecution;
  onDelete: () => void;
  onDownload: (result: BrowserFileResult) => void;
}

/** Single file card in the recipe file grid with execution-aware state. */
export function RecipeFileGridItem({
  file,
  index,
  activeStep,
  isBrowserPath,
  browserExec,
  onDelete,
  onDownload,
}: RecipeFileGridItemProps) {
  const result = activeStep === 3 && isBrowserPath ? browserExec.results[index] : undefined;
  const isFileProcessing =
    activeStep === 3 &&
    isBrowserPath &&
    browserExec.status === "processing" &&
    browserExec.fileProgress?.fileIndex === index;

  return (
    <FileCard
      file={file}
      result={result}
      isProcessing={isFileProcessing}
      isExecuting={activeStep === 3}
      onDelete={onDelete}
      onDownload={onDownload}
    />
  );
}
