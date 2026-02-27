"use client";

import { useCallback, useMemo, type ComponentProps } from "react";
import { useDropzone, type Accept } from "react-dropzone";

import { cn } from "@/lib/cn";

import { FileUploadContext, type FileUploadContextValue } from "./context";

export interface FileUploadProps
  extends Omit<ComponentProps<"div">, "onDrop" | "defaultValue"> {
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
  const onDrop = useCallback(
    (acceptedFiles: File[]) => {
      onValueChange([...value, ...acceptedFiles]);
    },
    [value, onValueChange],
  );

  const dropzone = useDropzone({
    onDrop,
    accept,
    multiple,
    maxFiles,
    maxSize,
    disabled,
    noClick: true,
    noKeyboard: true,
  });

  const removeFile = useCallback(
    (file: File) => onValueChange(value.filter((f) => f !== file)),
    [value, onValueChange],
  );

  const clearFiles = useCallback(
    () => onValueChange([]),
    [onValueChange],
  );

  const ctx = useMemo<FileUploadContextValue>(
    () => ({
      files: value,
      removeFile,
      clearFiles,
      isDragActive: dropzone.isDragActive,
      open: dropzone.open,
      disabled,
      getRootProps: dropzone.getRootProps,
      getInputProps: dropzone.getInputProps,
    }),
    [
      value,
      removeFile,
      clearFiles,
      dropzone.isDragActive,
      dropzone.open,
      disabled,
      dropzone.getRootProps,
      dropzone.getInputProps,
    ],
  );

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
