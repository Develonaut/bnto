"use client";

import { UploadIcon, XIcon } from "@/components/ui/icons";
import { cn } from "@/lib/cn";
import { Button } from "@/components/ui/Button";
import {
  FileUpload,
  FileUploadDropzone,
  FileUploadList,
  FileUploadItem,
  FileUploadItemMetadata,
  FileUploadItemDelete,
  FileUploadClear,
} from "@/components/ui/FileUpload";
import { getAcceptedTypes } from "../_lib/getAcceptedTypes";

interface FileDropZoneProps {
  /** The bnto slug — determines accepted file types. */
  slug: string;
  /** Currently selected files (controlled). */
  value: File[];
  /** Called when files change (added or removed). */
  onValueChange: (files: File[]) => void;
  /** When true, prevents new drops and file selection. */
  disabled?: boolean;
}

/**
 * Drag-and-drop file zone with batch selection and file list.
 *
 * Uses the diceui FileUpload compound component in controlled mode.
 * File validation (accepted types) is handled by the component's
 * built-in `accept` prop based on the bnto slug.
 */
export function FileDropZone({
  slug,
  value,
  onValueChange,
  disabled,
}: FileDropZoneProps) {
  const { accept, label } = getAcceptedTypes(slug);

  return (
    <FileUpload
      value={value}
      onValueChange={onValueChange}
      accept={accept === "*/*" ? undefined : accept}
      multiple
      disabled={disabled}
    >
      <FileUploadDropzone
        className={cn(
          "gap-3 rounded-xl border-border bg-muted/30 px-6 py-10",
          "motion-safe:transition-all motion-safe:duration-fast",
          "hover:border-muted-foreground/40 hover:bg-muted/50",
          "data-[dragging]:border-primary data-[dragging]:bg-primary/5 data-[dragging]:shadow-md",
        )}
      >
        <div
          className={cn(
            "rounded-full p-3 bg-muted text-muted-foreground",
            "motion-safe:transition-colors motion-safe:duration-fast",
          )}
        >
          <UploadIcon className="size-6" />
        </div>

        <div className="text-center">
          <p className="text-sm font-medium text-foreground">
            Drag & drop files here
          </p>
          <p className="mt-1 text-xs text-muted-foreground">
            or click to browse &middot; accepts {label}
          </p>
        </div>
      </FileUploadDropzone>

      {value.length > 0 && (
        <div className="space-y-3">
          <div className="flex items-center justify-between">
            <p className="text-sm font-medium text-foreground">
              {value.length} {value.length === 1 ? "file" : "files"} selected
            </p>
            <FileUploadClear asChild>
              <Button
                variant="outline"
                size="sm"
                className="text-muted-foreground"
              >
                Clear all
              </Button>
            </FileUploadClear>
          </div>

          <FileUploadList>
            {value.map((file) => (
              <FileUploadItem
                key={`${file.name}-${file.size}-${file.lastModified}`}
                value={file}
                className="rounded-lg border-border bg-card px-4 py-3"
              >
                <FileUploadItemMetadata />
                <FileUploadItemDelete asChild>
                  <button
                    type="button"
                    className="shrink-0 rounded-md p-1 text-muted-foreground hover:bg-muted hover:text-foreground motion-safe:transition-colors motion-safe:duration-fast"
                    aria-label={`Remove ${file.name}`}
                  >
                    <XIcon className="size-4" />
                  </button>
                </FileUploadItemDelete>
              </FileUploadItem>
            ))}
          </FileUploadList>
        </div>
      )}
    </FileUpload>
  );
}
