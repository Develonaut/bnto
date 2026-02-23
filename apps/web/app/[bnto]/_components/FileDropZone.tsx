"use client";

import { useCallback, useState } from "react";
import type { FileRejection } from "react-dropzone";
import { Upload } from "lucide-react";
import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button";
import { Dropzone } from "@/components/ui/dropzone";
import { getAcceptedTypes, toDropzoneAccept } from "../_lib/getAcceptedTypes";
import { FileList } from "./FileList";

interface FileDropZoneProps {
  /** The bnto slug — determines accepted file types. */
  slug: string;
  /** Called when files change (added or removed). */
  onFilesChange?: (files: File[]) => void;
}

/**
 * Drag-and-drop file zone with batch selection and file list.
 *
 * Validates dropped/selected files against the bnto's accepted types.
 * Rejected files are silently skipped — the accepted ones still get added.
 */
export function FileDropZone({ slug, onFilesChange }: FileDropZoneProps) {
  const [files, setFiles] = useState<File[]>([]);
  const [rejectedCount, setRejectedCount] = useState(0);

  const { label } = getAcceptedTypes(slug);
  const accept = toDropzoneAccept(slug);

  const handleDrop = useCallback(
    (accepted: File[], rejected: FileRejection[]) => {
      setRejectedCount(rejected.length);
      setFiles((prev) => {
        const updated = [...prev, ...accepted];
        onFilesChange?.(updated);
        return updated;
      });
    },
    [onFilesChange],
  );

  function handleRemove(index: number) {
    setFiles((prev) => {
      const updated = prev.filter((_, i) => i !== index);
      onFilesChange?.(updated);
      return updated;
    });
    setRejectedCount(0);
  }

  function handleClear() {
    setFiles([]);
    setRejectedCount(0);
    onFilesChange?.([]);
  }

  const hasFiles = files.length > 0;

  return (
    <div className="space-y-4">
      <Dropzone accept={accept} multiple onDrop={handleDrop}>
        {({ isDragActive }) => (
          <>
            <div
              className={cn(
                "rounded-full p-3",
                "motion-safe:transition-colors motion-safe:duration-fast",
                isDragActive
                  ? "bg-primary/10 text-primary"
                  : "bg-muted text-muted-foreground",
              )}
            >
              <Upload className="size-6" />
            </div>

            <div className="text-center">
              <p className="text-sm font-medium text-foreground">
                {isDragActive ? "Drop files here" : "Drag & drop files here"}
              </p>
              <p className="mt-1 text-xs text-muted-foreground">
                or click to browse &middot; accepts {label}
              </p>
            </div>
          </>
        )}
      </Dropzone>

      {rejectedCount > 0 && (
        <p className="text-center text-xs text-destructive" role="alert">
          {rejectedCount} {rejectedCount === 1 ? "file" : "files"} skipped
          &mdash; only {label} accepted
        </p>
      )}

      {hasFiles && (
        <div className="space-y-3">
          <div className="flex items-center justify-between">
            <p className="text-sm font-medium text-foreground">
              {files.length} {files.length === 1 ? "file" : "files"} selected
            </p>
            <Button
              variant="ghost"
              size="sm"
              onClick={handleClear}
              className="text-muted-foreground"
            >
              Clear all
            </Button>
          </div>
          <FileList files={files} onRemove={handleRemove} />
        </div>
      )}
    </div>
  );
}
