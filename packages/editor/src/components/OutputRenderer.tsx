"use client";

import type { Definition, BrowserFileResult } from "@bnto/core";
import { deriveOutputConfig } from "@bnto/core";
import { OutputFileCard } from "./OutputFileCard";

interface OutputRendererProps {
  definition: Definition;
  results: BrowserFileResult[];
  onDownload: (result: BrowserFileResult) => void;
}

/**
 * Generic output renderer — reads the output node from a recipe definition
 * and renders the appropriate result presentation.
 *
 * `write` mode renders download cards. `message` mode shows a summary.
 * `none` mode renders nothing. `overwrite` is CLI-only (no browser UI).
 */
export function OutputRenderer({ definition, results, onDownload }: OutputRendererProps) {
  const outputConfig = deriveOutputConfig(definition);

  if (outputConfig.mode === "message" || outputConfig.mode === "none") {
    return <MessageMode mode={outputConfig.mode} />;
  }

  // write mode (default) — also used for overwrite in browser context
  return <DownloadGrid results={results} onDownload={onDownload} />;
}

function MessageMode({ mode }: { mode: string }) {
  return (
    <div className="rounded-lg border border-dashed border-border p-8 text-center text-muted-foreground">
      {mode === "message" ? "Message" : "No output"} mode — no files to download
    </div>
  );
}

function DownloadGrid({
  results,
  onDownload,
}: {
  results: BrowserFileResult[];
  onDownload: (r: BrowserFileResult) => void;
}) {
  if (results.length === 0) return null;

  const gridClass =
    results.length === 1
      ? "grid grid-cols-1"
      : results.length === 2
        ? "grid grid-cols-1 gap-2 sm:grid-cols-2"
        : "grid grid-cols-1 gap-2 sm:grid-cols-2 lg:grid-cols-3";

  return (
    <div data-testid="output-renderer">
      <div className={gridClass}>
        {results.map((result, index) => (
          <OutputFileCard
            key={`${result.filename}-${index}`}
            result={result}
            onDownload={onDownload}
          />
        ))}
      </div>
    </div>
  );
}
