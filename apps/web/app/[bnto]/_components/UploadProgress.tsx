"use client";

import { CheckCircle2, Loader2, XCircle } from "lucide-react";
import type { FileUploadProgress } from "@bnto/core";
import { cn } from "@/lib/utils";
import { formatFileSize } from "@/src/utils/formatFileSize";

interface UploadProgressProps {
  files: FileUploadProgress[];
}

/**
 * Displays per-file upload progress with status indicators.
 * Renders a progress bar for each file being uploaded to R2.
 */
export function UploadProgress({ files }: UploadProgressProps) {
  if (files.length === 0) return null;

  return (
    <ul className="space-y-2">
      {files.map((file) => (
        <li
          key={file.fileName}
          className="overflow-hidden rounded-lg border border-border bg-card px-4 py-3"
          data-testid="upload-file"
          data-file-status={file.status}
        >
          <div className="flex items-center justify-between gap-3">
            <div className="min-w-0 flex-1">
              <p className="truncate text-sm font-medium text-foreground">
                {file.fileName}
              </p>
              <p className="text-xs text-muted-foreground">
                {formatFileSize(file.loaded)} / {formatFileSize(file.total)}
              </p>
            </div>
            <FileStatusIcon status={file.status} />
          </div>

          {file.status === "uploading" && (
            <div className="mt-2 h-1.5 overflow-hidden rounded-full bg-muted">
              <div
                className="h-full rounded-full bg-primary motion-safe:transition-[width] motion-safe:duration-fast"
                style={{ width: `${getPercent(file)}%` }}
              />
            </div>
          )}

          {file.status === "failed" && file.error && (
            <p className="mt-1 text-xs text-destructive">{file.error}</p>
          )}
        </li>
      ))}
    </ul>
  );
}

function FileStatusIcon({
  status,
}: {
  status: FileUploadProgress["status"];
}) {
  switch (status) {
    case "pending":
      return (
        <Loader2 className="size-4 shrink-0 text-muted-foreground" />
      );
    case "uploading":
      return (
        <Loader2
          className={cn(
            "size-4 shrink-0 text-primary",
            "motion-safe:animate-spin",
          )}
        />
      );
    case "completed":
      return (
        <CheckCircle2 className="size-4 shrink-0 text-green-600 dark:text-green-400" />
      );
    case "failed":
      return (
        <XCircle className="size-4 shrink-0 text-destructive" />
      );
  }
}

function getPercent(file: FileUploadProgress): number {
  if (file.total === 0) return 0;
  return Math.round((file.loaded / file.total) * 100);
}
