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
import { SizeDisplay } from "./SizeDisplay";

export function CompletedRow({
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
        <span className="flex items-center gap-1.5">
          <FileListName>{props.filename}</FileListName>
          {props.extension && (
            <Badge variant="outline" size="sm" className="shrink-0 uppercase">
              {props.extension}
            </Badge>
          )}
        </span>
        <FileListMeta>
          <SizeDisplay
            originalSize={props.originalSize}
            savings={props.savings}
            outputSize={props.outputSize}
          />
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
