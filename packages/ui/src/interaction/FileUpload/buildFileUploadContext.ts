import type { useDropzone } from "react-dropzone";
import type { FileUploadContextValue } from "./context";

interface BuildContextOptions {
  value: File[];
  removeFile: (file: File) => void;
  clearFiles: () => void;
  disabled: boolean;
  dropzone: ReturnType<typeof useDropzone>;
}

/** Pure builder for the context value -- keeps the hook small. */
export function buildFileUploadContext({
  value,
  removeFile,
  clearFiles,
  disabled,
  dropzone,
}: BuildContextOptions): FileUploadContextValue {
  return {
    files: value,
    removeFile,
    clearFiles,
    isDragActive: dropzone.isDragActive,
    open: dropzone.open,
    disabled,
    getRootProps: dropzone.getRootProps,
    getInputProps: dropzone.getInputProps,
  };
}
