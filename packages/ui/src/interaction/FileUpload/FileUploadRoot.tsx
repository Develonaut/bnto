"use client";

import type { ComponentProps } from "react";
import type { Accept } from "react-dropzone";

import { FileUploadContext } from "./context";
import { useFileUploadState } from "./useFileUploadState";
import { FileUploadShell } from "./FileUploadShell";

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
  ...shellProps
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
      <FileUploadShell disabled={disabled} {...shellProps} />
    </FileUploadContext.Provider>
  );
}
