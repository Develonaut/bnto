"use client";

import type { Definition } from "@bnto/nodes";
import type { BrowserFileResult } from "@bnto/core";
import { deriveOutputConfig } from "@bnto/core";
import { CheckCircle2Icon, DownloadIcon } from "@/components/ui/icons";
import { Button } from "@/components/ui/Button";
import { Card } from "@/components/ui/Card";
import { IconBadge } from "@/components/ui/IconBadge";
import { Row } from "@/components/ui/Row";
import { Stack } from "@/components/ui/Stack";
import { formatFileSize } from "@/src/utils/formatFileSize";

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

function OutputFileCard({
  result,
  onDownload,
}: {
  result: BrowserFileResult;
  onDownload: (result: BrowserFileResult) => void;
}) {
  // Trust boundary: metadata is Record<string, unknown> from WASM engine
  const originalSize = result.metadata.originalSize as number | undefined;
  const ratio = result.metadata.ratio as number | undefined;

  return (
    <Card
      className="flex items-center gap-3 rounded-lg px-4 py-3"
      elevation="sm"
      data-testid="output-file"
    >
      <Row className="min-w-0 flex-1 gap-3">
        <IconBadge variant="primary" size="lg">
          <CheckCircle2Icon className="size-5" />
        </IconBadge>
        <Stack className="min-w-0 flex-1 gap-0">
          <span className="truncate text-sm font-semibold">
            {result.filename}
          </span>
          <span className="truncate text-xs text-muted-foreground">
            {formatFileSize(result.blob.size)}
            {originalSize != null && ratio != null && (
              <span>
                {" "}
                &middot; {Math.round((1 - ratio) * 100)}% smaller
              </span>
            )}
          </span>
        </Stack>
      </Row>

      <Button
        variant="outline"
        size="icon"
        elevation="sm"
        onClick={() => onDownload(result)}
        aria-label={`Download ${result.filename}`}
      >
        <DownloadIcon className="size-4" />
      </Button>
    </Card>
  );
}
