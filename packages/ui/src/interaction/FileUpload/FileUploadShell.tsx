"use client";

import type { ComponentProps } from "react";

import { cn } from "../../utils/cn";

type FileUploadShellProps = ComponentProps<"div"> & { disabled: boolean };

/** Layout wrapper for the file-upload compound component. */
export function FileUploadShell({ disabled, children, className, ...props }: FileUploadShellProps) {
  return (
    <div
      data-slot="file-upload"
      data-disabled={disabled ? "" : undefined}
      {...props}
      className={cn("relative flex flex-col gap-2", className)}
    >
      {children}
    </div>
  );
}
