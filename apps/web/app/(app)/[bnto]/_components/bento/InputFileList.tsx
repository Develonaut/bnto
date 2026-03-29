"use client";

import {
  FileUploadDropzone,
  FileUploadList,
  FileUploadItem,
  FileUploadItemDelete,
  Text,
} from "@bnto/ui";

/** File list with items and add-more dropzone — shown when files are selected. */
export function InputFileList({ files }: { files: File[] }) {
  return (
    <div className="space-y-2">
      <Text size="xs" color="muted">
        {files.length} file{files.length !== 1 ? "s" : ""} selected
      </Text>
      <FileUploadList>
        {files.map((file, i) => (
          <FileUploadItem key={`${file.name}-${i}`} value={file} index={i}>
            <Text size="sm" className="truncate">
              {file.name}
            </Text>
            <FileUploadItemDelete />
          </FileUploadItem>
        ))}
      </FileUploadList>
      <FileUploadDropzone className="gap-2 px-3 py-3">
        <Text size="xs" color="muted">
          Drop more files or click to add
        </Text>
      </FileUploadDropzone>
    </div>
  );
}
