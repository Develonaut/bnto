"use client";

import { X } from "lucide-react";
import { formatFileSize } from "@/src/utils/formatFileSize";

interface FileListProps {
  files: File[];
  onRemove: (index: number) => void;
}

/**
 * Displays a list of selected files with name, size, and a remove button.
 * Renders nothing when the file list is empty.
 */
export function FileList({ files, onRemove }: FileListProps) {
  if (files.length === 0) return null;

  return (
    <ul className="space-y-2">
      {files.map((file, index) => (
        <li
          key={`${file.name}-${file.size}-${file.lastModified}`}
          className="flex items-center justify-between gap-3 rounded-lg border border-border bg-card px-4 py-3"
        >
          <div className="min-w-0 flex-1">
            <p className="truncate text-sm font-medium text-foreground">
              {file.name}
            </p>
            <p className="text-xs text-muted-foreground">
              {formatFileSize(file.size)}
            </p>
          </div>

          <button
            type="button"
            onClick={() => onRemove(index)}
            className="shrink-0 rounded-md p-1 text-muted-foreground hover:bg-muted hover:text-foreground motion-safe:transition-colors motion-safe:duration-fast"
            aria-label={`Remove ${file.name}`}
          >
            <X className="size-4" />
          </button>
        </li>
      ))}
    </ul>
  );
}
