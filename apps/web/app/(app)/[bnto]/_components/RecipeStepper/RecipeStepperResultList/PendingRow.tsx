"use client";

import {
  Button,
  FileIcon,
  FileListActions,
  FileListContent,
  FileListIcon,
  FileListItem,
  FileListMeta,
  FileListName,
  IconBadge,
  LoaderIcon,
  XIcon,
  formatFileSize,
} from "@bnto/ui";

export function PendingRow({
  file,
  isProcessing,
  isExecuting,
  onDelete,
}: {
  file: File;
  isProcessing: boolean;
  isExecuting: boolean;
  onDelete: () => void;
}) {
  return (
    <FileListItem aria-busy={isProcessing} data-testid="input-file">
      <FileListIcon>
        <IconBadge variant="primary" size="lg" aria-hidden="true">
          {isProcessing ? (
            <LoaderIcon className="size-5 motion-safe:animate-spin" />
          ) : (
            <FileIcon className="size-5" />
          )}
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
