import type { FileUploadProgress } from "@bnto/core";
import type { RunPhase } from "./RunButton";
import { SlideUp } from "@bnto/ui";
import { resolveCloudContent } from "./resolveCloudContent";

interface CloudResultsBlockProps {
  resolvedPhase: RunPhase;
  executionId: string | null;
  uploadProgress: { files: FileUploadProgress[] };
  clientError: string | null;
}

/** Cloud execution results block — wraps phase content in a SlideUp. */
export function CloudResultsBlock({
  resolvedPhase,
  executionId,
  uploadProgress,
  clientError,
}: CloudResultsBlockProps) {
  const content = resolveCloudContent(
    resolvedPhase,
    executionId,
    uploadProgress.files,
    clientError,
  );
  if (!content) return null;
  return <SlideUp>{content}</SlideUp>;
}
