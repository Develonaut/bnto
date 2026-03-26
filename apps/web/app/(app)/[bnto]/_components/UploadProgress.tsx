"use client";

import type { FileUploadProgress } from "@bnto/core";
import { UploadFileRow } from "./UploadFileRow";

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
        <UploadFileRow key={file.fileName} file={file} />
      ))}
    </ul>
  );
}
