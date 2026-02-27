"use client";

import { createContext, useContext } from "react";
import type { useDropzone } from "react-dropzone";

// ---------------------------------------------------------------------------
// Root context — shared state between all compound components
// ---------------------------------------------------------------------------

export interface FileUploadContextValue {
  files: File[];
  removeFile: (file: File) => void;
  clearFiles: () => void;
  isDragActive: boolean;
  open: () => void;
  disabled: boolean;
  getRootProps: ReturnType<typeof useDropzone>["getRootProps"];
  getInputProps: ReturnType<typeof useDropzone>["getInputProps"];
}

export const FileUploadContext =
  createContext<FileUploadContextValue | null>(null);

export function useFileUploadContext(consumer: string) {
  const ctx = useContext(FileUploadContext);
  if (!ctx)
    throw new Error(`\`${consumer}\` must be used within \`FileUpload\``);
  return ctx;
}

// ---------------------------------------------------------------------------
// Item context — per-file state for Item sub-components
// ---------------------------------------------------------------------------

export const FileUploadItemContext = createContext<{
  file: File;
} | null>(null);

export function useFileUploadItemContext(consumer: string) {
  const ctx = useContext(FileUploadItemContext);
  if (!ctx)
    throw new Error(
      `\`${consumer}\` must be used within \`FileUpload.Item\``,
    );
  return ctx;
}
