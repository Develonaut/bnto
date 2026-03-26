import type { BrowserExecution, BrowserFileResult, FileUploadProgress } from "@bnto/core";
import type { RunPhase } from "./RunButton";
import { BrowserResultsBlock } from "./BrowserResultsBlock";
import { CloudResultsBlock } from "./CloudResultsBlock";

interface RecipeResultsSectionProps {
  isBrowserPath: boolean;
  resolvedPhase: RunPhase;
  browserExec: BrowserExecution;
  onDownload: (result: BrowserFileResult) => void;
  executionId: string | null;
  uploadProgress: { files: FileUploadProgress[] };
  clientError: string | null;
}

/**
 * Unified results section for recipe pages.
 *
 * Delegates to BrowserResultsBlock or CloudResultsBlock
 * based on the execution path.
 */
export function RecipeResultsSection({
  isBrowserPath,
  resolvedPhase,
  browserExec,
  onDownload,
  executionId,
  uploadProgress,
  clientError,
}: RecipeResultsSectionProps) {
  if (isBrowserPath) {
    return <BrowserResultsBlock browserExec={browserExec} onDownload={onDownload} />;
  }

  return (
    <CloudResultsBlock
      resolvedPhase={resolvedPhase}
      executionId={executionId}
      uploadProgress={uploadProgress}
      clientError={clientError}
    />
  );
}
