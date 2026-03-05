import type { BrowserExecution, BrowserFileResult, FileUploadProgress } from "@bnto/core";
import type { RunPhase } from "./RunButton";
import { SlideUp } from "@bnto/ui";
import { BrowserExecutionProgress } from "./BrowserExecutionProgress";
import { BrowserExecutionResults } from "./BrowserExecutionResults";
import { ExecutionProgress } from "./ExecutionProgress";
import { ExecutionResults } from "./ExecutionResults";
import { UploadProgress } from "./UploadProgress";
import { ErrorCard } from "./ErrorCard";

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
 * Consolidates browser progress, browser results, cloud upload,
 * cloud progress, cloud results, and error states into a single
 * composition component. Each block slides up on entrance.
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
  // --- Browser path ---
  if (isBrowserPath) {
    if (browserExec.status === "processing") {
      return (
        <SlideUp>
          <BrowserExecutionProgress execution={browserExec} />
        </SlideUp>
      );
    }
    if (browserExec.status === "completed") {
      return (
        <SlideUp>
          <BrowserExecutionResults execution={browserExec} onDownload={onDownload} />
        </SlideUp>
      );
    }
    if (browserExec.status === "failed" && browserExec.error) {
      return (
        <SlideUp>
          <ErrorCard error={browserExec.error} />
        </SlideUp>
      );
    }
    return null;
  }

  // --- Cloud path ---
  if (uploadProgress.files.length > 0 && resolvedPhase === "uploading") {
    return (
      <SlideUp>
        <UploadProgress files={uploadProgress.files} />
      </SlideUp>
    );
  }
  if (executionId && resolvedPhase === "running") {
    return (
      <SlideUp>
        <ExecutionProgress executionId={executionId} />
      </SlideUp>
    );
  }
  if (executionId && resolvedPhase === "completed") {
    return (
      <SlideUp>
        <ExecutionResults executionId={executionId} />
      </SlideUp>
    );
  }
  if (executionId && resolvedPhase === "failed") {
    return (
      <SlideUp>
        <ExecutionProgress executionId={executionId} />
      </SlideUp>
    );
  }
  if (!executionId && resolvedPhase === "failed" && clientError) {
    return (
      <SlideUp>
        <ErrorCard error={clientError} />
      </SlideUp>
    );
  }

  return null;
}
