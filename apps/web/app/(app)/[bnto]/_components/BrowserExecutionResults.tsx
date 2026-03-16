"use client";

import { Button, DownloadIcon, ResultFileCard } from "@bnto/ui";
import type { BrowserExecution, BrowserFileResult } from "@bnto/core";
import { useFileResultProps } from "@bnto/core";

interface BrowserExecutionResultsProps {
  execution: BrowserExecution;
  onDownload: (result: BrowserFileResult) => void;
}

/** Displays browser execution output files with TinyPNG-style stats. */
export function BrowserExecutionResults({ execution, onDownload }: BrowserExecutionResultsProps) {
  const { results } = execution;
  if (results.length === 0) return null;

  return (
    <div
      className={
        results.length === 1
          ? "grid grid-cols-1"
          : results.length === 2
            ? "grid grid-cols-1 gap-2 sm:grid-cols-2"
            : "grid grid-cols-1 gap-2 sm:grid-cols-2 lg:grid-cols-3"
      }
      data-testid="browser-execution-results"
    >
      {results.map((result, index) => (
        <BrowserFileRow
          key={`${result.filename}-${index}`}
          result={result}
          onDownload={onDownload}
        />
      ))}
    </div>
  );
}

function BrowserFileRow({
  result,
  onDownload,
}: {
  result: BrowserFileResult;
  onDownload: (result: BrowserFileResult) => void;
}) {
  const props = useFileResultProps(result);

  return (
    <ResultFileCard
      filename={props.filename}
      extension={props.extension}
      outputSize={props.outputSize}
      originalSize={props.originalSize}
      savings={props.savings}
      action={
        <Button
          variant="outline"
          size="icon"
          elevation="sm"
          onClick={() => onDownload(result)}
          aria-label={`Download ${result.filename}`}
          data-testid="download-button"
        >
          <DownloadIcon className="size-4" />
        </Button>
      }
    />
  );
}
