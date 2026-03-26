"use client";

import { Button, DownloadIcon, LoaderIcon, formatFileSize } from "@bnto/ui";
import type { OutputFileUrl } from "@bnto/core";

interface OutputFileRowProps {
  name: string;
  sizeBytes: number;
  downloadUrl: OutputFileUrl | undefined;
  onDownload: (url: OutputFileUrl) => () => void;
}

/**
 * Individual file row in the execution results list.
 *
 * Shows file name, size, and a download button (or spinner while
 * the presigned URL is still loading).
 */
export function OutputFileRow({ name, sizeBytes, downloadUrl, onDownload }: OutputFileRowProps) {
  return (
    <li
      className="flex items-center justify-between gap-3 rounded-lg border border-border bg-background px-4 py-3"
      data-testid="output-file"
    >
      <div className="min-w-0 flex-1">
        <p className="truncate text-sm font-medium text-foreground">{name}</p>
        <p className="text-xs text-muted-foreground">{formatFileSize(sizeBytes)}</p>
      </div>

      {downloadUrl ? (
        <Button
          variant="outline"
          size="icon"
          onClick={onDownload(downloadUrl)}
          aria-label={`Download ${name}`}
          data-testid="download-button"
        >
          <DownloadIcon className="size-4" />
        </Button>
      ) : (
        <LoaderIcon className="size-4 shrink-0 text-muted-foreground motion-safe:animate-spin" />
      )}
    </li>
  );
}
