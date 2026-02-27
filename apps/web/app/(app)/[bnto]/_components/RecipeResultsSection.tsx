import type { BrowserExecution, BrowserFileResult, FileUploadProgress } from "@bnto/core";
import type { RunPhase } from "./RunButton";
import { Animate } from "@/components/ui/Animate";
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
  onDownloadAll: () => void;
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
  onDownloadAll,
  executionId,
  uploadProgress,
  clientError,
}: RecipeResultsSectionProps) {
  // --- Browser path ---
  if (isBrowserPath) {
    if (browserExec.status === "processing") {
      return (
        <Animate.SlideUp>
          <BrowserExecutionProgress execution={browserExec} />
        </Animate.SlideUp>
      );
    }
    if (browserExec.status === "completed") {
      return (
        <Animate.SlideUp>
          <BrowserExecutionResults
            execution={browserExec}
            onDownload={onDownload}
            onDownloadAll={onDownloadAll}
          />
        </Animate.SlideUp>
      );
    }
    if (browserExec.status === "failed" && browserExec.error) {
      return (
        <Animate.SlideUp>
          <ErrorCard error={browserExec.error} />
        </Animate.SlideUp>
      );
    }
    return null;
  }

  // --- Cloud path ---
  if (uploadProgress.files.length > 0 && resolvedPhase === "uploading") {
    return (
      <Animate.SlideUp>
        <UploadProgress files={uploadProgress.files} />
      </Animate.SlideUp>
    );
  }
  if (executionId && resolvedPhase === "running") {
    return (
      <Animate.SlideUp>
        <ExecutionProgress executionId={executionId} />
      </Animate.SlideUp>
    );
  }
  if (executionId && resolvedPhase === "completed") {
    return (
      <Animate.SlideUp>
        <ExecutionResults executionId={executionId} />
      </Animate.SlideUp>
    );
  }
  if (executionId && resolvedPhase === "failed") {
    return (
      <Animate.SlideUp>
        <ExecutionProgress executionId={executionId} />
      </Animate.SlideUp>
    );
  }
  if (!executionId && resolvedPhase === "failed" && clientError) {
    return (
      <Animate.SlideUp>
        <ErrorCard error={clientError} />
      </Animate.SlideUp>
    );
  }

  return null;
}
