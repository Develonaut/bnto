"use client";

import type { BrowserExecution, BrowserFileResult } from "@bnto/core";
import { FileCard } from "./FileCard";

interface RecipeFileGridItemProps {
  file: File;
  index: number;
  activeStep: 1 | 2 | 3;
  browserExec: BrowserExecution;
  onDelete: () => void;
  onDownload: (result: BrowserFileResult) => void;
}

/** Single file card in the recipe file grid with execution-aware state. */
export function RecipeFileGridItem({
  file,
  index,
  activeStep,
  browserExec,
  onDelete,
  onDownload,
}: RecipeFileGridItemProps) {
  const result = activeStep === 3 ? browserExec.results[index] : undefined;
  const isFileProcessing =
    activeStep === 3 &&
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
