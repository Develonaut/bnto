"use client";

import type { Definition } from "@bnto/nodes";
import type { BrowserFileResult } from "@bnto/core";
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
 * Phase 1: Only `download` mode is implemented. `display` and `preview` modes
 * render placeholder UI for forward compatibility.
 */
export function OutputRenderer({
  definition,
  results,
  onDownload,
}: OutputRendererProps) {
  const outputConfig = deriveOutputConfig(definition);

  if (outputConfig.mode === "display") {
    return (
      <div className="rounded-lg border border-dashed border-border p-8 text-center text-muted-foreground">
        Display output mode coming soon
      </div>
    );
  }

  if (outputConfig.mode === "preview") {
    return (
      <div className="rounded-lg border border-dashed border-border p-8 text-center text-muted-foreground">
        Preview output mode coming soon
      </div>
    );
  }

  // download mode (default)
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
