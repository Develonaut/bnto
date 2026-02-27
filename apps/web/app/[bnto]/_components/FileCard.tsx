"use client";

import { useEffect, useMemo } from "react";

import { Animate } from "@/components/ui/Animate";
import { Button } from "@/components/ui/Button";
import { Card } from "@/components/ui/Card";
import {
  FileIcon,
  FileTextIcon,
  FileCodeIcon,
  ImageIcon,
  TrashIcon,
} from "@/components/ui/icons";
import { cn } from "@/lib/cn";
import { formatFileSize } from "@/src/utils/formatFileSize";

interface FileCardProps {
  file: File;
  onRemove: () => void;
  disabled?: boolean;
  /** Position in a stagger cascade. Passed to Animate.ScaleIn. */
  index?: number;
}

/** Extract a short uppercase label from the file extension or MIME type. */
function getTypeLabel(file: File): string {
  const ext = file.name.split(".").pop()?.toUpperCase();
  if (ext) return ext;
  const sub = file.type.split("/").pop()?.toUpperCase();
  return sub ?? "FILE";
}

/** Whether the file is an image that the browser can preview. */
function isPreviewable(type: string): boolean {
  return type.startsWith("image/");
}

/** Renders the appropriate icon for a file's MIME type. */
function FileTypeIcon({ type, className }: { type: string; className?: string }) {
  if (type.startsWith("image/")) return <ImageIcon className={className} />;
  if (type.startsWith("text/csv") || type.includes("spreadsheet"))
    return <FileTextIcon className={className} />;
  if (type.includes("json") || type.includes("xml"))
    return <FileCodeIcon className={className} />;
  return <FileIcon className={className} />;
}

/**
 * Rich file preview card for the recipe page file list.
 *
 * Shows an image thumbnail (via `URL.createObjectURL` with cleanup)
 * for image files, or a type-appropriate icon for non-images.
 * Displays file name (truncated), file size, type badge, and a delete button.
 */
export function FileCard({ file, onRemove, disabled, index }: FileCardProps) {
  const thumbnailUrl = useMemo(
    () => (isPreviewable(file.type) ? URL.createObjectURL(file) : null),
    [file],
  );

  useEffect(() => {
    return () => {
      if (thumbnailUrl) URL.revokeObjectURL(thumbnailUrl);
    };
  }, [thumbnailUrl]);

  const typeLabel = getTypeLabel(file);

  return (
    <Animate.ScaleIn index={index} from={0.85}>
      <Card
        elevation="sm"
        className="group relative flex flex-col overflow-hidden"
        data-testid="file-card"
      >
        {/* Thumbnail or icon area */}
        <div className="flex aspect-square items-center justify-center overflow-hidden bg-muted/50">
          {thumbnailUrl ? (
            /* eslint-disable-next-line @next/next/no-img-element -- blob: URL, next/image optimization not applicable */
            <img
              src={thumbnailUrl}
              alt={file.name}
              className="size-full object-cover"
            />
          ) : (
            <FileTypeIcon type={file.type} className="size-10 text-muted-foreground/60" />
          )}
        </div>

        {/* Metadata */}
        <div className="flex min-w-0 flex-1 flex-col gap-0.5 px-3 py-2">
          <span className="truncate text-sm font-medium text-foreground">
            {file.name}
          </span>
          <div className="flex items-center gap-1.5">
            <span
              className={cn(
                "inline-flex items-center rounded-full px-1.5 py-0.5",
                "bg-primary/10 text-[10px] font-semibold uppercase leading-none text-primary",
              )}
            >
              {typeLabel}
            </span>
            <span className="text-xs text-muted-foreground">
              {formatFileSize(file.size)}
            </span>
          </div>
        </div>

        {/* Delete button — visible on hover/focus */}
        <Button
          variant="destructive"
          size="icon"
          elevation="sm"
          className={cn(
            "absolute right-1.5 top-1.5 size-7",
            "opacity-0 transition-opacity duration-fast",
            "group-hover:opacity-100 group-focus-within:opacity-100",
          )}
          onClick={onRemove}
          disabled={disabled}
          aria-label={`Remove ${file.name}`}
        >
          <TrashIcon className="size-3.5" />
        </Button>
      </Card>
    </Animate.ScaleIn>
  );
}
