"use client";

import { Card, CardContent, FileUpload, FileUploadDropzone } from "@bnto/ui";
import { DropzoneContent } from "../DropzoneContent";
import { InputFileList } from "./InputFileList";

interface InputCardProps {
  files: File[];
  onFilesChange: (files: File[]) => void;
  acceptLabel: string;
  dropzoneAccept: Record<string, string[]> | undefined;
  disabled: boolean;
}

/** File drop zone compartment — accepts files for processing. */
export function InputCard({
  files,
  onFilesChange,
  acceptLabel,
  dropzoneAccept,
  disabled,
}: InputCardProps) {
  return (
    <Card elevation="sm" className="flex flex-col p-5">
      <CardContent className="flex flex-1 flex-col gap-3 p-0">
        <FileUpload
          value={files}
          onValueChange={onFilesChange}
          accept={dropzoneAccept}
          multiple
          disabled={disabled}
        >
          {files.length === 0 ? (
            <FileUploadDropzone className="flex-1 gap-3 px-4 py-6">
              <DropzoneContent label={acceptLabel} />
            </FileUploadDropzone>
          ) : (
            <InputFileList files={files} />
          )}
        </FileUpload>
      </CardContent>
    </Card>
  );
}
