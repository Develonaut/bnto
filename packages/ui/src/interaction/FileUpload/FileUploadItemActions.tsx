"use client";

import type { ComponentProps } from "react";

import { cn } from "../../utils/cn";

export function FileUploadItemActions({
  className,
  ...props
}: ComponentProps<"div">) {
  return (
    <div
      data-slot="file-upload-item-actions"
      {...props}
      className={cn("flex shrink-0 items-center gap-1", className)}
    />
  );
}
