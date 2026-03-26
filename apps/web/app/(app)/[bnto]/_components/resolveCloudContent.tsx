import type { ReactNode } from "react";
import type { FileUploadProgress } from "@bnto/core";
import type { RunPhase } from "./RunButton";
import { ExecutionProgress } from "./ExecutionProgress";
import { ExecutionResults } from "./ExecutionResults";
import { UploadProgress } from "./UploadProgress";
import { ErrorCard } from "./ErrorCard";

const EXEC_PHASES = new Set<RunPhase>(["running", "failed"]);

/** Resolve which content to show for a cloud execution phase. */
export function resolveCloudContent(
  phase: RunPhase,
  executionId: string | null,
  uploadFiles: FileUploadProgress[],
  clientError: string | null,
): ReactNode | null {
  if (uploadFiles.length > 0 && phase === "uploading") {
    return <UploadProgress files={uploadFiles} />;
  }
  if (executionId && EXEC_PHASES.has(phase)) {
    return <ExecutionProgress executionId={executionId} />;
  }
  if (executionId && phase === "completed") {
    return <ExecutionResults executionId={executionId} />;
  }
  if (!executionId && phase === "failed" && clientError) {
    return <ErrorCard error={clientError} />;
  }
  return null;
}
