"use client";

import type { BrowserFileResult } from "@bnto/core";
import { CompletedFileRow } from "./CompletedFileRow";
import { PendingFileRow } from "./PendingFileRow";

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

  return (
    <PendingFileRow
      file={file}
      isProcessing={isProcessing}
      isExecuting={isExecuting}
      onDelete={onDelete}
    />
  );
}
