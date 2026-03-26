import type { BrowserExecution, BrowserFileResult } from "@bnto/core";
import { SlideUp } from "@bnto/ui";
import { BrowserExecutionProgress } from "./BrowserExecutionProgress";
import { BrowserExecutionResults } from "./BrowserExecutionResults";
import { ErrorCard } from "./ErrorCard";

interface BrowserResultsBlockProps {
  browserExec: BrowserExecution;
  onDownload: (result: BrowserFileResult) => void;
}

/**
 * Browser execution results block.
 *
 * Renders the appropriate UI based on browser execution status:
 * processing -> progress bar, completed -> results, failed -> error.
 */
export function BrowserResultsBlock({ browserExec, onDownload }: BrowserResultsBlockProps) {
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
