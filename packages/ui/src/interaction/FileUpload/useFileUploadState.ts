"use client";

import { useCallback, useMemo } from "react";
import { useDropzone, type Accept } from "react-dropzone";

import type { FileUploadContextValue } from "./context";

interface UseFileUploadStateOptions {
  value: File[];
  onValueChange: (files: File[]) => void;
  accept?: Accept;
  multiple?: boolean;
  maxFiles?: number;
  maxSize?: number;
  disabled: boolean;
}

/** Encapsulates dropzone setup and context value creation. */
export function useFileUploadState({
  value,
  onValueChange,
  accept,
  multiple,
  maxFiles,
  maxSize,
  disabled,
}: UseFileUploadStateOptions): FileUploadContextValue {
  const onDrop = useCallback(
    (acceptedFiles: File[]) => onValueChange([...value, ...acceptedFiles]),
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

  const clearFiles = useCallback(() => onValueChange([]), [onValueChange]);

  return useMemo<FileUploadContextValue>(
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
}
