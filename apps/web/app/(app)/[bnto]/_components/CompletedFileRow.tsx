"use client";

import { useCallback } from "react";
import type { BrowserFileResult } from "@bnto/core";
import { useFileResultProps } from "@bnto/core";
import {
  Badge,
  Button,
  CheckCircle2Icon,
  DownloadIcon,
  FileListActions,
  FileListContent,
  FileListIcon,
  FileListItem,
  FileListMeta,
  FileListName,
  IconBadge,
} from "@bnto/ui";

/**
 * Completed file result row — renders filename, extension badge,
 * size stats with savings, and a download button.
 *
 * Used by both BrowserExecutionResults (bnto page) and FileCard (bnto page).
 */
function CompletedFileRow({
  result,
  onDownload,
}: {
  result: BrowserFileResult;
  onDownload: (result: BrowserFileResult) => void;
}) {
  const props = useFileResultProps(result);
  const handleDownload = useCallback(() => onDownload(result), [onDownload, result]);
  const hasSavings = props.originalSize != null && props.savings != null;

  return (
    <FileListItem data-testid="output-file">
      <FileListIcon>
        <IconBadge variant="primary" size="lg" aria-hidden="true">
          <CheckCircle2Icon className="size-5" />
        </IconBadge>
      </FileListIcon>
      <FileListContent>
        <span className="flex items-center gap-1.5">
          <FileListName>{props.filename}</FileListName>
          {props.extension && (
            <Badge variant="outline" size="sm" className="shrink-0 uppercase">
              {props.extension}
            </Badge>
          )}
        </span>
        <FileListMeta>
          {hasSavings ? (
            <>
              <span className="line-through">{props.originalSize}</span>{" "}
              <span className="font-semibold text-primary">{props.savings}</span> {props.outputSize}
            </>
          ) : (
            props.outputSize
          )}
        </FileListMeta>
      </FileListContent>
      <FileListActions>
        <Button
          variant="outline"
          size="icon"
          elevation="sm"
          onClick={handleDownload}
          aria-label={`Download ${result.filename}`}
          data-testid="download-button"
        >
          <DownloadIcon className="size-4" />
        </Button>
      </FileListActions>
    </FileListItem>
  );
}

export { CompletedFileRow };
