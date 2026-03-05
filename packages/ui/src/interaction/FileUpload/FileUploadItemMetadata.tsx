"use client";

import type { ComponentProps } from "react";

import { FileIcon } from "../../icons";
import { cn } from "../../utils/cn";
import { formatFileSize } from "../../utils/formatFileSize";

import { useFileUploadItemContext } from "./context";

export function FileUploadItemMetadata({ className, ...props }: ComponentProps<"div">) {
  const { file } = useFileUploadItemContext("FileUpload.ItemMetadata");

  return (
    <div
      data-slot="file-upload-metadata"
      {...props}
      className={cn("flex min-w-0 flex-1 items-center gap-3", className)}
    >
      <div className="flex size-10 shrink-0 items-center justify-center rounded-full bg-primary/10 text-primary">
        <FileIcon className="size-5" />
      </div>
      <div className="flex min-w-0 flex-1 flex-col">
        <span className="truncate text-sm font-semibold">{file.name}</span>
        <span className="truncate text-xs text-muted-foreground">
          {file.type || "unknown"} &middot; {formatFileSize(file.size)}
        </span>
      </div>
    </div>
  );
}
