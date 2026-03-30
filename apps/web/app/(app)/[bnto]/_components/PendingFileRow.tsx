"use client";

import {
  Button,
  FileListActions,
  FileListContent,
  FileListIcon,
  FileListItem,
  FileListMeta,
  FileListName,
  IconBadge,
  FileIcon,
  LoaderIcon,
  XIcon,
  formatFileSize,
} from "@bnto/ui";

interface PendingFileRowProps {
  file: File;
  isProcessing: boolean;
  isExecuting: boolean;
  onDelete: () => void;
}

/** File row for files that are idle, queued, or currently processing. */
export function PendingFileRow({ file, isProcessing, isExecuting, onDelete }: PendingFileRowProps) {
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
      <PendingFileContent file={file} isProcessing={isProcessing} />
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

export function PendingFileContent({ file, isProcessing }: { file: File; isProcessing: boolean }) {
  return (
    <FileListContent>
      <FileListName>{file.name}</FileListName>
      <FileListMeta>{formatFileSize(file.size)}</FileListMeta>
      {isProcessing && (
        <span className="sr-only" role="status">
          Processing
        </span>
      )}
    </FileListContent>
  );
}
