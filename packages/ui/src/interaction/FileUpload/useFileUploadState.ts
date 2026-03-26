"use client";

import { useCallback, useMemo } from "react";
import { useDropzone, type Accept } from "react-dropzone";

import type { FileUploadContextValue } from "./context";
import { buildFileUploadContext } from "./buildFileUploadContext";

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
export function useFileUploadState(opts: UseFileUploadStateOptions): FileUploadContextValue {
  const { value, onValueChange, disabled, ...dzOpts } = opts;
  const onDrop = useCallback(
    (accepted: File[]) => onValueChange([...value, ...accepted]),
    [value, onValueChange],
  );
  const dz = useDropzone({ onDrop, ...dzOpts, disabled, noClick: true, noKeyboard: true });
  const removeFile = useCallback(
    (file: File) => onValueChange(value.filter((f) => f !== file)),
    [value, onValueChange],
  );
  const clearFiles = useCallback(() => onValueChange([]), [onValueChange]);
  return useMemo(
    () => buildFileUploadContext({ value, removeFile, clearFiles, disabled, dropzone: dz }),
    [value, removeFile, clearFiles, dz.isDragActive, dz.open, disabled, dz.getRootProps, dz.getInputProps],
  );
}
