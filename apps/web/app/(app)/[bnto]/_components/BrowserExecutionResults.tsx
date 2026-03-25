"use client";

import { FileList } from "@bnto/ui";
import type { BrowserExecution, BrowserFileResult } from "@bnto/core";
import { CompletedFileRow } from "./CompletedFileRow";

interface BrowserExecutionResultsProps {
  execution: BrowserExecution;
  onDownload: (result: BrowserFileResult) => void;
}

/** Displays browser execution output files with TinyPNG-style stats. */
export function BrowserExecutionResults({ execution, onDownload }: BrowserExecutionResultsProps) {
  const { results } = execution;
  if (results.length === 0) return null;

  return (
    <FileList
      className={
        results.length > 2
          ? "sm:grid-cols-2 lg:grid-cols-3"
          : results.length === 2
            ? "sm:grid-cols-2"
            : undefined
      }
      data-testid="browser-execution-results"
    >
      {results.map((result, index) => (
        <CompletedFileRow
          key={`${result.filename}-${index}`}
          result={result}
          onDownload={onDownload}
        />
      ))}
    </FileList>
  );
}
