"use client";

import { CheckCircle2Icon, LoaderIcon, XCircleIcon, cn, formatFileSize } from "@bnto/ui";
import type { FileUploadProgress } from "@bnto/core";

interface UploadFileRowProps {
  file: FileUploadProgress;
}

/** A single file's upload progress with status icon and optional progress bar. */
export function UploadFileRow({ file }: UploadFileRowProps) {
  return (
    <li
      className="overflow-hidden rounded-lg border border-border bg-card px-4 py-3"
      data-testid="upload-file"
      data-file-status={file.status}
    >
      <div className="flex items-center justify-between gap-3">
        <div className="min-w-0 flex-1">
          <p className="truncate text-sm font-medium text-foreground">{file.fileName}</p>
          <p className="text-xs text-muted-foreground">
            {formatFileSize(file.loaded)} / {formatFileSize(file.total)}
          </p>
        </div>
        <UploadFileStatusIcon status={file.status} />
      </div>

      {file.status === "uploading" && (
        <div className="mt-2 h-1.5 overflow-hidden rounded-full bg-muted">
          <div
            className="h-full rounded-full bg-primary motion-safe:transition-[width] motion-safe:duration-fast"
            style={{ width: `${getUploadPercent(file)}%` }}
          />
        </div>
      )}

      {file.status === "failed" && file.error && (
        <p className="mt-1 text-xs text-destructive">{file.error}</p>
      )}
    </li>
  );
}

export function UploadFileStatusIcon({ status }: { status: FileUploadProgress["status"] }) {
  switch (status) {
    case "pending":
      return <LoaderIcon className="size-4 shrink-0 text-muted-foreground" />;
    case "uploading":
      return (
        <LoaderIcon className={cn("size-4 shrink-0 text-primary", "motion-safe:animate-spin")} />
      );
    case "completed":
      return <CheckCircle2Icon className="size-4 shrink-0 text-success" />;
    case "failed":
      return <XCircleIcon className="size-4 shrink-0 text-destructive" />;
  }
}

const getUploadPercent = (file: FileUploadProgress): number => {
  if (file.total === 0) return 0;
  return Math.round((file.loaded / file.total) * 100);
};
