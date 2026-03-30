"use client";

import { useCallback } from "react";
import type { BrowserFileResult } from "@bnto/core";
import { useFileResultProps } from "@bnto/core";
import {
  Button,
  CheckCircle2Icon,
  DownloadIcon,
  FileListActions,
  FileListContent,
  FileListIcon,
  FileListItem,
  IconBadge,
} from "@bnto/ui";
import { CompletedFileMeta } from "./CompletedFileMeta";

/**
 * Completed file result row -- renders filename, extension badge,
 * size stats with savings, and a download button.
 */
export function CompletedFileRow({
  result,
  onDownload,
}: {
  result: BrowserFileResult;
  onDownload: (result: BrowserFileResult) => void;
}) {
  const props = useFileResultProps(result);
  const handleDownload = useCallback(() => onDownload(result), [onDownload, result]);

  return (
    <FileListItem data-testid="output-file">
      <FileListIcon>
        <IconBadge variant="primary" size="lg" aria-hidden="true">
          <CheckCircle2Icon className="size-5" />
        </IconBadge>
      </FileListIcon>
      <FileListContent>
        <CompletedFileMeta {...props} />
      </FileListContent>
      <FileListActions>
        <DownloadButton filename={result.filename} onClick={handleDownload} />
      </FileListActions>
    </FileListItem>
  );
}

export function DownloadButton({ filename, onClick }: { filename: string; onClick: () => void }) {
  return (
    <Button
      variant="outline"
      size="icon"
      elevation="sm"
      onClick={onClick}
      aria-label={`Download ${filename}`}
      data-testid="download-button"
    >
      <DownloadIcon className="size-4" />
    </Button>
  );
}
