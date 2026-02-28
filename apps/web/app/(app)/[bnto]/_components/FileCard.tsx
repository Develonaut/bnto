"use client";

import type { BrowserFileResult } from "@bnto/core";
import { Button } from "@/components/ui/Button";
import { Card } from "@/components/ui/Card";
import {
  CheckCircle2Icon,
  DownloadIcon,
  FileIcon,
  LoaderIcon,
  XIcon,
} from "@/components/ui/icons";
import { formatFileSize } from "@/src/utils/formatFileSize";

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
 *   - Completed: CheckCircle2Icon + name/output size + download button
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
  const icon = result ? (
    <CheckCircle2Icon className="size-5" />
  ) : isProcessing ? (
    <LoaderIcon className="size-5 motion-safe:animate-spin" />
  ) : (
    <FileIcon className="size-5" />
  );

  const subtitle = result ? (
    <>
      {formatFileSize(result.blob.size)}
      {result.metadata.originalSize != null && (
        <span> &middot; was {formatFileSize(result.metadata.originalSize as number)}</span>
      )}
    </>
  ) : (
    formatFileSize(file.size)
  );

  return (
    <Card
      className="flex items-center gap-3 rounded-lg px-4 py-4"
      elevation="sm"
      role="listitem"
      aria-busy={isProcessing}
      data-testid={result ? "output-file" : "input-file"}
    >
      <div className="flex min-w-0 flex-1 items-center gap-3">
        <div className="flex size-10 shrink-0 items-center justify-center rounded-full bg-primary/10 text-primary" aria-hidden="true">
          {icon}
        </div>
        <div className="flex min-w-0 flex-1 flex-col">
          <span className="truncate text-sm font-semibold">
            {result ? result.filename : file.name}
          </span>
          <span className="truncate text-xs text-muted-foreground">
            {subtitle}
          </span>
          {isProcessing && (
            <span className="sr-only" role="status">Processing</span>
          )}
          {result && (
            <span className="sr-only">Completed</span>
          )}
        </div>
      </div>

      {result ? (
        <Button
          variant="outline"
          size="icon"
          elevation="sm"
          onClick={() => onDownload(result)}
          aria-label={`Download ${result.filename}`}
        >
          <DownloadIcon className="size-4" />
        </Button>
      ) : !isExecuting ? (
        <Button
          variant="outline"
          size="icon"
          elevation="sm"
          onClick={onDelete}
          aria-label={`Remove ${file.name}`}
        >
          <XIcon className="size-4" />
        </Button>
      ) : null}
    </Card>
  );
}
