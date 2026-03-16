"use client";

import type { BrowserFileResult } from "@bnto/core";
import { useFileResultProps } from "@bnto/core";
import { Button, Card, IconBadge, ResultFileCard, Row, Stack } from "@bnto/ui";
import { CheckCircle2Icon, DownloadIcon, FileIcon, LoaderIcon, XIcon } from "@bnto/ui";
import { formatFileSize } from "@bnto/ui";

interface FileCardProps {
  file: File;
  /** Completed result for this file (undefined if not yet processed). */
  result?: BrowserFileResult;
  /** Whether this file is currently being processed by the engine. */
  isProcessing: boolean;
  /** Whether the execution phase is active (Phase 3). */
  isExecuting: boolean;
  /** Delete handler for removing files in Phase 2. */
  onDelete: () => void;
  /** Download handler for completed results. */
  onDownload: (result: BrowserFileResult) => void;
}

/**
 * Unified file card that persists across Phase 2 (configure) and Phase 3 (results).
 *
 * States:
 *   - Idle (Phase 2): FileIcon + name/size + delete button
 *   - Processing: LoaderIcon (spinning) + name/size
 *   - Completed: ResultFileCard with full stats + download button
 *   - Queued (Phase 3, not yet processed): FileIcon + name/size (no actions)
 */
export function FileCard({
  file,
  result,
  isProcessing,
  isExecuting,
  onDelete,
  onDownload,
}: FileCardProps) {
  if (result) {
    return <CompletedFileCard result={result} onDownload={onDownload} />;
  }

  const icon = isProcessing ? (
    <LoaderIcon className="size-5 motion-safe:animate-spin" />
  ) : (
    <FileIcon className="size-5" />
  );

  return (
    <Card elevation="sm" role="listitem" aria-busy={isProcessing} data-testid="input-file">
      <Row className="gap-3 rounded-lg px-4 py-3">
        <Row className="min-w-0 flex-1 gap-3">
          <IconBadge variant="primary" size="lg" aria-hidden="true">
            {icon}
          </IconBadge>
          <Stack className="min-w-0 flex-1 gap-0">
            <span className="truncate text-sm font-semibold">{file.name}</span>
            <span className="truncate text-xs text-muted-foreground">
              {formatFileSize(file.size)}
            </span>
            {isProcessing && (
              <span className="sr-only" role="status">
                Processing
              </span>
            )}
          </Stack>
        </Row>

        {!isExecuting && (
          <Button
            variant="outline"
            size="icon"
            elevation="sm"
            onClick={onDelete}
            aria-label={`Remove ${file.name}`}
          >
            <XIcon className="size-4" />
          </Button>
        )}
      </Row>
    </Card>
  );
}

/** Completed state — uses shared ResultFileCard with full stats. */
function CompletedFileCard({
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
      icon={<CheckCircle2Icon className="size-5" />}
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
