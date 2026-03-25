"use client";

import type { BrowserFileResult } from "@bnto/core";
import {
  Button,
  FileListActions,
  FileListContent,
  FileListIcon,
  FileListItem,
  FileListMeta,
  FileListName,
  IconBadge,
} from "@bnto/ui";
import { FileIcon, LoaderIcon, XIcon } from "@bnto/ui";
import { formatFileSize } from "@bnto/ui";
import { CompletedFileRow } from "./CompletedFileRow";

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
 *   - Completed: Full stats + download button (via CompletedFileRow)
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
    return <CompletedFileRow result={result} onDownload={onDownload} />;
  }

  const icon = isProcessing ? (
    <LoaderIcon className="size-5 motion-safe:animate-spin" />
  ) : (
    <FileIcon className="size-5" />
  );

  return (
    <FileListItem aria-busy={isProcessing} data-testid="input-file">
      <FileListIcon>
        <IconBadge variant="primary" size="lg" aria-hidden="true">
          {icon}
        </IconBadge>
      </FileListIcon>
      <FileListContent>
        <FileListName>{file.name}</FileListName>
        <FileListMeta>{formatFileSize(file.size)}</FileListMeta>
        {isProcessing && (
          <span className="sr-only" role="status">
            Processing
          </span>
        )}
      </FileListContent>
      {!isExecuting && (
        <FileListActions>
          <Button
            variant="outline"
            size="icon"
            elevation="sm"
            onClick={onDelete}
            aria-label={`Remove ${file.name}`}
          >
            <XIcon className="size-4" />
          </Button>
        </FileListActions>
      )}
    </FileListItem>
  );
}
