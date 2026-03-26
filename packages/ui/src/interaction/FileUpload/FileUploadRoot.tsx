"use client";

import type { ComponentProps } from "react";
import type { Accept } from "react-dropzone";

import { cn } from "../../utils/cn";

import { FileUploadContext } from "./context";
import { useFileUploadState } from "./useFileUploadState";

export interface FileUploadProps extends Omit<ComponentProps<"div">, "onDrop" | "defaultValue"> {
  value: File[];
  onValueChange: (files: File[]) => void;
  accept?: Accept;
  multiple?: boolean;
  maxFiles?: number;
  maxSize?: number;
  disabled?: boolean;
}

export function FileUploadRoot({
  value,
  onValueChange,
  accept,
  multiple,
  maxFiles,
  maxSize,
  disabled = false,
  children,
  className,
  ...props
}: FileUploadProps) {
  const ctx = useFileUploadState({
    value,
    onValueChange,
    accept,
    multiple,
    maxFiles,
    maxSize,
    disabled,
  });

  return (
    <FileUploadContext.Provider value={ctx}>
      <div
        data-slot="file-upload"
        data-disabled={disabled ? "" : undefined}
        {...props}
        className={cn("relative flex flex-col gap-2", className)}
      >
        {children}
      </div>
    </FileUploadContext.Provider>
  );
}
